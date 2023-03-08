/// cmd
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageRecall {
    /// 转发类型，PRIVATE-单聊(点对点)；GROUP-群聊；
    #[prost(string, tag = "1")]
    pub send_type: ::prost::alloc::string::String,
    /// 聊天对象ID。sendType=PRIVATE时，targetId为发送方用户ID；sendType=GROUP时，targetId为群组ID
    #[prost(string, tag = "2")]
    pub target_id: ::prost::alloc::string::String,
    /// 撤回的消息ID
    #[prost(string, tag = "3")]
    pub message_id: ::prost::alloc::string::String,
}
