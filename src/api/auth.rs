use crate::api::api_urls::{AUTO_LOGIN, FETCH_QR_CODE};
use crate::config::constant::{HTTP_CLIENT, MACHINE_ID, SDK_CONFIG};
use crate::config::http_result::HttpResult;
use crate::models::login_user::{LoginQrCode, LoginUser};
use crate::util::device_util::DEFAULT_PLATFORM;
use anyhow::Result;
use reqwest::header::HeaderMap;
use std::collections::HashMap;
use std::ops::Add;

use super::api_urls::FETCH_QR_CODE_BY_CODE;

/// 自动登录
pub async fn auto_login(token: String) -> Result<HttpResult<LoginUser>> {
    let mut headers = HeaderMap::new();
    headers.insert("pc-token", token.parse().unwrap());

    Ok(HTTP_CLIENT
        .c
        .post(SDK_CONFIG.http.prefix_url.clone().add(AUTO_LOGIN))
        .headers(headers)
        .send()
        .await?
        .json::<HttpResult<LoginUser>>()
        .await?)
}

/// 获取登录二维码
pub async fn login_qr_code() -> Result<HttpResult<LoginQrCode>> {
    let mut device_type = String::from("-1");
    if DEFAULT_PLATFORM == "windows" {
        device_type = String::from("3");
    } else if DEFAULT_PLATFORM == "macos" {
        device_type = String::from("4");
    }

    let mut data = HashMap::new();
    data.insert("deviceCode", MACHINE_ID.to_string());
    data.insert("deviceType", device_type);
    Ok(HTTP_CLIENT
        .c
        .post(SDK_CONFIG.http.prefix_url.clone().add(FETCH_QR_CODE))
        .json(&data)
        .send()
        .await?
        .json::<HttpResult<LoginQrCode>>()
        .await?)
}

/// 查询登录二维码状态
/// qr_code 二维码code
pub async fn login_qr_code_status_by_code(qr_code: String) -> Result<HttpResult<LoginQrCode>> {
    let mut data = HashMap::new();
    data.insert("qrCode", qr_code.replace("scan_login:", ""));

    Ok(HTTP_CLIENT
        .c
        .get(SDK_CONFIG.http.prefix_url.clone().add(FETCH_QR_CODE_BY_CODE))
        .query(&data)
        .send()
        .await?
        .json::<HttpResult<LoginQrCode>>()
        .await?)
}

/// 销毁二维码
/// qr_code 二维码code
// pub async fn destroy_login_qr_code(qr_code: String) -> Result<HttpResult<String>> {
//     let mut data = HashMap::new();
//     data.insert("qrCode", qr_code.replace("scan_login:", ""));

//     Ok(HTTP_CLIENT
//         .c
//         .post(SDK_CONFIG.http.prefix_url.clone().add(REMOVE_QR_CODE))
//         .form(&data)
//         .send()
//         .await?
//         .json::<HttpResult<String>>()
//         .await?)
// }

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn login_qr_code_test() {
        let resp = login_qr_code().await.unwrap();
        assert_eq!(resp.code, String::from("200"));
    }

    #[tokio::test]
    async fn login_qr_code_status_by_code_test() {
        let resp = login_qr_code().await.unwrap();
        let data = resp.data.unwrap();
        let resp1 = login_qr_code_status_by_code(data.qr_code).await.unwrap();
        assert_eq!(resp1.data.unwrap().status, 1);
    }
}