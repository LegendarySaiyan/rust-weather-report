use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Response {
    pub ok: bool,
    pub result: Vec<Update>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Update {
    MessageUpdate(MessageResult),
    ChatMemberUpdate(MyChatMemberResult),
}
impl HasUpdateId for Update {
    fn update_id(&self) -> i32 {
        match self {
            Update::MessageUpdate(message_result) => message_result.update_id(),
            Update::ChatMemberUpdate(my_chat_member_result) => my_chat_member_result.update_id(),
        }
    }
}

pub trait HasUpdateId {
    fn update_id(&self) -> i32;
}

#[derive(Deserialize, Debug)]
pub struct MessageResult {
    pub update_id: i32,
    pub message: Message,
}
impl HasUpdateId for MessageResult {
    fn update_id(&self) -> i32 {
        self.update_id
    }
}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub message_id: i64,
    pub from: From,
    pub chat: Chat,
    pub date: i64,
    pub text: String,
    pub entities: Option<Vec<Entities>>,
}

#[derive(Deserialize, Debug)]
pub struct From {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub username: Option<String>,
    pub language_code: Option<String>,
    #[serde(default)]
    pub is_premium: bool, // Также опционально
}

#[derive(Deserialize, Debug)]
pub struct Chat {
    pub id: i64,
    pub first_name: String,
    pub username: Option<String>,
    #[serde(rename = "type")]
    pub chat_type: String,
}

#[derive(Deserialize, Debug)]
pub struct MyChatMemberResult {
    pub update_id: i32,
    pub my_chat_member: MyChatMember,
}
impl HasUpdateId for MyChatMemberResult {
    fn update_id(&self) -> i32 {
        self.update_id
    }
}

#[derive(Deserialize, Debug)]
pub struct MyChatMember {
    pub chat: Chat,
    pub date: i64,
    pub from: From,
    pub old_chat_member: OldChatMember,
    pub new_chat_member: NewChatMember,
}

#[derive(Deserialize, Debug)]
pub struct OldChatMember {
    pub user: User,
    pub status: String,
}

#[derive(Deserialize, Debug)]
pub struct NewChatMember {
    pub user: User,
    pub status: String,
    pub until_date: Option<i64>,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub username: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Entities {
    pub offset: i64,
    pub length: i64,
    #[serde(rename = "type")]
    pub text_type: String,
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
    pub code: i64,
}
