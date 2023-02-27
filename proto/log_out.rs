/// cmd - 13
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogOut {
    /// 厂商编号
    #[prost(string, tag = "1")]
    pub manufacturer: ::prost::alloc::string::String,
}
