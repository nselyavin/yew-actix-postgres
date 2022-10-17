use rbatis::{crud_table, DateTimeNative};
use serde::{Serialize, Deserialize};

use super::user::User;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Default)]
pub struct Creator{
    id: u64, 
    name: String,
    license_num: Option<u64>,
}

