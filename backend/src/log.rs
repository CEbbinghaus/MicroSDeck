use log::{Level, Metadata, Record};
use std::fs::{OpenOptions, File};
use std::io::prelude::*;
use chrono;

use crate::env::{get_file_path, get_file_path_and_create_directory};

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
            .open(get_file_path_and_create_directory("backend.log").expect("The log file to exist."))
            .map(|f| Logger(f))
            .ok()
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let message = format!("{:?} {} \"{}\"", chrono::offset::Local::now(), record.level(), record.args());

            println!("{message}");

            // let mut file = OpenOptions::new()
            //     .write(true)
            //     .append(true)
            //     .create(true)
            //     .open(get_file_path_and_create_directory("backend.log").expect("The log file to exist."))
            //     .unwrap();

            if let Err(e) = writeln!(self.to_file(), "{message}") {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
    }

    fn flush(&self) {}
}
