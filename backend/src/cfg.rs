use anyhow::Result;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{
	fs::{self, File},
	io::Write,
	path::PathBuf,
};
use tracing::Level;

use crate::DATA_DIR;

lazy_static! {
	pub static ref CONFIG_PATH: PathBuf = DATA_DIR.join("config.toml");
	pub static ref CONFIG: Config = Config::load().unwrap_or_else(|| {
		let result = Config::new();
		result.write().expect("Write to succeed");
		result
	});
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Serialize, Deserialize)]
#[serde(remote = "Level")]
pub enum LogLevel {
	TRACE = 0,
	DEBUG = 1,
	INFO = 2,
	WARN = 3,
	ERROR = 4,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
	pub port: u16,
	pub scan_interval: u64,
	pub store_file: PathBuf,
	pub log_file: PathBuf,
	#[serde(with = "LogLevel")]
	pub log_level: Level,
}

impl Config {
	pub fn new() -> Self {
		Config {
			port: 12412,
			scan_interval: 5000,
			log_file: "microsdeck.log".into(),
			store_file: "store".into(),
			log_level: Level::INFO,
		}
	}
	pub fn write(&self) -> Result<()> {
		self.write_to_file(&CONFIG_PATH)
	}
	pub fn write_to_file(&self, path: &'_ PathBuf) -> Result<()> {
		fs::create_dir_all(path.parent().expect("The file to have a parent directory"))?;
		let mut file = File::create(path)?;
		Ok(file.write_all(Self::write_to_str(self)?.as_bytes())?)
	}
	pub fn write_to_str(&self) -> Result<String> {
		Ok(toml::ser::to_string(self)?)
	}

	pub fn load() -> Option<Self> {
		Self::load_from_file(&CONFIG_PATH)
	}
	pub fn load_from_file(path: &'_ PathBuf) -> Option<Self> {
		fs::read_to_string(path)
			.ok()
			.and_then(|val| Self::load_from_str(&val).ok())
	}
	pub fn load_from_str(content: &'_ str) -> Result<Self> {
		Ok(toml::de::from_str::<Self>(content)?)
	}
}
