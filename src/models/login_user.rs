use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    /// 用户头像
    img: String,
    /// 用户ID
    #[serde(rename = "userId")]
    user_id: String,
    /// 用户名称
    #[serde(rename = "userName")]
    user_name: String,
    /// 是否为国内
    #[serde(rename = "countryFlag")]
    country_flag: bool,
    /// 性别(1：男；0：女)
    gender: usize,
    /// 个性签名
    signature: String,
    /// 用户编码
    #[serde(rename = "userSn")]
    user_sn: usize,
    /// 邀请码
    #[serde(rename = "inviteCode")]
    invite_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginQrCode {
    #[serde(rename = "qrCode")]
    qr_code: String,
    #[serde(rename = "expireTime")]
    expire_time: usize,
    /// 状态 1.待扫码，2.已扫码，3.已登录，4.已失效
    status: usize,
    #[serde(rename = "pcToken")]
    pc_token: Option<String>,
    #[serde(rename = "imToken")]
    im_token: Option<String>,
    #[serde(rename = "userInfoVO")]
    user_info_vo: Option<LoginUser>,
}
