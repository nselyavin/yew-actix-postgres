use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Item{
    pub id: i64,
    pub name: String,
    pub description: String,
    pub cost: f32,
    pub components: Vec<String>,
}

static mut count: i64 = 0;

impl Item {
    pub fn new(id: i64)->Item{
        Item{
            id: id,
            name: "Name".to_string(),
            description: "Description".to_string(),
            cost: 0.0,
            components: vec!{"comp_1".to_string(), "comp_2".to_string()},
        }
    }
}