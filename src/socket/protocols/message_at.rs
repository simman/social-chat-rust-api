/// cmd
/// 120 - 获取消息记录列表（艾特历史消息）
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageAt {
    /// 转发类型，GROUP-群聊；
    #[prost(string, tag = "1")]
    pub send_type: ::prost::alloc::string::String,
    /// 聊天对象ID。sendType=PRIVATE时，targetId为发送方用户ID；sendType=GROUP时，targetId为群组ID
    #[prost(string, tag = "2")]
    pub target_id: ::prost::alloc::string::String,
    /// 最早消息时间戳，单位毫秒
    #[prost(int64, tag = "3")]
    pub timestamp: i64,
}
