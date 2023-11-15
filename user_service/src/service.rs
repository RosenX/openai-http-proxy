use abi::{
    timestamp_to_datetime, DbService, InternalError, PurchaseVerifyRequest, PurchaseVerifyResponse,
    UserActivityRequest, UserId, UserPurchaseDetail, UserServiceConfig, VipStatus,
    APP_STORE_VERIFY_URL, APP_STORE_VERIFY_URL_SANDBOX,
};
use apple_app_store_receipts::objects::{request_body::RequestBody, response_body::ResponseBody};
use async_trait::async_trait;
use tracing::info;

use crate::{UserService, UserServiceApi};

impl UserService {
    pub fn new(db: DbService, config: UserServiceConfig) -> Self {
        Self { db, config }
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
        .execute(self.db.as_ref())
        .await
        .map_err(|e| InternalError::DatabaseInsertError(e.to_string()))?;
        Ok(())
    }

    async fn purchase_verify(
        &self,
        user_id: &UserId,
        request: PurchaseVerifyRequest,
    ) -> Result<PurchaseVerifyResponse, InternalError> {
        let purchase_detail = self.get_purchase_detail(user_id, &request).await?;
        if purchase_detail.is_some() {
            return Ok(PurchaseVerifyResponse { verify_pass: true });
        }
        let verify_pass = self.verify_purchase(&request).await?;
        if !verify_pass {
            return Ok(PurchaseVerifyResponse { verify_pass });
        }
        self.save_purchase_detail(user_id, &request).await?;
        self.update_vip_status(user_id, &request).await?;
        Ok(PurchaseVerifyResponse { verify_pass })
    }

    async fn vip_status(&self, user_id: &UserId) -> Result<Option<VipStatus>, InternalError> {
        let status = self.get_vip_status(user_id).await?;
        Ok(status)
    }
}

impl UserService {
    pub async fn verify_purchase(
        &self,
        request: &PurchaseVerifyRequest,
    ) -> Result<bool, InternalError> {
        let verify_pass_prod = self
            .apple_verify(APP_STORE_VERIFY_URL.to_owned(), request)
            .await?;
        if verify_pass_prod {
            info!("[Purchase Verify] verify pass in prod");
            return Ok(true);
        }
        let verify_pass_sandbox = self
            .apple_verify(APP_STORE_VERIFY_URL_SANDBOX.to_owned(), request)
            .await?;
        if verify_pass_sandbox {
            info!("[Purchase Verify] verify pass in sandbox");
            return Ok(true);
        }
        info!("[Purchase Verify] verify fail");
        Ok(false)
    }

    async fn apple_verify(
        &self,
        host: String,
        request: &PurchaseVerifyRequest,
    ) -> Result<bool, InternalError> {
        let verify_request = RequestBody::new(
            request.purchase_detail.verify_data.as_str(),
            self.config.app_store_password.as_str(),
            None,
        );
        let resp: ResponseBody = reqwest::Client::new()
            .post(host)
            .json(&verify_request)
            .send()
            .await
            .map_err(|e| InternalError::HttpError(e.to_string()))?
            .json()
            .await
            .map_err(|e| InternalError::HttpError(e.to_string()))?;

        match resp {
            ResponseBody::Success(_) => Ok(true),
            ResponseBody::Error(e) => match e.is_retryable {
                Some(true) => Err(InternalError::HttpError(
                    "[Vip Verify] need retry".to_string(),
                )),
                _ => Ok(false),
            },
        }
    }

    async fn get_purchase_detail(
        &self,
        user_id: &UserId,
        request: &PurchaseVerifyRequest,
    ) -> Result<Option<UserPurchaseDetail>, InternalError> {
        let detail = sqlx::query_as(
            "
            SELECT
                *
            FROM purchase_detail
            WHERE user_id = $1 AND product_id = $2 AND purchase_time = $3",
        )
        .bind(user_id)
        .bind(request.purchase_detail.product_id.to_owned())
        .bind(timestamp_to_datetime(request.purchase_detail.purchase_time))
        .fetch_optional(self.db.as_ref())
        .await
        .map_err(|e| {
            InternalError::DatabaseSelectError(format!(
                "[Purchase Verify] get purchase detail {}",
                e
            ))
        })?;
        Ok(detail)
    }

    async fn save_purchase_detail(
        &self,
        user_id: &UserId,
        request: &PurchaseVerifyRequest,
    ) -> Result<(), InternalError> {
        sqlx::query(
            "
            INSERT INTO purchase_detail (
                user_id,
                product_id,
                purchase_time,
                source
            )
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (user_id, product_id, purchase_time) DO NOTHING
            ",
        )
        .bind(user_id)
        .bind(request.purchase_detail.product_id.to_owned())
        .bind(timestamp_to_datetime(request.purchase_detail.purchase_time))
        .bind(request.purchase_detail.source.to_owned())
        .execute(self.db.as_ref())
        .await
        .map_err(|_| {
            InternalError::DatabaseInsertError(
                "[Purchase Verify] Purchase detail insert error".to_string(),
            )
        })?;
        Ok(())
    }

    async fn update_vip_status(
        &self,
        user_id: &UserId,
        request: &PurchaseVerifyRequest,
    ) -> Result<(), InternalError> {
        let user_vip_status = self.get_vip_status(user_id).await?;
        let is_forever = request.is_forever();
        let duration = request.get_pro_duration_ms();
        match user_vip_status {
            Some(status) => {
                self.save_vip_status(
                    user_id,
                    is_forever | status.is_forever,
                    status.pro_end_time + duration,
                )
                .await?;
            }
            None => {
                self.save_vip_status(user_id, is_forever, request.get_pro_end_time())
                    .await?
            }
        }
        Ok(())
    }

    async fn save_vip_status(
        &self,
        user_id: &UserId,
        is_forever: bool,
        pro_end_time: i64,
    ) -> Result<(), InternalError> {
        sqlx::query(
            "
            INSERT INTO vip_status (
                user_id,
                is_forever,
                pro_end_time
            )
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id) DO UPDATE SET
                is_forever = EXCLUDED.is_forever,
                pro_end_time = EXCLUDED.pro_end_time
            ",
        )
        .bind(user_id)
        .bind(is_forever)
        .bind(timestamp_to_datetime(pro_end_time))
        .execute(self.db.as_ref())
        .await
        .map_err(|e| {
            InternalError::DatabaseInsertError(format!("[Purchase Verify] update vip status {}", e))
        })?;
        Ok(())
    }

    async fn get_vip_status(&self, user_id: &UserId) -> Result<Option<VipStatus>, InternalError> {
        let status = sqlx::query_as::<_, VipStatus>("SELECT * FROM vip_status WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(self.db.as_ref())
            .await
            .map_err(|e| {
                InternalError::DatabaseSelectError(format!("[Vip] get vip status {}", e))
            })?;
        Ok(status)
    }
}
