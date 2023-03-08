/// cmd - 121 语音消息已读确认
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageAudioRead {
    /// 会话ID
    #[prost(string, tag = "1")]
    pub target_id: ::prost::alloc::string::String,
    /// 转发类型 PRIVATE-单聊(点对点)；GROUP-群聊
    #[prost(string, tag = "2")]
    pub send_type: ::prost::alloc::string::String,
    /// 消息ID
    #[prost(string, tag = "3")]
    pub message_id: ::prost::alloc::string::String,
}
