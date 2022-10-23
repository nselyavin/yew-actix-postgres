use actix_web::{web, post, HttpRequest, Responder, HttpResponse};

use crate::{models::pharmacy::{Pharmacy, self}, AppState};





pub fn pharmacy_scope() -> actix_web::Scope {
    web::scope("")
}

#[post("/new")]
async fn new_pharmacy(
    _req: HttpRequest,
    _data: web::Json<Pharmacy>,
    _state: web::Data<AppState>,
) -> Responder {
    let pharmacy_data = _data.into_inner();

    match pharmacy_repository::create(){
        Ok(_) => HttpResponse::Ok().finish(),
        Err(msg) => HttpResponse::BadRequest().body(msg)
    }
}