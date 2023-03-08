/// cmd - 148
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoStatus {
    /// 群id
    #[prost(string, tag = "1")]
    pub group_id: ::prost::alloc::string::String,
    /// 音视频通话id
    #[prost(string, tag = "2")]
    pub room_id: ::prost::alloc::string::String,
    /// 通话人员列表
    #[prost(message, repeated, tag = "3")]
    pub ststus: ::prost::alloc::vec::Vec<VideoStatusList>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoStatusList {
    /// 用户id
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    /// 通话人员列表
    #[prost(message, repeated, tag = "2")]
    pub ststus: ::prost::alloc::vec::Vec<Status>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    /// 状态类型 video - 视频    audio - 音频
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// 状态 0-关闭 1-开启
    #[prost(int32, tag = "2")]
    pub state: i32,
}
