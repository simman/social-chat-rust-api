/// cmd - 560加入会议
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoRtcJoined {
    /// 音视频通话id
    #[prost(string, tag = "1")]
    pub room_id: ::prost::alloc::string::String,
    /// 通话用户列表
    #[prost(string, repeated, tag = "2")]
    pub peers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
