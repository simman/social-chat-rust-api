/// 查询群会话相关信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGroupInfo {
    /// 群组target ID
    #[prost(string, tag = "1")]
    pub target_id: ::prost::alloc::string::String,
}
