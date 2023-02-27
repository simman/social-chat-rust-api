/// cmd - 160加入会议
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoRtcJoin {
    /// 音视频通话id
    #[prost(string, tag = "1")]
    pub room_id: ::prost::alloc::string::String,
    /// 通话类型 PRIVATE-单聊点对点；GROUP-群聊
    #[prost(string, tag = "2")]
    pub room_type: ::prost::alloc::string::String,
}
