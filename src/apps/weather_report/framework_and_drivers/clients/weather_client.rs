use crate::apps::weather_report::adapters::acl::dto::WeatherResponse;
use crate::apps::weather_report::adapters::clients::weather_client::ABCWeatherClient;
use crate::infrastructure::settings::base::Settings;

pub struct WeatherClient<'a> {
    pub settings: &'a Settings,
    pub client: reqwest::Client,
}

impl<'a> ABCWeatherClient for WeatherClient<'a> {
    async fn get_current(&self, city: String) -> Result<WeatherResponse, reqwest::Error> {
        let params = [("key", (*self.settings).weather_token.clone()), ("q", city)];

        let request = self
            .client
            .post(format!("https://api.weatherapi.com/v1/current.json",))
            .query(&params)
            .send()
            .await?;

        let result = request.json::<WeatherResponse>().await?;

        Ok(result)
    }
}
