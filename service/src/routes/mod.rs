use rocket::fairing::AdHoc;

pub mod catcher;
pub mod feed;
pub mod post;
pub mod user;

pub fn api() -> AdHoc {
    AdHoc::on_ignite("Loading Routes Of Service", |rocket| async {
        rocket
            .attach(user::stage())
            .attach(feed::stage())
            .attach(post::stage())
            .attach(catcher::stage())
    })
}
