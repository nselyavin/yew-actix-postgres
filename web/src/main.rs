#![allow(unused_must_use)]
#[macro_use]
extern crate rbatis;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpRequest, HttpServer, Result};
use dotenv::dotenv;
use log::Level;
use std::env;
use std::sync::Arc;

mod config;
mod handlers;
mod models;
mod repositories;

use handlers::major_handlers::major_scope;
use handlers::user_handlers::user_scope;
use handlers::medicine_handlers::medicine_scope;
use handlers::creator_handlers::creator_scope;
use rbatis::{plugin::snowflake::Snowflake, rbatis::Rbatis};

//use handlers::medicine_handlers::medicine_scope;

#[derive(Clone)]
pub struct AppState {
    rb: Arc<Rbatis>,
    key: Arc<String>,
    sflake: Arc<Snowflake>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    simple_logger::init_with_level(Level::Info).unwrap();

    log::info!("Load config:");
    dotenv::from_filename("/usr/app/.env").ok();
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }

    let rb = Rbatis::new();
    log::info!("Link database");
    rb.link(env::var("DATABASE_URL").unwrap().as_str())
        .await
        .expect("faile to link database");

    let app_state = AppState {
        rb: Arc::new(rb),
        key: Arc::new(env::var("KEY").unwrap()),
        sflake: Arc::new(Snowflake::new(161476480000, 1, 1)),
    };

    log::info!("Start server");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://fr.localhost.com:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allow_any_header();

        App::new()
            .app_data(web::Data::new(app_state.to_owned()))
            .wrap(Logger::new("%{FOO}i"))
            .wrap(Logger::new("%{FOO}o"))
            .wrap(Logger::default())
            .wrap(cors)
            .service(user_scope())
            .service(medicine_scope())
            .service(creator_scope())
            .service(major_scope())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
