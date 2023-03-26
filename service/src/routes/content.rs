use crate::{
    auth_service::AuthorizedUser,
    common::responder::{ErrorResponse, SuccessResponse},
};

use abi::{FetchContentResponse, UserProfile};
use content_service::{ContentService, ContentServiceApi};
use rocket::{fairing::AdHoc, get, routes, State};
use rocket::{serde::json::Json, FromForm};
use user_service::{UserService, UserServiceApi};

#[derive(FromForm)]
struct FetchContentParam {
    is_latest: bool,
    latest_id: i32,
    nums: i32,
}

// if is_latest is true, return content those id is bigger than latest_content_id else return content those id is smaller than latest_content_id
#[get("/pull?<param..>")]
async fn get_latest_post(
    param: FetchContentParam,
    user: AuthorizedUser,
    user_service: &State<UserService>,
    content_service: &State<ContentService>,
) -> Result<SuccessResponse<FetchContentResponse>, ErrorResponse> {
    let user_profile: UserProfile = user.into();
    let user_contents = match param.is_latest {
        true => {
            user_service
                .query_latest_content(user_profile.id, param.latest_id, param.nums)
                .await?
        }
        false => {
            user_service
                .query_old_content(user_profile.id, param.latest_id, param.nums)
                .await?
        }
    };
    let content_ids = user_contents.iter().map(|c| c.content_id).collect();
    let contents = content_service.query_contents(content_ids).await?;
    Ok(SuccessResponse::Success(Json(FetchContentResponse {
        user_contents,
        contents,
    })))
}

// #[get("/pull/<feed_id>", data = "<req>")]
// async fn get_latest_post_by_id(
//     user: AuthorizedUser,
//     req: Json<PostReq>,
//     pool: &State<MySqlService>,
//     feed_id: i32,
// ) -> Result<SuccessResponse<Vec<UserPost>>, ErrorResponse> {
//     let posts =
//         UserPost::retrieve_latest_post_by_id(pool, user.id, req.latest_post_id, feed_id)
//             .await?;
//     Ok(SuccessResponse::Success(Json(posts)))
// }

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Loading Routes About Post", |rocket| async {
        rocket.mount("/post", routes![get_latest_post])
    })
}
