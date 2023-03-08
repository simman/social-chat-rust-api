/// cmd
/// 112 - 加入聊天室
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatroomJoinReq {
    /// 加入的聊天室ID
    #[prost(string, tag = "1")]
    pub chatroom_id: ::prost::alloc::string::String,
}
