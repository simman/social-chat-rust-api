/// cmd
/// 117 - 获取消息状态
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMessageStatus {
    #[prost(string, tag = "1")]
    pub send_type: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub target_id: ::prost::alloc::string::String,
    /// 消息ID列表
    #[prost(string, repeated, tag = "3")]
    pub message_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// 513 - 消息状态列表
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMessageStatusResult {
    #[prost(string, tag = "1")]
    pub target_id: ::prost::alloc::string::String,
    /// 消息状态列表
    #[prost(message, repeated, tag = "2")]
    pub message_status: ::prost::alloc::vec::Vec<MessageStatusResult>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageStatusResult {
    /// 消息ID
    #[prost(string, tag = "1")]
    pub message_ids: ::prost::alloc::string::String,
    /// 消息状态，0-未发送成功；1-发送成功；2-已送达；3-已已读；
    #[prost(int32, tag = "2")]
    pub status: i32,
}
