use bcrypt::{hash};
use super::responder::{FailureJsonResponder};

//TODO: config
pub const SECRET: &str = "";
pub const REFRESH_SECRET: &str = "";
pub const EXPIRATION_TIME: i64 = 10;
pub const REFRESH_EXPIRATION_TIME: i64 = 1000;

// hash password
//TODO: cost config
pub fn hash_password(origin_passwod: String) -> Result<String, FailureJsonResponder<String>> {
    match hash(origin_passwod, 4)  {
        Ok(hashed_password) => Ok(hashed_password),
        Err(err) => Err(err.into()),
    }
}