/// cmd
/// 522 - 获取会话扩展信息(群禁言信息等) ,协议文档: S2C_ExtendOperation.protos
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtendOperation {
    /// targetId
    #[prost(string, tag = "1")]
    pub target_id: ::prost::alloc::string::String,
    /// 转发类型，PRIVATE-单聊(点对点)；GROUP-群聊；CHATROOM-聊天室；SYSTEM-系统消息；
    #[prost(string, tag = "2")]
    pub send_type: ::prost::alloc::string::String,
    /// 扩展信息
    #[prost(message, repeated, tag = "3")]
    pub extends_configs: ::prost::alloc::vec::Vec<ItemExtendConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemExtendConfig {
    /// 1- 群全员禁言 2- 群禁言
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// 1 开启
    #[prost(string, tag = "2")]
    pub val: ::prost::alloc::string::String,
}
