use rbatis::{crud_table, DateTimeNative};
use serde::{Serialize, Deserialize};

#[crud_table(table_name: t_user)]
#[derive(Serialize, Deserialize)]
pub struct UserInfo{
    pub id: u64,
    pub username: String,
    pub email: String,
    pub created_date: DateTimeNative
}

#[derive(Deserialize)]
pub struct UserLogin{
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserAuth{
    pub email: String,
    pub username: String,
    pub password: String,
}