use log::{debug, info, warn};

use crate::apps::weather_report::adapters::clients::telegram_client::ABCTelegramClient;
use crate::apps::weather_report::adapters::use_cases::update_usecases::ABCUpdatesUsecase;

use crate::apps::weather_report::framework_and_drivers::clients::telegram_client::TelegramClient;
use crate::apps::weather_report::framework_and_drivers::clients::weather_client::WeatherClient;
use crate::apps::weather_report::framework_and_drivers::repositories::last_update_repo::LastUpdateRepo;

use crate::apps::weather_report::adapters::clients::weather_client::ABCWeatherClient;
use crate::toolbox::utils::{is_english, is_russian};

use crate::apps::weather_report::adapters::acl::dto::Rresult;
use crate::apps::weather_report::adapters::repositories::abc_repo::ABCLastUpdateRepo;

const MM: f32 = 0.75006168;

enum Lang {
    RUS,
    ENG,
}

pub struct UpdateUsecase<'a> {
    pub repo: LastUpdateRepo,
    pub telegram_client: TelegramClient<'a>,
    pub weather_client: WeatherClient<'a>,
}
impl<'a> ABCUpdatesUsecase for UpdateUsecase<'a> {
    async fn adopt_updates(&self) {
        let last_update = self.repo.get_last_update().await;

        let updates = self
            .telegram_client
            .get_updates(last_update.last_update_id)
            .await
            .unwrap();

        self.process_message(updates.result).await;
    }
}
impl<'a> UpdateUsecase<'a> {
    async fn process_message(&self, messages: Vec<Rresult>) {
        //! Telegram returns updates in a array. If you pass the offset, it will return data starting frow the offset.
        //! We pass the update_id of the last entity, that we processed. So there is no need to work with it in the
        //! next update. So we just start iterating from the second elem.

        let first_message = messages.first();
        let last_message = messages.last();

        let mut last_update = self.repo.get_last_update().await;

        let mut messages_slice = &messages[..];

        if let Some(message) = first_message {
            if last_update
                .last_update_id
                .is_some_and(|x| x == message.update_id)
            {
                if !last_update.was_used {
                    last_update.was_used = true;
                }
                messages_slice = &messages[1..];
            }
        }

        if let Some(message) = last_message {
            last_update.last_update_id = Some(message.update_id.clone());
            last_update.was_used = false;
        }

        for message in messages_slice.iter() {
            if message.message.text == "/command1" || message.message.text == "/start" {
                let text = String::from("Напишите название города/write the city name");
                self.send_message(message.message.chat.id.clone(), text)
                    .await;
            } else {
                let current = self.get_current(message.message.text.clone()).await;

                match current {
                    Err(_) => {
                        warn!("Couldn't get the info about {}", message.message.text);
                        self.send_message(
                            message.message.chat.id.clone(),
                            format!("Couldn't get the info about {}", message.message.text),
                        )
                        .await;
                    }
                    Ok(info) => {
                        self.send_message(message.message.chat.id.clone(), info)
                            .await;
                    }
                }
            }
        }

        self.repo.save(last_update).await;
    }
    async fn get_current(&self, city: String) -> Result<String, reqwest::Error> {
        let lang: Lang;

        if is_english(&city) {
            lang = Lang::ENG;
        } else if is_russian(&city) {
            lang = Lang::RUS;
        } else {
            warn!("Unexpected lang! Using english as default!");
            lang = Lang::ENG;
        };

        let weather_data = self.weather_client.get_current(city).await?;

        let current_temperature = weather_data.current.temp_c;
        let feels_like = weather_data.current.feelslike_c;
        let pressure_mm = weather_data.current.pressure_mb * MM;
        let precip_mm = weather_data.current.precip_mm;
        let humidity = weather_data.current.humidity;
        let vis_km = weather_data.current.vis_km;
        let wind = weather_data.current.wind_kph;
        let city = weather_data.location.name;
        let ultraviolet = weather_data.current.uv;

        let weather_string: String;

        match lang {
            Lang::RUS => {
                weather_string = format!(
                    "Текущая температура в городе {city}: {current_temperature}°C, ощущается как {feels_like}°C.\n\
                     Атмосферное давление: {pressure_mm} мм ртутного столба.\n\
                     Осадки: {precip_mm} мм.\n\
                     Влажность: {humidity}%.\n\
                     Ветер: {wind} км/ч.\n\
                     УФ-индекс: {ultraviolet}.\n\
                     Видимость: {vis_km} км."
                )
            }
            Lang::ENG => {
                weather_string = format!(
                    "Current temperture in the city {city}: is {current_temperature} °C, feels like {feels_like}.\n\
                     Pressure is {pressure_mm} mm.\n\
                     Precip is {precip_mm} мм.\n\
                     Humidity is {humidity}%.\n\
                     Wind is {wind} kp/h.\n\
                     Ultra violet index is {ultraviolet}.\n\
                     Visibility is {vis_km} in km."
                )
            }
        }
        Ok(weather_string)
    }
    async fn send_message(&self, chat_id: i32, text: String) {
        let result = self.telegram_client.send_message(chat_id, text).await;
        match result {
            Error => warn!("Couldn't send message"),
            Ok(..) => (),
        }
    }
}
