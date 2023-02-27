/// 全局操作，非特定对象操作，如封禁、全局群组禁言、全局聊天室禁言
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GlobalOperation {
    /// 被通知的用户ID
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    /// 设置类型：根据协议号确定
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
}
