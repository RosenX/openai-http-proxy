#[macro_use] extern crate rocket;

use crate::base::base_struct::CommonResponseCode;

struct UserLoginData {
    email: String,
    password: String
}

#[post("/user/create", data = "<user>")]
async fn user_login(user: Form<Strict<UserLoginData>>, db: DbConnection) -> Result<CommonResponseCode>{
    let email = user.into_inner().email;
    let password = user.into_inner().password;
    sqlx::query!("INSERT INTO posts (title, text) VALUES (?, ?)", post.title, post.text)
        .execute(&mut *db)
        .await?;

    Ok(Created::new("/").body(post))
}