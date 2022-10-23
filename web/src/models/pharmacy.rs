use rbatis::{crud_table, DateTimeNative};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, )]
pub struct Pharmacy{
    id: Option<u64>,
    name: String,
    location: Option<String>,
}