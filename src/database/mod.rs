use rocket::serde::Deserialize;
use rocket::async_trait;

mod setup_database;
pub use setup_database::setup_database;
#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DatabaseConfig {
    pub url: String,
    pub database: String,
}

#[async_trait]
pub trait DbOperator<Entity, ActiveModel, Model> {
    type Error;
    async fn insert_item(&self, model: ActiveModel) -> Result<Model, Self::Error>;
    async fn find_by_uniqe_field(&self, entity: Entity) -> Result<Model, Self::Error>;
}