use crate::apps::weather_report::domain::aggregates::last_update::LastUpdate;
use std::io::{self, Read};

pub trait ABCLastUpdateRepo {
    async fn get_last_update(&self) -> LastUpdate;
    async fn save(&self, last_update: LastUpdate) -> Result<(), io::Error>;
}
