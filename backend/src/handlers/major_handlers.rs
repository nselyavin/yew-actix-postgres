
use actix_web::{web, get, post, HttpRequest, HttpResponse, Responder};

use crate::models::user::{UserSignup};
use crate::repositories::user_repository::{self, create};
use crate::AppState;

pub fn major_scope() -> actix_web::Scope{
    web::scope("")
        .service(signup)
        .service(login)
        .service(info)
}

#[post("/signup")]
async fn signup(_req: HttpRequest, _data: web::Json<UserSignup>, _state: web::Data<AppState>) -> impl Responder{
    log::info!("Trying signup '{}'", _data.username);

    let user_data: UserSignup = _data.into_inner();
    match user_repository::create(&user_data, _state.rb.as_ref(), _state.sflake.as_ref()).await{
        Ok(user) => {
            user.respond_to(&_req)
        },
        Err(_) => {
            HttpResponse::BadRequest().finish()
        },
    }    
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