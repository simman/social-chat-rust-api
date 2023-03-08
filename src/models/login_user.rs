use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    /// 用户头像
    pub img: String,
    /// 用户ID
    #[serde(rename = "userId")]
    pub user_id: String,
    /// 用户名称
    #[serde(rename = "userName")]
    pub user_name: String,
    /// 是否为国内
    #[serde(rename = "countryFlag")]
    pub country_flag: bool,
    /// 性别(1：男；0：女)
    pub gender: usize,
    /// 个性签名
    pub signature: String,
    /// 用户编码
    #[serde(rename = "userSn")]
    pub user_sn: usize,
    /// 邀请码
    #[serde(rename = "inviteCode")]
    pub invite_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginQrCode {
    #[serde(rename = "qrCode")]
    pub qr_code: String,
    #[serde(rename = "expireTime")]
    pub expire_time: usize,
    /// 状态 1.待扫码，2.已扫码，3.已登录，4.已失效
    pub status: usize,
    #[serde(rename = "pcToken")]
    pub pc_token: Option<String>,
    #[serde(rename = "imToken")]
    pub im_token: Option<String>,
    #[serde(rename = "userInfoVO")]
    pub user_info_vo: Option<LoginUser>,
}
