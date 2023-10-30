use std::fmt::{Debug, Display};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LibraryFolder {
	pub contentid: String,
	pub label: String,
}

#[derive(Deserialize)]
pub struct AppState {
	pub appid: String,
	pub universe: i32,
	pub name: String,
	#[serde(rename(deserialize = "StateFlags"))]
	pub state_flags: Option<i32>,
	pub installdir: String,
	#[serde(rename(deserialize = "SizeOnDisk"))]
	pub size_on_disk: u64,
}

impl Display for AppState {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {})", self.appid, self.name)
	}
}

impl Debug for AppState {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {})", self.appid, self.name)
	}
}
