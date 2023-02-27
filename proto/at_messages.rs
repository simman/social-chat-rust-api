/// cmd
/// 520 - at消息列表
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AtMessages {
    /// 系统当前时间
    #[prost(int64, tag = "1")]
    pub timestamp: i64,
    /// at消息会话列表
    #[prost(message, repeated, tag = "2")]
    pub tags: ::prost::alloc::vec::Vec<Targets>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Targets {
    /// 会话id
    #[prost(string, tag = "1")]
    pub target_id: ::prost::alloc::string::String,
    /// at消息列表
    #[prost(message, repeated, tag = "2")]
    pub ats: ::prost::alloc::vec::Vec<Messages>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Messages {
    /// at消息id列表
    #[prost(string, repeated, tag = "1")]
    pub at_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// at消息附近的消息列表
    #[prost(message, repeated, tag = "2")]
    pub at_msgs: ::prost::alloc::vec::Vec<super::custom_message::Msg>,
}
