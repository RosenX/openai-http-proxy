use rocket::{FromForm};
use serde::{Deserialize, Serialize};
// use serde_repr::{Deserialize_repr, Serialize_repr};

// #[derive(Deserialize_repr, Serialize_repr, Clone, PartialEq, FromFormField)]
// #[repr(u8)]
// pub enum PostOpType {
//     Pull = 1,
//     Modify = 2
// }

#[derive(Deserialize, Clone, Serialize, FromForm)]
pub struct PostReq {
    // pub op_type: PostOpType,
    pub latest_post_id: i32,
}
