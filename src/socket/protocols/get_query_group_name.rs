/// 批量查询解散的群会话名称
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetQueryGroupName {
    /// target IDs
    #[prost(string, repeated, tag = "1")]
    pub target_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
