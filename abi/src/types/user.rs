use chrono::{DateTime, Utc};

use crate::{
    AuthResponse, Email, Id, InternalError, PasswordEncrypt, ProLevel, RegisterInfo, Tokens,
    UserProfile,
};

pub struct UserInformation {
    pub id: Id,
    pub username: String,
    pub email: Email,
    pub password: String,
    pub pro_level: i16, // todo
    pub pro_end_time: DateTime<Utc>,
    pub created_time: DateTime<Utc>,
}

impl TryFrom<RegisterInfo> for UserInformation {
    type Error = InternalError;
    fn try_from(value: RegisterInfo) -> Result<Self, Self::Error> {
        // let now_datetime = Utc::now().with_timezone(&FixedOffset::east_opt(28800).unwrap());
        let now_datetime = Utc::now();
        let info: RegisterInfo = value.hash()?;
        let user = UserInformation {
            id: 0, // todo!有没有更好的方法？
            username: info.username,
            email: info.email,
            password: info.password,
            pro_level: ProLevel::Free as i16,
            pro_end_time: now_datetime,
            created_time: now_datetime,
        };
        Ok(user)
    }
}

// impl Display for UserProfile {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "user_id: {}, email: {}, username: {}, pro_level: {}, pro_end_time: {}",
//             self.id, self.email, self.username, self.pro_level, self.pro_end_time
//         )
//     }
// }

impl From<UserInformation> for UserProfile {
    fn from(user_info: UserInformation) -> Self {
        Self {
            id: user_info.id,
            email: user_info.email,
            username: user_info.username,
            pro_level: user_info.pro_level as i32,
            pro_end_time: user_info.pro_end_time.timestamp(),
        }
    }
}

impl From<Tokens> for AuthResponse {
    fn from(value: Tokens) -> Self {
        Self {
            tokens: Some(value),
        }
    }
}
