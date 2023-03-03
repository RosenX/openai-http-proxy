use rocket::serde::Deserialize;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};
use crate::entities::{prelude::*, user_profile};

use crate::{common::{utils::crypto::{PasswordVerify, EncryptUtil}, errors::InternalError}};

use sea_orm::*;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginReq {
    email: String,
    password: String
}

impl PasswordVerify for LoginReq {
    type Error = InternalError;
    fn verify(self, target: &String) -> Result<bool, Self::Error> {
        EncryptUtil::verify_password(&self.password, 
            &target)
    }
}

impl LoginReq {
    pub async fn find_user_by_email(&self, db: &DatabaseConnection) 
        -> Result<Option<user_profile::Model>, InternalError> {
        let res = UserProfile::find()
        .filter(user_profile::Column::Email.eq(self.email.clone()))
        .one(db)
        .await
        .map_err(|err| InternalError::DatabaseError(err.to_string()))?;
        Ok(res)
    }
}

