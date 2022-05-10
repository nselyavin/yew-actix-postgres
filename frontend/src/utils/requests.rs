use futures::task::LocalSpawn;
use reqwest::header::HeaderValue;
use serde::{de::DeserializeOwned, ser::Error};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsValue, JsCast};
use gloo_storage::{LocalStorage, Storage, errors::StorageError};
use std::fmt::Debug;
use std::result;
use std::result::Result;
use std::sync::Arc;


use crate::models::{item::Item, user::*};

struct ReqResult<T> {
    code: Arc<u16>,
    body: Arc<T>,
}

#[derive(Clone, PartialEq, Deserialize)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}


pub fn get_token()-> Option<String>{
    let token = LocalStorage::get("pharmacy-token");
    match token{
        Ok(tok) => Some(tok),
        Err(err) => {
            log::info!("Failed get token: {}", err);
            None
        },
    }
}

pub fn set_token(token: Option<&HeaderValue>){
    log::info!("try set token: {:?}", token);
    if let Some(tok) = token{
        LocalStorage::set("pharmacy-token", tok.to_str().unwrap()).unwrap();
    }
}

pub fn remove_token(){
    LocalStorage::delete("pharmacy-token");
}


async fn request<T, U>(url: &str, method: reqwest::Method, body: Option<&U>) -> Result<T, u16>
where
T: DeserializeOwned + Debug + Send,
    U: Serialize + Debug + ?Sized,
{
    let allow_body = method == reqwest::Method::POST || method == reqwest::Method::PUT;
    let mut req = reqwest::Client::new()
        .request(method, format!("http://localhost:8080{}", url))
        .header("Content-Type", "application/json");
    

    if let Some(token) = get_token(){
        req = req.bearer_auth(token);
    }

    if allow_body{
        log::info!("Body add");
        req = req.json(&body.unwrap());
    }
    let res_resp = req.send().await;

    match res_resp {
        Ok(resp) => {
        log::info!("Response: {:?}", resp);

        match resp.status().is_success(){
            true => {
                log::info!("Headers: {:?}", resp.headers());
                set_token(resp.headers().get("pharmacy-token"));

                match resp.json::<T>().await{
                    Ok(data) => Ok(data),
                    Err(_) => {
                        log::info!("Failed parse body");
                        Err(0)
                    },
                }
            },
            false => Err(resp.status().as_u16())    
        }
    },
        Err(err) => {
            log::error!("Failed request: {}", err);
            Err(0)
        }
    }
}

pub fn GET_items() -> Result<Vec<Item>, u16> {
    let mut items = vec![];

    for i in 0..7 {
        items.push(Item::new(i));
    }
    Ok(items)
}

pub fn GET_items_last(amount: i32) -> Result<Vec<Item>, u16> {
    let mut items = Vec::new();

    Ok(items.to_owned())
}

pub fn GET_item(id: i64) -> Result<Item, i16> {
    Ok(Item::new(id))
}

pub async fn POST_login(data: &UserLogin) -> Result<UserInfo, u16> {
    let res = request::<UserInfo, UserLogin>("/login", reqwest::Method::POST, Some(&data)).await;
    res
}

pub fn POST_signup() -> result::Result<(), u16> {
    Ok(())
}

pub fn GET_user_detail() -> result::Result<UserInfo, u16> {
    // Ok(UserInfo::default())
    Err(404)
}
