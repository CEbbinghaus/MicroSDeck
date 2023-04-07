use usdpl_back::core::serdes::Primitive;
use usdpl_back::AsyncCallable;

pub struct ListGames{
}

impl ListGames {
    pub fn new() -> Self {
        return ListGames {  }
    }
}

#[async_trait::async_trait]
impl AsyncCallable for ListGames {
    async fn call(
        &self,
        _: Vec<usdpl_back::core::serdes::Primitive>,
    ) -> Vec<usdpl_back::core::serdes::Primitive> {
        match crate::db::list_games().await {
            Err(err) => {
                vec![usdpl_back::core::serdes::Primitive::String(format!("{err}"))]
            }
            Ok(res) => vec![Primitive::Json(res
                .iter()
                .map(|v| serde_json::to_string(v))
                .filter_map(|v| v.ok())
                .collect())],
        }
    }
}
