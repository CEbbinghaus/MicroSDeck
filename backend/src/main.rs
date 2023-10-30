mod api;
mod ds;
mod dto;
mod env;
mod err;
mod log;
mod sdcard;
mod steam;
mod watch;

use crate::{api::config, dto::CardEvent};
use crate::ds::Store;
use crate::env::*;
use crate::log::Logger;
use crate::watch::start_watch;
use ::log::{debug, error, info};
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
use tokio::sync::broadcast::{self, Sender};

static LOGGER: Lazy<Logger> = Lazy::new(|| Logger::new().expect("Logger to be created"));

pub fn init() -> Result<(), ::log::SetLoggerError> {
	::log::set_logger(&*LOGGER).map(|()| ::log::set_max_level(LevelFilter::Trace))
}

type MainResult = Result<(), Error>;

async fn run_server(datastore: Arc<Store>, sender: Sender<CardEvent>) -> MainResult {

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
			.app_data(web::Data::new(sender.clone()))
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

	match init() {
		Err(err) => {
			error!("Unable to Initialize:\n{}", err);
			return;
		}
		Ok(()) => debug!("Initialized..."),
	}
	
	info!(
		"{}@{} by {}",
		PACKAGE_NAME, PACKAGE_VERSION, PACKAGE_AUTHORS
	);

	let store_path = PathBuf::from(
		&std::env::var("STORE_PATH").unwrap_or(
			get_file_path_and_create_directory("store", &get_data_dir)
				.expect("should retrieve data directory"),
		),
	);

	debug!("Loading from store {:?}", store_path);
	let store: Arc<Store> =
		Arc::new(Store::read_from_file(store_path.clone()).unwrap_or(Store::new(Some(store_path))));

	store.clean_up();

	if !store.validate() {
		error!("Validity of the data is not guaranteed. Cannot run backend...");
		exit(1);
	}

	info!("Database Started...");
	info!("Starting Program...");

	let (txtx, _) = broadcast::channel::<CardEvent>(1);

	let server_future = run_server(store.clone(), txtx.clone()).fuse();

	let watch_future = start_watch(store.clone(), txtx.clone()).fuse();

	pin_mut!(server_future, watch_future);

	select! {
		result = server_future => match result {
			Ok(_) => info!("Server ran to completion..."),
			Err(err) => error!("Server exited with error: {err}")
		},
		result = watch_future => match result {
			Ok(_) => info!("Watch ran to completion.."),
			Err(err) => error!("Watch exited with error: {err}"),
		},
	};

	info!("Saving Database");
	if let Err(err) = store.write_to_file() {
		error!("Failed to write datastore to file {err}");
	}

	info!("Exiting...");
}
