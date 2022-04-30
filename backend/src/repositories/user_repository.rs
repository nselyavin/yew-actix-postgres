
use rbatis::{rbatis::Rbatis, snowflake::Snowflake, core::Result};

use crate::models::user::User;
use crate::models::user::UserSignup;

pub struct UserRepository;

impl UserRepository{
    pub fn create(user: &UserSignup, rb: &Rbatis) -> Result<UserSignup>{
        todo!()
    }

    pub fn update() -> Result<User>{
        todo!()
    }

    pub fn find_by_id(id: u64) -> Result<User>{
        todo!()
    }
}