use rbatis::{rbatis::Rbatis, snowflake::Snowflake, crud::CRUD, Uuid};

use crate::models::creator::{self, Creator};


pub async fn create(data: &Creator, rb: &Rbatis, sflake: &Snowflake) -> Result<Creator, String> {
    let creator = Creator {
        id: Some(sflake.generate().unsigned_abs()),
        name: data.name.clone(),
        license_num: data.license_num,
        created_date: Some(rbatis::DateTimeNative::now()),
    };

    match rb.save(&creator, &[]).await{
        Ok(res) => {
            log::info!("Successfully create creator {}", creator.name.clone());
            Ok(creator)
        },
        Err(err) => {
            log::error!("Failed create creator: {}", err.to_string());
            Err(err.to_string())
        }
    }
}

pub async fn find_by_id(id: u64, rb: &Rbatis) -> Result<Creator, String>{
    match rb.fetch_by_column("id", id).await{
        Ok(creator) => {
            Ok(creator)
        },
        Err(msg) => {
            log::warn!("Creator not found");
            Err(msg.to_string())
        },
    }
}

pub async fn find_by_name(name: String, rb: &Rbatis) -> Result<Creator, String>  {
    match rb.fetch_by_column("name", name).await{
        Ok(creator) => {
            Ok(creator)
        },
        Err(msg) => {
            log::warn!("Creator not found");
            Err(msg.to_string())
        },
    }
}