/// cmd
/// 111 - 删除消息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageDelete {
    /// 转发类型，PRIVATE-单聊(点对点)；GROUP-群聊；
    #[prost(string, tag = "1")]
    pub send_type: ::prost::alloc::string::String,
    /// 聊天对象ID。sendType=PRIVATE时，targetId为发送方用户ID；sendType=GROUP时，targetId为群组ID
    #[prost(string, tag = "2")]
    pub target_id: ::prost::alloc::string::String,
    /// 删除的消息ID列表，当IDs不为空，忽略timestamp
    #[prost(string, repeated, tag = "3")]
    pub message_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 删除消息的时间戳，优先取messageId
    #[prost(int64, tag = "4")]
    pub timestamp: i64,
}
