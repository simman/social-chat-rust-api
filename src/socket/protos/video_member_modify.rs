/// cmd - 147 修改群音视频会话用户
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoMemberModify {
    /// 音视频通话房间ID
    #[prost(string, tag = "1")]
    pub room_id: ::prost::alloc::string::String,
    /// 邀请的用户id
    #[prost(string, repeated, tag = "2")]
    pub user_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
