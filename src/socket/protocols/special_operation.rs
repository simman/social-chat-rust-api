/// 特定对象操作，如对特定用户、群组、聊天室、系统消息等的操作
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpecialOperation {
    /// 转发类型，PRIVATE-单聊(点对点)；GROUP-群聊；CHATROOM-聊天室；SYSTEM-系统消息；
    #[prost(string, tag = "1")]
    pub send_type: ::prost::alloc::string::String,
    /// 用户ID
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    /// 被操作对象的ID：用户ID/群组ID/聊天室ID/-(系统)
    #[prost(string, tag = "3")]
    pub target_id: ::prost::alloc::string::String,
    /// 消息免打扰：免打扰对象的ID
    /// 消息置顶：置顶对象的ID
    /// 聊天室封禁成员：聊天室ID
    /// 群聊禁言：群组ID
    ///
    /// 设置类型：根据协议号确定
    #[prost(string, tag = "4")]
    pub r#type: ::prost::alloc::string::String,
}
