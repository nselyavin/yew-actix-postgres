
use actix_web::cookie::Cookie;
use actix_web::cookie::time::Duration;
use actix_web::{web, get, post, HttpRequest, HttpResponse, Responder};
use std::env;
use crate::handlers::HandlersError;
use crate::models::user::{UserSignup, UserInfo, UserLogin};
use crate::repositories::user_repository::{self};
use crate::AppState;
use crate::config::crypto::Claims;

pub fn major_scope() -> actix_web::Scope{
    web::scope("")
        .service(signup)
        .service(login)
        .service(info)
}

#[post("/signup")]
async fn signup(_req: HttpRequest, _data: web::Json<UserSignup>, _state: web::Data<AppState>) -> impl Responder{
    let user_data: UserSignup = _data.into_inner();
    match user_repository::create(&user_data, _state.rb.as_ref(), _state.sflake.as_ref()).await{
        Some(user) => {
            let user_info = UserInfo{
                username: user.username,
                email: user.email,
                created_date: user.created_date.to_string(),
            };

            user_info.respond_to(&_req)
        },
        None => {
            HttpResponse::BadRequest().finish()
        },
    }    
}

#[post("/login")]
async fn login(_req:HttpRequest, _data: web::Json<UserLogin>, _state: web::Data<AppState>) -> impl Responder{
    let user = user_repository::find_by_email(&_data.email, _state.rb.as_ref()).await;

    if let None = user{
        let msg = format!("User not found by email {}", _data.email);
        return HttpResponse::BadRequest().body(HandlersError::new_str(msg))
    }

    let user = user.unwrap();

    if (bcrypt::verify(_data.password.as_str(), user.password.as_str()).unwrap()){
            let user_info = UserInfo{
                username: user.username,
                email: user.email,
                created_date: user.created_date.to_string(),
            };

            let token = Claims::gen_jwt(user.id, _state.key.as_ref()).await;

            match token{
                Ok(token_str) => {
                    let token_cookie = Cookie::build("pharma-token", token_str)
                        .http_only(true)
                        .max_age(Duration::days(env::var("COOKIE_MAX_AGE").unwrap().parse::<i64>().unwrap()))
                        .finish();

                    let mut respond = user_info.respond_to(&_req);
                    respond.add_cookie(&token_cookie);
                    respond
                },
                Err(err) => {
                    log::error!("Failed create token: {}", err);
                    let msg = format!("Failed create token {}", err);
                    HttpResponse::InternalServerError().body(HandlersError::new_str(msg))
                }
            }
    } else {
            log::info!("Bad password for user {}", user.username);
            let msg = format!("Bad password for user {}", user.username);
            HttpResponse::BadRequest().body(HandlersError::new_str(msg))
            
    }
}

#[get("/info/{id}")]
async fn info(_path: web::Path<String>) -> impl Responder{
    todo!("Repository.getUser");
    HttpResponse::Ok()
}