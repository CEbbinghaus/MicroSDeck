use std::collections::HashMap;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct LibraryFolder {
    pub contentid: u64,
    pub label: String,
}

#[derive(Deserialize, Debug)]
pub struct AppState {
    pub appid: String,
    pub universe: i32,
    pub name: String,
    pub stateflags: Option<i32>,
    pub installdir: String,
    pub last_updated: u64,
    pub size_on_disk: u64,
    pub staging_size: u64,
    pub buildid: u64,
    pub last_owner: u64,
    pub auto_update_behavior: u64,
    pub allow_other_downloads_while_running: u64,
    pub scheduled_auto_update: u64,
    pub installed_depots: HashMap<String, Depot>,
}

#[derive(Deserialize, Debug)]
pub struct Depot {
    pub manifest: String,
    pub size: u64,
    pub dlcappid: Option<u64>,
}
