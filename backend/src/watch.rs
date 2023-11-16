use crate::{ds::Store, dto::*, err::Error, sdcard::*, steam::*};
use log::{debug, error, info, trace};
use std::borrow::Borrow;
use std::path::{Path, PathBuf};
use std::{fs, sync::Arc, time::Duration};
use tokio::sync::broadcast::Sender;
use tokio::time::interval;

fn read_msd_directory(datastore: &Store, mount: &Option<String>) -> Result<(), Error> {
	let cid = get_card_cid().ok_or(Error::from_str("Unable to retrieve CID from MicroSD card"))?;

	let library: LibraryFolder = keyvalues_serde::from_str(&read_libraryfolder(mount)?)?;

	trace!("contentid: {}", library.contentid);

	let games: Vec<AppState> = get_steam_acf_files(mount)?
		.filter_map(|f| fs::read_to_string(f.path()).ok())
		.filter_map(|s| keyvalues_serde::from_str(s.as_str()).ok())
		.collect();

	trace!("Retrieved {} Games: {:?}", games.len(), games);

	if !datastore.contains_element(&cid) {
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
	for deleted_game in current_games
		.iter()
		.filter(|v| !games.iter().any(|g| g.appid == v.uid))
	{
		datastore.unlink(&deleted_game.uid, &cid)?
	}

	for game in games.iter() {
		if !datastore.contains_element(&game.appid) {
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

		datastore.link(&game.appid, &cid).expect("game to be added")
	}

	Ok(())
}

pub async fn start_watch(datastore: Arc<Store>, sender: Sender<CardEvent>) -> Result<(), Error> {
	let mut interval = interval(Duration::from_secs(1));

	let mut card_inserted = false;

	info!("Starting Watcher...");

	let mut mount: Option<String> = None;

	loop {
		interval.tick().await;

		// No card no worries.
		if !is_card_inserted() {
			// The card has been removed since the last check
			if card_inserted {
				debug!("Card was removed");
				let _ = sender.send(CardEvent::Removed);
			}
			card_inserted = false;
			mount = None;

			continue;
		}

		if !card_inserted {
			debug!("Card was inserted");
			let _ = sender.send(CardEvent::Inserted);
			mount = None;
		}

		card_inserted = true;

		let cid = match get_card_cid() {
			Some(v) => v,
			None => {
				error!("Unable to read Card ID");
				continue;
			}
		};

		if !has_libraryfolder(&mount) {
			debug!(
				"could not find library folder under mount {}",
				mount.clone().unwrap_or("mmcblk0".into())
			);
			debug!("trying to automatically determine label");

			if mount == None {
				if let Some(card) = datastore.get_card(&cid).ok() {
					if card.mount != None {
						debug!("MicroSD card had preexisting mount saved. Reusing that.");
					}
					mount = card.mount
				}
			}

			// Whatever we loaded did not work.
			if mount != None && !has_libraryfolder(&mount) {
				debug!("mount {mount:?} does not resolve library. Removing it");				
				mount = None;
			}

			if mount == None {
				for entry in Path::new("/dev/disk/by-label")
					.read_dir()?
					.filter_map(|dir| dir.ok())
				{
					if entry.path().canonicalize()? == PathBuf::from("/dev/mmcblk0p1") {
						let label = entry.file_name();
						info!("Found MicroSD Card label {label:?}");
						mount = Some(label.to_string_lossy().to_string());
					}
				}
			}

			let _ = datastore.update_card(&cid, |card| {
				card.mount = mount.clone();
				Ok(())
			});

			continue;
		}

		// Do we have changes in the steam directory. This should only occur when something has been added/deleted
		let hash = match datastore.is_hash_changed(&cid, &mount) {
			None => continue,
			Some(v) => v,
		};

		info!("Watcher Detected update");

		// Something went wrong during parsing. Not great
		if let Err(err) = read_msd_directory(datastore.borrow(), &mount) {
			error!("Problem reading MicroSD Card: \"{err}\"");
			continue;
		}

		// commit update
		datastore.update_hash(&cid, hash);

		let _ = sender.send(CardEvent::Updated);
	}
}
