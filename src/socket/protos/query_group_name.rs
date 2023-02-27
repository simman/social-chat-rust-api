/// 群信息回包
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGroupName {
    /// 信息
    #[prost(message, repeated, tag = "1")]
    pub group_infos: ::prost::alloc::vec::Vec<ItemGroupInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemGroupInfo {
    /// targetId
    #[prost(string, tag = "1")]
    pub target_id: ::prost::alloc::string::String,
    /// 群昵称
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
