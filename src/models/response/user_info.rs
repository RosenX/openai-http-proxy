use chrono::NaiveDateTime;
use rocket::serde::{Serialize, Deserialize};

use crate::entities::user_profile;

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct BasicProfile {
    pub user_id: i32,
    email: String,
    username: String,
    pro_level: bool,
    pro_end_time: Option<NaiveDateTime>,
}

impl From<user_profile::Model> for BasicProfile {
    fn from(user_model: user_profile::Model) -> Self {
        Self {
            user_id: user_model.id,
            email: user_model.email,
            username: user_model.username,
            pro_level: user_model.is_pro,
            pro_end_time: user_model.pro_end_time
        }
    }
}


#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserInfo {
    basic_profile: BasicProfile
}

impl UserInfo {
    pub fn new(basic_profile: BasicProfile) -> Self {
        Self {
            basic_profile: basic_profile
        }
    }
}