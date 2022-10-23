use actix_web::{
    http::StatusCode, post, web, HttpRequest, HttpResponse, HttpResponseBuilder, Responder,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{
    config::crypto::verify_jwt, models::medicine::MedicineReg, repositories::medicine_repository,
    AppState,
};

pub fn medicine_scope() -> actix_web::Scope {
    web::scope("/medicine").service(new_medicine)
}

#[post["/new"]]
async fn new_medicine(
    _req: HttpRequest,
    _state: web::Data<AppState>,
    _token: BearerAuth,
    _data: web::Json<MedicineReg>,
) -> impl Responder {
    let claims = verify_jwt(_token.token().to_string(), _state.key.as_ref()).await;

    if let Err(status) = claims {
        return HttpResponseBuilder::new(StatusCode::from_u16(status).unwrap()).finish();
    }

    let medicine_data: MedicineReg = _data.into_inner();
    let medicine = medicine_repository::create(&medicine_data, _state.rb.as_ref()).await;


    match medicine {
        Ok(_) => {
            let body = serde_json::to_string(&medicine).unwrap();
            HttpResponse::Ok().body(body)
        },
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}
