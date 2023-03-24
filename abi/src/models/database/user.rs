use chrono::{DateTime, Utc};

use crate::{
    Content, Email, FeedProfile, Id, InternalError, PasswordEncrypt, ProLevel, ReadStage,
    RegisterInfo, UserContent, UserFeed, UserProfile, DEFAULT_ID,
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
            created_time: now_datetime.timestamp(),
            tags: None,
            folder: None,
        }
    }
}

impl UserContent {
    pub fn new(user: UserProfile, content: Content) -> Self {
        Self {
            id: DEFAULT_ID,
            user_id: user.id,
            content_id: content.id,
            stage: ReadStage::Explore as i32, // todo
            tags: None,
            category: None,
            notes: None,
        }
    }
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
