use serde_json::{self, Value};
use std::fs;
use std::io::{self, Read};

use crate::apps::weather_report::adapters::repositories::abc_repo::ABCLastUpdateRepo;
use crate::apps::weather_report::domain::aggregates::last_update::{self, LastUpdate};

pub struct LastUpdateRepo;
impl ABCLastUpdateRepo for LastUpdateRepo {
    async fn get_last_update(&self) -> LastUpdate {
        let json_data = self.get_json_data().await.unwrap();

        let last_update_id = Some((json_data["last_update_id"].clone().as_i64()).unwrap());

        LastUpdate {
            last_update_id: Some((last_update_id.unwrap()) as i32),
            was_used: json_data["was_used"].clone().as_bool().unwrap(),
        }
    }

    async fn save(&self, last_update: LastUpdate) -> Result<(), io::Error> {
        let mut json_data = self.get_json_data().await.unwrap();

        let was_used_data = json_data["was_used"].clone().as_bool().unwrap();

        if was_used_data != last_update.was_used {
            json_data["was_used"] = serde_json::json!(last_update.was_used);
        }

        json_data["last_update_id"] = serde_json::json!(last_update.last_update_id.unwrap());
        std::fs::write(
            "./src/infrastructure/local_db/last_update.json",
            serde_json::to_string_pretty(&json_data).unwrap(),
        )
        .expect("Can't write to file");

        Ok(())
    }
}
impl LastUpdateRepo {
    async fn get_json_data(&self) -> Result<serde_json::Value, io::Error> {
        let res: Result<String, std::io::Error> =
            fs::read_to_string("./src/infrastructure/local_db/last_update.json");
        let s = match res {
            Ok(s) => s,
            Err(_) => panic!("Can't read file"),
        };

        let json_data: serde_json::Value = serde_json::from_str(&s).expect("Can't parse json");
        Ok(json_data)
    }
}
