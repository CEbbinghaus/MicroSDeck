// use usdpl_back::core::serdes::Primitive;
// use usdpl_back::AsyncCallable;

// pub struct ListCardsWithGames {}

// impl ListCardsWithGames {
//     pub fn new() -> Self {
//         return ListCardsWithGames {};
//     }
// }

// #[async_trait::async_trait]
// impl AsyncCallable for ListCardsWithGames {
//     async fn call(&self, _: Vec<Primitive>) -> Vec<Primitive> {
//         match crate::db::get_cards_with_games().await {
//             Err(err) => {
//                 vec![format!("{err}").into()]
//             }
//             Ok(res) => vec![Primitive::Json(serde_json::to_string(&res).unwrap_or("ERROR SERIALIZING JSON".into()))],
//         }
//     }
// }
