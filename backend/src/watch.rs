use crate::{ds::Store, dto::*, err::Error, sdcard::*, steam::*};
use log::{error, info, trace};
use std::collections::hash_map::DefaultHasher;
use std::fs::DirEntry;
use std::hash::{Hash, Hasher};
use std::{borrow::Borrow, collections::HashMap, fs, sync::Arc, time::Duration};
use tokio::sync::broadcast::Sender;
use tokio::time::interval;

const STEAM_LIB_FILE: &'static str = "/run/media/mmcblk0p1/libraryfolder.vdf";
const STEAM_LIB_FOLDER: &'static str = "/run/media/mmcblk0p1/steamapps/";

fn get_steam_acf_files() -> Result<impl Iterator<Item = DirEntry>, Error> {
    Ok(fs::read_dir(STEAM_LIB_FOLDER)?
        .into_iter()
        .filter_map(Result::ok)
        .filter(|f| f.path().extension().unwrap_or_default().eq("acf")))
}

struct ChangeSet {
    hashes: HashMap<String, u64>,
}

impl ChangeSet {
    pub fn new() -> Self {
        ChangeSet {
            hashes: HashMap::new(),
        }
    }

    pub fn update(&mut self, key: &String, hash: u64) {
        *self.hashes.entry(key.clone()).or_insert(0) = hash;
    }

    pub fn is_changed(&mut self, id: &String) -> Option<u64> {
        let file_metadata: Vec<_> = get_steam_acf_files()
            .ok()?
            .filter_map(|f| fs::metadata(f.path()).ok())
            .collect();

        let mut s = DefaultHasher::new();

        for metadata in file_metadata {
            metadata.len().hash(&mut s);
            metadata
                .modified()
                .expect("Last Modified time to exist")
                .hash(&mut s);
        }

        let hash = s.finish();

        match self.hashes.get(id) {
            // Nothing is present for this card.
            None => Some(hash),
            Some(value) => {
                // Hashes match so we have no updates
                if *value == hash {
                    None
                } else {
                    Some(hash)
                }
            }
        }
    }
}

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

pub async fn start_watch(datastore: Arc<Store>, sender: Sender<()>) -> Result<(), Error> {
    let mut interval = interval(Duration::from_secs(5));

    let mut changeset = ChangeSet::new();

    let mut card_inserted = false;

    loop {
        interval.tick().await;

        // No card no worries.
        if !is_card_inserted() {
            // The card has been removed since the last check
            if card_inserted {
                let _ = sender.send(());
            }

            card_inserted = false;
            continue;
        }

        // was the card inserted since the last check.
        let card_changed = !card_inserted;
        card_inserted = true;

        // There is no steam directory so it hasn't been formatted.
        if !is_card_steam_formatted() {
            continue;
        }

        let cid = match get_card_cid() {
            Some(v) => v,
            None => {
                error!("Unable to read Card ID");
                continue;
            }
        };

        // Do we have changes in the steam directory. This should only occur when something has been added/deleted
        let hash = match changeset.is_changed(&cid) {
            None => {
                // A new card has been inserted but no content on it changed.
                if card_changed {
                    let _ = sender.send(());
                }
                continue;
            }
            Some(v) => v,
        };

        info!("Watcher Detected update");

        // Something went wrong during parsing. Not great
        if let Err(err) = read_msd_directory(datastore.borrow()) {
            error!("Problem reading MicroSD Card: \"{err}\"");
            continue;
        }

        // commit update
        changeset.update(&cid, hash);

        let _ = sender.send(());
    }
}
