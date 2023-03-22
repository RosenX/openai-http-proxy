use abi::{DbPool, Email, InternalError, UserInformation};
use async_trait::async_trait;

use super::{UserManager, UserManagerOp};

impl UserManager {
    pub fn new(pool: DbPool) -> Self {
        UserManager { pool }
    }
}

#[async_trait]
impl UserManagerOp for UserManager {
    type Error = InternalError;
    async fn create(&self, user_profile: UserInformation) -> Result<UserInformation, Self::Error> {
        let user_info = sqlx::query_as!(
            UserInformation,
            r#"
        INSERT INTO user_information (
            username,
            email,
            password,
            pro_level,
            pro_end_time,
            created_time
        ) VALUES ($1,$2,$3,$4,$5,$6)
        RETURNING *
        "#,
            user_profile.username,
            user_profile.email,
            user_profile.password,
            user_profile.pro_level,
            user_profile.pro_end_time,
            user_profile.created_time
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(user_info)
    }

    async fn find_user_by_email(
        &self,
        email: &Email,
    ) -> Result<Option<UserInformation>, Self::Error> {
        let res = sqlx::query_as!(
            UserInformation,
            "SELECT * FROM user_information WHERE email = $1",
            email
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(res)
    }
}
