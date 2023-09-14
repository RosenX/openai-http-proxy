use abi::{timestamp_to_datetime, DbService, InternalError, UserActivityRequest};
use async_trait::async_trait;

use crate::{UserService, UserServiceApi};

impl UserService {
    pub fn new(db: DbService) -> Self {
        Self { db_service: db }
    }
}

#[async_trait]
impl UserServiceApi for UserService {
    async fn user_activity(&self, request: UserActivityRequest) -> Result<(), InternalError> {
        let date = timestamp_to_datetime(request.activity_time);
        sqlx::query(
            "
            INSERT INTO user_activity (
                device_id,
                date,
                device_type,
                user_id,
                use_times,
                feed_num,
                keyword_num,
                app_version,
                system,
                system_version
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (device_id, date) DO UPDATE SET
                use_times = EXCLUDED.use_times+user_activity.use_times,
                feed_num = EXCLUDED.feed_num,
                keyword_num = EXCLUDED.keyword_num,
                app_version = EXCLUDED.app_version,
                device_type = EXCLUDED.device_type,
                system = EXCLUDED.system,
                system_version = EXCLUDED.system_version
            ",
        )
        .bind(request.device_info.device_id)
        .bind(date)
        .bind(request.device_info.device_type)
        .bind(request.user_id)
        .bind(1)
        .bind(request.feed_num)
        .bind(request.keyword_num)
        .bind(request.app_version)
        .bind(request.device_info.system)
        .bind(request.device_info.system_version)
        .execute(self.db_service.as_ref())
        .await
        .map_err(|e| InternalError::DatabaseInsertError(e.to_string()))?;
        Ok(())
    }
}
