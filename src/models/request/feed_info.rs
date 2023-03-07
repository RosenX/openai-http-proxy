use crate::{
    common::errors::InternalError, database::DatabasePool, routes::authorization::AuthorizedUser,
};
use chrono::Utc;
use rocket::serde::Deserialize;

#[derive(Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ExistSourceInfo {
    pub url: String,
    pub name: Option<String>,
    pub icon: Option<String>,
}

impl ExistSourceInfo {
    fn new(url: String, name: Option<String>, icon: Option<String>) -> Self {
        Self {
            url: url,
            name: name,
            icon: icon,
        }
    }

    pub async fn create_user_feed(
        &self,
        pool: &DatabasePool,
        user: AuthorizedUser,
    ) -> Result<u64, InternalError> {
        let now_datetime = Utc::now();
        let res = sqlx::query_as!(
            UserFeed,
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
