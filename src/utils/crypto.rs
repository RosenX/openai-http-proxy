use bcrypt::{hash};
use anyhow::Result;

// hash password
//TODO: cost config
pub fn hash_password(origin_passwod: String) -> Result<String> {
    let hashed_password = hash(origin_passwod, 4)?;
    Ok(hashed_password)
}