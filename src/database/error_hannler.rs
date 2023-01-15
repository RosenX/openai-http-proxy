use rocket::{Responder, serde::json::Json};
use sea_orm::DbErr;
use crate::utils::prelude::*;

#[derive(Responder)]
#[response(status = 500, content_type = "json")]
pub struct DbErrorResponder(pub Json<StringBodyData>);

impl From<StringBodyData> for DbErrorResponder {
    fn from(err: StringBodyData) -> DbErrorResponder {
        DbErrorResponder(Json(err))
    }
}

impl From<DbErr> for DbErrorResponder {
    fn from(err: DbErr) -> DbErrorResponder {
        DbErrorResponder(Json(StringBodyData{data: err.to_string()}))
    }
}

// impl From<String> for DbErrorResponder {
//     fn from(string: String) -> DbErrorResponder {
//         DbErrorResponder { message: string }
//     }
// }

// impl From<&str> for DbErrorResponder {
//     fn from(str: &str) -> DbErrorResponder {
//         str.to_owned().into()
//     }
// }
