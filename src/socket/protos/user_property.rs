/// cmd - 14
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserProperty {
    /// 语言
    #[prost(string, tag = "1")]
    pub language: ::prost::alloc::string::String,
}
