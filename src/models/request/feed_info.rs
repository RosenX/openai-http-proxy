use crate::common::errors::InternalError;
use crate::entities::prelude::*;
use crate::entities::{subscribe_source};
use rocket::serde::Deserialize;
use sea_orm::*;

#[derive(Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ExistSourceInfo {
    pub url: String,
    pub name: Option<String>,
    pub icon: Option<String>,
}

impl From<ExistSourceInfo> for subscribe_source::ActiveModel {
    fn from(info: ExistSourceInfo) -> Self {
        Self {
            uri_identity: ActiveValue::Set(info.url),
            ..Default::default()
        }
    }
}

impl ExistSourceInfo {
    fn new(url: String, name: Option<String>, icon: Option<String>) -> Self {
        Self {
            url: url,
            name: name,
            icon: icon,
        }
    }

    pub async fn create_exist_source(&self, db: &DatabaseConnection) 
    -> Result<subscribe_source::Model, InternalError> {
        let source = SubscribeSource::find()
            .filter(subscribe_source::Column::UriIdentity.eq(self.url.clone()))
            .one(db)
            .await
            .map_err(|err| InternalError::DatabaseError(err.to_string()))?;

        let source = match source {
            None => {
                let source:subscribe_source::ActiveModel = self.clone().into();
                let source = source
                    .insert(db).await
                    .map_err(|err| InternalError::DatabaseError(err.to_string()))?;
                source
            },
            Some(source) => source,
        };
        Ok(source)
    }
}
