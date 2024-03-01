use std::fmt::{Debug, Display};

use serde::Deserialize;
use serde_alias::serde_alias;

#[derive(Deserialize, Debug)]
pub struct LibraryFolder {
	pub contentid: String,
	pub label: String,
}

#[serde_alias(CamelCase, PascalCase, LowerCase, SnakeCase)]
#[derive(Deserialize)]
pub struct AppState {
	pub appid: String,
	pub universe: i32,
	pub name: String,
	pub state_flags: Option<i32>,
	pub installdir: String,
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
