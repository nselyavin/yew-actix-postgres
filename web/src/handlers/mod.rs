use actix_web::{body::BoxBody, http::header::ContentType, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HandlersError {
    pub msg: String,
}

impl HandlersError {
    pub fn new_str(msg: String) -> String {
        serde_json::to_string(&HandlersError {
            msg: msg.to_owned(),
        })
        .unwrap()
    }
}

pub mod major_handlers;
pub mod medicine_handlers;
pub mod user_handlers;
pub mod creator_handlers;
pub mod pharmacy_handler;
