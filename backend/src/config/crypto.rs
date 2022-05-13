use actix_web::web::{self, block};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use std::{result::Result, task::Poll};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub id: u64,
    pub exp: i64,
}

pub async fn gen_jwt(id: u64, secret: &String) -> Result<String, jsonwebtoken::errors::Error> {
    let jwt_key = secret.clone();

    block(move || {
        let header = Header::default();
        let encoding_key = EncodingKey::from_secret(jwt_key.as_bytes());
        let exp = Utc::now()
            + Duration::days(env::var("COOKIE_MAX_AGE").unwrap().parse::<i64>().unwrap());

        let claim = Claims {
            id: id,
            exp: exp.timestamp(),
        };

        encode(&header, &claim, &encoding_key)
    })
    .await
    .unwrap()
}

pub async fn unwrap_jwt(
    _token: String,
    secret: &String,
) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let jwt_key = secret.clone();
    block(move || {
        let decoding_key = DecodingKey::from_secret(jwt_key.as_bytes());

        decode::<Claims>(&_token, &decoding_key, &Validation::default())
    })
    .await
    .unwrap()
}
