use anyhow::Result;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use std::{
	fs::{self, File},
	io::Write,
	path::PathBuf,
};
use tracing::Level;

use crate::{err::Error, CONFIG_PATH};

lazy_static! {
	pub static ref CONFIG: RwLock<Config> = RwLock::new(Config::load().unwrap_or_else(|| {
		let result = Config::new();
		result.write().expect("Write to succeed");
		result
	}));
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

#[derive(Serialize, Deserialize, Default)]
pub struct Startup {
	pub skip_validate: bool,
	pub skip_clean: bool,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Frontend {
	pub dismissed_docs: bool,
}
#[derive(Serialize, Deserialize)]
pub struct Backend {
	pub port: u16,
	pub scan_interval: u64,
	pub store_file: PathBuf,
	pub log_file: PathBuf,
	#[serde(with = "LogLevel")]
	pub log_level: Level,
	pub startup: Startup,
}

impl Default for Backend {
	fn default() -> Self {
		Backend {
			port: 12412,
			scan_interval: 5000,
			log_file: "microsdeck.log".into(),
			store_file: "store".into(),
			log_level: Level::INFO,
			startup: Default::default(),
		}
	}
}

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
	pub backend: Backend,
	pub frontend: Frontend,
}

impl Config {
	pub fn new() -> Self {
		Default::default()
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
		let content = fs::read_to_string(path)
			.ok();

		if let Some(ref content) = content {
			let result = Self::load_from_str(content);

			match result {
				Ok(val) => return Some(val),
				Err(ref err) => eprintln!("Unable to deserialize config: \"{}\"", err),
			}
		} else {
			eprintln!("No content found at config path \"{}\"", path.to_string_lossy());
		}
		None
	}
	pub fn load_from_str(content: &'_ str) -> Result<Self> {
		Ok(toml::de::from_str::<Self>(content)?)
	}
}

// TODO: Turn this Impl into a macro that generates the get and set functions
// Possibly using https://github.com/0xDEADFED5/set_field as the base
impl Config {
	pub fn get_property(&self, name: &'_ str) -> Result<String, Error> {
		let parts: Vec<&str> = name.split(":").collect();

		match parts[..] {
			["*"] => Ok(serde_json::to_string(&self).unwrap()),
			["backend"] => Ok(serde_json::to_string(&self.backend).unwrap()),
			["backend", "port"] => Ok(self.backend.port.to_string()),
			["backend", "scan_interval"] => Ok(self.backend.scan_interval.to_string()),
			["backend", "store_file"] => Ok(self.backend.store_file.to_string_lossy().to_string()),
			["backend", "log_file"] => Ok(self.backend.log_file.to_string_lossy().to_string()),
			["backend", "log_level"] => Ok(self.backend.log_level.to_string()),
			["backend", "startup"] => Ok(serde_json::to_string(&self.backend.startup).unwrap()),
			["backend", "startup", "skip_validate"] => Ok(self.backend.startup.skip_validate.to_string()),
			["backend", "startup", "skip_clean"] => Ok(self.backend.startup.skip_clean.to_string()),
			["frontend"] => Ok(serde_json::to_string(&self.frontend).unwrap()),
			["frontend", "dismissed_docs"] => Ok(self.frontend.dismissed_docs.to_string()),
			_ => Err(Error::from_str("Invalid property Name")),
		}
	}

	pub fn set_property(&mut self, name: &'_ str, value: &'_ str) -> Result<(), Error> {
		let parts: Vec<&str> = name.split(":").collect();

		let wrong_value_err = Error::from_str(&format!("The value provided \"{value}\" did not match the expected type"));

		match parts[..] {
			["*"] => {
				*self = serde_json::from_str(value).map_err(|_| wrong_value_err)?;
			}
			["backend"] => {
				self.backend = serde_json::from_str(value).map_err(|_| wrong_value_err)?;
			}
			["backend", "port"] => {
				self.backend.port = value.parse().map_err(|_| wrong_value_err)?;
			}
			["backend", "scan_interval"] => {
				self.backend.scan_interval = value.parse().map_err(|_| wrong_value_err)?;
			}
			["backend", "store_file"] => {
				self.backend.store_file = value.into();
			}
			["backend", "log_file"] => {
				self.backend.log_file = value.into();
			}
			["backend", "log_level"] => {
				self.backend.log_level = value.parse().map_err(|_| wrong_value_err)?;
			}
			["backend", "startup"] => {
				self.backend.startup = serde_json::from_str(value).map_err(|_| wrong_value_err)?;
			}
			["backend", "startup", "skip_validate"] => {
				self.backend.startup.skip_validate = value.parse().map_err(|_| wrong_value_err)?;
			}
			["backend", "startup", "skip_clean"] => {
				self.backend.startup.skip_clean = value.parse().map_err(|_| wrong_value_err)?;
			}
			["frontend"] => {
				self.frontend = serde_json::from_str(value).map_err(|_| wrong_value_err)?;
			}
			["frontend", "dismissed_docs"] => {
				self.frontend.dismissed_docs = value.parse().map_err(|_| wrong_value_err)?;
			}
			_ => return Err(Error::from_str("Invalid property Name")),
		}

		self.write().map_err(|err| Error::from_str(&err.to_string()))
	}
}