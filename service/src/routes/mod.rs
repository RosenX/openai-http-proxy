use rocket::fairing::AdHoc;

pub mod catcher;
pub mod content;
pub mod user;

pub fn api() -> AdHoc {
    AdHoc::on_ignite("Loading Routes Of Service", |rocket| async {
        rocket
            .attach(user::stage())
            .attach(content::stage())
            .attach(catcher::stage())
    })
}
