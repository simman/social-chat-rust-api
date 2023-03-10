/**
 * 系统级别协议（心跳、登入、登出等），2位十进制数
 * 规定：消息推送协议，请使用3位十进制数（发送消息和消息确认等包含from/to）
 */
/** 服务端->客户端协议，前缀S2C_ **/
pub const S2C_ACK: u16 = 1;
// ACK确认协议，协议文档：无
pub const S2C_BUSY: u16 = 2;
// 系统繁忙，协议文档：无
pub const S2C_KEY: u16 = 3;
// 返回aesKey
pub const S2C_RSAKEY: u16 = 4;
// 返回RSA公钥
pub const S2C_MAINTENANCE: u16 = 10;
// 服务端维护，协议文档：无
pub const S2C_HEARTBEAT: u16 = 11;
// 服务端-客户端心跳响应，协议文档：无
pub const S2C_ERROR: u16 = 12;
// 发生错误，协议文档：S2C_ErrorResponse.proto
pub const S2C_KICK: u16 = 15;
// 节点被踢出，服务端会断开连接，协议文档：无
pub const S2C_AUTH_SUCCESS: u16 = 16;
// 登录成功，协议文档：S2C_AuthSuccess.proto
pub const S2C_DESTORY_SUCCESS: u16 = 17;
// 销毁SDK（登出）成功，协议文档：无
pub const S2C_PROPERTY: u16 = 18; //sdk需要的配置，协议文档：S2C_Property.proto

pub const S2C_SEND_MESSAGE_STATE: u16 = 501;
// 消息状态确认，协议文档：S2C_MessageStatus.proto
pub const S2C_SEND_READ_STATE: u16 = 502;
// 消息阅读，协议文档：S2C_MessageRead.proto
pub const S2C_MESSAGE_MUTED: u16 = 503;
// 会话免打扰，协议文档：S2C_SpecialOperation.proto
pub const S2C_USER_CLOSURE: u16 = 504;
// 用户封禁，协议文档：无
pub const S2C_GLOBAL_GROUP_FORBIDDEN: u16 = 505;
// 全局群组禁言，协议文档：S2C_GlobalOperation.proto
pub const S2C_GLOBAL_CHATROOM_FORBIDDEN: u16 = 506;
// 全局聊天室禁言，协议文档：S2C_GlobalOperation.proto
pub const S2C_CHATROOM_DESTORY: u16 = 507;
// 聊天室销毁，协议文档：无
pub const S2C_CHATROOM_MEMBER_CLOSURE: u16 = 508;
// 聊天室封禁成员，协议文档：S2C_SpecialOperation.proto
pub const S2C_CHATROOM_MEMBER_FORBIDDEN: u16 = 509;
// 聊天室成员禁言，协议文档：S2C_SpecialOperation.proto
pub const S2C_SESSION_TOP: u16 = 510;
// 会话置顶，协议文档：S2C_SpecialOperation.proto
pub const S2C_MESSAGE_LIST_PAGED: u16 = 511;
// 分页聊天记录，协议文档：S2C_MessageRecords.proto
pub const S2C_CHATROOM_GET_PROP: u16 = 512;
// 获取聊天室属性，协议文档：C2S_ChatroomProperty.proto
pub const S2C_MESSAGE_STATUS_LIST: u16 = 513;
//消息状态列表，协议文档：C2S_GetMessageStatus.proto -> GetMessageStatusResult
pub const S2C_SESSION_UNREAD_COUNT: u16 = 514;
// 会话未读消息数，协议文档：S2C_SessionUnReadCount.proto
pub const S2C_GROUP_MEMBER_FORBIDDEN: u16 = 515;
// 群组成员禁言，协议文档：S2C_SpecialOperation.proto
pub const S2C_MESSAGE_LIST_UNRECEIVE: u16 = 516;
//离线消息分页推送协议文档：S2C_MessageRecords.proto
pub const S2C_GROUP_FORBIDDEN: u16 = 517;
//群组禁言，协议文档：S2C_SpecialOperation.proto
pub const S2C_GROUP_DESTORY: u16 = 518;
//群组解散，协议文档：无
pub const S2C_GROUP_WHITELIST: u16 = 519;
//群组白名单，协议文档：使用群组禁言协议
pub const S2C_AT_MESSAGES: u16 = 520;
//群组at消息列表，协议文档：S2C_AtMessages.proto
pub const S2C_MESSAGES_AT_LIST: u16 = 521;
//群组at消息列表，协议文档：S2C_AtMessages.proto
pub const S2C_GROUP_OUT: u16 = 522;
//群组退出，协议文档：无
pub const S2C_QUERY_GROUP_NAME: u16 = 523;
//群组昵称回包，协议文档：S2C_QueryGroupName.proto
pub const S2C_GROUP_MEMBER_FORBIDDEN_LIST: u16 = 524;
//群组成员禁言列表： S2C_GroupMemberForbidden.proto
pub const S2C_READ_NOTICE: u16 = 525; //消息阅读通知回包，协议文档: S2C_MessageRead.proto

