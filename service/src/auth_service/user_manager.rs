use abi::{DbService, Email, InternalError, UserInformation};
use async_trait::async_trait;

use super::{UserManager, UserManagerOp};

impl UserManager {
    pub fn new(db_service: DbService) -> Self {
        UserManager { db_service }
    }
}

#[async_trait]
impl UserManagerOp for UserManager {
    type Error = InternalError;
    async fn create(&self, user_profile: UserInformation) -> Result<UserInformation, Self::Error> {
        let sql = format!(
            r#"
            INSERT INTO user_information (
                username,
                email,
                password,
                pro_level,
                pro_end_time,
                created_time
            ) VALUES ('{}','{}','{}','{}','{}','{}')
            RETURNING *
            "#,
            user_profile.username,
            user_profile.email,
            user_profile.password,
            user_profile.pro_level,
            user_profile.pro_end_time,
            user_profile.created_time,
        );
        let user_info = sqlx::query_as::<_, UserInformation>(&sql)
            .fetch_one(self.db_service.as_ref())
            .await?;
        Ok(user_info)
    }

    async fn find_user_by_email(
        &self,
        email: &Email,
    ) -> Result<Option<UserInformation>, Self::Error> {
        let sql = format!(
            r#"
            SELECT * FROM user_information WHERE email = '{}'
            "#,
            email
        );
        let res = sqlx::query_as::<_, UserInformation>(&sql)
            .fetch_optional(self.db_service.as_ref())
            .await?;
        Ok(res)
    }
}
