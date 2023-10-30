use crate::{
	ds::Store,
	dto::{CardEvent, Game, MicroSDCard},
	env::PACKAGE_VERSION,
	err::Error,
	sdcard::{get_card_cid, is_card_inserted},
};
use actix_web::{delete, get, post, web, HttpResponse, Responder, Result};
use serde::Deserialize;
use std::{ops::Deref, sync::Arc};
use tokio::sync::broadcast::Sender;

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(health)
		.service(version)
		.service(listen)
		.service(save)
		.service(get_current_card)
		.service(get_current_card_id)
		.service(get_current_card_and_games)
		.service(get_games_on_current_card)
		.service(create_card)
		.service(delete_card)
		.service(list_cards)
		.service(get_card)
		.service(create_game)
		.service(create_games)
		.service(delete_game)
		.service(list_games)
		.service(get_game)
		.service(list_games_for_card)
		.service(list_cards_for_game)
		.service(list_cards_with_games)
		.service(create_link)
		.service(create_links)
		.service(delete_link)
		.service(delete_links);
}

#[get("/version")]
pub(crate) async fn version() -> impl Responder {
	HttpResponse::Ok().body(PACKAGE_VERSION)
}

#[get("/health")]
pub(crate) async fn health() -> impl Responder {
	HttpResponse::Ok()
}

#[get("/listen")]
pub(crate) async fn listen(sender: web::Data<Sender<CardEvent>>) -> Result<impl Responder> {
	Ok(web::Json(sender.subscribe().recv().await.map_err(
		|_| Error::from_str("Unable to retrieve update"),
	)?))
}

#[get("/list")]
pub(crate) async fn list_cards_with_games(datastore: web::Data<Arc<Store>>) -> impl Responder {
	web::Json(datastore.list_cards_with_games())
}

#[get("/list/games/{card_id}")]
pub(crate) async fn list_games_for_card(
	card_id: web::Path<String>,
	datastore: web::Data<Arc<Store>>,
) -> Result<impl Responder> {
	match datastore.get_games_on_card(&card_id) {
		Ok(value) => Ok(web::Json(value)),
		Err(err) => Err(actix_web::Error::from(err)),
	}
}

#[get("/list/cards/{game_id}")]
pub(crate) async fn list_cards_for_game(
	game_id: web::Path<String>,
	datastore: web::Data<Arc<Store>>,
) -> Result<impl Responder> {
	match datastore.get_cards_for_game(&game_id) {
		Ok(value) => Ok(web::Json(value)),
		Err(err) => Err(actix_web::Error::from(err)),
	}
}

#[get("/current")]
pub(crate) async fn get_current_card_and_games(
	datastore: web::Data<Arc<Store>>,
) -> Result<impl Responder> {
	if !is_card_inserted() {
		return Err(Error::from_str("No card is inserted").into());
	}

	let uid = get_card_cid().ok_or(Error::Error("Unable to evaluate Card Id".into()))?;

	Ok(web::Json(datastore.get_card_and_games(&uid)?))
}

#[get("/current/card")]
pub(crate) async fn get_current_card(datastore: web::Data<Arc<Store>>) -> Result<impl Responder> {
	if !is_card_inserted() {
		return Err(Error::from_str("No card is inserted").into());
	}

	let uid = get_card_cid().ok_or(Error::Error("Unable to evaluate Card Id".into()))?;

	Ok(web::Json(datastore.get_card(&uid)?))
}

#[get("/current/id")]
pub(crate) async fn get_current_card_id() -> Result<impl Responder> {
	if !is_card_inserted() {
		return Err(Error::from_str("No card is inserted").into());
	}

	Ok(get_card_cid().ok_or(Error::from_str("Unable to evaluate Card Id"))?)
}

#[get("/current/games")]
pub(crate) async fn get_games_on_current_card(
	datastore: web::Data<Arc<Store>>,
) -> Result<impl Responder> {
	if !is_card_inserted() {
		return Err(Error::from_str("No card is inserted").into());
	}

	let uid = get_card_cid().ok_or(Error::from_str("Unable to evaluate Card Id"))?;

	match datastore.get_games_on_card(&uid) {
		Ok(value) => Ok(web::Json(value)),
		Err(err) => Err(actix_web::Error::from(err)),
	}
}

