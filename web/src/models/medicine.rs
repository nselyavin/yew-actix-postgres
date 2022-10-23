use rbatis::{crud_table, DateTimeNative};
use serde::{Deserialize, Serialize};

// use super::user::User;

#[crud_table(table_name: medicine)]
#[derive(Serialize, Deserialize)]
pub struct Medicine {
    pub id: String,
    pub name: String,
    pub description: String,
    pub cost: f32,
    pub fk_creator_id: u64,
    pub created_date: DateTimeNative,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Default)]
pub struct MedicineReg {
    pub name: String,
    pub description: String,
    pub cost: f32,
    pub creator_id: u64,
}
