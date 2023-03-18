use sqlx::MySql;

pub type DbPool = sqlx::Pool<MySql>;
