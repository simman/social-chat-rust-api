/// cmd - 540
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoCall {
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
    /// 发起方用户ID
    #[prost(string, tag = "5")]
    pub user_id: ::prost::alloc::string::String,
    /// 同在音视频中的人员id
    #[prost(string, repeated, tag = "6")]
    pub members: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
