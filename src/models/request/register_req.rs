use chrono::Local;
use rocket::serde::Deserialize;
use sea_orm::ActiveValue;

use crate::{common::{utils::crypto::{PasswordEncrypt, EncryptUtil}, errors::InternalError}, entities::user_profile};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RegisterReq {
    username: String,
    email: String,
    password: String
}

impl PasswordEncrypt for RegisterReq {
    type Error = InternalError;
    fn hash(self) -> Result<Self, Self::Error> {
        let hash_password = EncryptUtil::hash_password(self.password)?;
        Ok(Self {
            email: self.email,
            username: self.username,
            password: hash_password
        })
    }
}

impl TryInto<user_profile::ActiveModel> for RegisterReq {
    type Error = InternalError;
    fn try_into(self) -> Result<user_profile::ActiveModel, Self::Error> {
        let now_datetime = Local::now().naive_local();
        let info = self.hash()?;
        let res = user_profile::ActiveModel {
            username: ActiveValue::Set(info.username.clone()),
            email: ActiveValue::Set(info.email),
            hash_password: ActiveValue::Set(info.password),
            created_time: ActiveValue::Set(now_datetime),
            ..Default::default()
        };
        Ok(res)
    }
}