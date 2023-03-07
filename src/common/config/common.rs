use rocket::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CommonConfig {
    pub default_name: String,
    pub default_icon: String,
}