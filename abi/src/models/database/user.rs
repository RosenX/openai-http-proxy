use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

use crate::{InternalError, PasswordEncrypt, RegisterReq};

#[derive(Clone, Debug)]
pub struct UserInformation {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    // 0-普通用户；1-VIP；2-SVIP
    pub pro_level: i32,
    pub pro_end_time: DateTime<Utc>,
    pub created_time: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize)]
pub struct UserFeed {
    pub user_id: i32,
    pub feed_id: i32,
    pub url: String,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub logo: Option<String>,
    pub description: Option<String>,
    #[serde(with = "ts_milliseconds")]
    pub created_time: DateTime<Utc>,
}

#[derive(Clone, Debug, FromRow, Serialize)]
pub struct UserPost {
    pub user_id: i32,
    pub feed_id: i32,
    pub feed_name: Option<String>,
    pub cover: Option<String>,
    pub stage: Option<i64>,
    pub post_id: i32,
    pub link: Option<String>,
    pub content: Option<String>,
    pub title: String,
    pub authors: Option<String>,
    pub tags: Option<String>,
    pub category: Option<String>,
    pub notes: Option<String>,
    #[serde(with = "ts_milliseconds")]
    pub publish_time: DateTime<Utc>,
}

impl TryFrom<RegisterReq> for UserInformation {
    type Error = InternalError;
    fn try_from(value: RegisterReq) -> Result<Self, Self::Error> {
        // let now_datetime = Utc::now().with_timezone(&FixedOffset::east_opt(28800).unwrap());
        let now_datetime = Utc::now();
        let info: RegisterReq = value.hash()?;
        let user = UserInformation {
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
