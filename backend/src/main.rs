mod api;
mod ds;
mod dto;
mod env;
mod err;
mod log;
mod watch;
mod sdcard;
mod steam;

use crate::api::config;
use crate::ds::Store;
use crate::env::get_file_path_and_create_directory;
use crate::log::Logger;
use crate::watch::start_watch;
use ::log::{info, trace, error};
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use env::get_data_dir;
use err::Error;
use futures::{pin_mut, select, FutureExt};
use once_cell::sync::Lazy;
use simplelog::LevelFilter;
use std::path::PathBuf;
use std::process::exit;
use std::sync::Arc;

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


#[tokio::main(worker_threads = 1)]
async fn main() {
    if cfg!(debug_assertions) {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    let store_path = PathBuf::from(
        &std::env::var("STORE_PATH").unwrap_or(
            get_file_path_and_create_directory("store", &get_data_dir)
                .expect("should retrieve data directory"),
        ),
    );

	println!("Loading from store \"{:?}\"", store_path);
    let store: Arc<Store> =
        Arc::new(Store::read_from_file(store_path.clone()).unwrap_or(Store::new(Some(store_path))));

	store.clean_up();

	if !store.validate() {
		error!("Validity of the data is not guaranteed. Cannot run backend...");
		exit(1);
	}

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
