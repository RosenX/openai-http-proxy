use bcrypt::{hash, verify};

use crate::common::errors::InternalError;

pub struct EncryptUtil;

impl EncryptUtil {
    pub fn hash_password(origin_passwod: &str) -> Result<String, InternalError> {
        let hashed_password = hash(origin_passwod, 4)?;
        Ok(hashed_password)
    }

    pub fn verify_password(input_password: &str,
        database_password: &str) -> Result<bool, InternalError> {
        let res = verify(input_password, database_password)?;
        Ok(res)
    }
}

pub trait PasswordEncrypt {
    type Error;
    fn hash(self) -> Result<Self, Self::Error>
    where Self: Sized;
}


pub trait PasswordVerify {
    type Error;
    fn verify(self, target: &str) -> Result<bool, Self::Error>;
}
