/// cmd - 51
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Handshake {
    /// 服务端(客户)信息
    #[prost(string, tag = "1")]
    pub server_info: ::prost::alloc::string::String,
}