#[post("/card/{id}")]
pub(crate) async fn create_card(
	id: web::Path<String>,
	body: web::Json<MicroSDCard>,
	datastore: web::Data<Arc<Store>>,
	sender: web::Data<Sender<CardEvent>>,
) -> Result<impl Responder> {
	if *id != body.uid {
		return Err(Error::from_str("uid did not match id provided").into());
	}

	match datastore.contains_element(&id) {
		// Merge the records allowing us to update all properties
		true => datastore.update_card(&id, move |existing_card| {
			existing_card.merge(body.deref().to_owned())?;
			Ok(())
		})?,
		// Insert a new card if it doesn't exist
		false => datastore.add_card(id.into_inner(), body.into_inner()),
	}

	_ = sender.send(CardEvent::Updated);
	Ok(HttpResponse::Ok())
}

#[delete("/card/{id}")]
pub(crate) async fn delete_card(
	id: web::Path<String>,
	datastore: web::Data<Arc<Store>>,
	sender: web::Data<Sender<CardEvent>>,
) -> Result<impl Responder> {
	datastore.remove_element(&id)?;

	_ = sender.send(CardEvent::Updated);
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
	if *id != body.uid {
		return Err(Error::from_str("uid did not match id provided").into());
	}

	let mut game = body.to_owned();

	if !cfg!(debug_assertions) {
		game.is_steam = false;
	}

	datastore.add_game(id.into_inner(), game);

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

#[post("/games")]
pub(crate) async fn create_games(
	body: web::Json<Vec<Game>>,
	datastore: web::Data<Arc<Store>>,
) -> impl Responder {
	for game in body.iter() {
		let mut game = game.to_owned();

		if !cfg!(debug_assertions) {
			game.is_steam = false;
		}

		datastore.add_game(game.uid.clone(), game);
	}

	HttpResponse::Ok()
}

#[derive(Deserialize)]
pub struct LinkBody {
	card_id: String,
	game_id: String,
}

#[derive(Deserialize)]
pub struct ManyLinkBody {
	card_id: String,
	game_ids: Vec<String>,
}

#[post("/link")]
pub(crate) async fn create_link(
	body: web::Json<LinkBody>,
	datastore: web::Data<Arc<Store>>,
	sender: web::Data<Sender<CardEvent>>,
) -> Result<impl Responder> {
	datastore.link(&body.game_id, &body.card_id)?;

	_ = sender.send(CardEvent::Updated);
	Ok(HttpResponse::Ok())
}

#[post("/linkmany")]
pub(crate) async fn create_links(
	body: web::Json<ManyLinkBody>,
	datastore: web::Data<Arc<Store>>,
	sender: web::Data<Sender<CardEvent>>,
) -> Result<impl Responder> {
	let data = body.into_inner();
	for game_id in data.game_ids.iter() {
		datastore.link(&game_id, &data.card_id)?;
	}

	_ = sender.send(CardEvent::Updated);
	Ok(HttpResponse::Ok())
}

#[post("/unlink")]
pub(crate) async fn delete_link(
	body: web::Json<LinkBody>,
	datastore: web::Data<Arc<Store>>,
	sender: web::Data<Sender<CardEvent>>,
) -> Result<impl Responder> {
	datastore.unlink(&body.game_id, &body.card_id)?;

	_ = sender.send(CardEvent::Updated);
	Ok(HttpResponse::Ok())
}

#[post("/unlinkmany")]
pub(crate) async fn delete_links(
	body: web::Json<ManyLinkBody>,
	datastore: web::Data<Arc<Store>>,
	sender: web::Data<Sender<CardEvent>>,
) -> Result<impl Responder> {
	let data = body.into_inner();
	for game_id in data.game_ids.iter() {
		datastore.unlink(&game_id, &data.card_id)?;
	}

	_ = sender.send(CardEvent::Updated);
	Ok(HttpResponse::Ok())
}

#[post("/save")]
pub(crate) async fn save(datastore: web::Data<Arc<Store>>) -> Result<impl Responder> {
	datastore.write_to_file()?;

	Ok(HttpResponse::Ok())
}
