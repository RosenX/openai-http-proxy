
use sea_orm::*;

const DATABASE_URL: &str = "mysql://root:1234qwer@localhost:3306";
const DB_NAME: &str = "feed_inbox";

pub async fn setup_database() -> Result<DatabaseConnection, DbErr> {
    let url = format!("{}/{}", DATABASE_URL, DB_NAME);
    let db = Database::connect(&url).await?;
    Ok(db)
}