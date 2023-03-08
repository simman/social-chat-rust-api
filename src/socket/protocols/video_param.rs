/// cmd - 149 会话消息传输
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoParam {
    /// 音视频通话房间ID
    #[prost(string, tag = "1")]
    pub room_id: ::prost::alloc::string::String,
    /// 操作对象用户ID  取消/挂断的对象id
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    /// 消息
    #[prost(string, tag = "3")]
    pub param: ::prost::alloc::string::String,
}
