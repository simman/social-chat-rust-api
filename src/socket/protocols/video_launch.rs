/// cmd - 140
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoLaunch {
    /// 音视频通话房间ID
    #[prost(string, tag = "1")]
    pub room_id: ::prost::alloc::string::String,
    /// 转发类型，PRIVATE-单聊(点对点)；GROUP-群聊；
    #[prost(string, tag = "2")]
    pub send_type: ::prost::alloc::string::String,
    /// 会话id
    #[prost(string, tag = "3")]
    pub target_id: ::prost::alloc::string::String,
    /// 1-音频 2-视频
    #[prost(string, tag = "4")]
    pub r#type: ::prost::alloc::string::String,
    /// 被呼叫的用户ID
    #[prost(string, repeated, tag = "5")]
    pub user_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
