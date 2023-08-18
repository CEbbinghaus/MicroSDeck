// pub mod list_games;
// pub mod list_cards;
// pub mod list_games_on_card;
// pub mod get_card_for_game;
// pub mod set_name_for_card;
// pub mod list_cards_with_games;

use std::error::Error;

// pub use crate::l;
use crate::dbo::Name;
use actix_web::{post, web, HttpResponse, HttpResponseBuilder, Responder, ResponseError, Result, get};
use serde::Deserialize;

#[get("/ListGames")]
pub(crate) async fn list_games() -> Result<impl Responder> {
    Ok(web::Json(crate::db::list_games().await.ok()))
    // match  {
    // 	Err(err) => HttpResponse::InternalServerError(),
    //     Ok(res) => HttpResponse::Ok().json(res).,
    // }
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
        Err(err) => Ok(HttpResponse::InternalServerError()),
        Ok(_) => Ok(HttpResponse::Ok()),
    }
    // Ok(HttpResponse::Ok())
}
