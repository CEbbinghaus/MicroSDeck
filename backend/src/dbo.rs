use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct MicroSDCard {
    pub uid: u64,
    pub name: String
}

unsafe impl Send for MicroSDCard {}
unsafe impl Sync for MicroSDCard {}

#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct Game {
    pub uid: u64,
    pub name: String,
    pub size: u64,
    pub card: Option<Arc<MicroSDCard>>
}

unsafe impl Send for Game {}
unsafe impl Sync for Game {}