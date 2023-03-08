/// cmd - 4
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RsaKeyReq {
    /// 用户Token
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    /// 客户端公钥
    #[prost(string, tag = "2")]
    pub client_pub_key: ::prost::alloc::string::String,
}
/// cmd - 4
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RsaKeyRes {
    /// 服务端公钥（加密）
    #[prost(string, tag = "1")]
    pub server_pub_key: ::prost::alloc::string::String,
}
