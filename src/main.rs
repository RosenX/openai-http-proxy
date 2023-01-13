mod entities;

use futures::executor::block_on;
use sea_orm::*;
use entities::{prelude::*, *};



const DATABASE_URL: &str = "mysql://root:1234qwer@localhost:3306";
const DB_NAME: &str = "feed_inbox";

async fn db_init() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    db.execute(Statement::from_string(
        db.get_database_backend(),
        format!("CREATE DATABASE IF NOT EXISTS `{}`;", DB_NAME),
    )).await?;
    let url = format!("{}/{}", DATABASE_URL, DB_NAME);
    let db = Database::connect(&url).await?;

    let user = user_profile::ActiveModel {
        username: ActiveValue::Set("luosen".to_owned()),
        email: ActiveValue::Set("luosen@example.com".to_owned()),
        hash_password: ActiveValue::Set("1234qwer".to_owned()),
        ..Default::default()
    };
    let res = UserProfile::insert(user).exec(&db).await?;
    Ok(())
}



fn main() {
    if let Err(err) = block_on(db_init()) {
        panic!("{}", err);
    }
}