use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Default)]
pub struct MedicineInfo{
    id: String,
    pub name: String,
    pub creator_name: String,
    pub cost: f32,
    pub description: String,
    pub created_date: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Default)]
pub struct MedicineReg{
    pub name: String,
    pub description: String,
    pub cost: f32
}

