use rbatis::{crud_table, DateTimeNative};
use serde::{Serialize, Deserialize};

// use super::user::User;

#[crud_table(table_name: medicine)]
#[derive(Serialize, Deserialize)]
pub struct Medicine{
    pub id: String,
    pub name: String,
    pub description: String,
    pub cost: f32,
    pub creator_name: String,
    pub created_date: DateTimeNative,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Default)]
pub struct MedicineReg{
    pub name: String,
    pub description: String,
    pub cost: f32
}
