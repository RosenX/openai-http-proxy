use crate::common::responder::{SuccessResponse, ErrorResponse};
use crate::common::utils::crypto::PasswordVerify;
use crate::database::{DatabasePool};
use crate::database::user_profile::UserProfile;
use crate::models::request::login_req::LoginReq;
use crate::models::request::register_req::RegisterReq;
use crate::models::response::user_info::{BasicProfile, UserInfo};
use crate::routes::authorization::{JsonWebTokenTool, JwtToken, Token, AuthorizedUser};

use crate::common::errors::InternalError;
use rocket::fairing::AdHoc;
use rocket::serde::json::{Json};
use rocket::{post, State, routes, get};
use log::{info};

#[post("/register", data = "<info>")]
async fn register_by_email(
    info: Json<RegisterReq>, 
    db: &State<DatabasePool>,
    jwt: &State<JsonWebTokenTool>) 
    ->  Result<SuccessResponse<JwtToken>, ErrorResponse>
{
    let info = UserProfile::try_from(info.into_inner())?;
    info!("{}", info);
    let user_id = info
        .create_user(db.inner())
        .await
        .map_err(|e| InternalError::DuplicateEmail(e.to_string()))?;
    let tokens = jwt.encode_tokens(BasicProfile::from(info))?; 
    Ok(SuccessResponse::Created(Json(tokens)))
}

#[post("/login", data = "<info>")]
async fn login_by_email(
    info: Json<LoginReq>, 
    db: &State<DatabasePool>,
    jwt: &State<JsonWebTokenTool>
) ->  Result<SuccessResponse<JwtToken>, ErrorResponse>
{
    let req = info.into_inner();
    match req.find_user_by_email(db.inner()).await? {
        Some(user) => {
            match req.verify(&user.password) {
                Ok(true) => {
                    let token = jwt.encode_tokens(user.into())?; 
                    Ok(SuccessResponse::Accepted(Json(token)))
                },
                _ => Err(InternalError::WrongPassword.into()),
            }
        },
        None => Err(InternalError::UserNotExist.into()),
    }
}

#[post("/refresh-token", data = "<refresh_token>", format = "json")]
fn refresh_token(
    refresh_token: Json<Token>,
    jwt: &State<JsonWebTokenTool>
) ->  Result<SuccessResponse<JwtToken>, ErrorResponse>
{
    let data =  jwt.decode_refresh_token(refresh_token.into_inner())?;
    let new_token = jwt.encode_tokens(data.data)?;
    Ok(SuccessResponse::Created(new_token.into()))
}

#[get("/me", format = "json")]
fn get_profile(
    user: AuthorizedUser,
) ->  Result<SuccessResponse<UserInfo>, ErrorResponse>
{
    let user_info = UserInfo::new(user);
    Ok(SuccessResponse::Success(user_info.into()))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("User Stage", |rocket| async {
        rocket.mount("/user", routes![
            register_by_email, 
            login_by_email, 
            refresh_token,
            get_profile,
        ])
    })
}
