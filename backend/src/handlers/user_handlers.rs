use actix_web::{web, HttpRequest, Responder, HttpResponse, get, post};
use crate::AppState;

pub fn user_scope() -> actix_web::Scope{
    web::scope("/users")
        .service(users)
}

#[get("")]
async fn users(_req: HttpRequest, _state: web::Data<AppState>) -> impl Responder{
    log::info!("List of users");
    
    HttpResponse::Ok()
}