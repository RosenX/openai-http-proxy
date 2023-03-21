use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Serialize)]
pub struct CreateFeedRequest {
    pub url: String,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct PostReq {
    pub last_content_id: i32,
}
