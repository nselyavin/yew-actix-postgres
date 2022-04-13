
use std::fmt::Result;

use serde::ser::Error;

use crate::models::{*, item::Item};
use std::result;

// TODO реализовать запросы на сервер
pub fn GET_items() -> result::Result<Vec<item::Item>, i32>{
    let mut items = vec!();

    for i in 0..7{
        items.push(Item::new(i));
    }
    Ok(items)
}

pub fn GET_items_last(amount: i32) -> result::Result<Vec<item::Item>, i32>{
    let mut items = Vec::new();

    Ok(items.to_owned())
}

pub fn GET_item(id: i64) -> result::Result<item::Item, i32>{
    Ok(Item::new(id))
}

pub fn POST_login()-> result::Result<user::User, i32>{
    Ok(user::User::new())
}