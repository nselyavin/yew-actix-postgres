
use actix_web::{web, get, post, HttpRequest, HttpResponse, Responder};

use crate::models::user::UserSignup;
use crate::repositories::user_repository;

pub fn major_scope() -> actix_web::Scope{
    web::scope("")
        .service(signup)
        .service(login)
        .service(info)
}

#[post("/signup")]
async fn signup(_data: web::Json<UserSignup>) -> impl Responder{
    log::info!("Trying signup '{}'", _data.username);

    //user_repository.create(_data.try_into::<UserSignup>(), );
    
    
    HttpResponse::Ok()
}

#[post("/login")]
async fn login(_data: web::Json<UserSignup>) -> impl Responder{
    todo!("Repository.getUser");

    // create token 
    HttpResponse::Ok()
}

#[get("/info/{id}")]
async fn info(_path: web::Path<String>) -> impl Responder{
    todo!("Repository.getUser");
    HttpResponse::Ok()
}