use crate::{
    ds::Store,
    dto::{Game, MicroSDCard},
    err::Error,
    sdcard::{get_card_cid, is_card_inserted},
};
use actix_web::{delete, get, post, web, HttpResponse, Responder, Result};
use serde::Deserialize;
use std::sync::Arc;

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_card)
        .service(delete_card)
        .service(list_cards)
        .service(get_card)
        
        .service(create_game)
        .service(delete_game)
        .service(list_games)
        .service(get_game)
        .service(list_games_on_card)
        .service(list_cards_with_games)
        .service(get_cards_for_game)
        .service(get_games_on_current_card)
        .service(set_name_for_card)
        .service(create_link)
        .service(save);
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
        Err(err) => Err(actix_web::Error::from(err)),
    }
}

#[get("/GetCardsForGame/{uid}")]
pub(crate) async fn get_cards_for_game(
    uid: web::Path<String>,
    datastore: web::Data<Arc<Store>>,
) -> Result<impl Responder> {
    match datastore.get_cards_for_game(&uid) {
        Ok(value) => Ok(web::Json(value)),
        Err(err) => Err(actix_web::Error::from(err)),
    }
}

#[get("/GetGamesOnCurrentCard")]
pub(crate) async fn get_games_on_current_card(
    datastore: web::Data<Arc<Store>>,
) -> Result<impl Responder> {
    if !is_card_inserted() {
        return Err(Error::Error("No card is inserted".into()).into());
    }

    let uid = get_card_cid().ok_or(Error::Error("Unable to evaluate Card Id".into()))?;

    match datastore.get_games_on_card(&uid) {
        Ok(value) => Ok(web::Json(value)),
        Err(err) => Err(actix_web::Error::from(err)),
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
    datastore.update_card(&body.id, |card| card.name = body.name.clone())?;
    Ok(HttpResponse::Ok())
}

#[post("/card/{id}")]
pub(crate) async fn create_card(
    id: web::Path<String>,
    body: web::Json<MicroSDCard>,
    datastore: web::Data<Arc<Store>>,
) -> impl Responder {
    datastore.add_card(id.into_inner(), body.into_inner());

    HttpResponse::Ok()
}

#[delete("/card/{id}")]
pub(crate) async fn delete_card(
    id: web::Path<String>,
    datastore: web::Data<Arc<Store>>,
) -> Result<impl Responder> {
    datastore.remove_element(&id)?;

    Ok(HttpResponse::Ok())
}

#[get("/card/{id}")]
pub(crate) async fn get_card(
    id: web::Path<String>,
    datastore: web::Data<Arc<Store>>,
) -> Result<impl Responder> {
    Ok(web::Json(datastore.get_card(&id)?))
}

#[get("/cards")]
pub(crate) async fn list_cards(datastore: web::Data<Arc<Store>>) -> impl Responder {
    web::Json(datastore.list_cards())
}

#[post("/game/{id}")]
pub(crate) async fn create_game(
    id: web::Path<String>,
    body: web::Json<Game>,
    datastore: web::Data<Arc<Store>>,
) -> Result<impl Responder> {
    datastore.add_game(id.into_inner(), body.into_inner());

    Ok(HttpResponse::Ok())
}

#[delete("/game/{id}")]
pub(crate) async fn delete_game(
    id: web::Path<String>,
    datastore: web::Data<Arc<Store>>,
) -> Result<impl Responder> {
    datastore.remove_element(&id)?;

    Ok(HttpResponse::Ok())
}

#[get("/game/{id}")]
pub(crate) async fn get_game(
    id: web::Path<String>,
    datastore: web::Data<Arc<Store>>,
) -> Result<impl Responder> {
    Ok(web::Json(datastore.get_game(&id)?))
}

#[get("/games")]
pub(crate) async fn list_games(datastore: web::Data<Arc<Store>>) -> impl Responder {
    web::Json(datastore.list_games())
}

#[derive(Deserialize)]
pub struct LinkBody {
    card_id: String,
    game_id: String,
}

#[post("/link")]
pub(crate) async fn create_link(
    body: web::Json<LinkBody>,
    datastore: web::Data<Arc<Store>>,
) -> Result<impl Responder> {
    datastore.link(&body.game_id, &body.card_id)?;

    Ok(HttpResponse::Ok())
}

#[post("/save")]
pub(crate) async fn save(datastore: web::Data<Arc<Store>>) -> Result<impl Responder> {
    datastore.write_to_file()?;

    Ok(HttpResponse::Ok())
}
