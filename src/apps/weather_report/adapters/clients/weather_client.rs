use crate::apps::weather_report::adapters::acl::dto::WeatherResponse;

pub trait ABCWeatherClient {
    async fn get_current(&self, city: String) -> Result<WeatherResponse, reqwest::Error>;
}
