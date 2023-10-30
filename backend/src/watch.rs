use crate::{ds::Store, dto::*, err::Error, sdcard::*, steam::*};
use log::{error, info, trace, debug};
use std::{borrow::Borrow, fs, sync::Arc, time::Duration};
use tokio::sync::broadcast::Sender;
use tokio::time::interval;

fn read_msd_directory(datastore: &Store) -> Result<(), Error> {
	let cid = get_card_cid().ok_or(Error::from_str("Unable to retrieve CID from MicroSD card"))?;
	let res = fs::read_to_string(STEAM_LIB_FILE)?;

	let library: LibraryFolder = keyvalues_serde::from_str(res.as_str())?;

	trace!("contentid: {}", library.contentid);

	let games: Vec<AppState> = get_steam_acf_files()?
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
				name: library.label,
				position: 0,
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
	let mut interval = interval(Duration::from_secs(5));

	let mut card_inserted = false;

	info!("Starting Watcher...");

	loop {
		interval.tick().await;

		debug!("Watch loop");

		// No card no worries.
		if !is_card_inserted() {
			// The card has been removed since the last check
			if card_inserted {
				debug!("Card was removed");
				let _ = sender.send(CardEvent::Removed);
			}

			card_inserted = false;
			continue;
		}

		// was the card inserted since the last check.
		let card_changed = !card_inserted;
		card_inserted = true;

		// There is no steam directory so it hasn't been formatted.
		if !is_card_steam_formatted() {
			debug!("card is not steam formatted");
			continue;
		}

		let cid = match get_card_cid() {
			Some(v) => v,
			None => {
				error!("Unable to read Card ID");
				continue;
			}
		};

		if card_changed {
			let _ = sender.send(CardEvent::Inserted);
		}

		// Do we have changes in the steam directory. This should only occur when something has been added/deleted
		let hash = match datastore.is_hash_changed(&cid) {
			None => continue,
			Some(v) => v,
		};

		info!("Watcher Detected update");

		// Something went wrong during parsing. Not great
		if let Err(err) = read_msd_directory(datastore.borrow()) {
			error!("Problem reading MicroSD Card: \"{err}\"");
			continue;
		}

		// commit update
		datastore.update_hash(&cid, hash);

		let _ = sender.send(CardEvent::Updated);
	}
}
