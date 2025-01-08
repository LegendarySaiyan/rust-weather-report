use serde_json;
use std::fs;
use std::io::{self, Read};

pub struct Settings {
    pub telegram_token: String,
    pub weather_token: String,
}

impl Settings {
    pub fn new() -> Result<Self, io::Error> {
        let res: Result<String, std::io::Error> =
            fs::read_to_string("./src/infrastructure/local_db/credentials.json");
        let s = match res {
            Ok(s) => s,
            Err(_) => panic!("Can't read file"),
        };

        let json_data: serde_json::Value = serde_json::from_str(&s).expect("Can't parse json");

        let telegram_token = json_data["telegram_key"].as_str().clone().unwrap();

        let weather_token = json_data["weather_key"].as_str().clone().unwrap();

        Ok(Self {
            telegram_token: String::from(telegram_token),
            weather_token: String::from(weather_token),
        })
    }
}
