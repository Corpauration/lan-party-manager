use crate::model::utils::{Username, ValidString};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct Login {
    pub username: Username,
    pub password: ValidString,
}

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub biscuit: String,
    pub role: String,
    pub user_id: Uuid,
}
