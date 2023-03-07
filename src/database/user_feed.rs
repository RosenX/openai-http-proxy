use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Clone, Debug, FromRow)]
pub struct UserFeed {
    pub id: i32,
    pub user_id: i32,
    pub url: String,
    pub name: String,
    pub icon: Option<String>,
    pub created_time: DateTime<Utc>,
}