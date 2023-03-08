/// cmd - 3
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyReq {
    /// 用户token
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
}
/// cmd - 3
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyRes {
    /// aesKey
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
}
