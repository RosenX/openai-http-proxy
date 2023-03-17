use crate::common::responder::{ErrorResponse, SuccessResponse};
use crate::common::service::jwt_service::{JwtService, JwtToken, Token};
use crate::common::service::mysql_service::MySqlService;
use crate::common::utils::crypto::PasswordVerify;
use crate::database::user_profile::UserProfile;
use crate::models::request::login_req::LoginReq;
use crate::models::request::register_req::RegisterReq;
use crate::models::response::user_info::UserInfo;
use crate::routes::authorization::{AuthorizedUser};

use crate::common::errors::InternalError;
use log::info;
use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket::{get, post, routes, State};

#[post("/register", data = "<info>")]
async fn register_by_email(
    info: Json<RegisterReq>,
    db: &State<MySqlService>,
    jwt: &State<JwtService>,
) -> Result<SuccessResponse<JwtToken>, ErrorResponse> {
    let info = UserProfile::try_from(info.into_inner())?;
    info.create_user(db.inner()).await?;
    let tokens = jwt.encode_tokens(info.into())?;
    Ok(SuccessResponse::Created(Json(tokens)))
}

#[post("/login", data = "<info>")]
async fn login_by_email(
    info: Json<LoginReq>,
    db: &State<MySqlService>,
    jwt: &State<JwtService>,
) -> Result<SuccessResponse<JwtToken>, ErrorResponse> {
    let req = info.into_inner();
    match req.find_user_by_email(db.inner()).await? {
        Some(user) => match req.verify(&user.password) {
            Ok(true) => {
                let token = jwt.encode_tokens(user.into())?;
                info!("{}", token);
                Ok(SuccessResponse::Success(Json(token)))
            }
            _ => Err(InternalError::WrongPassword.into()),
        },
        None => Err(InternalError::UserNotExist.into()),
    }
}

#[post("/refresh_token", data = "<refresh_token>", format = "json")]
fn refresh_token(
    refresh_token: Token,
    jwt: &State<JwtService>,
) -> Result<SuccessResponse<JwtToken>, ErrorResponse> {
    let data = jwt.decode_refresh_token(refresh_token)?;
    let new_token = jwt.encode_tokens(data.data)?;
    info!("{}", new_token);
    Ok(SuccessResponse::Created(new_token.into()))
}

#[get("/me", format = "json")]
fn get_profile(user: AuthorizedUser) -> Result<SuccessResponse<UserInfo>, ErrorResponse> {
    let user_info = UserInfo::new(user);
    Ok(SuccessResponse::Success(user_info.into()))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Loading Routes About User", |rocket| async {
        rocket.mount(
            "/user",
            routes![
                register_by_email,
                login_by_email,
                refresh_token,
                get_profile,
            ],
        )
    })
}
