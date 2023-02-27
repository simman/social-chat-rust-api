/// cmd - 16
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthSuccess {
    /// 客户端ID appid_userid_deviceType_deviceNo
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// 用户唯一标识 identifier = appid_userid
    #[prost(string, tag = "2")]
    pub identifier: ::prost::alloc::string::String,
    /// 用户ID
    #[prost(string, tag = "3")]
    pub user_id: ::prost::alloc::string::String,
    /// 用户名
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// 头像
    #[prost(string, tag = "5")]
    pub icon: ::prost::alloc::string::String,
    /// 登录时间
    #[prost(int64, tag = "6")]
    pub timestamp: i64,
}
