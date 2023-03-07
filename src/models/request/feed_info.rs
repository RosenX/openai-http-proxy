use crate::{
    common::{errors::InternalError, config::{common::CommonConfig}}, database::DatabasePool, routes::authorization::AuthorizedUser,
};
use chrono::Utc;
use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ExistSourceInfo {
    pub url: String,
    pub name: Option<String>,
    pub icon: Option<String>,
}

impl ExistSourceInfo {
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
