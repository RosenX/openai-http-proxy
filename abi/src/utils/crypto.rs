use bcrypt::{hash, verify};

use crate::{InternalError, LoginInfo, RegisterInfo};

pub struct EncryptUtil;

impl EncryptUtil {
    pub fn hash_password(origin_passwod: &str) -> Result<String, InternalError> {
        let hashed_password = hash(origin_passwod, 4)?;
        Ok(hashed_password)
    }

    pub fn verify_password(
        input_password: &str,
        database_password: &str,
    ) -> Result<bool, InternalError> {
        let res = verify(input_password, database_password)?;
        Ok(res)
    }
}

pub trait PasswordEncrypt {
    type Error;
    fn hash(self) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

pub trait PasswordVerify {
    type Error;
    fn verify(self, target: &str) -> Result<bool, Self::Error>;
}

impl PasswordEncrypt for RegisterInfo {
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

impl PasswordVerify for LoginInfo {
    type Error = InternalError;
    fn verify(self, target: &str) -> Result<bool, Self::Error> {
        EncryptUtil::verify_password(&self.password, target)
    }
}
