/// cmd
/// 502 - 消息阅读状态
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageRead {
    /// 转发类型，PRIVATE-单聊(点对点)；GROUP-群聊；
    #[prost(string, tag = "1")]
    pub send_type: ::prost::alloc::string::String,
    /// 聊天对象ID。sendType=PRIVATE时，targetId为发送方用户ID；sendType=GROUP时，targetId为群组ID
    #[prost(string, tag = "2")]
    pub target_id: ::prost::alloc::string::String,
    /// 已读时间
    #[prost(int64, tag = "3")]
    pub timestamp: i64,
}
/// cmd
/// 106 - 消息阅读确认
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageReadConfirm {
    /// 转发类型，PRIVATE-单聊(点对点)；GROUP-群聊；
    #[prost(string, tag = "1")]
    pub send_type: ::prost::alloc::string::String,
    /// 聊天对象ID。sendType=PRIVATE时，targetId为发送方用户ID；sendType=GROUP时，targetId为群组ID
    #[prost(string, tag = "2")]
    pub target_id: ::prost::alloc::string::String,
    /// 最后一条消息的时间
    #[prost(int64, tag = "3")]
    pub last_timestamp: i64,
}
