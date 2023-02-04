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
pub trait DbOperator<In, Out> {
    type Error;
    async fn insert_item(&self, model: In) -> Result<Out, Self::Error>;
}