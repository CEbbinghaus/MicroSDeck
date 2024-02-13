use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::cfg::CONFIG;
use crate::{get_file_path_and_create_directory, LOG_DIR};

pub fn create_subscriber() {
	let log_file_path = get_file_path_and_create_directory(&CONFIG.log_file, &LOG_DIR).expect("Log file to be created");

	let file = std::fs::OpenOptions::new()
		.create(true)
		.append(true)
		.open(log_file_path)
		.expect("Log file to be created");

	let file_writer = tracing_subscriber::fmt::layer()
		.json()
		.with_writer(file);

	let subscriber = tracing_subscriber::registry()
		.with(file_writer);
		
	if cfg!(debug_assertions) {
		subscriber.with(tracing_subscriber::fmt::layer().pretty()).init();
	} else {
		subscriber.init();
	}
}
