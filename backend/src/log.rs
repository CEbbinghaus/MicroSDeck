use chrono;
use log::{Level, Metadata, Record};
use std::env;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::str::FromStr;

use crate::env::{get_file_path_and_create_directory, get_log_dir};
use crate::err::Error;

pub struct Logger {
	file: File,
	max_level: Level,
}

impl Logger {
	pub fn to_file(&self) -> &File {
		&self.file
	}

	pub fn new() -> Option<Logger> {
		let file_path = get_file_path_and_create_directory("backend.log", &get_log_dir)
			.expect("to retrieve the log file path");

		let file = OpenOptions::new()
			.write(true)
			.append(true)
			.create(true)
			.open(&file_path)
			.ok()?;

		let max_level = env::var("LOG_LEVEL")
			.map_err(Error::from)
			.and_then(|v| Level::from_str(&v).map_err(Error::from))
			.unwrap_or({
				if cfg!(debug_assertions) {
					Level::Debug
				} else {
					Level::Info
				}
			});

		println!("Logging enabled to {file_path} with level {max_level}");

		Some(Logger { file, max_level })
	}
}

impl log::Log for Logger {
	fn enabled(&self, metadata: &Metadata) -> bool {
		metadata.level() <= self.max_level //  && metadata.target() != "tracing::span"
	}

	fn log(&self, record: &Record) {
		if self.enabled(record.metadata()) {
			let current_time = chrono::offset::Local::now();

			println!(
				"{} {}: {}",
				current_time.format("%H:%M:%S"),
				record.level(),
				record.args()
			);

			let message = format!(
				"{} {} @ {}:{} {} \"{}\"",
				current_time.naive_utc(),
				record.level(),
				record.file().unwrap_or("UNKNOWN"),
				record.line().unwrap_or(0),
				record.metadata().target(),
				record.args()
			);

			if let Err(e) = writeln!(self.to_file(), "{message}") {
				eprintln!("Couldn't write to file: {}", e);
			}
		}
	}

	fn flush(&self) {}
}
