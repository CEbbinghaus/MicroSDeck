use log::{Level, Metadata, Record};
use std::fs::OpenOptions;
use std::io::prelude::*;
use chrono;

use crate::env::{get_file_path, get_file_path_and_create_directory};

pub struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let message = format!("{:?} {} \"{}\"", chrono::offset::Local::now(), record.level(), record.args());

            println!("{message}");

            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(get_file_path_and_create_directory("backend.log").expect("The log file to exist."))
                .unwrap();

            if let Err(e) = writeln!(file, "{message}") {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
    }

    fn flush(&self) {}
}
