
use sea_orm::*;

use super::DatabaseConfig;

pub async fn setup_database(config: &DatabaseConfig) -> Result<DatabaseConnection, DbErr> {
    let url = format!("{}/{}", config.url, config.database);
    let db = Database::connect(&url).await?;
    Ok(db)
}