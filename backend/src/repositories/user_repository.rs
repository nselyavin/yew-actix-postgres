use bcrypt::{hash, verify, DEFAULT_COST};
use rbatis::crud::CRUD;
use rbatis::{core::Result, rbatis::Rbatis, snowflake::Snowflake};

use crate::models::user::User;
use crate::models::user::UserSignup;

pub async fn create(user_data: &UserSignup, rb: &Rbatis, sflake: &Snowflake) -> Result<User> {
    log::info!("Create user");
    let user = User {
        id: sflake.generate().unsigned_abs(),
        username: user_data.username.clone(),
        email: user_data.email.clone(),
        created_date: rbatis::DateTimeNative::now(),
        password: Some(hash(user_data.password.clone(), 8).unwrap()),
    };

    match rb.save(&user, &[]).await{
        Ok(val) => 
        {
            log::info!("Successfully create user {}", user.username.clone());
            Ok(user)
        },
        Err(err) => {
            log::error!("Failed create user");
            Err(err)
        }
    }
}

pub async fn update() -> Result<User> {
    todo!()
}

pub async fn find_by_id(id: u64) -> Result<User> {
    todo!()
}
