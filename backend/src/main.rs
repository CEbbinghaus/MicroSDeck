#![allow(unused_imports)]

mod api;
mod db;
mod dbo;
mod err;
mod steam;

use futures::executor::block_on;
use futures::future::join;
use futures::{Future, StreamExt};
use std::env;
use std::{fs, time::Duration};
use steam::*;
use surrealdb::engine::local::{Db, File, Mem};
use surrealdb::Surreal;
use tokio_udev::*;

// Creates a new static instance of the client
static DB: Surreal<Db> = Surreal::init();

use simplelog::{LevelFilter, WriteLogger};

use usdpl_back::{core::serdes::Primitive, Instance};

use crate::dbo::{Game, MicroSDCard};

const PORT: u16 = 54321; // TODO replace with something unique

const PACKAGE_NAME: &'static str = env!("CARGO_PKG_NAME");
const PACKAGE_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const PACKAGE_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

// #[tokio::main]
async fn run_server() -> Result<(), ()> {
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

    println!("Starting backend...");

    Instance::new(PORT)
        .register("hello", |_: Vec<Primitive>| {
            vec![format!("Hello {}", PACKAGE_NAME).into()]
        })
        .register("ping", |_: Vec<Primitive>| vec!["pong".into()])
        .register_async("list_games", crate::api::list_games::ListGames::new())
        .register_async("list_cards", crate::api::list_cards::ListCards::new())
        .register_async(
            "list_games_on_card",
            crate::api::list_games_on_card::ListGamesOnCard::new(),
        )
        .register_async(
            "get_card_for_game",
            crate::api::get_card_for_game::GetCardForGame::new(),
        )
        .register_async(
            "set_name_for_card",
            crate::api::set_name_for_card::SetNameForCard::new(),
        )
        .register_async(
            "list_cards_with_games",
            crate::api::list_cards_with_games::ListCardsWithGames::new(),
        )
        .run()
        .await
}

// #[tokio::main]
async fn run_monitor() -> Result<(), Box<dyn Send + Sync + std::error::Error>> {
    let monitor = MonitorBuilder::new()?.match_subsystem("mmc")?;

    let mut socket = AsyncMonitorSocket::new(monitor.listen()?)?;

    println!("Now listening for Device Events...");
    while let Some(Ok(event)) = socket.next().await {
        if event.event_type() != EventType::Bind {
            continue;
        }

        println!(
            "Device {} was Bound",
            event.devpath().to_str().unwrap_or("UNKNOWN")
        );

        if let Ok(res) = fs::read_to_string("/run/media/mmcblk0p1/libraryfolder.vdf") {
            println!("Steam MicroSD card detected.");

            let library: LibraryFolder = keyvalues_serde::from_str(res.as_str())?;

            println!("contentid: {}", library.contentid);

            let files: Vec<_> = fs::read_dir("/run/media/mmcblk0p1/steamapps/")?
                .into_iter()
                .filter_map(Result::ok)
                .filter(|f| f.path().extension().unwrap_or_default().eq("acf"))
                .collect();

            println!("Found {} Files", files.len());

            let games: Vec<AppState> = files
                .iter()
                .filter_map(|f| fs::read_to_string(f.path()).ok())
                .filter_map(|s| keyvalues_serde::from_str(s.as_str()).ok())
                .collect();

            println!("Retrieved {} Games", games.len());

            for game in games.iter() {
                println!("Found App \"{}\"", game.name);
            }

            if let Ok(None) = db::get_card(library.contentid.clone()).await {
                db::add_sd_card(&MicroSDCard {
                    uid: library.contentid.clone(),
                    name: library.label,
                    games: games.iter().map(|v| db::get_id("game", v.appid.clone())).collect(),
                })
                .await?;
            }

            for game in games.iter() {
                if let Ok(None) = db::get_game(game.appid.clone()).await {
                    db::add_game(&Game {
                        uid: game.appid.clone(),
                        name: game.name.clone(),
                        size: game.size_on_disk,
                        card: db::get_id("card", library.contentid.clone()),
                    })
                    .await?
                }
            }
        }
    }
    Ok(())
}

async fn setup_db() {
    // let ds = Datastore::new("/var/etc/Database.file").await?;
    // match DB.connect::<Mem>(()).await {
    match DB.connect::<File>("/tmp/dbDebug.tmp").await {
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
    env::set_var("RUST_BACKTRACE", "1");

    println!(
        "{}@{} by {}",
        PACKAGE_NAME, PACKAGE_VERSION, PACKAGE_AUTHORS
    );
    println!("Starting Program...");

    setup_db().await;

    // db::setup_test_data().await.expect("Test data to be set up"); 

    println!("{:?}", db::list_games().await);

    println!("Database Started...");

    let server_future = run_server();

    let monitor_future = run_monitor();

    // let web_server = ;

    let (server_res, monitor_ress) = join(server_future, monitor_future).await;

    if server_res.is_err() || monitor_ress.is_err() {
        println!("There was an error.");
    }
    // while !handle1.is_finished() && !handle2.is_finished() {
    //     std::thread::sleep(Duration::from_millis(1));
    // }

    println!("Exiting...");
}
