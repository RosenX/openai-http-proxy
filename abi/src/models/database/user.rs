use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    Content, Email, FeedProfile, InternalError, PasswordEncrypt, RegisterReq, UserId, UserProfile,
};

#[derive(Clone, Debug)]
pub struct UserInformation {
    pub id: UserId,
    pub username: String,
    pub email: Email,
    pub password: String,
    // 0-普通用户；1-VIP；2-SVIP
    pub pro_level: i32,
    pub pro_end_time: DateTime<Utc>,
    pub created_time: DateTime<Utc>,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct UserFeed {
    pub user_id: UserId,
    pub feed_id: i32,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub logo: Option<String>,
    pub description: Option<String>,
    #[serde(with = "ts_milliseconds")]
    pub created_time: DateTime<Utc>,
}

impl UserFeed {
    pub fn new(user: UserProfile, feed: FeedProfile) -> Self {
        let now_datetime = Utc::now();
        Self {
            user_id: user.id,
            feed_id: feed.id,
            name: None,
            icon: None,
            logo: None,
            description: None,
            created_time: now_datetime,
        }
    }
}

#[derive(Deserialize, Clone, Serialize)]
pub struct UserPost {
    pub user_id: UserId,
    pub post_id: i32,
    pub stage: i32,
    pub tags: Option<String>,
    pub category: Option<String>,
    pub notes: Option<String>,
}

impl UserPost {
    pub fn new(user: UserProfile, content: Content) -> Self {
        Self {
            user_id: user.id,
            post_id: content.id,
            stage: 0,
            tags: None,
            category: None,
            notes: None,
        }
    }
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
