use serde::{Deserialize, Serialize};
use validator::Validate;
use wasm_bindgen::JsValue;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Default)]
pub struct UserInfo {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub created_date: String,
}

#[derive(Deserialize, Debug, Clone, Validate, Serialize, PartialEq)]
pub struct UserLogin {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 3))]
    pub password: String,
}

impl UserLogin {
    pub fn default() -> UserLogin {
        UserLogin {
            email: String::default(),
            password: String::default(),
        }
    }

    pub fn is_empty(&self)->bool{
        !(self.email.len() > 0 && self.password.len() > 0)
    }
}

#[derive(Deserialize, Debug, Validate, Clone, Serialize, PartialEq)]
pub struct UserSignup {
    #[validate(email)]
    pub email: String,
    pub username: String,
    #[validate(length(min = 3))]
    pub password: String,
}

impl UserSignup {
    pub fn default() -> UserSignup {
        UserSignup {
            username: String::default(),
            email: String::default(),
            password: String::default(),
        }
    }

    pub fn is_empty(&self)->bool{
        !(self.email.len() > 0 && self.username.len() > 0 && self.password.len() > 0)
    }
}

#[derive(Deserialize, Debug, Serialize, PartialEq, Clone)]
pub struct UserToken{
    pub token: String
}