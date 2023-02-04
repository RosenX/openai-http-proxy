use log::error;
use rocket::async_trait;
use sea_orm::DatabaseConnection;
use sea_orm::*;

use crate::database::DbOperator;

use super::user_profile;

#[async_trait]
impl DbOperator<user_profile::ActiveModel, user_profile::Model> for DatabaseConnection {
    type Error = anyhow::Error;
    async fn insert_item(&self, item: user_profile::ActiveModel) 
        -> Result<user_profile::Model, Self::Error> 
    {
        let entity = item.insert(self).await.map_err(|err| {
            error!("insert_item: {:?}", err);
            err
        })?;
        Ok(entity)
    }
}