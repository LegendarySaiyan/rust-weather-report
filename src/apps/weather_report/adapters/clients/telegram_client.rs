use crate::apps::weather_report::adapters::acl::dto::Response;

pub trait ABCTelegramClient {
    async fn get_updates(&self, offset: Option<i32>) -> Result<Response, reqwest::Error>;
    async fn send_message(&self, chat_id: i64, text: String) -> Result<bool, reqwest::Error>;
}
