use std::{result::Result, sync::Arc};
use futures::SinkExt;
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

pub async fn get_sd_card_for_game(
    game_id: String,
) -> Result<Option<MicroSDCard>, Box<dyn std::error::Error>> {
    let result: Option<MicroSDCard> = crate::DB
        .query("SELECT card.* from $id")
        .bind(("id", get_id("game", game_id)))
        .await?
        .take("card")?;
    Ok(result)
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

// pub async fn list_games_on_card(card_id: String) -> Result<Vec<Game>, Box<dyn std::error::Error>> {
//     let result: Vec<Game> = crate::DB
//         .query("SELECT * FROM game WHERE card=$card")
//         .bind(("card", get_id("card", card_id)))
//         .await?
//         .take(0)?;

//     Ok(result)
// }

pub async fn list_games_on_card(card_id: String) -> Result<Vec<Game>, Box<dyn std::error::Error>> {
    let result: Vec<Vec<Option<Game>>> = crate::DB
        .query("SELECT games.*.* FROM $card")
        .bind(("card", get_id("card", card_id)))
        .await?
        .take("games")?;

    Ok(result.iter().flat_map(|f| f.iter().flat_map(|f| f.clone()).collect::<Vec<Game>>()).collect())
}



pub async fn list_cards_with_games() -> Result<Vec<(MicroSDCard,Vec<Game>)>, Box<dyn std::error::Error>> {
    let mut query = crate::DB
        .query("SELECT * FROM card")
        .query("SELECT games.*.* FROM card")
        .await?;

    let card: Vec<Option<MicroSDCard>> = query.take(0)?;
    let games: Vec<Vec<Option<Game>>> = query.take((1, "games"))?;

    if games.len() != card.len() {
        return Error::new_boxed("Games and Cards did not match in count");
    }

    let total = games.len();

    let mut result: Vec<(MicroSDCard, Vec<Game>)> = vec![];

    for i in 0..total {
        let card = card.get(i).unwrap();
        let games = games.get(i).unwrap();

        if let None = card {
            continue;
        }

        let valid_games: Vec<Game> = games.iter().filter_map(|v| v.to_owned()).collect();

        result.push(((card.as_ref()).unwrap().to_owned(), valid_games));
    }

    Ok(result)
}
