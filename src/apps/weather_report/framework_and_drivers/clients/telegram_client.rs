use log::{debug, info, warn};
use reqwest::RequestBuilder;

use crate::apps::weather_report::adapters::acl::dto::Response;
use crate::apps::weather_report::adapters::clients::telegram_client::ABCTelegramClient;
use crate::infrastructure::settings::base::Settings;

pub struct TelegramClient<'a> {
    pub settings: &'a Settings,
    pub client: reqwest::Client,
}

impl<'a> ABCTelegramClient for TelegramClient<'a> {
    async fn get_updates(&self, offset: Option<i32>) -> Result<Response, reqwest::Error> {
        let mut request: RequestBuilder;

        if offset.is_some() {
            let params = [("offset", offset.unwrap())];

            request = self
                .client
                .get(format!(
                    "https://api.telegram.org/bot{}/getUpdates",
                    (*self.settings).telegram_token
                ))
                .query(&params)
        } else {
            request = self.client.get(format!(
                "https://api.telegram.org/bot{}/getUpdates",
                (*self.settings).telegram_token
            ))
        }

        let response = request.send().await?;
        info!("Got response: {:#?}", response);
        let result = response.json::<Response>().await?;

        Ok(result)
    }

    async fn send_message(&self, chat_id: i32, text: String) -> Result<bool, reqwest::Error> {
        let params = [("chat_id", chat_id.to_string()), ("text", text)];

        let request = self
            .client
            .post(format!(
                "https://api.telegram.org/bot{}/sendMessage",
                (*self.settings).telegram_token
            ))
            .query(&params)
            .send()
            .await?;

        let text_result = request.text().await?;

        let mut result = true;

        if text_result.find("\"ok\":false").is_some() {
            result = false;
            warn!("{text_result}")
        }

        Ok(result)
    }
}
