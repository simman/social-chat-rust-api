/// cmd -  查询
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoQuery {
    /// targetId
    #[prost(string, tag = "1")]
    pub target_id: ::prost::alloc::string::String,
}
