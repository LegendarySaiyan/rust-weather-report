#[derive(Clone, Copy, Debug)]
pub struct LastUpdate {
    pub last_update_id: Option<i32>,
    pub was_used: bool,
}
