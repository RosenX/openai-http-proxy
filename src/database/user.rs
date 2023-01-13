use serde::serialize;

type Url = String;

#[derive(Serialize)]
pub struct UserProfile {
    id: i32,
    username: String,
    email: String,
    image: Option<Url>,
    hash_password: String,
    is_pro: bool,
    pro_end_time: u64
}