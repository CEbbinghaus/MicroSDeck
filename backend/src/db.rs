use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::sync::Arc;
use crate::err::Error;

use surrealdb::{Datastore, Session, Response};
use surrealdb::sql::{parse, Value, Thing, Id};
use crate::dbo::*;

pub struct DBConnection {
    pub datastore: Arc<Datastore>,
    pub session: Arc<Session>,
}

impl DBConnection {
    pub async fn add_game(self: &Self, game: &Game) -> Result<(), surrealdb::Error>{
        let ast = parse("CREATE $id SET uid=$uid, name=$name, size=$size, card=$card").expect("Query to compile");
    
        let vars = Some(BTreeMap::from([
            ("id".into(), Value::Thing(Thing { tb: "game".into(), id: game.uid.into()})),
            ("uid".into(), game.uid.into()),
            ("name".into(), game.name.clone().into()),
            ("size".into(), game.size.into()),
            {
                if let Some(card) = game.card.borrow() {
                    ("card".into(), Value::Thing(Thing { tb: "card".into(), id: card.uid.clone().into() }))
                } else {
                    ("".into(), Value::Null)
                }

            }
        ]));
    
        self.datastore.process(ast, &self.session, vars, false).await?;
    
        Ok(())
    }
    
    pub async fn add_sd_card(self: &Self, card: &MicroSDCard)-> Result<(), surrealdb::Error> {
        let ast = parse("CREATE $id SET uid=$uid, name=$name, games=$games").expect("Query to compile");

        let vars: Option<BTreeMap<String, Value>> = Some(BTreeMap::from([
            ("id".into(), Value::Thing(Thing { tb: "card".into(), id: card.uid.clone().into()})),
            ("uid".into(), card.uid.clone().into()),
            ("name".into(), card.name.clone().into())
        ]));
    
        self.datastore.process(ast, &self.session, vars, false).await?;
        Ok(())
    }
    
    pub async fn get_sd_card_for_game(self: &Self, game_id: u64)-> Result<MicroSDCard, Box<dyn std::error::Error>> {
        let ast = parse("SELECT * FROM $id;").expect("Query to compile");
    
        let vars: Option<BTreeMap<String, Value>> = Some(BTreeMap::from([
            ("id".into(), Value::Thing(Thing{tb: "game".into(), id: Id::Number(game_id.try_into().unwrap())}))
        ]));
    
        let result = self.datastore.process(ast, &self.session, vars, false).await?;

        if let Response { result: Ok(value), ..} = &result[0] {
            let str = serde_json::to_string(value)?;
    
            // This is stupid since we are serializing it as JSON & deserializing that back into the struct.
            // Ideal solution would be to deserialize the Value tree directly to struct
            if let Some(v) = serde_json::from_str::<Vec<MicroSDCard>>(str.as_str())?.into_iter().next() {
                return Ok(v);
            } else {
                return Err(Box::new(Error::Error(format!("Unable to deserialize Card \n{}", str))));
            }
        }
    
        Err(Box::new(Error::Error("No Card could be found".to_string())))
    }
    
    pub async fn list_games(self: &Self)-> Result<Vec<Game>, Box<dyn std::error::Error>> {
        let ast = parse("SELECT card.*, * FROM game").expect("Query to compile");
    
        let result = self.datastore.process(ast, &self.session, None, false).await?;
    
        if let Response { result: Ok(value), ..} = &result[0] {
            let str = serde_json::to_string(value)?;

            // This is stupid since we are serializing it as JSON & deserializing that back into the struct.
            // Ideal solution would be to deserialize the Value tree directly to struct
            if let Ok(v) = serde_json::from_str::<Vec<Game>>(str.as_str()){
                return Ok(v);
            } else {
                return Err(Box::new(Error::Error(format!("Unable to deserialize Games\n{}", str))));
            }
        }
    
        Err(Box::new(Error::Error("Unable to retrieve all games".to_string())))
    }
    
    pub async fn list_sd_cards(self: &Self)-> Result<Vec<MicroSDCard>, Box<dyn std::error::Error>> {
        let ast = parse("SELECT uid, name FROM card").expect("Query to compile");
    
        let result = self.datastore.process(ast, &self.session, None, false).await?;
    
        if let Response { result: Ok(value), ..} = &result[0] {
            let str = serde_json::to_string(value)?;

            // This is stupid since we are serializing it as JSON & deserializing that back into the struct.
            // Ideal solution would be to deserialize the Value tree directly to struct
            if let Ok(v) = serde_json::from_str::<Vec<MicroSDCard>>(str.as_str()){
                return Ok(v);
            } else {
                return Err(Box::new(Error::Error(format!("Unable to deserialize MicroSDCards\n{}", str.as_str()))));
            }
        }
    
        Err(Box::new(Error::Error("Unable to retrieve all MicroSDCards".to_string())))
    }
}


pub async fn setup_test_data(connection: &DBConnection) -> Result<(), Box<dyn std::error::Error>> {

    // let ds = Datastore::new("memory").await?;
    // // let ds = Datastore::new("/var/etc/Database.file").await?;
    
    // println!("Constructed Database.");
    
    // let ses = Session::for_db("","");
    
    // let connection = DBConnection {
    //     datastore: Arc::from(ds),
    //     session: Arc::from(ses)
    // };


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

    connection.add_game(&game).await?;
    connection.add_sd_card(game.card.as_deref().expect("Card to exist")).await?;

    for response in connection.list_games().await? {
        println!("Recieved {:?}", response);
    }

    for response in connection.list_sd_cards().await? {
        println!("Recieved {:?}", response);
    }

    println!("Found Card for game {} {:?}", game.uid, connection.get_sd_card_for_game(game.uid).await?);

    // let ast = parse("CREATE person:B SET name=$Name, friend = person:A")?;

    // ds.process(ast, &ses, Some(BTreeMap::from([
    //     ("Name".into(), "Frank".into())
    // ])), false).await?;

    // let ast = parse("CREATE person:A SET name=$Name, age=23")?;
    
    // ds.process(ast, &ses, Some(BTreeMap::from([
    //     ("Name".to_string(), Value::Strand(Strand("Anthony".to_string()))),
    //     ("Test".into(), "Anthony".into())
    // ])), false).await?;

    // let res = ds.execute("SELECT friend.* FROM person:B;", &ses, None, false).await?;
    // // let ast = "USE NS test DB test; CREATE person:me SET name=\"Chris\", age=23";
    // // let res = ds.execute(ast, &ses, None, false).await?;

    // for response in res {
    //    let result = response.result?;
        
    //     println!("Recieved {:?}", result);
    // }

    Ok(())
}