use std::fmt::{Debug};
use std::result::Result;
use gloo_net::http::Request;
use serde::{ser::Error, de::DeserializeOwned};
use std::result;
use crossbeam::channel;
use tokio::runtime::Handle;
//use web_sys::{Request, RequestInit};

use crate::models::{item::Item, user::*};

enum Method{
    Get,
    Post,
    Update,
    Delete,
}

// TODO реализовать запросы на сервер
async fn request<'a, T>(url: &'a str, method: &'a str) -> Result<i32, u16>
where T: DeserializeOwned + Debug + Send
{
    // Gloo request
    let resp = Request::get("localhost:8080/login")
        .send().await.unwrap();

    if resp.status() == 200{
        match resp.json::<T>().await{
            Ok(obj) => {
                Ok(12)
            },
            Err(err) => {
                log::error!("Failed parse object {:?}", err);
                Err(500)
            },
        }
    } else {
        log::error!("Failed requset {}: {:?}", resp.status(), resp.body());
        Err(resp.status())
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

pub fn POST_login() -> Result<(), i16> {
    Ok(())
}

pub fn POST_signup() -> result::Result<(), u16> {
    let (tx, rx) = channel::bounded(1);

    let handle = Handle::current();

    handle.spawn(async move{ 
        let res: Result<i32, u16> = request::<i32>("/login", "get").await;
        let _ = tx.send(res);
    });

    let res = rx.recv().unwrap();

    Ok(())
}

pub fn GET_user_detail() -> result::Result<UserInfo, u16>{
    Ok(UserInfo::default())
}