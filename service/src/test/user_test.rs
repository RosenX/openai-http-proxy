use rocket::{self, uri};
// use rocket::local::blocking::Client;
use rocket::local::asynchronous::Client;
use rocket::http::Status;
use crate::rocket_app;
use crate::routes::user;

#[test]
fn test_user_create() {
    let client = Client::tracked(rocket_app()).expect("invalid rocket instance");
    let mut response = client.get(uri!(user::user_register)).dispatch();
    assert_eq!(response.status(), Status::Ok);
}
