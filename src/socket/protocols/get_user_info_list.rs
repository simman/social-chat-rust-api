/// 查询用户相关信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserInfoList {
    /// user ID
    #[prost(string, repeated, tag = "1")]
    pub user_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
