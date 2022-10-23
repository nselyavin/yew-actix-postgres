use actix_web::{
    http::StatusCode, post, web, HttpRequest, HttpResponse, HttpResponseBuilder, Responder, get,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use rbatis::crud::CRUD;

use crate::{
    config::crypto::verify_jwt,
    models::{creator::Creator},
    repositories::creator_repository,
    AppState
};

pub fn creator_scope() -> actix_web::Scope {
    web::scope("/creator").service(new_creator)
}

#[post("/new")]
async fn new_creator(
    _req: HttpRequest, 
    _state: web::Data<AppState>, 
    _token: BearerAuth, 
    _data: web::Json<Creator>
) -> impl Responder{
    log::warn!("{:?}", _req);
    let claims = verify_jwt(_token.token().to_string(), _state.key.as_ref()).await;

    if let Err(status) = claims {
        return HttpResponseBuilder::new(StatusCode::from_u16(status).unwrap()).finish();
    }

    let creator_data: Creator = _data.into_inner();
    let creator = creator_repository::create(&creator_data, _state.rb.as_ref(), _state.sflake.as_ref()).await;

    match creator {
        Ok(_) => {
            let body = serde_json::to_string(&creator).unwrap();
            HttpResponse::Ok().body(body)
        },
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

#[get("/get/{id}")]
async fn get_creator(
    _req: HttpRequest, 
    _id: web::Path<(u64, )>,
    _state: web::Data<AppState>, 
) -> impl Responder{
    let id: u64 = _id.into_inner().0;
    let creator = creator_repository::find_by_id(id, _state.rb.as_ref()).await;

    match creator {
        Ok(_) => {
            let body = serde_json::to_string(&creator).unwrap();
            HttpResponse::Ok().body(body)
        },
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

