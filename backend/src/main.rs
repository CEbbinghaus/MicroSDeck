mod api;
mod ds;
mod dto;
mod env;
mod err;
mod log;
mod sdcard;
mod steam;

use crate::api::config;
use crate::ds::Store;
use crate::dto::{Game, MicroSDCard};
use crate::log::Logger;
use crate::sdcard::is_card_inserted;
use ::log::{info, trace, warn};
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use env::get_data_dir;
use err::Error;
use futures::{pin_mut, select, FutureExt};
use once_cell::sync::Lazy;
use sdcard::get_card_cid;
use simplelog::LevelFilter;
use std::borrow::Borrow;
use std::path::PathBuf;
use std::sync::Arc;
use std::{fs, time::Duration};
use steam::*;
use tokio::time::sleep;

static LOGGER: Lazy<Logger> = Lazy::new(|| Logger::new().expect("Logger to be created"));

const PORT: u16 = 12412; // TODO replace with something unique

const PACKAGE_NAME: &'static str = env!("CARGO_PKG_NAME");
const PACKAGE_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const PACKAGE_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

pub fn init() -> Result<(), ::log::SetLoggerError> {
    ::log::set_logger(&*LOGGER).map(|()| ::log::set_max_level(LevelFilter::Trace))
}

type MainResult = Result<(), Error>;

async fn run_server(datastore: Arc<Store>) -> MainResult {
    // let log_filepath = format!("/tmp/{}.log", PACKAGE_NAME);
    // WriteLogger::init(
    //     #[cfg(debug_assertions)]
    //     {
    //         LevelFilter::Debug
    //     },
    //     #[cfg(not(debug_assertions))]
    //     {
    //         LevelFilter::Info
    //     },
    //     Default::default(),
    //     std::fs::File::create(&log_filepath).unwrap(),
    // )
    // .unwrap();

    info!("Starting HTTP server...");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin()
            .max_age(3600);

        App::new()
            .wrap(cors)
            // .app_data(web::Data::new(api::AppState{datastore: datastore.clone()}))
            .app_data(web::Data::new(datastore.clone()))
            .configure(config)
    })
    .workers(1)
    .bind(("0.0.0.0", PORT))?
    .run()
    .await
    .map_err(|err| err.into())
}

fn read_msd_directory(datastore: &Store) -> MainResult {
    let cid = match get_card_cid() {
        None => {
            warn!("Unable to retrieve CID from MicroSD card");
            return Error::new_res("Unable");
        }
        Some(v) => v,
    };

    let res = match fs::read_to_string("/run/media/mmcblk0p1/libraryfolder.vdf") {
        Ok(value) => value,
        Err(_) => return Error::new_res("Unable to parse library"),
    };

    trace!("Steam MicroSD card detected.");

    let library: LibraryFolder = keyvalues_serde::from_str(res.as_str())?;

    trace!("contentid: {}", library.contentid);

    let files: Vec<_> = fs::read_dir("/run/media/mmcblk0p1/steamapps/")?
        .into_iter()
        .filter_map(Result::ok)
        .filter(|f| f.path().extension().unwrap_or_default().eq("acf"))
        .collect();

    trace!("Found {} Files", files.len());

    let games: Vec<AppState> = files
        .iter()
        .filter_map(|f| fs::read_to_string(f.path()).ok())
        .filter_map(|s| keyvalues_serde::from_str(s.as_str()).ok())
        .collect();

    trace!("Retrieved {} Games", games.len());

    for game in games.iter() {
        trace!("Found App \"{}\"", game.name);
    }

    datastore.add_card(
        cid.clone(),
        MicroSDCard {
            uid: cid.clone(),
            libid: library.contentid.clone(),
            name: library.label,
        },
    );

    // Remove any games that are linked to the card in the database but on the card

    datastore
        .get_games_on_card(&cid)
        .expect("games to be retrieved")
        .iter()
        .filter(|v| !games.iter().any(|g| g.appid == v.uid))
        .for_each(|v| {
            datastore
                .remove_game_from_card(&v.uid, &cid)
                .expect("game and card to be unlinked")
        });

    for game in games.iter() {
        datastore.add_game(
            game.appid.clone(),
            Game {
                uid: game.appid.clone(),
                name: game.name.clone(),
                size: game.size_on_disk,
            },
        );

        datastore.link(&game.appid, &cid).expect("game to be added")
    }

    Ok(())
}

async fn start_watch(datastore: Arc<Store>) -> MainResult {
    loop {
        sleep(Duration::from_secs(5)).await;

        if is_card_inserted() {
            read_msd_directory(datastore.borrow())?;
        }
    }
}

#[tokio::main(worker_threads = 2)]
async fn main() {
    if cfg!(debug_assertions) {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    let store_path = PathBuf::from(&std::env::var("STORE_PATH").unwrap_or(get_data_dir()));
    let store: Arc<Store> =
        Arc::new(Store::read_from_file(store_path.clone()).unwrap_or(Store::new(Some(store_path))));

    info!("Database Started...");

    match init() {
        Err(err) => {
            eprintln!("Unable to Initialize:\n{}", err);
            return;
        }
        Ok(()) => trace!("Initialized..."),
    }

    info!(
        "{}@{} by {}",
        PACKAGE_NAME, PACKAGE_VERSION, PACKAGE_AUTHORS
    );

    info!("Starting Program...");

    let server_future = run_server(store.clone()).fuse();

    let watch_future = start_watch(store.clone()).fuse();

    pin_mut!(server_future, watch_future);

    select! {
        result = server_future => result.expect("Server Exited..."),
        result = watch_future => result.expect("Watch Exited..."),
    };

    info!("Saving Database");
    store.write_to_file().expect("Saving Datatbase to succeed");

    info!("Exiting...");
}
