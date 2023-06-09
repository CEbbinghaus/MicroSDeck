use usdpl_back::core::serdes::Primitive;
use usdpl_back::AsyncCallable;

use crate::dbo::Name;

pub struct SetNameForCard{
}

impl SetNameForCard {
    pub fn new() -> Self {
        return SetNameForCard {  }
    }
}

#[async_trait::async_trait]
impl AsyncCallable for SetNameForCard {

    async fn call(
        &self,
        args: Vec<Primitive>,
    ) -> Vec<Primitive> {
		let Some(id_prim) = args.first() else {
			return vec![Primitive::String("No value provided for argument ID".into())];
		};

		let id = match id_prim {
			Primitive::String(v) => v.to_owned(),
			_ => return vec![Primitive::String("Value for Argument ID was not a string".into())],
		};

		let Some(id_prim) = args.get(1) else {
			return vec![Primitive::String("No value provided for argument Name".into())];
		};

		let name = match id_prim {
			Primitive::String(v) => v.clone(),
			_ => return vec![Primitive::String("Value for Argument Name was not a string".into())],
		};

        match crate::db::update_sd_card_name(id, Name { name }).await {
            Err(err) => {
                vec![Primitive::String(format!("{err}"))]
            }
            Ok(_) => {
				vec![Primitive::Bool(true)]
			}
        }
    }
}
