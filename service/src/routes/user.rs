use crate::common::responder::{ErrorResponse, SuccessResponse};
use crate::routes::AuthService;

use abi::{EncodeJwt, RegisterReq, RegisterResponse};
use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket::{post, routes, State};
use user_service::{UserService, UserServiceApi};

#[post("/register", data = "<request>")]
async fn register_by_email(
    request: Json<RegisterReq>,
    user_service: &State<UserService>,
    auth_service: &State<AuthService>,
) -> Result<SuccessResponse<RegisterResponse>, ErrorResponse> {
    let response = user_service.register_by_email(request.into_inner()).await?;
    let tokens = response.encode_tokens(&auth_service.config.jwt)?; // todo
    Ok(SuccessResponse::Success(Json(tokens.into())))
}

// #[post("/login", data = "<info>")]
// async fn login_by_email(
//     info: Json<LoginReq>,
//     db: &State<DbPool>,
//     jwt: &State<JwtService>,
// ) -> Result<SuccessResponse<JwtToken>, ErrorResponse> {
//     let req = info.into_inner();
//     match req.find_user_by_email(db.inner()).await? {
//         Some(user) => match req.verify(&user.password) {
//             Ok(true) => {
//                 let token = jwt.encode_tokens(user.into())?;
//                 info!("{}", token);
//                 Ok(SuccessResponse::Success(Json(token)))
//             }
//             _ => Err(InternalError::WrongPassword.into()),
//         },
//         None => Err(InternalError::UserNotExist.into()),
//     }
// }

// #[post("/refresh_token", data = "<refresh_token>", format = "json")]
// fn refresh_token(
//     refresh_token: Token,
// ) -> Result<SuccessResponse<JwtToken>, ErrorResponse> {
//     let data = jwt.decode_refresh_token(refresh_token)?;
//     let new_token = jwt.encode_tokens(data.data)?;
//     info!("{}", new_token);
//     Ok(SuccessResponse::Created(new_token.into()))
// }

// #[get("/me", format = "json")]
// fn get_profile(user: AuthorizedUser) -> Result<SuccessResponse<UserInfo>, ErrorResponse> {
//     let user_info = UserInfo::new(user);
//     Ok(SuccessResponse::Success(user_info.into()))
// }

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Loading Routes About User", |rocket| async {
        rocket.mount(
            "/user",
            routes![
                register_by_email,
                // login_by_email,
                // refresh_token,
                // get_profile,
            ],
        )
    })
}
