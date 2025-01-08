use crate::apps::weather_report::framework_and_drivers::clients;
use crate::apps::weather_report::framework_and_drivers::repositories::last_update_repo::LastUpdateRepo;
use crate::apps::weather_report::usecases::update_usecases::UpdateUsecase;
use crate::infrastructure::settings::base::Settings;
use clients::{telegram_client::TelegramClient, weather_client::WeatherClient};

pub struct ApplicationIoC<'a> {
    settings: &'a Settings,
    pub update_usecase: UpdateUsecase<'a>,
}
impl<'a> ApplicationIoC<'a> {
    pub fn new<'b: 'a>(settings: &'b Settings) -> Self {
        Self {
            settings: settings,
            update_usecase: UpdateUsecase {
                repo: LastUpdateRepo,
                telegram_client: TelegramClient {
                    settings: settings,
                    client: reqwest::Client::new(),
                },
                weather_client: WeatherClient {
                    settings: settings,
                    client: reqwest::Client::new(),
                },
            },
        }
    }
}
