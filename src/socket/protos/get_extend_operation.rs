/// 获取会话扩展信息(群禁言信息等)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetExtendOperation {
    /// 转发类型，PRIVATE-单聊(点对点)；GROUP-群聊；CHATROOM-聊天室；SYSTEM-系统消息；
    #[prost(string, tag = "1")]
    pub send_type: ::prost::alloc::string::String,
    /// 被操作对象的ID：用户ID/群组ID/聊天室ID/-(系统)
    #[prost(string, tag = "2")]
    pub target_id: ::prost::alloc::string::String,
}
