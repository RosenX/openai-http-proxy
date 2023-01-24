use thiserror;
use anyhow::Error;

#[derive(thiserror::Error, Debug)]
enum Errors<T> {
    // 数据库相关
    #[error("数据库连接失败")]
    Disconnected(T),
    #[error("数据已存在")]
    ItemExist(T),

    // 哈希相关
    HashFail(T)

    // JWT相关
}

type ErrorInfo = Error<String>;

impl from<BcryptError> for Error {
    impl From<BcryptError> for FailureJsonResponder<String> {
        fn from(err: BcryptError) -> FailureJsonResponder<String> {
            FailureJsonResponder(Json(BodyData{data: err.to_string()}))
        }
    }
}