use std::vec;

use rbatis::{crud::CRUD, rbatis::Rbatis, snowflake::Snowflake, Uuid};

use crate::models::{medicine::{self, Medicine, MedicineReg}, pharmacy::Pharmacy};

pub async fn create(data: &MedicineReg, rb: &Rbatis) -> Result<Medicine, String> {
    log::info!("Create medicine");

    let medicine = Medicine {
        id: Uuid::new().to_string(),
        name: data.name.clone(),
        description: data.description.clone(),
        cost: data.cost,
        fk_creator_id: data.creator_id,
        created_date: rbatis::DateTimeNative::now(),
    };

    match rb.save(&medicine, &[]).await {
        Ok(res) => {
            log::info!("Successfully create medicine {}", medicine.name.clone());
            Ok(medicine)
        }
        Err(err) => {
            log::error!("Failed create medicine: {}", err.to_string());
            Err(err.to_string())
        }
    }
}

pub async fn find_by_id(id: String, rb: &Rbatis) -> Option<Medicine> {
    match rb.fetch_by_column("id", id).await {
        Ok(medicine) => Some(medicine),
        Err(_) => {
            log::warn!("Medicine not found");
            None
        }
    }
}

pub async fn get_pharmacy_by_id(id: String, rb: &Rbatis) -> Vec<Pharmacy>{
    vec![]
}