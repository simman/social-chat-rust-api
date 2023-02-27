/// cmd
/// 101 - 发送/接收点对点消息
/// 102 - 发送/接收群聊消息
/// 103 - 发送/接收聊天室消息
/// 104 - 接收系统消息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Msg {
    /// 消息ID，UUID去掉"-"
    #[prost(string, tag = "1")]
    pub message_id: ::prost::alloc::string::String,
    /// 转发类型，PRIVATE-单聊(点对点)；GROUP-群聊；CHATROOM-聊天室；SYSTEM-系统消息；
    #[prost(string, tag = "2")]
    pub send_type: ::prost::alloc::string::String,
    /// 发送消息时，无需设置，TCP服务端根据cmd(101/102/103/104)自动设置
    ///
    /// 发送人用户ID，sendType=SYSTEM(系统消息)或者时messageType=QX:NoticeMsg(通知消息)时，from="QX:SYSTEM"
    #[prost(string, tag = "3")]
    pub from: ::prost::alloc::string::String,
    /// 接收人/群/聊天室ID
    #[prost(string, tag = "4")]
    pub to: ::prost::alloc::string::String,
    /// QX:NoticeMsg-通知消息；QX:TxtMsg-文本消息；QX:ImgMsg-图片消息；QX:ImgTextMsg-图文消息；QX:AcMsg-语音消息；QX:VcMsg-视频消息；QX:FileMsg-文件消息；QX:GeoMsg-地理位置消息；
    #[prost(string, tag = "5")]
    pub message_type: ::prost::alloc::string::String,
    /// QX:StusMsg-消息输入状态；XX:yyy-消息输入状态；(XX:yyy为第三方自己设定的)；QX:RecallMsg-消息撤回；QX:Reply-回复消息;QX:Record-聊天记录消息;QX:Event-事件消息
    ///
    /// 消息发送时间（服务端时间），发送消息时，客户端无需设置，服务端收到消息时会设置为当前的服务端时间，单位为纳秒（1毫秒 = 100,0000纳秒）
    #[prost(int64, tag = "6")]
    pub timestamp: i64,
    /// 忽略的用户
    #[prost(string, repeated, tag = "7")]
    pub ignores: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "100")]
    pub notice: ::core::option::Option<Notice>,
    #[prost(message, optional, tag = "101")]
    pub text: ::core::option::Option<Text>,
    #[prost(message, optional, tag = "102")]
    pub image: ::core::option::Option<Image>,
    #[prost(message, optional, tag = "103")]
    pub image_text: ::core::option::Option<ImageText>,
    #[prost(message, optional, tag = "104")]
    pub audio: ::core::option::Option<Audio>,
    #[prost(message, optional, tag = "105")]
    pub video: ::core::option::Option<Video>,
    #[prost(message, optional, tag = "106")]
    pub file: ::core::option::Option<File>,
    #[prost(message, optional, tag = "107")]
    pub geo: ::core::option::Option<Geo>,
    #[prost(message, optional, tag = "108")]
    pub act: ::core::option::Option<Act>,
    #[prost(message, optional, tag = "109")]
    pub vct: ::core::option::Option<Vct>,
    #[prost(message, optional, tag = "202")]
    pub stus: ::core::option::Option<Stus>,
    #[prost(message, optional, tag = "203")]
    pub custom: ::core::option::Option<Custom>,
    #[prost(message, optional, tag = "204")]
    pub recall: ::core::option::Option<Recall>,
    #[prost(message, optional, tag = "205")]
    pub remove: ::core::option::Option<Remove>,
    /// 回复类型
    #[prost(message, optional, tag = "206")]
    pub reply: ::core::option::Option<Reply>,
    /// 消息记录类型
    #[prost(message, optional, tag = "207")]
    pub record: ::core::option::Option<Record>,
    /// 事件消息类型
    #[prost(message, optional, tag = "208")]
    pub event: ::core::option::Option<Event>,
    /// 语音消息已读标记 0 未读， 1已读
    #[prost(int32, tag = "209")]
    pub audio_read: i32,
}
/// QX:NoticeMsg-通知消息；
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Notice {
    /// 通知内容
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    /// 被操作用户
    #[prost(message, repeated, tag = "2")]
    pub users: ::prost::alloc::vec::Vec<UserInfo>,
    /// 操作用户
    #[prost(message, optional, tag = "3")]
    pub operate_user: ::core::option::Option<UserInfo>,
    /// 通知消息类型 1),//加入聊天室	2),//退出聊天室	3),//群组解散4),//群组同步5),//群组创建	6),//加入群组7),//退出群组8),//群组成员禁言（添加）9),//群组成员禁言（移除）10),//群组整体禁言（添加）11);//群组整体禁言（移除）
    #[prost(int32, tag = "4")]
    pub r#type: i32,
    /// 扩展消息
    #[prost(string, tag = "5")]
    pub extra: ::prost::alloc::string::String,
}
/// 通知消息  触发该消息的对象
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInfo {
    /// 用户id
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    /// 用户名称
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
/// QX:TxtMsg-文本消息；
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Text {
    /// 文本内容
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    /// 扩展消息
    #[prost(string, tag = "2")]
    pub extra: ::prost::alloc::string::String,
    /// 被@人的用户Id（全体成员：-1）
    #[prost(string, repeated, tag = "3")]
    pub at_tos: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QX:ImgMsg-图片消息；
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Image {
    /// 源图片地址
    #[prost(string, tag = "1")]
    pub origin_url: ::prost::alloc::string::String,
    /// 缩略图地址
    #[prost(string, tag = "2")]
    pub breviary_url: ::prost::alloc::string::String,
    /// 图片宽度
    #[prost(int32, tag = "3")]
    pub width: i32,
    /// 图片高度
    #[prost(int32, tag = "4")]
    pub height: i32,
    /// 文件大小，单位byte
    #[prost(int64, tag = "5")]
    pub size: i64,
    /// 扩展消息
    #[prost(string, tag = "6")]
    pub extra: ::prost::alloc::string::String,
}
/// QX:ImgTextMsg-图文消息；
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageText {
    /// 标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 内容
    #[prost(string, tag = "2")]
    pub content: ::prost::alloc::string::String,
    /// 图片地址
    #[prost(string, tag = "3")]
    pub image_url: ::prost::alloc::string::String,
    /// 链接跳转地址
    #[prost(string, tag = "4")]
    pub redirect_url: ::prost::alloc::string::String,
    /// 来源信息
    #[prost(string, tag = "5")]
    pub tag: ::prost::alloc::string::String,
    /// 扩展消息
    #[prost(string, tag = "6")]
    pub extra: ::prost::alloc::string::String,
}
/// QX:AcMsg-语音消息；
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Audio {
    /// 语音资源地址
    #[prost(string, tag = "1")]
    pub origin_url: ::prost::alloc::string::String,
    /// 语音时长
    #[prost(int32, tag = "2")]
    pub duration: i32,
    /// 文件大小，单位byte
    #[prost(int64, tag = "3")]
    pub size: i64,
    /// 扩展消息
    #[prost(string, tag = "4")]
    pub extra: ::prost::alloc::string::String,
}
/// QX:VcMsg-视频消息；
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Video {
    /// 视频资源地址
    #[prost(string, tag = "1")]
    pub origin_url: ::prost::alloc::string::String,
    /// 视频首图地址
    #[prost(string, tag = "2")]
    pub head_url: ::prost::alloc::string::String,
    /// 视频宽度
    #[prost(int32, tag = "3")]
    pub width: i32,
    /// 视频高度
    #[prost(int32, tag = "4")]
    pub height: i32,
    /// 视频时长
    #[prost(int32, tag = "5")]
    pub duration: i32,
    /// 文件大小，单位byte
    #[prost(int64, tag = "6")]
    pub size: i64,
    /// 扩展消息
    #[prost(string, tag = "7")]
    pub extra: ::prost::alloc::string::String,
}
/// QX:FileMsg-文件消息；
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct File {
    /// 文件名
    #[prost(string, tag = "1")]
    pub file_name: ::prost::alloc::string::String,
    /// 文件资源地址
    #[prost(string, tag = "2")]
    pub origin_url: ::prost::alloc::string::String,
    /// 文件类型\[pdf/word/excel\]
    #[prost(string, tag = "3")]
    pub r#type: ::prost::alloc::string::String,
    /// 文件大小，单位byte
    #[prost(int64, tag = "4")]
    pub size: i64,
    /// 扩展消息
    #[prost(string, tag = "5")]
    pub extra: ::prost::alloc::string::String,
}
/// QX:GeoMsg-地理位置消息；
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Geo {
    /// 地址title
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 地址
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    /// 经度
    #[prost(float, tag = "3")]
    pub lon: f32,
    /// 维度
    #[prost(float, tag = "4")]
    pub lat: f32,
    /// 预览图片
    #[prost(string, tag = "5")]
    pub preview_url: ::prost::alloc::string::String,
    /// 扩展消息
    #[prost(string, tag = "6")]
    pub extra: ::prost::alloc::string::String,
}
/// QX:ActMsg-语音通话；
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Act {
    /// 通话时长(秒)
    #[prost(int32, tag = "1")]
    pub duration: i32,
    /// 内容
    #[prost(string, tag = "2")]
    pub content: ::prost::alloc::string::String,
    /// 通话结束类型 0正常结束，1已取消，2已拒接，3连接异常，4连接断开，5对方无响应
    #[prost(int32, tag = "3")]
    pub end_type: i32,
    /// 扩展消息
    #[prost(string, tag = "4")]
    pub extra: ::prost::alloc::string::String,
}
/// QX:VctMsg-视频通话；
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vct {
    /// 通话时长(秒)
    #[prost(int32, tag = "1")]
    pub duration: i32,
    /// 内容
    #[prost(string, tag = "2")]
    pub content: ::prost::alloc::string::String,
    /// 通话结束类型 0正常结束，1已取消，2已拒接，3连接异常，4连接断开，5对方无响应
    #[prost(int32, tag = "3")]
    pub end_type: i32,
    /// 扩展消息
    #[prost(string, tag = "4")]
    pub extra: ::prost::alloc::string::String,
}
/// QX:StusMsg-消息输入状态；
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stus {
    /// TODO: 需分析参数
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    /// 扩展消息
    #[prost(string, tag = "2")]
    pub extra: ::prost::alloc::string::String,
}
/// XX:yyy-消息输入状态；(XX:yyy为第三方自己设定的)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Custom {
    /// 消息内容（JSON）
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    /// 是否push推送，1-推送，0-不推送
    #[prost(int32, tag = "2")]
    pub notice: i32,
    /// 推送展示内容
    #[prost(string, tag = "3")]
    pub notice_text: ::prost::alloc::string::String,
    /// 扩展消息
    #[prost(string, tag = "4")]
    pub extra: ::prost::alloc::string::String,
}
/// QX:RecallMsg-消息撤回；
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Recall {
    /// 被撤回的消息ID
    #[prost(string, tag = "1")]
    pub target_message_id: ::prost::alloc::string::String,
    /// 扩展消息
    #[prost(string, tag = "2")]
    pub extra: ::prost::alloc::string::String,
    /// 消息撤回用户ID
    #[prost(string, tag = "3")]
    pub recall_user_id: ::prost::alloc::string::String,
}
/// QX:RemoveMsg-消息删除；
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Remove {
    /// 被删除的消息ID
    #[prost(string, tag = "1")]
    pub target_message_id: ::prost::alloc::string::String,
    /// 扩展消息
    #[prost(string, tag = "2")]
    pub extra: ::prost::alloc::string::String,
}
/// QX:Reply-消息回复
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reply {
    /// 被回复的消息
    #[prost(string, tag = "1")]
    pub reply: ::prost::alloc::string::String,
    /// 回复消息
    #[prost(string, tag = "2")]
    pub msg: ::prost::alloc::string::String,
    /// 扩展消息
    #[prost(string, tag = "3")]
    pub extra: ::prost::alloc::string::String,
}
/// QX:Record-记录消息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Record {
    /// 记录
    #[prost(string, repeated, tag = "1")]
    pub records: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 扩展消息
    #[prost(string, tag = "2")]
    pub extra: ::prost::alloc::string::String,
}
/// QX:Event事件消息，仅转发不保存消息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    /// 事件消息
    #[prost(string, tag = "1")]
    pub event: ::prost::alloc::string::String,
    /// 是否push推送，1-推送，0-不推送
    #[prost(int32, tag = "2")]
    pub notice: i32,
    /// 推送展示内容
    #[prost(string, tag = "3")]
    pub notice_text: ::prost::alloc::string::String,
    /// 扩展消息
    #[prost(string, tag = "4")]
    pub extra: ::prost::alloc::string::String,
}
