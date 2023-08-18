use std::{result::Result, sync::Arc};
use actix_web::ResponseError;
use futures::SinkExt;
use serde::Deserialize;
use surrealdb::sql::{Id, Thing};

use crate::{dbo::*, err::Error};

type DynError = Box<dyn Send + Sync + std::error::Error>;
// #[derive(Debug)]
// struct DynError(Box<dyn Send + Sync + std::error::Error>);

// impl std::fmt::Display for DynError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }

// impl ResponseError for DynError {
//     fn status_code(&self) -> actix_web::http::StatusCode {
//         actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
//     }

//     fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
//         let mut res = actix_web::HttpResponse::new(self.status_code());
//         res.set_body(actix_web::body::BoxBody::new(format!("{}",self)))
//     }
// }

pub fn get_id(table: &str, id: String) -> Thing {
    Thing::from((table.to_string(), Id::String(id)))
}

pub async fn add_game(id: String, game: &Game) -> Result<(), DynError> {
    let _game: Game = crate::DB.create(("game", id)).content(game).await?;
    Ok(())
}

pub async fn add_sd_card(id: String, card: &MicroSDCard) -> Result<(), DynError> {
    let _card: MicroSDCard = crate::DB.create(("card", id)).content(card).await?;
    Ok(())
}

pub async fn update_sd_card_name(id: String, name: Name) -> Result<(), DynError> {
    let _: Name = crate::DB.update(("card", id)).merge(name).await?;
    Ok(())
}

pub async fn get_game(id: String) -> Result<Option<Game>, DynError> {
    Ok(crate::DB.select(("game", id)).await?)
}

pub async fn get_card(id: String) -> Result<Option<MicroSDCard>, DynError> {
    Ok(crate::DB.select(("card", id)).await?)
}

pub async fn list_games() -> Result<Vec<Game>, DynError> {
    Ok(crate::DB.select("game").await?)
}

pub async fn list_cards() -> Result<Vec<MicroSDCard>, DynError> {
    Ok(crate::DB.select("card").await?)
}

pub async fn add_game_to_card(game_id: String, card_id: String) -> Result<(), DynError>{
    // We delete the record first to make sure we are never adding a duplicate
    let _ = crate::DB
        .query("DELETE contains WHERE in=$card AND out=$game; RELATE $card->contains->$game;")
        .bind(("card", get_id("card", card_id)))
        .bind(("game", get_id("game", game_id)))
        .await?;

    Ok(())
}

pub async fn remove_game_from_card(game_id: String, card_id: String) -> Result<(), DynError>{
    let _ = crate::DB
        .query("DELETE contains WHERE in=$card AND out=$game;")
        .bind(("card", get_id("card", card_id)))
        .bind(("game", get_id("game", game_id)))
        .await?;

    Ok(())
}

pub async fn remove_game(game_id: String) -> Result<(), DynError> {
    let _ = crate::DB
        .query("DELETE contains WHERE out=$game; DELETE $game;")
        .bind(("game", get_id("game", game_id)))
        .await?;

    Ok(())
}

pub async fn remove_card(card_id: String) -> Result<(), DynError> {
    let _ = crate::DB
        .query("DELETE contains WHERE in=$card; DELETE $card;")
        .bind(("card", get_id("card", card_id)))
        .await?;

    Ok(())
}

pub async fn get_games_on_card(card_id: String) -> Result<Vec<Game>, DynError> {
    let result: Vec<Vec<Option<Game>>> = crate::DB
        .query("SELECT ->contains->game.* as games FROM $card")
        .bind(("card", get_id("card", card_id)))
        .await?
        .take("games")?;

    Ok(result.iter().flat_map(|f| f.iter().filter_map(|v| v.to_owned())).collect())
}

pub async fn get_cards_for_game(game_id: String) -> Result<Vec<MicroSDCard>, DynError> {
    let result:  Vec<Vec<Option<MicroSDCard>>> = crate::DB
        .query("SELECT <-contains<-card.* as cards FROM $game;")
        .bind(("game", get_id("game", game_id)))
        .await?
        .take("cards")?;

    Ok(result.iter().flat_map(|f| f.iter().filter_map(|v| v.to_owned())).collect())
}

pub async fn get_cards_with_games() -> Result<Vec<(MicroSDCard,Vec<Game>)>, DynError> {
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
