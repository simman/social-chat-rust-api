/// cmd - 561会议offer消息
/// cmd - 562会议answer消息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoRtcOffer {
    /// 音视频通话id
    #[prost(string, tag = "1")]
    pub room_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub sdp: ::prost::alloc::string::String,
}
