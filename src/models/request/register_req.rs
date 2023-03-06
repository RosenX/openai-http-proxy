use std::fmt::Display;

use chrono::Local;
use log::info;
use rocket::serde::Deserialize;
use sea_orm::{ActiveValue, DatabaseConnection};

use crate::{common::{utils::crypto::{PasswordEncrypt, EncryptUtil}, errors::InternalError}, entities::user_profile};

use sea_orm::*;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RegisterReq {
    username: String,
    email: String,
    password: String
}

impl Display for RegisterReq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.username, self.email, self.password)
    }
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
        info!("{}", info);
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

impl RegisterReq {
    pub async fn create_user(self, db: &DatabaseConnection) -> 
        Result<user_profile::Model, InternalError> {
        let user: user_profile::ActiveModel = self.try_into()?;
        let user_model = user.insert(db)
        .await
        .map_err(|err| InternalError::DuplicateEmail(err.to_string()))?;
        Ok(user_model)
    }
}