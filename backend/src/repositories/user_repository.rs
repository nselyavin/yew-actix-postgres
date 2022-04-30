
use rbatis::{rbatis::Rbatis, snowflake::Snowflake, core::Result};

use models::users::UserInfo;

pub struct UserRepository;

impl UserRepository{
    pub fn create(user: &UserLogin, rb: &Rbatis) -> Result<UserInfo>{
        todo!()
    }

    pub fn update() -> Result<UserInfo>{
        todo!()
    }

    pub fn find_by_id(id: u64) -> Result<UserInfo>{
        todo!()
    }
}