use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{
    Content, Email, FeedProfile, InternalError, PasswordEncrypt, RegisterReq, UserId, UserProfile,
    DEFAULT_ID,
};

enum ProLevel {
    Normal,
    _Pro,
    _SPro,
}

impl From<ProLevel> for i16 {
    fn from(value: ProLevel) -> Self {
        match value {
            ProLevel::Normal => 0,
            ProLevel::_Pro => 1,
            ProLevel::_SPro => 2,
        }
    }
}

#[derive(Clone, Debug)]
pub struct UserInformation {
    pub id: UserId,
    pub username: String,
    pub email: Email,
    pub password: String,
    // 0-普通用户；1-VIP；2-SVIP
    pub pro_level: i16,
    pub pro_end_time: DateTime<Utc>,
    pub created_time: DateTime<Utc>,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct UserFeed {
    pub id: i32,
    pub user_id: UserId,
    pub feed_id: i32,
    pub name: Option<String>,
    pub logo: Option<String>,
    pub description: Option<String>,
    pub folder: Option<String>,
    pub tags: Option<String>,
    pub created_time: DateTime<Utc>,
}

impl UserFeed {
    pub fn new(user: UserProfile, feed: FeedProfile) -> Self {
        let now_datetime = Utc::now();
        Self {
            id: DEFAULT_ID,
            user_id: user.id,
            feed_id: feed.id,
            name: None,
            logo: None,
            description: None,
            created_time: now_datetime,
            tags: None,
            folder: None,
        }
    }
}

#[derive(Deserialize, Clone, Serialize)]
pub enum ReadStage {
    Explore = 0,
    Focus = 1,
    Seen = 2,
    Archive = 3,
}

impl From<ReadStage> for i16 {
    fn from(value: ReadStage) -> Self {
        match value {
            ReadStage::Explore => 0,
            ReadStage::Focus => 1,
            ReadStage::Seen => 2,
            ReadStage::Archive => 3,
        }
    }
}

#[derive(Deserialize, Clone, Serialize, FromRow)]
pub struct UserContent {
    pub id: i32,
    pub user_id: UserId,
    pub content_id: i32,
    pub stage: i16,
    pub tags: Option<String>,
    pub category: Option<String>,
    pub notes: Option<String>,
}

impl UserContent {
    pub fn new(user: UserProfile, content: Content) -> Self {
        Self {
            id: DEFAULT_ID,
            user_id: user.id,
            content_id: content.id,
            stage: ReadStage::Explore.into(),
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
            pro_level: ProLevel::Normal.into(),
            pro_end_time: now_datetime,
            created_time: now_datetime,
        };
        Ok(user)
    }
}
