use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FeedReq {
    pub url: String,
}
