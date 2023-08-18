// use usdpl_back::core::serdes::Primitive;
// use usdpl_back::AsyncCallable;

// pub struct ListGamesOnCard {}

// impl ListGamesOnCard {
//     pub fn new() -> Self {
//         return ListGamesOnCard {};
//     }
// }

// #[async_trait::async_trait]
// impl AsyncCallable for ListGamesOnCard {
//     async fn call(&self, args: Vec<Primitive>) -> Vec<Primitive> {
//         let Some(id_prim) = args.first() else {
// 			return vec![Primitive::String("No value provided for argument ID".into())];
// 		};

//         let id = match id_prim {
//             Primitive::String(v) => v.to_owned(),
//             _ => {
//                 return vec![Primitive::String(
//                     "Value for Argument ID was not a String".into(),
//                 )]
//             }
//         };

//         match crate::db::get_games_on_card(id).await {
//             Err(err) => {
//                 vec![Primitive::String(format!("{err}"))]
//             }
//             Ok(res) => vec![Primitive::Json(
//                 serde_json::to_string(&res).unwrap_or("ERROR SERIALIZING JSON".into()),
//             )],
//         }
//     }
// }
