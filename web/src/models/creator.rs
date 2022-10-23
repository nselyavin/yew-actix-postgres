use rbatis::{crud_table, DateTimeNative};
use serde::{Serialize, Deserialize};

use super::user::User;

#[crud_table(table_name: creator)]
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Default)]
pub struct Creator{
    pub id: Option<u64>, 
    pub name: String,
    pub license_num: Option<u64>,
    pub created_date: Option<DateTimeNative>,
}

