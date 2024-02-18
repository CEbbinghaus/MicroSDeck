use crate::cfg::CONFIG;
use crate::{ds::Store, dto::*, err::Error, sdcard::*, steam::*};
use std::borrow::Borrow;
use std::path::{Path, PathBuf};
use std::{fs, sync::Arc, time::Duration};
use tokio::sync::broadcast::Sender;
use tokio::time::interval;
use tracing::{debug, error, info, span, trace, warn};

fn read_msd_directory(datastore: &Store, mount: &Option<String>) -> Result<(), Error> {
	let cid = get_card_cid().ok_or(Error::from_str("Unable to retrieve CID from MicroSD card"))?;

	let library: LibraryFolder = keyvalues_serde::from_str(&read_libraryfolder(mount)?)?;

	debug!(
		?library,
		"Read & deserialized library from {}", LIBRARY_FOLDER_FILE
	);

	let games: Vec<AppState> = get_steam_acf_files(mount)?
		.filter_map(|f| match fs::read_to_string(f.path()) {
			Ok(value) => Some(value),
			Err(err) => {
				error!(%err, path=?f.path(), "Unable to read Steam ACF file {:?}", f.path());
				None
			}
		})
		.filter_map(|s| match keyvalues_serde::from_str(s.as_str()) {
			Ok(value) => Some(value),
			Err(err) => {
				error!(%err, contents=s.as_str(), "Unable to Deserialize Steam ACF file");
				None
			}
		})
		.collect();

	debug!(
		game_count = games.len(),
		?games,
		"Retrieved {} Games from acf files",
		games.len()
	);

	if !datastore.contains_element(&cid) {
		debug!(cid, "No MicroSD card found, creating new card");

		datastore.add_card(
			cid.clone(),
			MicroSDCard {
				uid: cid.clone(),
				libid: library.contentid.clone(),
				mount: mount.clone(),
				name: library.label,
				position: u32::MAX,
				hidden: false,
			},
		);
	}

	// Remove any games that are linked to the card in the database but on the card
	let current_games = datastore.get_games_on_card(&cid)?;
	debug!(
		?current_games,
		"Retrieved {} Games from database",
		current_games.len()
	);
	for deleted_game in current_games
		.iter()
		.filter(|v| v.is_steam && !games.iter().any(|g| g.appid == v.uid))
	{
		debug!(game = ?deleted_game, cid, "Game was removed from MicroSD card");
		datastore.unlink(&deleted_game.uid, &cid)?
	}

	for game in games.iter() {
		if !datastore.contains_element(&game.appid) {
			debug!(?game, "Game not found in database. Adding game");
			datastore.add_game(
				game.appid.clone(),
				Game {
					uid: game.appid.clone(),
					name: game.name.clone(),
					size: game.size_on_disk,
					is_steam: true,
				},
			);
		}

		debug!(?game, cid, "Linking game to MicroSD card");
		datastore.link(&game.appid, &cid).expect("game to be added")
	}

	Ok(())
}

pub async fn start_watch(datastore: Arc<Store>, sender: Sender<CardEvent>) -> Result<(), Error> {
	let mut interval = interval(Duration::from_millis(CONFIG.scan_interval));

	let mut card_inserted = false;

	info!("Starting Watcher...");

	let mut mount: Option<String> = None;

	loop {
		interval.tick().await;

		let _ = span!(tracing::Level::INFO, "watch cycle", mount).entered();

		// No card no worries.
		if !is_card_inserted() {
			// The card has been removed since the last check
			if card_inserted {
				debug!("Card was removed");
				trace!("Sending Removed event");
				let _ = sender.send(CardEvent::Removed);
			}
			card_inserted = false;
			mount = None;

			continue;
		}

		if !card_inserted {
			debug!("Card was inserted");
			trace!("Sending Inserted event");
			let _ = sender.send(CardEvent::Inserted);
			mount = None;
		}

		card_inserted = true;

		let cid = match get_card_cid() {
			Some(v) => {
				trace!(card_id = v, "{}", v);
				v
			}
			None => {
				error!("Unable to read Card ID");
				continue;
			}
		};

		// If we have a mount point and it does not resolve to the library folder, we need to determine the mount point
		if !has_libraryfolder(&mount) {
			debug!(
				mount = mount.clone().unwrap_or(DEFAULT_MOUNT.into()),
				"could not find library folder under existing mount",
			);
			debug!("trying to automatically determine mount point");

			if mount.is_none() {
				// Try and retrieve the mount from the database
				if let Ok(card) = datastore.get_card(&cid) {
					if card.mount.is_some() {
						debug!(
							mount = card.mount,
							"MicroSD card had preexisting mount saved. Reusing that."
						);
					}
					mount = card.mount
				}
			}

			// Whatever we loaded did not work.
			if mount.is_some() && !has_libraryfolder(&mount) {
				warn!(
					mount = mount,
					"loaded mount does not resolve library. Resetting mount"
				);
				mount = None;
			}

			if mount.is_none() {
				trace!("No mount found. Trying to determine mount point");

				for entry in Path::new("/dev/disk/by-label")
					.read_dir()?
					.filter_map(|dir| dir.ok())
				{
					trace!(path = ?entry.path().canonicalize()?, "testing label for mount point of MicroSD Card");
					if entry.path().canonicalize()? == PathBuf::from("/dev/mmcblk0p1") {
						let label = entry.file_name();
						info!(label = ?label, "Found MicroSD Card label");
						mount = Some(label.to_string_lossy().to_string());
					}
				}
			}

			debug!(mount = mount, "Updating card's mount point");
			let _ = datastore.update_card(&cid, |card| {
				card.mount = mount.clone();
				Ok(())
			});

			continue;
		}

		// Do we have changes in the steam directory. This should only occur when something has been added/deleted
		let hash = match datastore.is_hash_changed(&cid, &mount) {
			None => {
				debug!("No hash found. Skipping iteration");
				continue;
			}
			Some(v) => v,
		};

		info!(hash = hash, "Watcher Detected update");

		// Something went wrong during parsing. Not great
		if let Err(err) = read_msd_directory(datastore.borrow(), &mount) {
			error!(%err, "Failed to read MicroSD card library data, Reason: \"{}\"", err);
			continue;
		}

		// commit update
		trace!(hash, "Updating hash in database");
		datastore.update_hash(&cid, hash);

		trace!("Sending Updated event");
		let _ = sender.send(CardEvent::Updated);
	}
}
