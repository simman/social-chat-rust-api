/// cmd - 548 音视频通话邀请结果/音视频通话人员变化
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoCallResult {
    /// 音视频通话房间ID
    #[prost(string, tag = "1")]
    pub room_id: ::prost::alloc::string::String,
    /// 转发类型，PRIVATE-单聊(点对点)；GROUP-群聊；
    #[prost(string, tag = "2")]
    pub send_type: ::prost::alloc::string::String,
    /// 会话id
    #[prost(string, tag = "3")]
    pub target_id: ::prost::alloc::string::String,
    /// 邀请结果
    #[prost(message, repeated, tag = "4")]
    pub result: ::prost::alloc::vec::Vec<Result>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Result {
    /// 结果 0-呼叫成功，1-对方忙  2-挂断
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    /// 用户ID
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
}
