use cookie::Expiration;
use cookie::time::OffsetDateTime;
// use reqwasm::http::{Method, Request};
use serde::{de::DeserializeOwned, ser::Error};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsValue, JsCast};
use wasm_cookies::CookieOptions;
use std::fmt::Debug;
use std::result;
use std::result::Result;
use std::sync::Arc;
use web_sys::{Request, RequestInit, Response, RequestMode, RequestCredentials};


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


async fn new_get_request(url: &str)-> Request
{
    Request::new_with_str(format!("http://localhost:8080{}", url).as_str()).unwrap()
}
    
async fn new_request<U>(url: &str, method: &str, body: &U)-> Request
where     
    U: Serialize + Debug + ?Sized,
{
    let ser_body = serde_json::to_string(body).unwrap();
    let js_body = wasm_bindgen::JsValue::from_str(ser_body.as_str());
    let mut opts = web_sys::RequestInit::new();
    opts.method(method);
    opts.mode(RequestMode::Cors);
    opts.credentials(RequestCredentials::SameOrigin);
    opts.body(Some(&js_body));

    let req = Request::new_with_str_and_init(format!("http://localhost:8080{}", url).as_str(), &opts).unwrap();
    req
}


async fn request<T, U>(url: &str, method: &str, body: Option<&U>) -> Result<T, u16>
where
T: DeserializeOwned + Debug + Send,
    U: Serialize + Debug + ?Sized,
{
    let mut req = match method{
        "get" => new_get_request(url).await,
        "post" => new_request(url, method, &body.unwrap()).await,
        _ => return Err(404),
    };
    req.headers().set("Content-Type", "application/json").unwrap();
        
    let window = web_sys::window().unwrap();
    let resp = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&req)).await.unwrap();
    let resp: Response = resp.dyn_into().unwrap();
    log::info!("Resp: {:?}", resp.headers().get("set-cookie"));
    
    // if let Some(cook) = resp.headers().get("set-cookie"){
    //     let token = cookie::Cookie::parse(cook).unwrap(); 

    //     let options = wasm_cookies::CookieOptions{
    //         path: token.path(),
    //         domain: token.domain(),
    //         expires: Some(token.expires_datetime().unwrap().to_string()),
    //         secure: token.secure().unwrap(),
    //         same_site: wasm_cookies::SameSite::default(),
    //     };

    //     log::info!("Try set token: {:?}", token);
    //     wasm_cookies::set_raw(token.name(), token.value(), &options)
    // }
    
    // match resp.json::<T>().await {
    //     Ok(val) => Ok(val),
    //     Err(err) => Err(resp.status()),
    // }

    Err(100)
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
    let res = request::<UserInfo, UserLogin>("/login", "post", Some(&data)).await;
    res
}

pub fn POST_signup() -> result::Result<(), u16> {
    Ok(())
}

pub fn GET_user_detail() -> result::Result<UserInfo, u16> {
    // Ok(UserInfo::default())
    Err(404)
}
