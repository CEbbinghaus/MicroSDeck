use std::sync::Mutex;

use usdpl_back::core::serdes::Primitive;
use usdpl_back::AsyncCallable;

use crate::db::DBConnection;

pub struct GetGames<'a> {
    database: Mutex<&'a DBConnection>,
}

impl GetGames<'_> {
    pub fn new(db: &DBConnection) -> Self {
        Self {
            database: Mutex::new(db),
        }
    }
}

async fn test(db: &DBConnection) -> Vec<usdpl_back::core::serdes::Primitive> {
		match db.list_games().await {
		Err(_) => {
			vec![]
		}
		Ok(res) => res
			.iter()
			.map(|v| serde_json::to_string(v))
			.filter_map(|v| v.ok())
			.map(|v| Primitive::Json(v))
			.collect(),
	}
}

#[async_trait::async_trait]
impl AsyncCallable for GetGames<'_> {
    async fn call(
        &self,
        params: Vec<usdpl_back::core::serdes::Primitive>,
    ) -> Vec<usdpl_back::core::serdes::Primitive> {

        let lockedConnection = *self.database.lock().unwrap();

		return test(lockedConnection).await;

        // match lockedConnection.list_games().await {
        //     Err(_) => {
        //         vec![]
        //     }
        //     Ok(res) => res
        //         .iter()
        //         .map(|v| serde_json::to_string(v))
        //         .filter_map(|v| v.ok())
        //         .map(|v| Primitive::Json(v))
        //         .collect(),
        // }
    }
}
