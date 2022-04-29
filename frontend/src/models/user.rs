
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User{
    pub username: String,
    pub email: String,
}

impl User{
    pub fn new()-> User{
        User{
            username: "Fume".to_string(),
            email: "email".to_string(),
        }
    }
}