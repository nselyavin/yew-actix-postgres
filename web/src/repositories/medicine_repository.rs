use rbatis::{rbatis::Rbatis, snowflake::Snowflake, crud::CRUD, Uuid};

use crate::models::medicine::{MedicineReg, Medicine, self};


pub async fn create(data: &MedicineReg, rb: &Rbatis, creator_name: String)->Option<Medicine>{
    log::info!("Create medicine");

    let medicine = Medicine{
        id: Uuid::new().to_string(),
        name: data.name.clone(),
        description: data.description.clone(),
        cost: data.cost,
        creator_name: creator_name,
        created_date: rbatis::DateTimeNative::now(),
    };

    match rb.save(&medicine, &[]).await{
        Ok(res) => {
            log::info!("Successfully create medicine {}", medicine.name.clone());
            Some(medicine)
        },
        Err(err) => {
            log::error!("Failed create medicine: {}", err.to_string());
            None
        }
    }
}

pub async fn find_by_id(id: String, rb: &Rbatis)->Option<Medicine>{
    match rb.fetch_by_column("id", id).await {
        Ok(medicine) => {
            Some(medicine)
        },
        Err(_) => {
            log::warn!("Medicine not found");
            None
        },
    }

}