/// cmd
/// 113 - 设置聊天室属性
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPropertyReq {
    /// 属性名称
    #[prost(string, tag = "1")]
    pub prop_name: ::prost::alloc::string::String,
    /// 属性值
    #[prost(string, tag = "2")]
    pub prop_value: ::prost::alloc::string::String,
    /// 是否自动删除：1-是；0-否；
    #[prost(int32, tag = "3")]
    pub auto_delete: i32,
    /// 聊天室ID
    #[prost(string, tag = "4")]
    pub chatroom_id: ::prost::alloc::string::String,
}
/// 114 - 删除聊天室属性
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelPropertyReq {
    /// 属性名称
    #[prost(string, tag = "1")]
    pub prop_name: ::prost::alloc::string::String,
    /// 聊天室ID
    #[prost(string, tag = "2")]
    pub chatroom_id: ::prost::alloc::string::String,
}
/// 115 - 获取聊天室属性（请求）
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPropertyReq {
    /// 属性名称
    #[prost(string, tag = "1")]
    pub prop_name: ::prost::alloc::string::String,
    /// 聊天室ID
    #[prost(string, tag = "2")]
    pub chatroom_id: ::prost::alloc::string::String,
}
/// 512 - 获取聊天室属性（响应）
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPropertyResultRet {
    /// 属性名称
    #[prost(string, tag = "1")]
    pub prop_name: ::prost::alloc::string::String,
    /// 属性值
    #[prost(string, tag = "2")]
    pub prop_value: ::prost::alloc::string::String,
    /// 聊天室ID
    #[prost(string, tag = "3")]
    pub chatroom_id: ::prost::alloc::string::String,
}
