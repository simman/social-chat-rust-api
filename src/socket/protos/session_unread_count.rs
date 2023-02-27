/// cmd
/// 514 - 会话未读消息数
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionUnReadCount {
    /// 一条记录一个会话
    #[prost(message, repeated, tag = "1")]
    pub un_reads: ::prost::alloc::vec::Vec<SessionUnRead>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionUnRead {
    /// 发送方ID
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    /// 接收方ID
    #[prost(string, tag = "2")]
    pub to: ::prost::alloc::string::String,
    /// 转发类型，PRIVATE-单聊(点对点)；GROUP-群聊；CHATROOM-聊天室；SYSTEM-系统消息；
    #[prost(string, tag = "3")]
    pub send_type: ::prost::alloc::string::String,
    /// 当前会话未读消息数量
    #[prost(int32, tag = "4")]
    pub count: i32,
    /// 最新一条消息
    #[prost(message, optional, tag = "5")]
    pub last_msg: ::core::option::Option<super::custom_message::Msg>,
    /// 当前会话at未读消息数量
    #[prost(int32, tag = "6")]
    pub at_count: i32,
    /// 第一条消息
    #[prost(message, optional, tag = "7")]
    pub first_msg: ::core::option::Option<super::custom_message::Msg>,
}
