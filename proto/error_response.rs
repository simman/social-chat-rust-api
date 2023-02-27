/// cmd
/// 12 - 错误消息响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorResponse {
    /// 响应编码
    #[prost(int32, tag = "1")]
    pub code: i32,
    /// 301 - 非法请求，拒绝服务，服务端需强制断开TCP连接
    /// 302 - 服务未知异常
    /// xxx - 服务业务异常，根据业务场景跑出BizException设置对应的错误编码
    ///
    /// 响应内容
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
