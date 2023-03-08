/// cmd - 549
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoGroup {
    /// 群id
    #[prost(string, tag = "1")]
    pub group_id: ::prost::alloc::string::String,
    /// 音视频通话类型 1-音频 2-视频
    #[prost(int32, tag = "2")]
    pub r#type: i32,
    /// 音视频通话id
    #[prost(string, tag = "3")]
    pub room_id: ::prost::alloc::string::String,
    /// 通话人员列表
    #[prost(message, repeated, tag = "4")]
    pub users: ::prost::alloc::vec::Vec<VideoUserStatus>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoUserStatus {
    /// 通话人员 userId
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    /// 人员状态 0 - 未建立流  1-已建立流
    #[prost(int32, tag = "2")]
    pub r#in: i32,
}
