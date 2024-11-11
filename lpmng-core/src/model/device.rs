use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Device {
    pub id: Uuid,
    pub mac: String,
    pub user_id: Uuid,
    pub internet: bool,
    pub date_time: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct DeviceInput {
    pub user_id: Uuid,
}

pub struct NewDevice {
    pub mac: String,
    pub user_id: Uuid,
    pub internet: bool,
    pub date_time: NaiveDateTime,
}
