use reqwasm::http::{Method, Request};
use serde::{de::DeserializeOwned, ser::Error};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use std::result;
use std::result::Result;
use std::sync::Arc;
use web_sys::RequestMode;

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


async fn newGetRequest(url: &str)-> Request{
    Request::new(url)
}

async fn newRequest<U>(url: &str, method: Method, body: &U)-> Request
where     
    U: Serialize + Debug + ?Sized,
{
    let ser_body = serde_json::to_string(body).unwrap();
    Request::new(url).method(method).body(ser_body)
}


async fn request<T, U>(url: &str, method: Method, body: Option<&U>) -> Result<T, u16>
where
    T: DeserializeOwned + Debug + Send,
    U: Serialize + Debug + ?Sized,
{
    let mut ser = "".to_string();
    if let Some(val) = body {
        ser = serde_json::to_string(&val).unwrap();
    }

    let mut req = match method{
        Method::GET => newGetRequest(url).await,
        Method::POST => newRequest(url, method, &body.unwrap()).await,
        _ => return Err(404),
    };
        
    let resp = req.send().await.unwrap();

    match resp.json::<T>().await {
        Ok(val) => Ok(val),
        Err(err) => Err(resp.status()),
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
    let res = request::<UserInfo, UserLogin>("/login", Method::POST, Some(&data)).await;
    res
}

pub fn POST_signup() -> result::Result<(), u16> {
    Ok(())
}

pub fn GET_user_detail() -> result::Result<UserInfo, u16> {
    // Ok(UserInfo::default())
    Err(404)
}
