use std::{result::Result, sync::Arc};
use futures::SinkExt;
use serde::Deserialize;
use surrealdb::sql::{Id, Thing};

use crate::{dbo::*, err::Error};

pub fn get_id(table: &str, id: String) -> Thing {
    Thing::from((table.to_string(), Id::String(id)))
}

pub async fn add_game(id: String, game: &Game) -> Result<(), surrealdb::Error> {
    let _game: Game = crate::DB.create(("game", id)).content(game).await?;
    Ok(())
}

pub async fn add_sd_card(id: String, card: &MicroSDCard) -> Result<(), surrealdb::Error> {
    let _card: MicroSDCard = crate::DB.create(("card", id)).content(card).await?;
    Ok(())
}

pub async fn update_sd_card_name(id: String, name: Name) -> Result<(), surrealdb::Error> {
    let _: Name = crate::DB.update(("card", id)).merge(name).await?;
    Ok(())
}

pub async fn get_game(id: String) -> Result<Option<Game>, Box<dyn std::error::Error>> {
    Ok(crate::DB.select(("game", id)).await?)
}

pub async fn get_card(id: String) -> Result<Option<MicroSDCard>, Box<dyn std::error::Error>> {
    Ok(crate::DB.select(("card", id)).await?)
}

pub async fn list_games() -> Result<Vec<Game>, Box<dyn std::error::Error>> {
    Ok(crate::DB.select("game").await?)
}

pub async fn list_cards() -> Result<Vec<MicroSDCard>, Box<dyn std::error::Error>> {
    Ok(crate::DB.select("card").await?)
}

pub async fn add_game_to_card(game_id: String, card_id: String) -> Result<(), Box<surrealdb::Error>>{
    // We delete the record first to make sure we are never adding a duplicate
    let _ = crate::DB
        .query("DELETE contains WHERE in=$card AND out=$game; RELATE $card->contains->$game;")
        .bind(("card", get_id("card", card_id)))
        .bind(("game", get_id("game", game_id)))
        .await?;

    Ok(())
}

pub async fn remove_game_from_card(game_id: String, card_id: String) -> Result<(), Box<surrealdb::Error>>{
    let _ = crate::DB
        .query("DELETE contains WHERE in=$card AND out=$game;")
        .bind(("card", get_id("card", card_id)))
        .bind(("game", get_id("game", game_id)))
        .await?;

    Ok(())
}

pub async fn remove_game(game_id: String) -> Result<(), Box<surrealdb::Error>> {
    let _ = crate::DB
        .query("DELETE contains WHERE out=$game; DELETE $game;")
        .bind(("game", get_id("game", game_id)))
        .await?;

    Ok(())
}

pub async fn remove_card(card_id: String) -> Result<(), Box<surrealdb::Error>> {
    let _ = crate::DB
        .query("DELETE contains WHERE in=$card; DELETE $card;")
        .bind(("card", get_id("card", card_id)))
        .await?;

    Ok(())
}

pub async fn get_games_on_card(card_id: String) -> Result<Vec<Game>, Box<surrealdb::Error>> {
    let result: Vec<Vec<Option<Game>>> = crate::DB
        .query("SELECT ->contains->game.* as games FROM $card")
        .bind(("card", get_id("card", card_id)))
        .await?
        .take("games")?;

    Ok(result.iter().flat_map(|f| f.iter().filter_map(|v| v.to_owned())).collect())
}

pub async fn get_cards_for_game(game_id: String) -> Result<Vec<MicroSDCard>, Box<surrealdb::Error>> {
    let result:  Vec<Vec<Option<MicroSDCard>>> = crate::DB
        .query("SELECT <-contains<-card.* as cards FROM $game;")
        .bind(("game", get_id("game", game_id)))
        .await?
        .take("cards")?;

    Ok(result.iter().flat_map(|f| f.iter().filter_map(|v| v.to_owned())).collect())
}

pub async fn get_cards_with_games() -> Result<Vec<(MicroSDCard,Vec<Game>)>, Box<dyn std::error::Error>> {
    let mut response = crate::DB
        .query("select ->contains->game.* as games, (SELECT * FROM $parent.id)[0] as card FROM card;")
        .await?;

    let card: Vec<MicroSDCard> = response.take((0, "card"))?;
    let games: Vec<Vec<Game>> = response.take((0, "games"))?;

    if games.len() != card.len() {
        println!("Response: {:#?}", response);
        return Error::new_boxed(format!("Games and Cards did not match in count. Games: {}, Cards: {}", games.len(), card.len()).as_str());
    }

    Ok(card.iter().map(|f| f.to_owned()).zip(games.iter().map(|f| f.to_owned())).collect())
}