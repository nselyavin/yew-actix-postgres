use crate::config::crypto::{gen_jwt, Claims};
use crate::handlers::HandlersError;
use crate::models::medicine::Medicine;
use crate::models::user::{UserInfo, UserLogin, UserSignup, UserToken};
use crate::repositories::medicine_repository;
use crate::repositories::user_repository;
use crate::AppState;
use actix_web::cookie::time::{Duration, OffsetDateTime};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use actix_web::{http::header, http::header::HeaderName, http::header::HeaderValue};
use rbatis::crud::CRUD;

pub fn major_scope() -> actix_web::Scope {
    web::scope("").service(signup).service(login).service(info)
}

#[post("/signup")]
async fn signup(
    _req: HttpRequest,
    _data: web::Json<UserSignup>,
    _state: web::Data<AppState>,
) -> impl Responder {
    let user_data: UserSignup = _data.into_inner();
    match user_repository::create(&user_data, _state.rb.as_ref(), _state.sflake.as_ref()).await {
        Some(_) => HttpResponse::Ok().finish(),
        None => HttpResponse::Conflict().finish(),
    }
}

#[post("/login")]
async fn login(
    _req: HttpRequest,
    _data: web::Json<UserLogin>,
    _state: web::Data<AppState>,
) -> impl Responder {
    log::info!("Try login: {}, {}", _data.email, _data.password);
    let user = user_repository::find_by_email(&_data.email, _state.rb.as_ref()).await;

    if let None = user {
        let msg = format!("User not found by email {}", _data.email);
        return HttpResponse::BadRequest().body(HandlersError::new_str(msg));
    }

    let user = user.unwrap();

    if (bcrypt::verify(_data.password.as_str(), user.password.as_str()).unwrap()) {
        let token = gen_jwt(user.id, _state.key.as_ref()).await;

        match token {
            Ok(token_str) => {
                let body = serde_json::to_string(&UserToken { token: token_str }).unwrap();

                HttpResponse::Ok().body(body)
            }
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
async fn info(
    _req: HttpRequest,
    _uuid: web::Path<String>,
    _state: web::Data<AppState>,
) -> impl Responder {
    match medicine_repository::find_by_id(_uuid.to_string(), _state.rb.as_ref()).await {
        Some(medicine) => {
            let body = serde_json::to_string(&medicine).unwrap();

            HttpResponse::Ok().body(body)
        }
        None => HttpResponse::NotFound().finish(),
    }
}
