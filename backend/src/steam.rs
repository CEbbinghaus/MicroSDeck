use std::collections::HashMap;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct LibraryFolder {
    pub contentid: String,
    pub label: String,
}

#[derive(Deserialize, Debug)]
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