use usdpl_back::core::serdes::Primitive;
use usdpl_back::AsyncCallable;

pub struct ListCards{
}

impl ListCards {
    pub fn new() -> Self {
        return ListCards {  }
    }
}

#[async_trait::async_trait]
impl AsyncCallable for ListCards {
    async fn call(
        &self,
        _: Vec<usdpl_back::core::serdes::Primitive>,
    ) -> Vec<usdpl_back::core::serdes::Primitive> {
        match crate::db::list_cards().await {
            Err(err) => {
                vec![usdpl_back::core::serdes::Primitive::String(format!("{err}"))]
            }
            Ok(res) => vec![Primitive::Json(res
                .iter()
                .map(|v| serde_json::to_string(v))
                .filter_map(|v| v.ok())
                // .map(|v| Primitive::Json(v))
                .collect())],
        }
    }
}
