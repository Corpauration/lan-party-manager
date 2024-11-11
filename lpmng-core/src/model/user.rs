use crate::model::utils::{Email, Phone, Username, ValidString};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
    pub phone: String,
    pub role: String,
    pub is_allowed: bool,
}

impl User {
    pub fn into_view(self) -> UserView {
        UserView {
            id: self.id,
            username: self.username,
            firstname: self.firstname,
            lastname: self.lastname,
            email: self.email,
            phone: self.phone,
            role: self.role,
            is_allowed: self.is_allowed,
        }
    }
}

#[derive(Clone, Deserialize)]
pub struct UserInput {
    pub username: Username,
    pub firstname: ValidString,
    pub lastname: ValidString,
    pub email: Email,
    pub password: ValidString,
    pub phone: Phone,
}

impl UserInput {
    pub fn into_unchecked(self) -> UserInputUnchecked {
        UserInputUnchecked {
            username: self.username.into(),
            firstname: self.firstname.into(),
            lastname: self.lastname.into(),
            email: self.email.into(),
            password: self.password.into(),
            phone: self.phone.into(),
        }
    }
}

#[derive(Clone)]
pub struct UserInputUnchecked {
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
    pub phone: String,
}

#[derive(Clone, Serialize)]
pub struct UserView {
    pub id: Uuid,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub phone: String,
    pub role: String,
    pub is_allowed: bool,
}

#[derive(Clone, Deserialize)]
pub struct UserPatch {
    pub id: Uuid,
    pub username: Option<Username>,
    pub firstname: Option<ValidString>,
    pub lastname: Option<ValidString>,
    pub email: Option<Email>,
    pub phone: Option<Phone>,
    pub role: Option<ValidString>,
    pub is_allowed: Option<bool>,
}
