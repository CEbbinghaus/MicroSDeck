use std::{borrow::BorrowMut, sync::Arc};

use crate::{dbo::Name, err::Error, sdcard::{is_card_inserted, get_card_cid}, ds::Store};
use actix_web::{get, post, web, HttpResponse, HttpResponseBuilder, Responder, ResponseError, Result};
use serde::Deserialize;

#[get("/ListGames")]
pub(crate) async fn list_games(datastore: web::Data<Arc<Store>>) -> Result<impl Responder> {
    Ok(web::Json(datastore.list_games()))
}

#[get("/ListCards")]
pub(crate) async fn list_cards(datastore: web::Data<Arc<Store>>) -> impl Responder {
    web::Json(datastore.list_cards())
}

#[get("/ListCardsWithGames")]
pub(crate) async fn list_cards_with_games(datastore: web::Data<Arc<Store>>) -> impl Responder {
    web::Json(datastore.list_cards_with_games())
}

#[get("/ListGamesOnCard/{card_id}")]
pub(crate) async fn list_games_on_card(
    card_id: web::Path<String>,
    datastore: web::Data<Arc<Store>>,
) -> Result<impl Responder> {

    match datastore.get_games_on_card(&card_id) {
        Ok(value) => Ok(web::Json(value)),
        Err(err) => Err(actix_web::Error::from(err))
    }
}

#[get("/GetCardForGame/{uid}")]
pub(crate) async fn get_card_for_game(
    uid: web::Path<String>,
    datastore: web::Data<Arc<Store>>,
) -> Result<impl Responder> {
    match datastore.get_cards_for_game(&uid) {
        Ok(value) => Ok(web::Json(value)),
        Err(err) => Err(actix_web::Error::from(err))
    }
}

#[get("/GetGamesOnCurrentCard")]
pub(crate) async fn get_games_on_current_card(datastore: web::Data<Arc<Store>>) -> Result<impl Responder> {
    if !is_card_inserted() {
        return Err(Error::Error("No card is inserted".into()).into());
    }

    let uid = get_card_cid().ok_or(Error::Error("Unable to evaluate Card Id".into()))?;

    match datastore.get_games_on_card(&uid) {
        Ok(value) => Ok(web::Json(value)),
        Err(err) => Err(actix_web::Error::from(err))
    }
}

#[derive(Deserialize)]
pub struct SetNameForCardBody {
    id: String,
    name: String,
}

#[post("/SetNameForCard")]
pub(crate) async fn set_name_for_card(
    body: web::Json<SetNameForCardBody>,
    datastore: web::Data<Arc<Store>>,
) -> Result<impl Responder> {
    match datastore.update_card(&body.id, |card| card.name = body.name.clone()) {
        Ok(value) => Ok(web::Json(value)),
        Err(err) => Err(actix_web::Error::from(err))
    }
}
