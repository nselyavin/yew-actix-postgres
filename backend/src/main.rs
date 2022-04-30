
#![allow(unused_must_use)]
#[macro_use]
extern crate rbatis;

use actix_web::{middleware::Logger, App,  HttpRequest, HttpServer,  Result};
use log::Level;

mod repositories;
mod models;
mod handlers;
mod config;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    simple_logger::init_with_level(Level::Info).unwrap();

    HttpServer::new(|| {
        App::new().wrap(Logger::default())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
