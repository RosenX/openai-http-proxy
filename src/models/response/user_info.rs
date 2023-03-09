use std::fmt::Display;

use rocket::serde::{Deserialize, Serialize};

use crate::database::user_profile::UserProfile;

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct BasicUserProfile {
    pub id: i32,
    email: String,
    username: String,
    pro_level: i32,
    pro_end_time: String,
}

impl Display for BasicUserProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "user_id: {}, email: {}, username: {}, pro_level: {}, pro_end_time: {}",
            self.id, self.email, self.username, self.pro_level, self.pro_end_time
        )
    }
}

impl From<UserProfile> for BasicUserProfile {
    fn from(user_profile: UserProfile) -> Self {
        Self {
            id: user_profile.id,
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
    basic_profile: BasicUserProfile,
}

impl UserInfo {
    pub fn new(basic_profile: BasicUserProfile) -> Self {
        Self {
            basic_profile: basic_profile,
        }
    }
}
