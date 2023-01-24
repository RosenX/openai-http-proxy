use bcrypt::{hash};
use anyhow::Result;
use rocket::http::hyper::Error;
use super::responder::{FailureJsonResponder};

// hash password
//TODO: cost config
pub fn hash_password(origin_passwod: String) -> Result<String> {
    match hash(origin_passwod, 4)  {
        Ok(hashed_password) => Ok(hashed_password),
        Err(err) => Err(err.into()),
    }
}