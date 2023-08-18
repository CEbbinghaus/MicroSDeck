// use usdpl_back::core::serdes::Primitive;
// use usdpl_back::AsyncCallable;

// pub struct ListGames{
// }

// impl ListGames {
//     pub fn new() -> Self {
//         return ListGames {  }
//     }
// }

// #[async_trait::async_trait]
// impl AsyncCallable for ListGames {
//     async fn call(
//         &self,
//         _: Vec<Primitive>,
//     ) -> Vec<Primitive> {
//         match crate::db::list_games().await {
//             Err(err) => {
//                 vec![Primitive::String(format!("{err}"))]
//             }
//             Ok(res) => vec![Primitive::Json(serde_json::to_string(&res).unwrap_or("ERROR SERIALIZING JSON".into()))],
//         }
//     }
// }
