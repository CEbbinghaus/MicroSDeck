use slotmap::{DefaultKey, SlotMap};
use std::{
    collections::{HashMap, HashSet},
    fs::{read_to_string, write},
    path::PathBuf,
    sync::Mutex,
};

use serde::{Deserialize, Serialize};

use crate::{
    dto::{Game, MicroSDCard},
    err::Error,
};

#[derive(Serialize, Deserialize, Clone)]
enum StoreElement {
    Game(Game),
    Card(MicroSDCard),
}

impl StoreElement {
    fn as_game(&self) -> Option<Game> {
        match self {
            Self::Game(game) => Some(game.clone()),
            _ => None,
        }
    }

    fn as_card(&self) -> Option<MicroSDCard> {
        match self {
            Self::Card(card) => Some(card.clone()),
            _ => None,
        }
    }
}

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
            .or_insert_with(|| self.nodes.insert(Node::from_card(card)));
    }

    pub fn add_game(&mut self, id: String, card: Game) {
        self.node_ids
            .entry(id)
            .or_insert_with(|| self.nodes.insert(Node::from_game(card)));
    }

    pub fn update_card<F>(&mut self, card_id: &str, mut func: F) -> Result<(), Error>
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

    pub fn link(&mut self, a_id: &str, b_id: &str) -> Result<(), Error> {
        let a_key = self.node_ids.get(a_id);
        let b_key = self.node_ids.get(b_id);
        let (a_key, b_key) = a_key
            .zip(b_key)
            .ok_or_else(|| Error::from_str("Either Game or Card could not be found"))?;

        self.nodes[*a_key].links.insert(*b_key);
        self.nodes[*b_key].links.insert(*a_key);

        Ok(())
    }

    pub fn remove_item(&mut self, id: &str) -> Result<(), Error> {
        let element_key = self
            .node_ids
            .remove(id)
            .ok_or_else(|| Error::from_str("Id not present"))?;

        for key in self.nodes.remove(element_key).unwrap().links {
            self.nodes[key].links.remove(&element_key);
        }

        Ok(())
    }

    pub fn remove_game_from_card(&mut self, game_id: &str, card_id: &str) -> Result<(), Error> {
        let game_key = self.node_ids.get(game_id);
        let card_key = self.node_ids.get(card_id);
        let (game_key, card_key) = game_key
            .zip(card_key)
            .ok_or_else(|| Error::from_str("Either Game or Card could not be found"))?;

        self.nodes[*game_key].links.remove(card_key);
        self.nodes[*card_key].links.remove(game_key);

        Ok(())
    }

    pub fn get_card(&self, card_id: &str) -> Result<MicroSDCard, Error> {
        self.node_ids
            .get(card_id)
            .map_or(Error::new_res("Card Id not present"), |key| {
                Ok(self.nodes[*key]
                    .element
                    .as_card()
                    .expect("Expected card but game was returned"))
            })
    }

    pub fn get_game(&self, game_id: &str) -> Result<Game, Error> {
        self.node_ids
            .get(game_id)
            .map_or(Error::new_res("Game Id not present"), |key| {
                Ok(self.nodes[*key]
                    .element
                    .as_game()
                    .expect("Expected game but card was returned"))
            })
    }

    pub fn get_games_on_card(&self, card_id: &str) -> Result<Vec<Game>, Error> {
        let card_key = self
            .node_ids
            .get(card_id)
            .ok_or_else(|| Error::from_str("Card Id not present"))?;

        let games = self.nodes[*card_key]
            .links
            .iter()
            .filter_map(|game_key| self.nodes[*game_key].element.as_game())
            .collect();

        Ok(games)
    }

    pub fn get_cards_for_game(&self, game_id: &str) -> Result<Vec<MicroSDCard>, Error> {
        let game_key = self
            .node_ids
            .get(game_id)
            .ok_or_else(|| Error::from_str("Game Id not present"))?;

        let cards = self.nodes[*game_key]
            .links
            .iter()
            .filter_map(|game_key| self.nodes[*game_key].element.as_card())
            .collect();

        Ok(cards)
    }

    pub fn list_cards(&self) -> Vec<MicroSDCard> {
        self.nodes
            .iter()
            .filter_map(|node| node.1.element.as_card())
            .collect()
    }

    pub fn list_games(&self) -> Vec<Game> {
        self.nodes
            .iter()
            .filter_map(|node| node.1.element.as_game())
            .collect()
    }

    pub fn list_cards_with_games(&self) -> Vec<(MicroSDCard, Vec<Game>)> {
        self.nodes
            .iter()
            .filter_map(|node| {
                node.1.element.as_card().map(|v| {
                    (v, {
                        node.1
                            .links
                            .iter()
                            .filter_map(|key: &DefaultKey| self.nodes[*key].element.as_game())
                            .collect()
                    })
                })
            })
            .collect()
    }
}

pub struct Store {
    data: Mutex<StoreData>,
    file: Option<PathBuf>,
}

impl Store {
    pub fn new(file: Option<PathBuf>) -> Self {
        Store {
            data: Mutex::new(StoreData {
                nodes: SlotMap::new(),
                node_ids: HashMap::new(),
            }),
            file,
        }
    }

    pub fn read_from_file(file: PathBuf) -> Result<Self, Error> {
        let contents = read_to_string(&file).map_err(|e| Error::from(e))?;
        let data: StoreData = serde_json::from_str(&contents).map_err(|e| Error::from(e))?;
        Ok(Store {
            data: Mutex::new(data),
            file: Some(file),
        })
    }

    #[allow(dead_code)]
    pub fn set_file(&mut self, file: PathBuf) {
        self.file = Some(file);
    }

    pub fn write_to_file(&self) -> Result<(), Error> {
        write(
            self.file
                .as_ref()
                .ok_or(Error::from_str("No Path specified"))?,
            serde_json::to_string(&self.data)?,
        )?;
        Ok(())
    }

    pub fn add_card(&self, id: String, card: MicroSDCard) {
        self.data.lock().unwrap().add_card(id, card)
    }

    pub fn add_game(&self, id: String, card: Game) {
        self.data.lock().unwrap().add_game(id, card)
    }

    pub fn update_card<F>(&self, card_id: &str, func: F) -> Result<(), Error>
    where
        F: FnMut(&mut MicroSDCard),
    {
        self.data.lock().unwrap().update_card(card_id, func)
    }

    pub fn link(&self, a_id: &str, b_id: &str) -> Result<(), Error> {
        self.data.lock().unwrap().link(a_id, b_id)
    }

    pub fn remove_element(&self, game_id: &str) -> Result<(), Error> {
        self.data.lock().unwrap().remove_item(game_id)
    }

    pub fn remove_game_from_card(&self, game_id: &str, card_id: &str) -> Result<(), Error> {
        self.data
            .lock()
            .unwrap()
            .remove_game_from_card(game_id, card_id)
    }

    pub fn get_card(&self, card_id: &str) -> Result<MicroSDCard, Error> {
        self.data.lock().unwrap().get_card(card_id)
    }

    pub fn get_game(&self, game_id: &str) -> Result<Game, Error> {
        self.data.lock().unwrap().get_game(game_id)
    }

    pub fn get_games_on_card(&self, card_id: &str) -> Result<Vec<Game>, Error> {
        self.data.lock().unwrap().get_games_on_card(card_id)
    }

    pub fn get_cards_for_game(&self, game_id: &str) -> Result<Vec<MicroSDCard>, Error> {
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