//--------------------------------------------------------即时视频协议-------------------------------
pub const S2C_VIDEO_CALL: u16 = 540;
//音视频呼叫，协议文档：S2C_VideoCall.proto
pub const S2C_VIDEO_ANSWER: u16 = 541;
//对方接听，协议文档：S2C_VideoAnswered.proto
pub const S2C_VIDEO_REFUSE: u16 = 543;
//对方拒接，协议文档：S2C_VideoAnswered.proto
pub const S2C_VIDEO_CANCEL: u16 = 544;
//对方已取消，协议文档：S2C_VideoAnswered.proto
pub const S2C_VIDEO_OUT_TIME: u16 = 545;
//呼叫超时，协议文档：S2C_VideoAnswered.proto
pub const S2C_VIDEO_RING_OFF: u16 = 546;
//挂断，协议文档：S2C_VideoAnswered.proto
pub const S2C_VIDEO_SWITCH: u16 = 547;
//音视频切换，协议文档：S2C_VideoAnswered.proto
pub const S2C_VIDEO_CALL_RESULT: u16 = 548;
//音视频邀请结果，协议文档：S2C_VideoCallResult.proto
pub const S2C_VIDEO_GROUP: u16 = 549; //查询群组是否存在音视频通话结果，协议文档：S2C_VideoGroup.proto
                                      //pub const //: u16S2C_VIDEO_GROUP_DESTROY =; 550//房间销毁，协议文档：S2C_VideoAnswered.proto

pub const S2C_RTC_SIGNAL_JOINED: u16 = 560;
//rtc加入，协议文档:S2C_VideoRtcJoined.proto
pub const S2C_RTC_SIGNAL_OFFER: u16 = 561;
//rtc answer协议文档：C2S_VideoRtcOffer.proto
pub const S2C_RTC_SIGNAL_ANSWER: u16 = 562;
//rtc answer协议文档：C2S_VideoRtcOffer.proto
pub const S2C_RTC_SIGNAL_CANDIDATE: u16 = 563;
//rtc Candidate，协议文档：C2S_VideoRtcCandidate.proto
pub const S2C_RTC_SIGNAL_JOIN: u16 = 564; //rtc 加入，协议文档：:S2C_VideoRtcJoined.proto
                                          //--------------------------------------------------------即时视频协议end-------------------------------

/** 客户端->服务端协议，前缀C2S_ **/
pub const C2S_KEY: u16 = 3;
// 请求aesKey
pub const C2S_RSAKEY: u16 = 4;
// 请求RSA公钥
pub const C2S_HEARTBEAT: u16 = 11;
// 客户端-服务端心跳请求，协议文档：无
pub const C2S_AUTH: u16 = 12;
// 登录认证请求，协议文档：C2S_Auth.proto
pub const C2S_DESTORY: u16 = 13;
// 请求销毁SDK（登出），协议文档：C2S_LogOut.proto
pub const C2S_UPDATE_USER_INFO: u16 = 14; //修改用户属性协议文档：C2S_UserInfo.proto

