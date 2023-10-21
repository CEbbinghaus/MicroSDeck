use slotmap::{DefaultKey, SlotMap};
use std::{
    collections::{HashMap, HashSet},
    fs::{read_to_string, write},
    hash::*,
    path::PathBuf,
    sync::Mutex,
};
// use petgraph::*;
// use petgraph::prelude::GraphMap;
use serde::{Deserialize, Serialize};

use crate::{
    dbo::{Game, MicroSDCard},
    err::Error,
};

#[derive(Serialize, Deserialize, Clone)]
enum StoreElement {
    Game(Game),
    Card(MicroSDCard),
}

impl StoreElement {
    fn is_game(self) -> bool {
        match self {
            Self::Game(_) => true,
            _ => false,
        }
    }

    fn as_game(self) -> Option<Game> {
        match self {
            Self::Game(game) => Some(game.clone()),
            _ => None,
        }
    }

    fn is_card(self) -> bool {
        match self {
            Self::Card(_) => true,
            _ => false,
        }
    }

    fn as_card(self) -> Option<MicroSDCard> {
        match self {
            Self::Card(card) => Some(card.clone()),
            _ => None,
        }
    }
}

// #[derive(Serialize, Deserialize)]
// struct Store {
//     graph: GraphMap<u64, (), Undirected>,
//     elements: HashMap<u64, StoreElement>
// }

// impl Store {
//     pub fn new() -> Self {
//         Store {
//             graph: GraphMap::new(),
//             elements: HashMap::new()
//         }
//     }

//     pub fn from_file(file: &PathBuf) -> Result<Self, Error>{
//         let contents = read_to_string(file).map_err(|e| Error::from(e))?;
//         serde_json::from_str(&contents).map_err(|e| Error::from(e))
//     }
// }

// fn calculate_hash<T: Hash>(t: &T) -> u64 {
//     let mut s = std::collections::hash_map::DefaultHasher::new();
//     t.hash(&mut s);
//     s.finish()
// }

#[derive(Serialize, Deserialize)]
struct Node {
    element: StoreElement,
    links: HashSet<DefaultKey>,
}

