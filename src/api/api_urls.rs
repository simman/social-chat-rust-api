pub const AUTO_LOGIN: &str = "/user/web/autoLogin";
pub const FETCH_QR_CODE: &str = "/user/web/createQrCode";
pub const FETCH_QR_CODE_BY_CODE: &str = "/user/web/queryQrByCode";
pub const REMOVE_QR_CODE: &str = "/user/web/removeQrCode";

pub const USER_FRIEND_LIST: &str = "/userfriend/list";
pub const ADD_FRIEND: &str = "/userfriend/request/addFriend";
pub const AGREE_FRIEND: &str = "/userfriend/request/agree";

pub const APPLY_FORAGAIN_ADDFRIEND: &str = "/userfriend/request/applyForAgainAddFriend";
pub const DEL_FRIEND: &str = "/userfriend/request/delFriend";
pub const DEL_RELATION: &str = "/userfriend/del/relation";
pub const UNPROCESSED: &str = "/userfriend/new/unprocessed";
pub const FETCH_NEW_FRIEND: &str = "/userfriend/new";
pub const EMOTICONS_LIST: &str = "/emoticons/list";
pub const EMOTICONS_USER_LIST: &str = "/emoticons/user/list";
pub const ADD_FRIEND_NOTES: &str = "/userfriend/addNotes";
pub const SEARCH_USER: &str = "/userfriend/query/key";
pub const USER_INFO: &str = "/baseinfo/userId";
pub const FRIEND_STATUS: &str = "/userfriend/friendId";
pub const USER_INFO_BATCH: &str = "/baseinfo/list/userids";
pub const UPDATE_USER_INFO: &str = "/baseinfo/update";
pub const FETCH_GROUP_INFO: &str = "/user/group/userId/info";
pub const CREATE_GROUP: &str = "/user/group/create";
pub const UPDATE_GROUP_INFO: &str = "/user/group/update";
pub const FETCH_GROUP_MEMBER_INFO: &str = "/instantMessage/queryGroupMemberInfo";
pub const FETCH_GROUP_USER_LIST: &str = "/user/group/user/list";
pub const JOIN_GROUP: &str = "/user/group/userFriend/joinGroup";
pub const BATCH_QUITE_GROUP: &str = "/user/group/user/batch/quite";
pub const QUITE_GROUP: &str = "/user/group/user/quite";
pub const DISMISS_GROUP: &str = "/user/group/dismiss";
pub const CREATE_GROUP_NOTICE: &str = "/user/group/createGroupNotice";
pub const UPDATE_FORBIDDEN_WORDS: &str = "/user/group/updateForbiddenWords";
pub const UPDATE_GROUP_MESRECALL: &str = "/user/group/updateGroupMesRecall";
pub const UPDATE_USER_GAG: &str = "/user/group/updateUserGag";
pub const REMOVE_USER_GAG: &str = "/user/group/removeUserGag";
pub const UPDATE_GROUP_SETTINGS: &str = "/user/group/updateGroupSettings";
pub const UPDATE_MEMBER_NICKNAME: &str = "/user/group/updateMemberNickname";
pub const QUERY_GROUP_NOTICE: &str = "/user/group/queryGroupNotice";
pub const ADD_GROUP_MANAGER: &str = "/user/group/addGroupManager";
pub const BATCH_ADD_MANAGER: &str = "/user/group/batchAddManager";
pub const BATCH_REMOVE_MANAGER: &str = "/user/group/batchRemoveManager";
pub const QUERY_GROUP_MANAGER_LIST: &str = "/user/group/queryGroupManagerList";