pub const C2S_SEND_P2P_MESSAGE: u16 = 101;
// 发送点对点消息，协议文档：C2S_CustomMessage.proto
pub const C2S_SEND_GROUP_MESSAGE: u16 = 102;
// 发送群聊消息，协议文档：C2S_CustomMessage.proto
pub const C2S_SEND_CHATROOM_MESSAGE: u16 = 103;
// 发送聊天室消息，协议文档：C2S_CustomMessage.proto
pub const C2S_SEND_SYSTEM_MESSAGE: u16 = 104;
// 发送系统消息，协议文档：C2S_CustomMessage.proto
pub const C2S_RECV_CONFIRM: u16 = 105;
// 消息送达确认，协议文档：C2S_MessageContext.proto
pub const C2S_MESSAGE_READ: u16 = 106;
// 消息阅读确认，协议文档：C2S_MessageRead.proto
pub const C2S_MESSAGE_MUTED: u16 = 107;
// 会话免打扰，协议文档：S2C_SpecialOperation.proto
pub const C2S_SESSION_TOP: u16 = 108;
// 会话置顶，协议文档：S2C_SpecialOperation.proto
pub const C2S_MESSAGE_LOAD_PAGED: u16 = 109;
// 获取消息记录列表（历史消息），协议文档：C2S_MessageLoad.proto
pub const C2S_MESSAGE_RECALL: u16 = 110;
// 撤回消息，协议文档：C2S_MessageRecall.proto
pub const C2S_MESSAGE_DEL: u16 = 111;
// 删除消息，协议文档：C2S_MessageDelete.proto
pub const C2S_JOIN_CHATROOM: u16 = 112;
// 加入聊天室，协议文档：C2S_ChatroomJoin.proto
pub const C2S_EXIT_CHATROOM: u16 = 113;
// 退出聊天室，协议文档：C2S_ChatroomJoin.proto
pub const C2S_CHATROOM_SET_PROP: u16 = 114;
// 设置聊天室属性，协议文档：C2S_ChatroomProperty.proto
pub const C2S_CHATROOM_DEL_PROP: u16 = 115;
// 删除聊天室属性，协议文档：C2S_ChatroomProperty.proto
pub const C2S_CHATROOM_GET_PROP: u16 = 116;
// 获取聊天室属性，协议文档：C2S_ChatroomProperty.proto
pub const C2S_GET_INIT_DATA: u16 = 117;
// 获取消息状态，协议文档：C2S_GetMessageStatus.proto -> GetMessageStatus
pub const C2S_GET_MESSAGE_TOP: u16 = 118;
// 获取置顶信息协议文档:C2S_GetSpecialOperation.proto
pub const C2S_GET_MESSAGE_MUTED: u16 = 119;
// 获取免打扰信息协议文档:C2S_GetSpecialOperation.proto
pub const C2S_MESSAGE_AT_LIST: u16 = 120;
// 获取消息记录列表（艾特历史消息），协议文档：C2S_MessageAt.proto
pub const C2S_MESSAGE_AUDIO_READ: u16 = 121;
// 语音消息已读确认 协议文档：C2S_C2SMessageAudioRead.proto
pub const C2S_GET_QUERY_GROUP_NAME: u16 = 122;
// 批量查询解散的群会话名称协议文档： C2S_GetQueryGroupName.proto
pub const C2S_GET_GROUP_MEMBER_FORBIDDEN: u16 = 123; // 群组成员禁言查询，协议文档：C2S_GetGroupInfo.proto

//--------------------------------------------------------即时视频协议-------------------------------
pub const C2S_VIDEO_LAUNCH: u16 = 140;
//发起音视频，协议文档：C2S_VideoLaunch.proto
pub const C2S_VIDEO_ANSWER: u16 = 141;
//音视频接听，协议文档：C2S_VideoAnswer.proto
pub const C2S_VIDEO_REFUSE: u16 = 142;
//音视频拒接，协议文档：C2S_VideoAnswer.proto
pub const C2S_VIDEO_CANCEL: u16 = 143;
//取消呼叫，协议文档：C2S_VideoAnswer.proto
pub const C2S_VIDEO_RING_OFF: u16 = 144;
//通话挂断，协议文档：C2S_VideoAnswer.proto
pub const C2S_VIDEO_SWITCH: u16 = 145;
//通话类型被切换，协议文档：C2S_VideoAnswer.proto
pub const C2S_VIDEO_ERROR: u16 = 146; //通话异常，协议文档：C2S_VideoAnswer.proto

pub const C2S_VIDEO_MEMBER_MODIFY: u16 = 147;
//修改群音视频通话人员中途邀请群成员加入，协议文档：C2S_VideoMemberModify.proto
pub const C2S_VIDEO_STATUS: u16 = 148;
//群聊用户状态，协议文档：C2S_VideoStatus.proto
pub const C2S_VIDEO_PARAM: u16 = 149;
//通话消息传输，协议文档：C2S_VideoParam.proto
pub const C2S_VIDEO_JOIN: u16 = 151;
//群通话主动加入 协议文档： C2S_VideoJoin.proto
pub const C2S_GROUP_VIDEO_QUERY: u16 = 152;
// 查询群通话 协议文档：C2S_VideoQuery.proto
pub const C2S_VIDEO_BEAT: u16 = 153; //房间心跳 协议文档：C2S_VideoBeat.proto

pub const C2S_RTC_SIGNAL_JOIN: u16 = 160;
//rtc加入，协议文档:C2S_VideoRtcJoin.proto
pub const C2S_RTC_SIGNAL_OFFER: u16 = 161;
//rtc offer，协议文档：C2S_VideoRtcOffer.proto
pub const C2S_RTC_SIGNAL_ANSWER: u16 = 162;
//rtc answer协议文档：C2S_VideoRtcOffer.proto
pub const C2S_RTC_SIGNAL_CANDIDATE: u16 = 163; //rtc Candidate，协议文档：C2S_VideoRtcCandidate.proto

//--------------------------------------------------------即时视频协议end-------------------------------

/** 服务端(客户) -> 服务端(服务) 协议，前缀SC2SS_**/
pub const SC2SS_HANDSHAKE: u16 = 51;
// 服务端-服务端握手信息，请求：SC2SS_Handshake.proto
/** 服务端(服务) -> 服务端(客户) 协议，前缀SS2SC_**/
pub const ALL: u16 = u16::MAX; //
