use std::error::Error;

use crate::dbo::Name;
use actix_web::{get, post, web, HttpResponse, HttpResponseBuilder, Responder, ResponseError, Result};
use serde::Deserialize;

#[get("/ListGames")]
pub(crate) async fn list_games() -> Result<impl Responder> {
    Ok(web::Json(crate::db::list_games().await.ok()))
}

#[get("/ListCards")]
pub(crate) async fn list_cards() -> Result<impl Responder> {
    Ok(web::Json(crate::db::list_cards().await.ok()))
}

#[get("/ListCardsWithGames")]
pub(crate) async fn list_cards_with_games() -> Result<impl Responder> {
    Ok(web::Json(crate::db::get_cards_with_games().await.ok()))
}

#[derive(Deserialize)]
pub struct Card {
    id: String,
}

#[get("/ListGamesOnCard")]
pub(crate) async fn list_games_on_card(
    body: web::Json<Card>,
) -> Result<impl Responder> {
    Ok(web::Json(crate::db::get_games_on_card(body.id.to_owned()).await.ok()))
}

#[derive(Deserialize)]
pub struct Game {
    uid: String,
}

#[get("/GetCardForGame")]
pub(crate) async fn get_card_for_game(
    body: web::Json<Game>,
) -> Result<impl Responder> {
    Ok(web::Json(crate::db::get_cards_for_game(body.uid.to_owned()).await.ok()))
}

#[derive(Deserialize)]
pub struct SetNameForCardBody {
    id: String,
    name: String,
}

#[post("/SetNameForCard")]
pub(crate) async fn set_name_for_card(
    body: web::Json<SetNameForCardBody>,
) -> Result<impl Responder> {
    match crate::db::update_sd_card_name(
        body.id.to_owned(),
        Name {
            name: body.name.to_owned(),
        },
    )
    .await
    {
        Err(_) => Ok(HttpResponse::InternalServerError()),
        Ok(_) => Ok(HttpResponse::Ok()),
    }
}
