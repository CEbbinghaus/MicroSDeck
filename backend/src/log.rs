use log::{Level, Metadata, Record};
use std::fs::{OpenOptions, File};
use std::io::prelude::*;
use chrono;

use crate::env::{get_file_path, get_file_path_and_create_directory, get_log_dir};

pub struct Logger(File);

impl Logger {
    pub fn to_file(&self) -> &File {
        &self.0
    }

    pub fn new() -> Option<Logger> {
        OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(get_file_path_and_create_directory("backend.log", &get_log_dir).expect("The log file to exist."))
            .map(|f| Logger(f))
            .ok()
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info && metadata.target() != "tracing::span"
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            
            let current_time = chrono::offset::Local::now();

            println!("{} {}: {}",
                record.level(),
                current_time.format("%d/%m/%Y %H:%M:%S"),
                record.args());

            let message = format!("{} {} @ {}:{} {} \"{}\"", current_time.naive_utc(), record.level(), record.file().unwrap_or("UNKNOWN"), record.line().unwrap_or(0), record.metadata().target(), record.args());

            if let Err(e) = writeln!(self.to_file(), "{message}") {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
    }

    fn flush(&self) {}
}
