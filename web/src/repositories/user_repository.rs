use bcrypt::{hash, verify, DEFAULT_COST};
use rbatis::crud::CRUD;
use rbatis::{core::Result, rbatis::Rbatis, snowflake::Snowflake};

use crate::models::user::User;
use crate::models::user::UserSignup;

pub async fn create(user_data: &UserSignup, rb: &Rbatis, sflake: &Snowflake) -> Option<User> {
    log::info!("Create user");
    let user = User {
        id: sflake.generate().unsigned_abs(),
        username: user_data.username.clone(),
        email: user_data.email.clone(),
        created_date: rbatis::DateTimeNative::now(),
        password: hash(user_data.password.clone(), 8).unwrap(),
    };

    match rb.save(&user, &[]).await {
        Ok(res) => {
            log::info!("Successfully create user {}", user.username.clone());
            Some(user)
        }
        Err(err) => {
            log::error!("Failed create user: {}", err.to_string());
            None
        }
    }
}

pub async fn update() -> Result<User> {
    todo!()
}

pub async fn find_by_id(id: u64) -> Result<User> {
    todo!()
}

pub async fn find_by_email(email: &String, rb: &Rbatis) -> Option<User> {
    let res: Result<User> = rb.fetch_by_column("email", email).await;
    match res {
        Ok(user) => Some(user),
        Err(err) => {
            log::error!("Failed find by email: {}", err.to_string());
            None
        }
    }
}
