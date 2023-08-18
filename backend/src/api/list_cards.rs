// use usdpl_back::core::serdes::Primitive;
// use usdpl_back::AsyncCallable;

// pub struct ListCards {}

// impl ListCards {
//     pub fn new() -> Self {
//         return ListCards {};
//     }
// }

// #[async_trait::async_trait]
// impl AsyncCallable for ListCards {
//     async fn call(&self, _: Vec<Primitive>) -> Vec<Primitive> {
//         match crate::db::list_cards().await {
//             Err(err) => {
//                 vec![Primitive::String(format!("{err}"))]
//             }
//             Ok(res) => vec![Primitive::Json(
//                 serde_json::to_string(&res).unwrap_or("ERROR SERIALIZING JSON".into()),
//             )],
//         }
//     }
// }
