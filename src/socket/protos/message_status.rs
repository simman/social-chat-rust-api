/// cmd
/// 501 - 消息状态确认
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageStatus {
    /// 发送方ID
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    /// 接收方ID
    #[prost(string, tag = "2")]
    pub to: ::prost::alloc::string::String,
    /// 转发类型，PRIVATE-单聊(点对点)；GROUP-群聊；CHATROOM-聊天室；SYSTEM-系统消息；
    #[prost(string, tag = "3")]
    pub send_type: ::prost::alloc::string::String,
    /// 消息ID
    #[prost(string, repeated, tag = "4")]
    pub message_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 消息响应类型：2-消息送达；3-消息已读
    #[prost(int32, tag = "5")]
    pub state: i32,
}
