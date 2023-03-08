use crate::{
    common::{errors::InternalError, config::{common::CommonConfig}}, database::{DatabasePool, user_feed::UserFeed}, routes::{authorization::AuthorizedUser, feed},
};
use chrono::Utc;
use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FeedInfo {
    pub url: String,
    pub name: Option<String>,
    pub icon: Option<String>,
}

impl From<UserFeed> for FeedInfo {
    fn from(value: UserFeed) -> Self {
        Self {
            url: value.url,
            name: Some(value.name),
            icon: value.icon
        }
    }
}

impl FeedInfo {
    pub fn complete_info(&mut self, config: &CommonConfig) {
        self.name = Some(config.default_name.clone());
        self.icon = Some(config.default_icon.clone());
    }

    pub async fn create_user_feed(
        &self,
        pool: &DatabasePool,
        user: AuthorizedUser,
    ) -> Result<u64, InternalError> {
        let now_datetime = Utc::now();
        let res = sqlx::query!(
            r#"
            INSERT INTO user_feed (
                user_id, 
                url, 
                name, 
                icon,
                created_time
            ) VALUES (?,?,?,?,?)
            "#,
            user.user_id,
            self.url,
            self.name,
            self.icon,
            now_datetime,
        )
        .execute(pool)
        .await?
        .last_insert_id();
        Ok(res)
    }
}
