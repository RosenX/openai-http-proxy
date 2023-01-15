use rocket::{serde::{Serialize, json::Json}, Responder};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct BodyData<T> {
    pub data: T
}

pub type StringBodyData = BodyData<String>;



#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct SuccessJsonResponder(pub Json<StringBodyData>);

impl From<StringBodyData> for SuccessJsonResponder {
    fn from(err: StringBodyData) -> SuccessJsonResponder {
        SuccessJsonResponder(Json(err))
    }
}