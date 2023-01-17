use bcrypt::BcryptError;
use rocket::{serde::{Serialize, json::Json}, Responder};
use sea_orm::DbErr;

use crate::routes::authorization::JwtToken;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct BodyData<T> {
    pub data: T
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct SuccessJsonResponder<T>(pub Json<BodyData<T>>);

impl From<BodyData<String>> for SuccessJsonResponder<String> {
    fn from(data: BodyData<String>) -> SuccessJsonResponder<String> {
        SuccessJsonResponder(Json(data))
    }
}

impl From<BodyData<JwtToken>> for SuccessJsonResponder<JwtToken> {
    fn from(data: BodyData<JwtToken>) -> SuccessJsonResponder<JwtToken> {
        SuccessJsonResponder(Json(data))
    }
}

#[derive(Responder)]
#[response(status = 500, content_type = "json")]
pub struct FailureJsonResponder<T>(pub Json<BodyData<T>>);

impl From<BodyData<String>> for FailureJsonResponder<String> {
    fn from(data: BodyData<String>) -> FailureJsonResponder<String> {
        FailureJsonResponder(Json(data))
    }
}

impl From<DbErr> for FailureJsonResponder<String> {
    fn from(err: DbErr) -> FailureJsonResponder<String> {
        FailureJsonResponder(Json(BodyData{data: err.to_string()}))
    }
}

impl From<BcryptError> for FailureJsonResponder<String> {
    fn from(err: BcryptError) -> FailureJsonResponder<String> {
        FailureJsonResponder(Json(BodyData{data: err.to_string()}))
    }
}

impl From<jsonwebtoken::errors::Error>  for FailureJsonResponder<String> {
    fn from(err: jsonwebtoken::errors::Error) -> FailureJsonResponder<String> {
        FailureJsonResponder(Json(BodyData{data: err.to_string()}))
    }
}