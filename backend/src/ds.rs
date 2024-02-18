use crate::{
	dto::{Game, MicroSDCard},
	env::PACKAGE_VERSION,
	err::Error,
	sdcard::get_steam_acf_files,
};
use semver::Version;
use serde::{Deserialize, Serialize};
use slotmap::{DefaultKey, SlotMap};
use std::{
	borrow::BorrowMut,
	collections::{hash_map::DefaultHasher, HashMap, HashSet},
	fs::{self, read_to_string, write},
	hash::{Hash, Hasher},
	path::PathBuf,
	sync::RwLock,
};
use tracing::{error, instrument};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) enum StoreElement {
	Game(Game),
	Card(MicroSDCard),
}

impl StoreElement {
	pub fn as_game(&self) -> Option<Game> {
		match self {
			Self::Game(game) => Some(game.clone()),
			_ => None,
		}
	}

	pub fn as_card(&self) -> Option<MicroSDCard> {
		match self {
			Self::Card(card) => Some(card.clone()),
			_ => None,
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
struct Node {
	pub(crate) element: StoreElement,
	pub(crate) links: HashSet<DefaultKey>,
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

fn default_version() -> Version {
	Version::parse(PACKAGE_VERSION).unwrap()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StoreData {
	#[serde(default = "default_version")]
	version: Version,
	nodes: SlotMap<DefaultKey, Node>,
	node_ids: HashMap<String, DefaultKey>,
	#[serde(default)]
	hashes: HashMap<String, u64>,
}

impl StoreData {
	#[instrument(skip(self))]
	pub fn add_card(&mut self, id: String, card: MicroSDCard) {
		self.node_ids
			.entry(id)
			.or_insert_with(|| self.nodes.insert(Node::from_card(card)));
	}

	#[instrument(skip(self))]
	pub fn add_game(&mut self, id: String, game: Game) {
		self.node_ids
			.entry(id)
			.or_insert_with(|| self.nodes.insert(Node::from_game(game)));
	}

	#[instrument(skip(self, func))]
	pub fn update_card<F>(&mut self, card_id: &str, mut func: F) -> Result<(), Error>
	where
		F: FnMut(&mut MicroSDCard) -> Result<(), Error>,
	{
		let node = self
			.node_ids
			.get(card_id)
			.ok_or(Error::from_str("Card Id not present"))?;

		match self.nodes.get_mut(*node).unwrap().element {
			StoreElement::Card(ref mut card) => {
				func(card)?;
			}
			StoreElement::Game(_) => return Err(Error::from_str("Expected Card, got Game")),
		}

		Ok(())
	}

	#[instrument(skip(self))]
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

	#[instrument(skip(self))]
	pub fn unlink(&mut self, a_id: &str, b_id: &str) -> Result<(), Error> {
		let game_key = self.node_ids.get(a_id);
		let card_key = self.node_ids.get(b_id);
		let (game_key, card_key) = game_key
			.zip(card_key)
			.ok_or_else(|| Error::from_str("Either Game or Card could not be found"))?;

		self.nodes[*game_key].links.remove(card_key);
		self.nodes[*card_key].links.remove(game_key);

		Ok(())
	}

	#[instrument(skip(self))]
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

	#[instrument(skip(self))]
	pub fn contains_element(&self, card_id: &str) -> bool {
		self.node_ids.contains_key(card_id)
	}

	#[instrument(skip(self))]
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

	#[instrument(skip(self))]
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

	#[instrument(skip(self))]
	pub fn get_card_and_games(&self, card_id: &str) -> Result<(MicroSDCard, Vec<Game>), Error> {
		let card_key = self
			.node_ids
			.get(card_id)
			.ok_or_else(|| Error::from_str("Card Id not present"))?;

		let node = &self.nodes[*card_key];

		let card = node
			.element
			.as_card()
			.ok_or(Error::from_str("Element was not a card"))?;
		let games = node
			.links
			.iter()
			.filter_map(|game_key| self.nodes[*game_key].element.as_game())
			.collect();

		Ok((card, games))
	}

	#[instrument(skip(self))]
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

	#[instrument(skip(self))]
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

	#[instrument(skip(self))]
	pub fn list_cards(&self) -> Vec<MicroSDCard> {
		self.nodes
			.iter()
			.filter_map(|node| node.1.element.as_card())
			.collect()
	}

	#[instrument(skip(self))]
	pub fn list_games(&self) -> Vec<Game> {
		self.nodes
			.iter()
			.filter_map(|node| node.1.element.as_game())
			.collect()
	}

	#[instrument(skip(self))]
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

impl StoreData {
	pub fn delete_hash(&mut self, key: &str) {
		self.hashes.remove(key);
	}

	pub fn update_hash(&mut self, key: &str, hash: u64) {
		*self.hashes.entry(key.to_string()).or_insert(0) = hash;
	}

	pub fn is_hash_changed(&self, id: &'_ str, mount: &Option<String>) -> Option<u64> {
		let file_metadata: Vec<_> = get_steam_acf_files(mount)
			.ok()?
			.filter_map(|f| fs::metadata(f.path()).ok())
			.collect();

		let mut s = DefaultHasher::new();

		mount.hash(&mut s);

		for metadata in file_metadata {
			metadata.len().hash(&mut s);
			metadata
				.modified()
				.expect("Last Modified time to exist")
				.hash(&mut s);
		}

		let hash = s.finish();

		match self.hashes.get(id) {
			// Nothing is present for this card.
			None => Some(hash),
			Some(value) => {
				// Hashes match so we have no updates
				if *value == hash {
					None
				} else {
					Some(hash)
				}
			}
		}
	}
}

#[derive(Debug)]
pub struct Store {
	data: RwLock<StoreData>,
	file: Option<PathBuf>,
}

impl Store {
	pub fn new(file: Option<PathBuf>) -> Self {
		Store {
			data: RwLock::new(StoreData {
				version: Version::parse(PACKAGE_VERSION).unwrap(),
				nodes: SlotMap::new(),
				node_ids: HashMap::new(),
				hashes: HashMap::new(),
			}),
			file,
		}
	}

	pub fn read_from_file(file: PathBuf) -> Result<Self, Error> {
		let contents = read_to_string(&file).map_err(Error::from)?;
		let store_data: StoreData = serde_json::from_str(&contents).map_err(Error::from)?;
		Ok(Store {
			data: RwLock::new(store_data),
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

	fn try_write_to_file(&self) {
		if self.file.is_none() {
			return;
		}

		if let Err(err) = self.write_to_file() {
			error!(%err, "Unable to write datastore to file \"{}\"", err);
		}
	}

	pub fn validate(&self) -> bool {
		let data = self.data.read().unwrap();

		let mut result = true;

		{
			let mut dead_node_ids: Vec<(&String, &DefaultKey)> = vec![];

			for pair in &data.node_ids {
				if !data.nodes.contains_key(*pair.1) {
					dead_node_ids.push(pair);
				}
			}

			if !dead_node_ids.is_empty() {
				result &= false;
				error!(?dead_node_ids, "Found dead node_ids");
			}
		}

		result
	}

	/// Removes any whitespace from the card uid
	pub fn clean_up(&self) {
		let mut data = self.data.write().unwrap();

		let cleaned_node_ids: HashMap<String, DefaultKey> = data
			.node_ids
			.iter()
			.map(|f| (f.0.trim().to_string(), *f.1))
			.collect();

		data.node_ids = cleaned_node_ids;

		for node in data.nodes.borrow_mut() {
			match node.1.element {
				StoreElement::Card(ref mut card) => {
					card.uid = card.uid.trim().to_string();
				}
				StoreElement::Game(_) => {}
			}
		}
	}

	pub fn add_card(&self, id: String, card: MicroSDCard) {
		self.data.write().unwrap().add_card(id, card);
		self.try_write_to_file()
	}

	pub fn add_game(&self, id: String, game: Game) {
		self.data.write().unwrap().add_game(id, game);
		self.try_write_to_file()
	}

	pub fn update_card<F>(&self, card_id: &str, func: F) -> Result<(), Error>
	where
		F: FnMut(&mut MicroSDCard) -> Result<(), Error>,
	{
		self.data.write().unwrap().update_card(card_id, func)?;
		self.try_write_to_file();
		Ok(())
	}

	pub fn link(&self, a_id: &str, b_id: &str) -> Result<(), Error> {
		self.data.write().unwrap().link(a_id, b_id)?;
		self.try_write_to_file();
		Ok(())
	}

	pub fn unlink(&self, a_id: &str, b_id: &str) -> Result<(), Error> {
		self.data.write().unwrap().unlink(a_id, b_id)?;
		self.try_write_to_file();
		Ok(())
	}

	pub fn remove_element(&self, id: &str) -> Result<(), Error> {
		// these two operations have to happen within a single lock otherwise the try_write_to_file causes a deadlock
		{
			let mut lock = self.data.write().unwrap();
			lock.remove_item(id)?;
			lock.delete_hash(id);
		}
		self.try_write_to_file();
		Ok(())
	}

	pub fn contains_element(&self, id: &str) -> bool {
		self.data.read().unwrap().contains_element(id)
	}

	pub fn get_card(&self, card_id: &str) -> Result<MicroSDCard, Error> {
		self.data.read().unwrap().get_card(card_id)
	}

	pub fn get_game(&self, game_id: &str) -> Result<Game, Error> {
		self.data.read().unwrap().get_game(game_id)
	}

	pub fn get_card_and_games(&self, card_id: &str) -> Result<(MicroSDCard, Vec<Game>), Error> {
		self.data.read().unwrap().get_card_and_games(card_id)
	}

	pub fn get_games_on_card(&self, card_id: &str) -> Result<Vec<Game>, Error> {
		self.data.read().unwrap().get_games_on_card(card_id)
	}

	pub fn get_cards_for_game(&self, game_id: &str) -> Result<Vec<MicroSDCard>, Error> {
		self.data.read().unwrap().get_cards_for_game(game_id)
	}

	pub fn list_cards(&self) -> Vec<MicroSDCard> {
		self.data.read().unwrap().list_cards()
	}

	pub fn list_games(&self) -> Vec<Game> {
		self.data.read().unwrap().list_games()
	}

	pub fn list_cards_with_games(&self) -> Vec<(MicroSDCard, Vec<Game>)> {
		self.data.read().unwrap().list_cards_with_games()
	}

	pub fn is_hash_changed(&self, key: &str, mount: &Option<String>) -> Option<u64> {
		self.data.read().unwrap().is_hash_changed(key, mount)
	}

	pub fn update_hash(&self, key: &str, hash: u64) {
		self.data.write().unwrap().update_hash(key, hash)
	}
}
