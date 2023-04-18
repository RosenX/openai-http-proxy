use abi::{ClientInfo, DbService, Email, Id, InternalError, UserInformation};
use async_trait::async_trait;

pub struct UserManager {
    db_service: DbService,
}

#[async_trait]
pub trait UserManagerOp {
    type Error;
    async fn create(&self, user_profile: UserInformation) -> Result<UserInformation, Self::Error>;
    async fn find_user_by_email(
        &self,
        email: &Email,
    ) -> Result<Option<UserInformation>, Self::Error>;
    async fn register_client(
        &self,
        user_id: Id,
        client_name: String,
    ) -> Result<ClientInfo, Self::Error>;
}

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

    async fn register_client(
        &self,
        user_id: Id,
        client_name: String,
    ) -> Result<ClientInfo, Self::Error> {
        let sql = format!(
            r#"
            INSERT INTO user_device (user_id, device_name) VALUES ('{}','{}')
            RETURNING *
            "#,
            user_id, client_name
        );
        let client = sqlx::query_as::<_, ClientInfo>(&sql)
            .fetch_one(self.db_service.as_ref())
            .await?;
        Ok(client)
    }
}
