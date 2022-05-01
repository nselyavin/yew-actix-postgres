use serde::{Serialize, Deserialize};
use actix_web::web::{self, block};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use chrono::{Duration, Utc};
use std::{result::Result, task::Poll};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims{
    pub id: u64,
    pub exp: i64,
}

impl Claims{
    pub async fn gen_jwt(id: u64, secret: &String)-> Result<String, jsonwebtoken::errors::Error> {        
        let jwt_key = secret.clone();
        
        block(move ||{
            let header = Header::default();
            let encoding_key = EncodingKey::from_secret(jwt_key.as_bytes());
            let exp = Utc::now() + Duration::days(env::var("COOKIE_MAX_AGE").unwrap().parse::<i64>().unwrap());
            
            let claim = Claims{
                id: id,
                exp: exp.timestamp()
            };
            
            encode(&header, &claim, &encoding_key)
        })
        .await.unwrap()
    }
    
    // TODO, implement token logout system
}