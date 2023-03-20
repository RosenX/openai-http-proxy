use abi::{DbPool, InternalError, UserInformation};
use async_trait::async_trait;

use crate::{UserManager, UserManagerOp};

impl UserManager {
    pub fn new(pool: DbPool) -> Self {
        UserManager { pool }
    }
}

#[async_trait]
impl UserManagerOp for UserManager {
    type Error = InternalError;
    async fn create(
        &self,
        mut user_profile: UserInformation,
    ) -> Result<UserInformation, Self::Error> {
        let id = sqlx::query!(
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
            user_profile.username,
            user_profile.email,
            user_profile.password,
            user_profile.pro_level,
            user_profile.pro_end_time,
            user_profile.created_time
        )
        .execute(&self.pool)
        .await?
        .last_insert_id();
        user_profile.id = id as i32; // todo
        Ok(user_profile)
    }
}
