use usdpl_back::core::serdes::Primitive;
use usdpl_back::AsyncCallable;

pub struct GetGames{
}

impl GetGames {
    pub fn new() -> Self {
        return GetGames {  }
    }
}

#[async_trait::async_trait]
impl AsyncCallable for GetGames {
    async fn call(
        &self,
        _: Vec<usdpl_back::core::serdes::Primitive>,
    ) -> Vec<usdpl_back::core::serdes::Primitive> {
        match crate::db::list_games().await {
            Err(_) => {
                vec![usdpl_back::core::serdes::Primitive::String("None".to_string())]
            }
            Ok(res) => res
                .iter()
                .map(|v| serde_json::to_string(v))
                .filter_map(|v| v.ok())
                .map(|v| Primitive::Json(v))
                .collect(),
        }
    }
}
