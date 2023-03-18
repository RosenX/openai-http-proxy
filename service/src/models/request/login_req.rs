use abi::DbPool;
use rocket::serde::Deserialize;

use crate::{
    common::{
        errors::InternalError,
        utils::crypto::{EncryptUtil, PasswordVerify},
    },
    database::user_profile::UserProfile,
};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginReq {
    pub email: String,
    pub password: String,
}

impl PasswordVerify for LoginReq {
    type Error = InternalError;
    fn verify(self, target: &str) -> Result<bool, Self::Error> {
        EncryptUtil::verify_password(&self.password, target)
    }
}

impl LoginReq {
    pub async fn find_user_by_email(
        &self,
        db: &DbPool,
    ) -> Result<Option<UserProfile>, InternalError> {
        let res = sqlx::query_as!(
            UserProfile,
            "SELECT * FROM user_profile WHERE email = ?",
            self.email
        )
        .fetch_optional(db)
        .await?;
        Ok(res)
    }
}
