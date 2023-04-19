mod content;
mod model;
mod request;
mod response;
mod user;

pub use model::*;
pub use request::*;
pub use response::*;
use sqlx::{postgres::PgArguments, query::Query, Postgres};
pub use user::*;

use crate::{DbService, DecodeJwt, EncodeJwt, Id, InternalError, JwtConfig, Token};

impl EncodeJwt for UserProfile {
    type Error = InternalError;
    fn encode_tokens(self, config: &JwtConfig) -> Result<JwtTokens, Self::Error> {
        let tokens = JwtTokens {
            access_token: self
                .clone()
                .encode_token(&config.access_key, config.access_expiration_hour)?,
            refresh_token: self
                .encode_token(&config.refresh_key, config.refresh_expiration_hour)?,
        };
        Ok(tokens)
    }
}

impl DecodeJwt<UserProfile> for Token {
    type Error = InternalError;
    fn decode_access_token(self, config: &JwtConfig) -> Result<UserProfile, Self::Error> {
        let payload = self.decode(&config.access_key)?;
        Ok(payload.data)
    }
    fn decode_refresh_token(self, config: &JwtConfig) -> Result<UserProfile, Self::Error> {
        let payload = self.decode(&config.refresh_key)?;
        Ok(payload.data)
    }
}

pub trait OptionDisplay {
    fn display(self) -> String;
}

impl OptionDisplay for Option<String> {
    fn display(self) -> String {
        match self {
            Some(string) => string,
            None => "null".to_string(),
        }
    }
}

impl OptionDisplay for Option<i64> {
    fn display(self) -> String {
        match self {
            Some(i64) => i64.to_string(),
            None => "null".to_string(),
        }
    }
}

impl OptionDisplay for Option<bool> {
    fn display(self) -> String {
        match self {
            Some(bool) => bool.to_string(),
            None => "null".to_string(),
        }
    }
}

impl OptionDisplay for Option<Vec<String>> {
    fn display(self) -> String {
        match self {
            // to posgres array
            Some(vec) => format!("{{{}}}", vec.join(",")),
            None => "null".to_string(),
        }
    }
}

// 定义一个SqlValue枚举类型，用于表示不同的数据类型
pub enum SqlValue {
    String(String),
    I32(i32),
    I64(i64),
    NullableI32(Option<i32>),
    NullableI64(Option<i64>),
    NullableString(Option<String>),
    NullableStringArray(Option<Vec<String>>),
    StringArray(Vec<String>),
    Boolean(bool),
}

pub trait Bing {
    type Q;
    fn bing(&self, query: Self::Q) -> Self::Q;
}

impl SqlValue {
    fn bind(self, query: Query<Postgres, PgArguments>) -> Query<Postgres, PgArguments> {
        match self {
            SqlValue::String(string) => query.bind(string),
            SqlValue::I32(i32) => query.bind(i32),
            SqlValue::I64(i64) => query.bind(i64),
            SqlValue::NullableI32(i32) => query.bind(i32),
            SqlValue::NullableI64(i64) => query.bind(i64),
            SqlValue::NullableString(string) => query.bind(string),
            SqlValue::StringArray(vec) => query.bind(vec),
            SqlValue::Boolean(bool) => query.bind(bool),
            SqlValue::NullableStringArray(vec) => query.bind(vec),
        }
    }
}

// impl Owned for SqlValue
impl ToOwned for SqlValue {
    type Owned = SqlValue;
    fn to_owned(&self) -> SqlValue {
        match self {
            SqlValue::String(string) => SqlValue::String(string.to_owned()),
            SqlValue::I32(i32) => SqlValue::I32(i32.to_owned()),
            SqlValue::I64(i64) => SqlValue::I64(i64.to_owned()),
            SqlValue::NullableI32(i32) => SqlValue::NullableI32(i32.to_owned()),
            SqlValue::NullableI64(i64) => SqlValue::NullableI64(i64.to_owned()),
            SqlValue::NullableString(string) => SqlValue::NullableString(string.to_owned()),
            SqlValue::StringArray(vec) => SqlValue::StringArray(vec.to_owned()),
            SqlValue::Boolean(bool) => SqlValue::Boolean(bool.to_owned()),
            SqlValue::NullableStringArray(vec) => SqlValue::NullableStringArray(vec.to_owned()),
        }
    }
}

pub trait InsertSqlProvider {
    fn sql_columns() -> String;
    fn sql_values(&self, user_id: Id) -> Vec<SqlValue>;
}

// 一个通用的生成SQL插入语句的函数
pub fn generate_insert_query<T: InsertSqlProvider>(
    table_name: &str,
    data: Vec<T>,
    user_id: Id,
) -> (String, Vec<SqlValue>) {
    let columns = T::sql_columns();
    let mut insert_query = format!("INSERT INTO {} ({}) VALUES ", table_name, columns);
    let mut bindings: Vec<SqlValue> = Vec::new();

    for (i, item) in data.iter().enumerate() {
        let values = item.sql_values(user_id);
        insert_query.push('(');
        for (j, value) in values.iter().enumerate() {
            insert_query.push_str(&format!("${},", i * values.len() + j + 1));
            bindings.push(value.to_owned());
        }
        insert_query.pop(); // 移除最后一个逗号
        insert_query.push_str("),");
    }

    insert_query.pop(); // 移除最后一个逗号
    (insert_query, bindings)
}

// 在事务中执行批量插入
pub async fn execute_bulk_insert<T: InsertSqlProvider>(
    database: &DbService,
    table_name: &str,
    data: Vec<T>,
    user_id: Id,
) -> Result<(), sqlx::Error> {
    let (insert_query, bindings) = generate_insert_query(table_name, data, user_id);

    let mut tx = database.begin().await?;
    let mut query = sqlx::query(&insert_query);

    for binding in bindings {
        query = binding.bind(query);
    }

    query.execute(&mut tx).await?;
    tx.commit().await?;
    Ok(())
}
