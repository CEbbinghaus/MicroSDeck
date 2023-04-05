#![allow(unused_imports)]

mod db;
mod dbo;
mod err;
mod api;
mod steam;

use futures::StreamExt;
use surrealdb::engine::local::{Db,Mem,File};
use surrealdb::Surreal;
use std::env;
use std::{fs, time::Duration};
use tokio_udev::*;
use steam::*;

// Creates a new static instance of the client
static DB: Surreal<Db> = Surreal::init();

use simplelog::{LevelFilter, WriteLogger};

use usdpl_back::{core::serdes::Primitive, Instance};

const PORT: u16 = 54321; // TODO replace with something unique

const PACKAGE_NAME: &'static str = env!("CARGO_PKG_NAME");
const PACKAGE_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const PACKAGE_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

#[tokio::main]
async fn run_server() -> Result<(), ()> {
    let log_filepath = format!("/tmp/{}.log", PACKAGE_NAME);
    WriteLogger::init(
        #[cfg(debug_assertions)]
        {
            LevelFilter::Debug
        },
        #[cfg(not(debug_assertions))]
        {
            LevelFilter::Info
        },
        Default::default(),
        std::fs::File::create(&log_filepath).unwrap(),
    )
    .unwrap();

    println!("Starting backend...");

    Instance::new(PORT)
        .register("hello", |_: Vec<Primitive>| {
            vec![format!("Hello {}", PACKAGE_NAME).into()]
        })
        .register("ping", |_: Vec<Primitive>| {
            vec!["pong".into()]
        })
        .register_async("list_all_games", crate::api::get_games::GetGames::new())
        .run()
        .await
}


#[tokio::main]
async fn run_monitor() -> Result<(), Box<dyn Send+Sync+std::error::Error>> {
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

            let result: LibraryFolder = keyvalues_serde::from_str(res.as_str())?;

            println!("contentid: {}", result.contentid);

            let mut files = fs::read_dir("/run/media/mmcblk0p1/steamapps/")?
                .into_iter()
                .filter_map(Result::ok)
                .filter(|f| f.path().extension().unwrap_or_default().eq("acf"));

            while let Some(file) = files.next() {
                let appstr = fs::read_to_string(file.path())?;
                let manifest: AppState = keyvalues_serde::from_str(appstr.as_str())?;

                println!("Found App \"{}\"", manifest.name);
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn setup_db() {
    // let ds = Datastore::new("/var/etc/Database.file").await?;

    match DB.connect::<Mem>(()).await {
        Err(_) => panic!("Unable to construct Database"),
        Ok(_) => {
            DB.use_ns("").use_db("").await.expect("Unable to select Namespace and Database");
            db::setup_test_data().await.expect("Test data to be set up");
        }
    }
}

pub fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    println!("{}@{} by {}", PACKAGE_NAME, PACKAGE_VERSION, PACKAGE_AUTHORS);
    println!("Starting Program...");

    setup_db();

    println!("Database Started...");

    let handle1 = std::thread::spawn(move || run_server());

    let handle2 = std::thread::spawn(move || run_monitor());

    while !handle1.is_finished() && !handle2.is_finished() {
        std::thread::sleep(Duration::from_millis(1));
    }

    println!("Exiting...");
}