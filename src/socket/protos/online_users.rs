/// 用户在线列表回包
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnlineUsers {
    /// 信息
    #[prost(message, repeated, tag = "1")]
    pub online_users: ::prost::alloc::vec::Vec<ItemOnlineUser>,
}
/// 默认状态是离线
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemOnlineUser {
    /// userId
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    /// 类型（0 离线， 1在线）
    #[prost(int32, tag = "2")]
    pub r#type: i32,
}
