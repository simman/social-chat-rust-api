/// cmd - 141 接听
/// cmd - 142 拒接
/// cmd - 143 取消
/// cmd - 144 挂断
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoAnswer {
    /// 音视频通话房间ID
    #[prost(string, tag = "1")]
    pub room_id: ::prost::alloc::string::String,
    /// 操作对象用户ID  取消/挂断的对象id
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
}
