use std::sync::Arc;

use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;


#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct Name {
    pub name: String
}

impl From<&str> for Name {
    fn from(value: &str) -> Self {
        return Name { name: value.to_string() }
    }
}

impl From<String> for Name {
    fn from(value: String) -> Self {
        return Name { name: value.to_string() }
    }
}


#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct MicroSDCard {
    pub uid: u64,
    pub name: String,
    pub games: Vec<Thing>
}

#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct Game {
    pub uid: u64,
    pub name: String,
    pub size: u64,
    pub card: Thing
}