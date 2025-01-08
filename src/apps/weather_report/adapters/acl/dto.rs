use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Response {
    pub ok: bool,
    pub result: Vec<Rresult>,
}

#[derive(Deserialize, Debug)]
pub struct Rresult {
    pub update_id: i32,
    pub message: Message,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub message_id: i32,
    pub from: From,
    pub chat: Chat,
    pub date: i32,
    pub text: String,
}

#[derive(Deserialize, Debug)]
pub struct From {
    pub id: i32,
    pub is_bot: bool,
    pub first_name: String,
    pub username: Option<String>,
    pub language_code: String,
}

#[derive(Deserialize, Debug)]
pub struct Chat {
    pub id: i32,
    pub first_name: String,
    pub username: Option<String>,
    #[serde(rename = "type")]
    pub chat_type: String,
}

#[derive(Deserialize, Debug)]
pub struct WeatherResponse {
    pub location: Location,
    pub current: Current,
}

#[derive(Deserialize, Debug)]
pub struct Location {
    pub name: String,
    pub region: String,
    pub country: String,
    pub lat: f32,
    pub lon: f32,
    pub tz_id: String,
    pub localtime_epoch: i32,
    pub localtime: String,
}

#[derive(Deserialize, Debug)]
pub struct Current {
    pub last_updated_epoch: i32,
    pub last_updated: String,
    pub temp_c: f32,
    pub temp_f: f32,
    pub is_day: i32,
    pub condition: Condition,
    pub wind_mph: f32,
    pub wind_kph: f32,
    pub wind_degree: i32,
    pub wind_dir: String,
    pub pressure_mb: f32,
    pub pressure_in: f32,
    pub precip_mm: f32,
    pub precip_in: f32,
    pub humidity: i32,
    pub cloud: i32,
    pub feelslike_c: f32,
    pub feelslike_f: f32,
    pub windchill_c: f32,
    pub windchill_f: f32,
    pub heatindex_c: f32,
    pub heatindex_f: f32,
    pub dewpoint_c: f32,
    pub dewpoint_f: f32,
    pub vis_km: f32,
    pub vis_miles: f32,
    pub uv: f32,
    pub gust_mph: f32,
    pub gust_kph: f32,
}

#[derive(Deserialize, Debug)]
pub struct Condition {
    pub text: String,
    pub icon: String,
    pub code: i32,
}
