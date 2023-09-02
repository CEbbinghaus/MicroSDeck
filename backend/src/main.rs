#![allow(unused_imports)]
// #![feature(const_deref)]

mod api;
mod db;
mod dbo;
mod env;
mod err;
mod log;
mod sdcard;
mod steam;

use crate::db::{add_game_to_card, get_cards_with_games, get_games_on_card, remove_game_from_card};
use crate::log::Logger;
use crate::sdcard::is_card_inserted;
use actix_cors::Cors;
use actix_web::{HttpServer, App, web};
use env::{get_file_path_and_create_directory, get_data_dir};
use futures::{select, FutureExt, pin_mut};
use ::log::{info, trace, warn};
use futures::executor::block_on;
use futures::{join, future};
use futures::{Future, StreamExt};
use once_cell::sync::Lazy;
use sdcard::get_card_cid;
use tokio::time::{Sleep, sleep};
use std::borrow::Borrow;
use std::fs::{read, OpenOptions};
use std::ops::Deref;
use std::path::Path;
use std::vec;
use std::{fs, time::Duration};
use steam::*;
use surrealdb::engine::local::{Db, File, Mem};
use surrealdb::Surreal;
use tokio_udev::*;

// Creates a new static instance of the client
static DB: Lazy<Surreal<Db>> = Lazy::new(Surreal::init);

use simplelog::{LevelFilter, WriteLogger};

// use usdpl_back::{core::serdes::Primitive, Instance};

use crate::dbo::{Game, MicroSDCard};

static LOGGER: Lazy<Logger> = Lazy::new(|| Logger::new().expect("Logger to be created"));

const PORT: u16 = 12412; // TODO replace with something unique

const PACKAGE_NAME: &'static str = env!("CARGO_PKG_NAME");
const PACKAGE_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const PACKAGE_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

pub fn init() -> Result<(), ::log::SetLoggerError> {
    ::log::set_logger(&*LOGGER).map(|()| ::log::set_max_level(LevelFilter::Trace))
}

type MainResult = Result<(), Box<dyn Send + Sync + std::error::Error>>;

// #[actix_web::main]
async fn run_server() -> MainResult {
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

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(crate::api::list_cards)
            .service(crate::api::list_games)
            .service(crate::api::list_games_on_card)
            .service(crate::api::list_cards_with_games)
            .service(crate::api::set_name_for_card)
            .service(crate::api::get_card_for_game)
    })
    .bind(("0.0.0.0", PORT))?
    .run()
    .await
    .map_err(|err| err.into())
}

async fn read_msd_directory() -> MainResult {
    let cid = match get_card_cid() {
        None => {
            warn!("Unable to retrieve CID from MicroSD card");
            return Ok(());
        }
        Some(v) => v,
    };

    if let Ok(res) = fs::read_to_string("/run/media/mmcblk0p1/libraryfolder.vdf") {
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

        match db::get_card(cid.clone()).await {
            Ok(None) => {
                db::add_sd_card(
                    cid.clone(),
                    &MicroSDCard {
                        uid: cid.clone(),
                        libid: library.contentid.clone(),
                        name: library.label,
                    },
                )
                .await?;
                info!("Wrote MicroSD card {} to Database", library.contentid);
            }
            Ok(Some(_)) => trace!("MicroSD card {} already in Database", library.contentid),
            Err(err) => warn!(
                "Unable to write card {} to Database:\n{}",
                library.contentid, err
            ),
        }

        futures::future::try_join_all(
            get_games_on_card(cid.clone()).await?
                .iter()
                .filter(|v| !games.iter().any(|g| g.appid == v.uid))
                .map(|v| remove_game_from_card(v.uid.clone(), cid.clone())),
            )
        .await?;

        for game in games.iter() {
            match db::get_game(game.appid.clone()).await {
                Ok(None) => {
                    db::add_game(
                        game.appid.clone(),
                        &Game {
                            uid: game.appid.clone(),
                            name: game.name.clone(),
                            size: game.size_on_disk,
                        },
                    )
                    .await?;
                    info!(
                        "Wrote Game {} with id {} to Database",
                        game.name, game.appid
                    );
                }
                Ok(Some(_)) => trace!(
                    "Game {} with id {} already in Database",
                    game.name, game.appid
                ),
                Err(err) => warn!(
                    "Unable to write Game {} with id {} to Database:\n{}",
                    game.name, game.appid, err
                ),
            }

            add_game_to_card(game.appid.clone(), cid.clone())
                .await
                .unwrap_or_else(|err| panic!("Query to work {:#?}", err));
        }
    }

    Ok(())
}

async fn run_monitor() -> MainResult {
    let monitor = MonitorBuilder::new()?.match_subsystem("mmc")?;

    let mut socket = AsyncMonitorSocket::new(monitor.listen()?)?;

    info!("Now listening for Device Events...");
    while let Some(Ok(event)) = socket.next().await {
        if event.event_type() != EventType::Bind {
            continue;
        }

        info!(
            "Device {} was Bound",
            event.devpath().to_str().unwrap_or("UNKNOWN")
        );

        read_msd_directory().await?;
    }
    Ok(())
}

async fn start_watch() -> MainResult {
    loop {
        sleep(Duration::from_secs(5)).await;

        if is_card_inserted() {
            read_msd_directory().await?;
        }

        
    }
}

async fn setup_db() {
    // let ds = Datastore::new("/var/etc/Database.file").await?;
    // match DB.connect::<Mem>(()).await {

    // let file = match std::env::var("DECKY_PLUGIN_RUNTIME_DIR") {
    //     Err(_) => {
    //         if cfg!(debug_assertions) {
    //             Path::new("/tmp").join("MicroSDeck").join("data.db")
    //         } else {
    //             panic!("Unable to proceed");
    //         }
    //     }
    //     Ok(loc) => Path::new(loc.as_str()).join("data.db"),
    // };

    let file = get_file_path_and_create_directory("data.db", &get_data_dir).expect("Data Directory to exist");

    match DB.connect::<File>(file.as_str()).await {
        Err(_) => panic!("Unable to construct Database"),
        Ok(_) => {
            DB.use_ns("")
                .use_db("")
                .await
                .expect("Namespace and Database to be avaliable");
        }
    }
}

#[tokio::main]
async fn main() {
    if cfg!(debug_assertions) {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    match init() {
        Err(err) => {
            eprintln!("Unable to Initialize:\n{}", err);
            return;
        }
        Ok(()) => trace!("Initialized"),
    }

    info!(
        "{}@{} by {}",
        PACKAGE_NAME, PACKAGE_VERSION, PACKAGE_AUTHORS
    );

    info!("Starting Program...");

    setup_db().await;

    // if is_card_inserted() {
    //     // Try reading the directory when we launch the app. That way we ensure that if a car is currently inserted we still detect it
    //     let _ = read_msd_directory().await;
    // }

    info!("Database Started...");

    let server_future = run_server().fuse();

    let monitor_future = run_monitor().fuse();

    let watch_future = start_watch().fuse();

    pin_mut!(server_future, monitor_future, watch_future);

    select! {
        result = server_future => result.expect("Server Exited..."),
        result = monitor_future => result.expect("Monitor Exited..."),
        result = watch_future => result.expect("Watch Exited..."),
    };

    info!("Exiting...");
}
