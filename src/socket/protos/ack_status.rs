/// cmd
/// 1 - ACK确认协议
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AckStatus {
    /// 1-成功；1000-9999-失败可重发；10000以上-失败不可重发(重发无意义)
    #[prost(int32, tag = "1")]
    pub code: i32,
    /// 错误消息，code=1(成功确认)时，message为空
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// ACK时间，如果是发送消息的ACK，那么这个时间就是消息发送的服务端时间
    #[prost(int64, tag = "3")]
    pub timestamp: i64,
}
