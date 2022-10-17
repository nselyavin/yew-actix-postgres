use actix_web::{web, HttpRequest, Responder, HttpResponse, get, HttpResponseBuilder, http::StatusCode};
use actix_web_httpauth::{extractors::bearer::BearerAuth};
use chrono::Utc;
use rbatis::crud::CRUD;
use crate::{AppState, config::crypto::{unwrap_jwt, verify_jwt}, models::user::{UserInfo, User}};

pub fn user_scope() -> actix_web::Scope{
    web::scope("/user")
        .service(user_detail)
}

#[get("/detail")]
async fn user_detail(_req: HttpRequest, _state: web::Data<AppState>, _token: BearerAuth) -> impl Responder{
    log::info!("User detail: {}", _token.token());

    let claims = verify_jwt(_token.token().to_string(), _state.key.as_ref()).await;
    if let Err(status) = claims{
        return HttpResponseBuilder::new(StatusCode::from_u16(status).unwrap()).finish();
    }
    let claims = claims.unwrap();

    let user: Result<User, _> = _state.rb.fetch_by_column("id", claims.id).await;

    match user{
        Ok(user_data) => {

            let body = serde_json::to_string(&UserInfo{
                id: user_data.id,
                username: user_data.username,
                email: user_data.email,
                created_date: user_data.created_date.to_string(),
            }).unwrap();

            HttpResponse::Ok().body(body)
        },
        Err(_) => {
            HttpResponse::Unauthorized().finish()        
        },
    }
}