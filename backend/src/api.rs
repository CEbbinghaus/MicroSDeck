use crate::{dbo::Name, err::Error, sdcard::{is_card_inserted, get_card_cid}};
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
    match crate::db::get_cards_with_games().await {
        Ok(res) => Ok(web::Json(res)),
        Err(err) => Err(Error::from(err).into())
    }
}

#[get("/ListGamesOnCard/{card_id}")]
pub(crate) async fn list_games_on_card(
    card_id: web::Path<String>
) -> Result<impl Responder> {
    match crate::db::get_games_on_card(card_id.to_owned()).await {
        Ok(res) => Ok(web::Json(res)),
        Err(err) => Err(Error::from(err).into())
    }
}

#[get("/GetCardForGame/{uid}")]
pub(crate) async fn get_card_for_game(
    uid: web::Path<String>
) -> Result<impl Responder> {
    Ok(web::Json(crate::db::get_cards_for_game(uid.to_owned()).await.ok()))
}

#[get("/GetGamesOnCurrentCard")]
pub(crate) async fn get_games_on_current_card() -> Result<impl Responder> {
    if !is_card_inserted() {
        return Err(Error::Error("No card is inserted".into()).into());
    }

    let uid = get_card_cid().ok_or(Error::Error("Unable to evaluate Card Id".into()))?;

    Ok(web::Json(crate::db::get_games_on_card(uid).await.ok()))
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
