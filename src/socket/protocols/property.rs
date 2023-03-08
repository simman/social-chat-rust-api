/// cmd - 18
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Property {
    /// 音视频通话ice服务器
    #[prost(string, repeated, tag = "1")]
    pub ice_services: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 特殊字符
    #[prost(string, tag = "2")]
    pub special_characters: ::prost::alloc::string::String,
    /// WebRTC硬解码白名单
    #[prost(string, tag = "3")]
    pub webrtc_whitelist: ::prost::alloc::string::String,
}
