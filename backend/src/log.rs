use crate::cfg::CONFIG;
use crate::{get_file_path_and_create_directory, LOG_DIR};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

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
		));

	let subscriber = tracing_subscriber::registry().with(file_writer);

	if cfg!(debug_assertions) {
		subscriber
			.with(tracing_subscriber::fmt::layer().pretty())
			.init();
	} else {
		subscriber.init();
	}
}
