mod error;
mod models;
mod utils;

pub use error::*;
pub use models::*;
pub use utils::*;

use sqlx::MySql;

pub type DbPool = sqlx::Pool<MySql>;
