use std::{result::Result, sync::Arc};
use futures::SinkExt;
use surrealdb::sql::{Id, Thing};

use crate::dbo::*;

pub fn get_id(table: &str, id: u64) -> Thing {
    Thing::from((table.to_string(), Id::Number(id as i64)))
}

pub async fn add_game(game: &Game) -> Result<(), surrealdb::Error> {
    let _game: Game = crate::DB.create(("game", game.uid)).content(game).await?;

    Ok(())
}

pub async fn add_sd_card(card: &MicroSDCard) -> Result<(), surrealdb::Error> {
    let _card: MicroSDCard = crate::DB.create(("card", card.uid)).content(card).await?;
    Ok(())
}

pub async fn update_sd_card_name(card_id: u64, name: Name) -> Result<(), surrealdb::Error> {
    let _: Name = crate::DB.update(("card", card_id)).merge(name).await?;
    Ok(())
}

pub async fn get_sd_card_for_game(
    game_id: u64,
) -> Result<Option<MicroSDCard>, Box<dyn std::error::Error>> {
    let result: Option<MicroSDCard> = crate::DB
        .query("SELECT card.* from $id")
        .bind(("id", get_id("game", game_id)))
        .await?
        .take("card")?;
    Ok(result)
}

pub async fn get_game(id: u64) -> Result<Option<Game>, Box<dyn std::error::Error>> {
    Ok(crate::DB.select(("game", id)).await?)
}

pub async fn get_card(id: u64) -> Result<Option<MicroSDCard>, Box<dyn std::error::Error>> {
    Ok(crate::DB.select(("card", id)).await?)
}

pub async fn list_games() -> Result<Vec<Game>, Box<dyn std::error::Error>> {
    Ok(crate::DB.select("game").await?)
}

pub async fn list_cards() -> Result<Vec<MicroSDCard>, Box<dyn std::error::Error>> {
    Ok(crate::DB.select("card").await?)
}

// pub async fn list_games_on_card(card_id: u64) -> Result<Vec<Game>, Box<dyn std::error::Error>> {
//     let result: Vec<Game> = crate::DB
//         .query("SELECT * FROM game WHERE card=$card")
//         .bind(("card", get_id("card", card_id)))
//         .await?
//         .take(0)?;

//     Ok(result)
// }

pub async fn list_games_on_card(card_id: u64) -> Result<Vec<Game>, Box<dyn std::error::Error>> {
    let result: Vec<Vec<Option<Game>>> = crate::DB
        .query("SELECT games.*.* FROM $card")
        .bind(("card", get_id("card", card_id)))
        .await?
        .take("games")?;

    Ok(result.iter().flat_map(|f| f.iter().flat_map(|f| f.clone()).collect::<Vec<Game>>()).collect())
}



// pub async fn list_card_with_games(card_id: u64) -> Result<(MicroSDCard,Vec<Game>), Box<dyn std::error::Error>> {
//     let mut query = crate::DB
//         .query("SELECT * FROM game WHERE card=$card")
//         .query("SELECT * FROM $card")
//         .bind(("card", get_id("card", card_id)))
//         .await?;

//     let games: Vec<Game> = query.take(0)?;
//     let card: Option<MicroSDCard> = query.take(1)?;

//     match card {
//         None => Err(Box::new(crate::err::Error::Error("No Microsd card found".into()))),
//         Some(card) => Ok((card, games))
//     }
// }

// pub async fn list_cards_with_games() -> Result<Vec<(MicroSDCard, Vec<Game>)>, Box<dyn std::error::Error>> {
//     let mut query = crate::DB
//         .query("SELECT *, games.* FROM card")
//         .await?;

//     let games: Vec<MicroSDCard> = query.take(0)?;
//     let card: Option<MicroSDCard> = query.take(1)?;

//     match card {
//         None => Err(Box::new(crate::err::Error::Error("No Microsd card found".into()))),
//         Some(card) => Ok((card, games))
//     }
// }

pub async fn setup_test_data() -> Result<(), Box<dyn std::error::Error>> {
    let card = MicroSDCard {
        uid: 1234,
        name: "Test".to_string(),
        games: vec![Thing::from(get_id("game", 123)), Thing::from(get_id("game", 124))]
    };

    let game = Game {
        uid: 123,
        name: "TestMcTestFace".to_string(),
        size: 64,
        card: Thing::from(get_id("card", card.uid)),
    };

    add_game(&game).await?;
    add_sd_card(&card).await?;

    for response in list_games().await? {
        println!("Recieved {:?}", response);
    }

    for response in list_cards().await? {
        println!("Recieved {:?}", response);
    }

    println!(
        "Found Card for game {} {:?}",
        game.uid,
        get_sd_card_for_game(game.uid).await?
    );

    update_sd_card_name(card.uid, "This is a new Name".into()).await?;

    println!(
        "Found Card for game {} {:?}",
        game.uid,
        get_sd_card_for_game(game.uid).await?
    );

    println!(
        "Found Games on Card {} {:?}",
        card.uid,
        list_games_on_card(card.uid).await?
    );
    Ok(())
}