impl Node {
    pub fn from_card(card: MicroSDCard) -> Self {
        Node {
            element: StoreElement::Card(card),
            links: HashSet::new(),
        }
    }
    pub fn from_game(game: Game) -> Self {
        Node {
            element: StoreElement::Game(game),
            links: HashSet::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct StoreData {
    nodes: SlotMap<DefaultKey, Node>,
    node_ids: HashMap<String, DefaultKey>,
}


impl StoreData {
    pub fn add_card(&mut self, id: String, card: MicroSDCard) {
        self.node_ids
            .entry(id)
            .or_insert(self.nodes.insert(Node::from_card(card)));
    }

    pub fn add_game(&mut self, id: String, card: Game) {
        let entry = self.node_ids.entry(id);

        entry.or_insert(self.nodes.insert(Node::from_game(card)));
    }

    pub fn update_card<F>(&mut self, card_id: &String, mut func: F) -> Result<(), Error>
    where
        F: FnMut(&mut MicroSDCard),
    {


        let node = self
            .node_ids
            .get(card_id)
            .ok_or(Error::Error("Card Id not present".into()))?;

        match self.nodes.get_mut(*node).unwrap().element {
            StoreElement::Card(ref mut card) => {
                func(card);
            }
            StoreElement::Game(_) => return Err(Error::Error("Expected Card, got Game".into())),
        }

        Ok(())
    }

    pub fn add_game_to_card(&mut self, game_id: &String, card_id: &String) -> Result<(), Error> {
        let (game_key, card_key) = match (self.node_ids.get(game_id), self.node_ids.get(card_id)) {
            (Some(game_key), Some(card_key)) => (game_key, card_key),
            _ => return Error::new("Either Game or Card could not be found"),
        };

        self.nodes[*game_key].links.insert(*card_key);
        self.nodes[*card_key].links.insert(*game_key);

        Ok(())
    }

    pub fn remove_game(&mut self, game_id: &String) -> Result<(), Error> {
        let game_key = match self.node_ids.get(game_id) {
            None => return Err(Error::Error("Game Id not present".into())),
            Some(key) => key,
        };

        // remove all links pointing to this game.
        for key in self.nodes[*game_key].links.clone() {
            self.nodes[key].links.remove(&key);
        }

        self.nodes.remove(*game_key);

        self.node_ids.remove(game_id);

        Ok(())
    }

    pub fn remove_card(&mut self, card_id: &String) -> Result<(), Error> {
        let card_key = match self.node_ids.get(card_id) {
            None => return Err(Error::Error("Card Id not present".into())),
            Some(key) => key,
        };

        // for key in &self.nodes[*card_key].links {
        //     self.nodes[*key].links.remove(key);
        // }

        self.nodes[*card_key].links.clone().iter().for_each(|key| { self.nodes[*key].links.remove(&key); });

        self.nodes.remove(*card_key);

        self.node_ids.remove(card_id);

        Ok(())
    }

    pub fn remove_game_from_card(
        &mut self,
        game_id: &String,
        card_id: &String,
    ) -> Result<(), Error> {

        let (game_key, card_key) = match (self.node_ids.get(game_id), self.node_ids.get(card_id)) {
            (Some(game_key), Some(card_key)) => (game_key, card_key),
            _ => return Error::new("Either Game or Card could not be found"),
        };

        self.nodes[*game_key].links.remove(card_key);
        self.nodes[*card_key].links.remove(game_key);

        Ok(())
    }

    pub fn get_card(&self, card_id: &String) -> Result<MicroSDCard, Error> {
        self.node_ids
            .get(card_id)
            .map_or(Error::new("Card Id not present"), |key| {
                Ok(self.nodes[*key]
                    .element
                    .clone()
                    .as_card()
                    .expect("Expected card but game was returned"))
            })
    }

    pub fn get_game(&self, game_id: &String) -> Result<Game, Error> {
        self.node_ids
            .get(game_id)
            .map_or(Error::new("Game Id not present"), |key| {
                Ok(self.nodes[*key]
                    .element
                    .clone()
                    .as_game()
                    .expect("Expected game but card was returned"))
            })
    }

    pub fn get_games_on_card(&self, card_id: &String) -> Result<Vec<Game>, Error> {
        match self.node_ids.get(card_id) {
            None => Error::new("Card Id not present"),
            Some(card_key) => Ok(self.nodes[*card_key]
                .links
                .iter()
                .filter_map(|game_key| self.nodes[*game_key].element.clone().as_game())
                .collect()),
        }
    }

    pub fn get_cards_for_game(&self, game_id: &String) -> Result<Vec<MicroSDCard>, Error> {
        match self.node_ids.get(game_id) {
            None => Error::new("Game Id not present"),
            Some(game_key) => Ok(self.nodes[*game_key]
                .links
                .iter()
                .filter_map(|game_key| {
                    self.nodes
                        .get(*game_key)
                        .expect("element to exist")
                        .element
                        .clone()
                        .as_card()
                })
                .collect()),
        }
    }

    pub fn list_cards(&self) -> Vec<MicroSDCard> {
        self.nodes
            .iter()
            .filter_map(|node| node.1.element.clone().as_card())
            .collect()
    }

    pub fn list_games(&self) -> Vec<Game> {
        self.nodes
            .iter()
            .filter_map(|node| node.1.element.clone().as_game())
            .collect()
    }

    pub fn list_cards_with_games(&self) -> Vec<(MicroSDCard, Vec<Game>)> {
        self.nodes
            .iter()
            .filter_map(|node| {
                node.1.element.clone().as_card().map(|v| {
                    (v, {
                        node.1
                            .links
                            .iter()
                            .filter_map(|key: &DefaultKey| self.nodes[*key].element.clone().as_game())
                            .collect()
                    })
                })
            })
            .collect()
    }
}


// #[derive(Serialize, Deserialize)]
pub struct Store {
    data: Mutex<StoreData>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            data: Mutex::new(StoreData {
                nodes: SlotMap::new(),
                node_ids: HashMap::new(),
            }),
        }
    }

    pub fn read_from_file(file: &PathBuf) -> Result<Self, Error> {
        let contents = read_to_string(file).map_err(|e| Error::from(e))?;
        let data: StoreData = serde_json::from_str(&contents).map_err(|e| Error::from(e))?;
        Ok(Store {
            data: Mutex::new(data)
        })
    }

    pub fn write_to_file(&self, file: &PathBuf) -> Result<(), Error> {
        write(file, serde_json::to_string(&self.data)?)?;
        Ok(())
    }

    pub fn add_card(&self, id: String, card: MicroSDCard) {
        self.data.lock().unwrap().add_card(id, card)
    }

    pub fn add_game(&self, id: String, card: Game) {
        self.data.lock().unwrap().add_game(id, card)
    }

    pub fn update_card<F>(&self, card_id: &String, func: F) -> Result<(), Error>
    where
        F: FnMut(&mut MicroSDCard),
    {
        self.data.lock().unwrap().update_card(card_id, func)
    }

    pub fn add_game_to_card(&self, game_id: &String, card_id: &String) -> Result<(), Error> {
        self.data.lock().unwrap().add_game_to_card(game_id, card_id)
    }

    pub fn remove_game(&self, game_id: &String) -> Result<(), Error> {
        self.data.lock().unwrap().remove_game(game_id)
    }

    pub fn remove_card(&self, card_id: &String) -> Result<(), Error> {
        self.data.lock().unwrap().remove_card(card_id)
    }

    pub fn remove_game_from_card(
        &self,
        game_id: &String,
        card_id: &String,
    ) -> Result<(), Error> {
        self.data.lock().unwrap().remove_game_from_card(game_id, card_id)
    }

    pub fn get_card(&self, card_id: &String) -> Result<MicroSDCard, Error> {
        self.data.lock().unwrap().get_card(card_id)
    }

    pub fn get_game(&self, game_id: &String) -> Result<Game, Error> {
        self.data.lock().unwrap().get_game(game_id)
    }

    pub fn get_games_on_card(&self, card_id: &String) -> Result<Vec<Game>, Error> {
        self.data.lock().unwrap().get_games_on_card(card_id)
    }

    pub fn get_cards_for_game(&self, game_id: &String) -> Result<Vec<MicroSDCard>, Error> {
        self.data.lock().unwrap().get_cards_for_game(game_id)
    }

    pub fn list_cards(&self) -> Vec<MicroSDCard> {
        self.data.lock().unwrap().list_cards()
    }

    pub fn list_games(&self) -> Vec<Game> {
        self.data.lock().unwrap().list_games()
    }

    pub fn list_cards_with_games(&self) -> Vec<(MicroSDCard, Vec<Game>)> {
        self.data.lock().unwrap().list_cards_with_games()
    }
}