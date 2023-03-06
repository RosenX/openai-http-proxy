use crate::common::responder::{SuccessResponse, ErrorResponse};
use crate::common::utils::crypto::PasswordVerify;
use crate::models::request::login_req::LoginReq;
use crate::models::request::register_req::RegisterReq;
use crate::models::response::user_info::{BasicProfile, UserInfo};
use crate::routes::authorization::{JsonWebTokenTool, JwtToken, Token, AuthorizedUser};

use crate::common::errors::InternalError;
use rocket::fairing::AdHoc;
use rocket::serde::json::{Json};
use rocket::{post, State, routes, get};
use log::{info};
use sea_orm::*;


#[post("/register", data = "<info>")]
async fn register_by_email(
    info: Json<RegisterReq>, 
    db: &State<DatabaseConnection>,
    jwt: &State<JsonWebTokenTool>) 
    ->  Result<SuccessResponse<JwtToken>, ErrorResponse>
{
    let info = info.into_inner();
    info!("{}", info);
    let user_model = info.create_user(db.inner()).await?;
    let tokens = jwt.encode_tokens(BasicProfile::from(user_model))?; 
    Ok(SuccessResponse::Created(Json(tokens)))
}

#[post("/login", data = "<info>")]
async fn login_by_email(
    info: Json<LoginReq>, 
    db: &State<DatabaseConnection>,
    jwt: &State<JsonWebTokenTool>
) ->  Result<SuccessResponse<JwtToken>, ErrorResponse>
{
    let info = info.into_inner();
    match info.find_user_by_email(db.inner()).await? {
        Some(user) => {
            match info.verify(&user.hash_password) {
                Ok(true) => {
                    let token = jwt.encode_tokens(BasicProfile::from(user))
                        .map_err(|err| InternalError::JsonWebTokenError(err.to_string()))?; 
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
    let data =  jwt
        .decode_refresh_token(refresh_token.into_inner())
        .map_err(|err| InternalError::JsonWebTokenError(err.to_string()))?;

    let new_token = jwt
        .encode_tokens(data.data)
        .map_err(|err| InternalError::JsonWebTokenError(err.to_string()))?;
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
