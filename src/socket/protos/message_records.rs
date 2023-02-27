/// cmd
/// 511 -  分页聊天记录
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageRecord {
    /// 转发类型，PRIVATE-单聊(点对点)；GROUP-群聊；
    #[prost(string, tag = "1")]
    pub send_type: ::prost::alloc::string::String,
    /// 聊天对象ID。sendType=PRIVATE时，targetId为发送方用户ID；sendType=GROUP时，targetId为群组ID
    #[prost(string, tag = "2")]
    pub target_id: ::prost::alloc::string::String,
    /// 最早消息时间戳，单位毫秒
    #[prost(int64, tag = "3")]
    pub timestamp: i64,
    /// 搜索类型0-搜索时间戳前的数据，1-搜索该时间戳后的数据
    #[prost(int32, tag = "4")]
    pub search_type: i32,
    /// 分页消息数据
    #[prost(message, repeated, tag = "5")]
    pub msgs: ::prost::alloc::vec::Vec<super::custom_message::Msg>,
}
