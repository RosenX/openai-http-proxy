use std::fmt::Display;
use rocket::serde::Deserialize;
use crate::{common::{utils::crypto::{PasswordEncrypt, EncryptUtil}, errors::InternalError}};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RegisterReq {
    pub username: String,
    pub email: String,
    pub password: String
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