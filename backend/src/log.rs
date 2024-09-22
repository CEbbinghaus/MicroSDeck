use crate::cfg::CONFIG;
use crate::{get_file_path_and_create_directory, LOG_DIR};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{filter, Layer};

const IGNORED_MODULES: [&'static str; 6] = [
	"actix_http::h1::decoder",
	"actix_http::h1::dispatcher",
	"actix_http::h1::timer",
	"actix_server::signals",
	"actix_server::worker",
	"mio::poll",
];

pub fn create_subscriber() {
	let log_file_path = get_file_path_and_create_directory(&CONFIG.log_file, &LOG_DIR)
		.expect("Log file to be created");

	let file = std::fs::OpenOptions::new()
		.create(true)
		.append(true)
		.open(log_file_path)
		.expect("Log file to be created");

	let file_writer = tracing_subscriber::fmt::layer()
		.json()
		.with_writer(file)
		.with_filter(tracing_subscriber::filter::LevelFilter::from_level(
			CONFIG.log_level,
		))
		.with_filter(filter::filter_fn(|metadata| {
			metadata
				.module_path()
				.is_some_and(|module| !IGNORED_MODULES.contains(&module))
		}));

	let subscriber = tracing_subscriber::registry().with(file_writer);

	if cfg!(debug_assertions) {
		subscriber
			.with(
				tracing_subscriber::fmt::layer()
					.pretty()
					.with_filter(tracing_subscriber::filter::LevelFilter::from_level(
						CONFIG.log_level,
					))
					.with_filter(filter::filter_fn(|metadata| {
						metadata
							.module_path()
							.is_some_and(|module| !IGNORED_MODULES.contains(&module))
					})),
			)
			.init();
	} else {
		subscriber.init();
	}
}
