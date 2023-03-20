use std::fmt::Display;

use serde::Deserialize;

use crate::{EncryptUtil, InternalError, PasswordEncrypt};

#[derive(Deserialize)]
pub struct RegisterReq {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl Display for RegisterReq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.username, self.email, self.password)
    }
}

// todo，优化一下
impl PasswordEncrypt for RegisterReq {
    type Error = InternalError;
    fn hash(self) -> Result<Self, Self::Error> {
        let hash_password = EncryptUtil::hash_password(self.password.as_str())?;
        Ok(Self {
            email: self.email,
            username: self.username,
            password: hash_password,
        })
    }
}
