pub mod routes;
use actix_http::HttpMessage;
pub use actix_web::{get, post, web,  App, cookie, Error, guard, HttpRequest, HttpResponse, HttpServer, Responder, Result};
pub use actix_web::web::Json;
pub use cookie::Cookie;
extern crate time;
use time::Duration;
pub use serde::{Deserialize, Serialize};
pub use std::sync::Mutex;


#[derive(Serialize, Deserialize)]
struct Answer{
    username: String,
    age: i32,
}

async fn index(_req: HttpRequest) -> impl Responder{
    let cookie_val = _req.cookie("name2");

    if let Some(val) = cookie_val{
        println!("name2: {}", val.value())
    }

    let body = serde_json::to_string(
        &Answer {
            username: String::from("Leha"), 
            age: 12}
    ).unwrap();

    let cookie = Cookie::build("name2", "value2")
    .path("/")
    .secure(true)
    .http_only(true)
    .max_age(Duration::minutes(30))
    .finish();

    HttpResponse::Ok()
    .set_header("X-Test", "value")
    .set_header("Content-Type", "application/json")
    .cookie(cookie)
    .body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let serv = HttpServer::new(|| {
        App::new().route("/", web::get().to(index))
    }).workers(4)
    .bind("127.0.0.1:8080")?
    .run();

    serv.await
}
