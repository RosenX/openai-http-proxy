
use sea_orm::*;

use crate::config::DatabaseConfig;

pub async fn setup_database(config: &DatabaseConfig) -> Result<DatabaseConnection, DbErr> {
    let url = format!("{}/{}", config.url, config.database);
    let db = Database::connect(&url).await?;
    Ok(db)
}