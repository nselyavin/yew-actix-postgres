use actix_web::{
    body::{BoxBody, MessageBody},
    http::header::ContentType,
    HttpResponse, Responder,
};
use rbatis::{crud_table, DateTimeNative};
use serde::{Deserialize, Serialize};

#[crud_table(table_name: t_user)]
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub created_date: DateTimeNative,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub created_date: String,
}

#[derive(Deserialize, Debug, Serialize, PartialEq, Clone)]
pub struct UserToken {
    pub token: String,
}

#[derive(Deserialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserSignup {
    pub email: String,
    pub username: String,
    pub password: String,
}

impl Responder for UserInfo {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
