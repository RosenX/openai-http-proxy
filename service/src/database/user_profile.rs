use chrono::{DateTime, Utc};
use sqlx::FromRow;

use crate::common::service::mysql_service::MySqlService;
use crate::common::utils::crypto::PasswordEncrypt;
use crate::{common::errors::InternalError, models::request::register_req::RegisterReq};

#[derive(Clone, Debug, FromRow)]
pub struct UserProfile {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    // 0-普通用户；1-VIP；2-SVIP
    pub pro_level: i32,
    pub pro_end_time: DateTime<Utc>,
    pub created_time: DateTime<Utc>,
}

// impl Display for UserProfile {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "
//             username: {},
//             email: {},
//             password: {},
//             pro_level: {},
//             pro_end_time: {},
//             created_time: {}",
//             self.username,
//             self.email,
//             self.password,
//             self.pro_level,
//             format!("{}", self.pro_end_time.format("%Y-%m-%d %H:%M:%S")),
//             format!("{}", self.created_time.format("%Y-%m-%d %H:%M:%S")),
//         )
//     }
// }

impl TryFrom<RegisterReq> for UserProfile {
    type Error = InternalError;
    fn try_from(value: RegisterReq) -> Result<Self, Self::Error> {
        // let now_datetime = Utc::now().with_timezone(&FixedOffset::east_opt(28800).unwrap());
        let now_datetime = Utc::now();
        let info: RegisterReq = value.hash()?;
        let user = UserProfile {
            id: 0, // todo!有没有更好的方法？
            username: info.username,
            email: info.email,
            password: info.password,
            pro_level: 0,
            pro_end_time: now_datetime,
            created_time: now_datetime,
        };
        Ok(user)
    }
}

impl UserProfile {
    pub async fn create_user(&self, pool: &MySqlService) -> Result<u64, InternalError> {
        let res = sqlx::query_as!(
            UserProfile,
            r#"
            INSERT INTO user_profile (
                username,
                email,
                password,
                pro_level,
                pro_end_time,
                created_time
            ) VALUES (?,?,?,?,?,?)
            "#,
            self.username,
            self.email,
            self.password,
            self.pro_level,
            self.pro_end_time,
            self.created_time // format!("{}", self.pro_end_time.format("%Y-%m-%d %H:%M:%S")),
                              // format!("{}", self.created_time.format("%Y-%m-%d %H:%M:%S")),
        )
        .execute(pool)
        .await?
        .last_insert_id();
        Ok(res)
    }
}
