use usdpl_back::core::serdes::Primitive;
use usdpl_back::AsyncCallable;

pub struct ListGamesOnCard {}

impl ListGamesOnCard {
    pub fn new() -> Self {
        return ListGamesOnCard {};
    }
}

#[async_trait::async_trait]
impl AsyncCallable for ListGamesOnCard {
    async fn call(
        &self,
        args: Vec<usdpl_back::core::serdes::Primitive>,
    ) -> Vec<usdpl_back::core::serdes::Primitive> {
        let Some(id_prim) = args.first() else {
			return vec![Primitive::String("No value provided for argument ID".into())];
		};

        let id: u64 = match id_prim {
            Primitive::F64(v) => v.round() as u64,
            _ => {
                return vec![Primitive::String(
                    "Value for Argument ID was not a number".into(),
                )]
            }
        };

        match crate::db::list_games_on_card(id).await {
            Err(err) => {
                vec![usdpl_back::core::serdes::Primitive::String(format!(
                    "{err}"
                ))]
            }
            Ok(res) => vec![Primitive::Json(
                res.iter()
                    .map(|v| serde_json::to_string(v))
                    .filter_map(|v| v.ok())
                    .collect(),
            )],
        }
    }
}
