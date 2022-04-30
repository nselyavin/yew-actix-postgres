use actix_web::{web, HttpRequest, Responder, HttpResponse, get, post};


pub fn user_scope() -> actix_web::Scope{
    web::scope("/users")
        .service(users)
}

#[get("")]
async fn users(_req: HttpRequest) -> impl Responder{
    log::info!("List of users");
    HttpResponse::Ok()
}