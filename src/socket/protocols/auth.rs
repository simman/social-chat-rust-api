/// cmd - 12
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthReq {
    /// APPID
    #[prost(string, tag = "1")]
    pub app_id: ::prost::alloc::string::String,
    /// 鉴权token
    #[prost(string, tag = "2")]
    pub token: ::prost::alloc::string::String,
    /// 设备编号，UUID生成，去掉“-”
    #[prost(string, tag = "3")]
    pub device_no: ::prost::alloc::string::String,
    /// 设备类型：1-Android；2-IOS
    #[prost(string, tag = "4")]
    pub device_type: ::prost::alloc::string::String,
    /// 设备厂商编号
    #[prost(string, tag = "5")]
    pub device_code: ::prost::alloc::string::String,
    /// 设备系统版本
    #[prost(string, tag = "6")]
    pub system_version: ::prost::alloc::string::String,
    /// 语言
    #[prost(string, tag = "7")]
    pub language: ::prost::alloc::string::String,
}
