/// cmd - 150 视频群聊主动加入
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoJoin {
    /// targetId
    #[prost(string, tag = "1")]
    pub target_id: ::prost::alloc::string::String,
    /// 音视频通话房间ID
    #[prost(string, tag = "2")]
    pub room_id: ::prost::alloc::string::String,
}
