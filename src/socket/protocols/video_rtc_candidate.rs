/// cmd - 163会议Candidate消息
/// cmd - 563会议Candidate消息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoRtcCandidate {
    /// 音视频通话id
    #[prost(string, tag = "1")]
    pub room_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub to: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub candidate: ::core::option::Option<Candidate>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Candidate {
    #[prost(string, tag = "1")]
    pub sdp_mid: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub sdp_m_line_index: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub sdp: ::prost::alloc::string::String,
}
