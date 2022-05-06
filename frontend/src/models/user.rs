use serde::{Deserialize, Serialize};
use validator::Validate;
use wasm_bindgen::JsValue;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct UserInfo {
    pub username: String,
    pub email: String,
    pub created_date: String,
}

impl UserInfo {
    pub fn default() -> UserInfo {
        UserInfo {
            username: "Fume".to_string(),
            email: "email".to_string(),
            created_date: "no time".to_string(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, Validate, Serialize, PartialEq)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

impl UserLogin {
    pub fn default() -> UserLogin {
        UserLogin {
            email: "test@test.tu".to_string(),
            password: "1324".to_string(),
        }
    }
}

#[derive(Deserialize, Debug, Validate, Serialize, PartialEq)]
pub struct UserSignup {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl UserSignup {
    pub fn default() -> UserSignup {
        UserSignup {
            username: "test".to_string(),
            email: "test@test.tu".to_string(),
            password: "1324".to_string(),
        }
    }
}
