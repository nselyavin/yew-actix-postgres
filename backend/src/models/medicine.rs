use rbatis::{crud_table, DateTimeNative};
use serde::{Serialize, Deserialize};

use super::user::UserInfo;

#[crud_table(table_name: medicine)]
#[derive(Serialize, Deserialize)]
pub struct Medicine{
    id: String,
    user_id: i64,
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct MedicineInfo{
    id: String,
    user_name: String,
    name: String,
}