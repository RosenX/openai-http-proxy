#[derive(serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterInfo {
    #[prost(string, tag = "1")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub password: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginInfo {
    #[prost(string, tag = "1")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientInfo {
    #[prost(string, tag = "1")]
    pub client_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthRequest {
    #[prost(message, optional, tag = "1")]
    pub client_info: ::core::option::Option<ClientInfo>,
    #[prost(oneof = "auth_request::Request", tags = "2, 3")]
    pub request: ::core::option::Option<auth_request::Request>,
}
/// Nested message and enum types in `AuthRequest`.
pub mod auth_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        #[prost(message, tag = "2")]
        RegisterInfo(super::RegisterInfo),
        #[prost(message, tag = "3")]
        LoginInfo(super::LoginInfo),
    }
}
