use usdpl_back::core::serdes::Primitive;
use usdpl_back::AsyncCallable;

pub struct GetCardForGame{
}

impl GetCardForGame {
    pub fn new() -> Self {
        return GetCardForGame {  }
    }
}

#[async_trait::async_trait]
impl AsyncCallable for GetCardForGame {
    async fn call(
        &self,
        args: Vec<usdpl_back::core::serdes::Primitive>,
    ) -> Vec<usdpl_back::core::serdes::Primitive> {
		let Some(id_prim) = args.first() else {
			return vec![Primitive::String("No value provided for argument ID".into())];
		};

		let id: u64 = match id_prim {
			Primitive::F64(v) => v.round() as u64,
			_ => return vec![Primitive::String("Value for Argument ID was not a number".into())],
		};

        match crate::db::get_sd_card_for_game(id).await {
            Err(err) => {
                vec![usdpl_back::core::serdes::Primitive::String(format!("{err}"))]
            }
            Ok(res) => {
				match res {
					None => vec![Primitive::String("No MicroSDCard could be found".into())],
					Some(val) => vec![Primitive::Json(serde_json::to_string(&val).expect("Serialization to work"))]
				}
			}
        }
    }
}
