use actix_web::{web, Responder, HttpResponse, HttpRequest, post, HttpResponseBuilder, http::StatusCode};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use rbatis::crud::CRUD;

use crate::{AppState, models::{medicine::MedicineReg, user::User}, config::crypto::verify_jwt, repositories::medicine_repository};

pub fn medicine_scope() -> actix_web::Scope{
    web::scope("/medicine")
        .service(new_medicine)
}

#[post["/new"]]
async fn new_medicine(_req: HttpRequest, _state: web::Data<AppState>, _token: BearerAuth, _data: web::Json<MedicineReg>) -> impl Responder{
    let claims = verify_jwt(_token.token().to_string(), _state.key.as_ref()).await;

    if let Err(status) = claims{
        return HttpResponseBuilder::new(StatusCode::from_u16(status).unwrap()).finish();
    }
    let claims = claims.unwrap();

    let user: Result<User, _> = _state.rb.fetch_by_column("id", claims.id).await;

    match user{
        Ok(user_data) => {
            let medicine_data: MedicineReg = _data.into_inner();
            let medicine = medicine_repository::create(&medicine_data, _state.rb.as_ref(), user_data.username).await;
            
            let body = serde_json::to_string(&medicine).unwrap();
            match medicine{
                Some(_) => HttpResponse::Ok().body(body),
                None => HttpResponse::InternalServerError().finish(),
            }
        },
        Err(_) => {
            HttpResponse::Unauthorized().finish()        
        },
    }
}