use rocket::serde::{Deserialize, Serialize};

use crate::database::user_profile::UserProfile;

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct BasicProfile {
    pub user_id: i32,
    email: String,
    username: String,
    pro_level: i32,
    pro_end_time: String,
}

impl From<UserProfile> for BasicProfile {
    fn from(user_profile: UserProfile) -> Self {
        Self {
            user_id: user_profile.user_id,
            email: user_profile.email,
            username: user_profile.username,
            pro_level: user_profile.pro_level,
            pro_end_time: format!("{}", user_profile.pro_end_time.format("%Y-%m-%d %H:%M:%S")),
        }
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserInfo {
    basic_profile: BasicProfile,
}

impl UserInfo {
    pub fn new(basic_profile: BasicProfile) -> Self {
        Self {
            basic_profile: basic_profile,
        }
    }
}
