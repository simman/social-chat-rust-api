/// cmd - 542 其他设备接听
/// cmd - 545 呼叫超时
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoAnswered {
    /// 音视频通话房间ID
    #[prost(string, tag = "1")]
    pub room_id: ::prost::alloc::string::String,
    /// 操作用户ID 类型是取消时可以不传
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    /// 通话挂断时的TYPE  7群解散结束通话, 8退出群聊结束通话
    #[prost(int32, tag = "3")]
    pub end_type: i32,
}
