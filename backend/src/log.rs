use std::fs::File;

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;
use crate::cfg::CONFIG;
use crate::{get_file_path_and_create_directory, LOG_DIR};

pub fn create_subscriber() {
	let log_file_path = get_file_path_and_create_directory(&CONFIG.log_file, &LOG_DIR).expect("Log file to be created");
	let file = File::create(log_file_path).expect("Log file to be created");
	let mut file_writer = tracing_subscriber::fmt::layer().with_writer(file);

	file_writer.set_ansi(false);

	let subscriber = tracing_subscriber::registry()
		.with(
			file_writer.with_filter(tracing_subscriber::filter::LevelFilter::from_level(CONFIG.log_level))
		);
		
	if cfg!(debug_assertions) {
		subscriber.with(tracing_subscriber::fmt::layer().pretty()).init();
	} else {
		subscriber.init();
	}
}
