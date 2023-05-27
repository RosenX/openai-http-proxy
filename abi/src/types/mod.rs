mod content;
mod model;
mod request;
mod response;
mod user;

use chrono::{DateTime, Utc};
pub use model::*;
pub use request::*;
pub use response::*;
use sqlx::{postgres::PgArguments, query::Query, Postgres};

pub use user::*;

use crate::{
    DbService, DecodeJwt, EncodeJwt, Id, InternalError, JwtConfig, Token, INSERT_CHUNK_SIZE,
};

impl EncodeJwt for UserProfile {
    type Error = InternalError;
    fn encode_tokens(self, config: &JwtConfig) -> Result<JwtTokens, Self::Error> {
        let tokens = JwtTokens {
            access_token: self
                .clone()
                .encode_token(&config.access_key, config.access_expiration_hour)
                .map_err(|e| InternalError::JwtEncodeError(e.to_string()))?,
            refresh_token: self
                .encode_token(&config.refresh_key, config.refresh_expiration_hour)
                .map_err(|e| InternalError::JwtEncodeError(e.to_string()))?,
        };
        Ok(tokens)
    }
}

impl DecodeJwt<UserProfile> for Token {
    type Error = InternalError;
    fn decode_access_token(self, config: &JwtConfig) -> Result<UserProfile, Self::Error> {
        let payload = self
            .decode(&config.access_key)
            .map_err(|e| InternalError::InvalidToken(e.to_string()))?;
        Ok(payload.data)
    }
    fn decode_refresh_token(self, config: &JwtConfig) -> Result<UserProfile, Self::Error> {
        let payload = self
            .decode(&config.refresh_key)
            .map_err(|e| InternalError::InvalidToken(e.to_string()))?;
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
    Datetime(DateTime<Utc>),
    NullableDatetime(Option<DateTime<Utc>>),
    EnumFeedType(FeedTypeServer),
    NullableEnumFeedType(Option<FeedTypeServer>),
    I32Array(Vec<i32>),
    Bool(bool),
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
            SqlValue::Datetime(datetime) => query.bind(datetime),
            SqlValue::NullableDatetime(datetime) => query.bind(datetime),
            SqlValue::EnumFeedType(feed_type) => query.bind(feed_type),
            SqlValue::I32Array(vec) => query.bind(vec),
            SqlValue::Bool(bool) => query.bind(bool),
            SqlValue::NullableEnumFeedType(feed_type) => query.bind(feed_type),
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
            SqlValue::Datetime(datetime) => SqlValue::Datetime(datetime.to_owned()),
            SqlValue::NullableDatetime(datetime) => SqlValue::NullableDatetime(datetime.to_owned()),
            SqlValue::EnumFeedType(feed_type) => SqlValue::EnumFeedType(feed_type.to_owned()),
            SqlValue::I32Array(vec) => SqlValue::I32Array(vec.to_owned()),
            SqlValue::Bool(bool) => SqlValue::Bool(bool.to_owned()),
            SqlValue::NullableEnumFeedType(feed_type) => {
                SqlValue::NullableEnumFeedType(feed_type.to_owned())
            }
        }
    }
}

pub trait DbTableName {
    fn table_name() -> String;
}

pub trait InsertSqlProvider: DbTableName {
    fn sql_columns() -> String;
    fn sql_values(&self, user_id: Id, client_id: Id) -> Vec<SqlValue>;
    fn sql_conflict(client_id: Id) -> String;
}

// 一个通用的生成SQL插入语句的函数
pub fn generate_insert_query<T: InsertSqlProvider>(
    data: &[T],
    user_id: Id,
    client_id: Id,
) -> (String, Vec<SqlValue>) {
    let columns = T::sql_columns();
    let mut insert_query = format!("INSERT INTO {} ({}) VALUES ", T::table_name(), columns);
    let mut bindings: Vec<SqlValue> = Vec::new();

    for (i, item) in data.iter().enumerate() {
        let values = item.sql_values(user_id, client_id);
        insert_query.push('(');
        for (j, value) in values.iter().enumerate() {
            insert_query.push_str(&format!("${},", i * values.len() + j + 1));
            bindings.push(value.to_owned());
        }
        insert_query.pop(); // 移除最后一个逗号
        insert_query.push_str("),");
    }
    insert_query.pop(); // 移除最后一个逗号
    insert_query.push_str(T::sql_conflict(client_id).as_str());
    // info!("insert binding: {}", bindings);

    (insert_query, bindings)
}

// 在事务中执行批量插入
pub async fn execute_bulk_insert<T: InsertSqlProvider>(
    database: &DbService,
    data: Vec<T>,
    user_id: Id,
    client_id: Id,
) -> Result<(), InternalError> {
    // 开启事务
    let mut tx = database
        .begin()
        .await
        .map_err(|e| InternalError::CouldNotStartTransaction(e.to_string()))?;

    for chunk in data.chunks(INSERT_CHUNK_SIZE) {
        let (insert_query, bindings) = generate_insert_query(chunk, user_id, client_id);
        let mut query = sqlx::query(&insert_query);

        for binding in bindings {
            query = binding.bind(query);
        }

        query
            .execute(&mut tx)
            .await
            .map_err(|e| InternalError::DatabaseInsertError(e.to_string()))?;
    }

    tx.commit()
        .await
        .map_err(|e| InternalError::CouldNotStartTransaction(e.to_string()))?;
    Ok(())
}
