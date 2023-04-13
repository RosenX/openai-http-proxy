use chrono::{DateTime, Utc};
use sqlx::{postgres::PgRow, FromRow, Row};

use crate::{
    AuthResponse, Email, Id, InternalError, PasswordEncrypt, ProLevel, ProLevelPostgres,
    RegisterInfo, Tokens, UserProfile,
};

pub struct UserInformation {
    pub id: Id,
    pub username: String,
    pub email: Email,
    pub password: String,
    pub pro_level: ProLevel,
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
            pro_level: ProLevel::Normal,
            pro_end_time: now_datetime,
            created_time: now_datetime,
        };
        Ok(user)
    }
}

impl FromRow<'_, PgRow> for UserInformation {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let pro_level: ProLevelPostgres = row.get("pro_level");
        Ok(Self {
            id: row.try_get("id")?,
            username: row.try_get("username")?,
            email: row.try_get("email")?,
            password: row.try_get("password")?,
            pro_level: ProLevel::from(pro_level),
            pro_end_time: row.try_get("pro_end_time")?,
            created_time: row.try_get("created_time")?,
        })
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
