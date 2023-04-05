use std::{result::Result, sync::Arc};
use crate::dbo::*;

pub async fn add_game(game: &Game) -> Result<(), surrealdb::Error>{
    Ok(crate::DB.create(("game", game.uid))
        .content(game).await?)
}

pub async fn add_sd_card(card: &MicroSDCard)-> Result<(), surrealdb::Error> {
    Ok(crate::DB.create(("card", card.uid))
        .content(card).await?)
}

pub async fn get_sd_card_for_game(game_id: u64)-> Result<MicroSDCard, Box<dyn std::error::Error>> {
    Ok(crate::DB.select(("game", game_id)).await?)
}

pub async fn list_games()-> Result<Vec<Game>, Box<dyn std::error::Error>> {
    Ok(crate::DB.select(("game")).await?)
}

pub async fn list_sd_cards()-> Result<Vec<MicroSDCard>, Box<dyn std::error::Error>> {
    Ok(crate::DB.select(("card")).await?)
}

pub async fn setup_test_data() -> Result<(), Box<dyn std::error::Error>> {

    let card = Arc::new(MicroSDCard {
        uid: 1234,
        name: "Test".to_string()
    });

    let game = Arc::new(Game {
        uid: 123,
        name: "TestMcTestFace".to_string(),
        size: 64,
        card: Some(card)
    });

    add_game(&game).await?;
    add_sd_card(game.card.as_deref().expect("Card to exist")).await?;

    for response in list_games().await? {
        println!("Recieved {:?}", response);
    }

    for response in list_sd_cards().await? {
        println!("Recieved {:?}", response);
    }

    println!("Found Card for game {} {:?}", game.uid, get_sd_card_for_game(game.uid).await?);

    Ok(())
}