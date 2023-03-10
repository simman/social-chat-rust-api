use crate::{
    config::{
        constant::{IM_HTTP_CLIENT, SDK_CONFIG},
        http_result::IMHttpResult,
    },
    util::{base64_util, rsa_util},
};
use anyhow::{Error, Result};
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ops::Add};

pub async fn get_server_socket_url() -> Result<String> {
    let mut headers = HeaderMap::new();
    headers.insert("token", SDK_CONFIG.sdk.im_token.clone().parse().unwrap());

    #[derive(Debug, Serialize, Deserialize)]
    struct ServerPublicKeyResult {
        p: String,
    }

    let resp = IM_HTTP_CLIENT
        .c
        .get(SDK_CONFIG.im_server.prefix_url.clone().add("/public"))
        .headers(headers.clone())
        .send()
        .await?
        .json::<IMHttpResult<ServerPublicKeyResult>>()
        .await?;

    if 1000 != resp.code {
        return Err(Error::msg(resp.message));
    }

    let public_key =
        rsa_util::get_pub_key_pair_with_public_key(resp.data.unwrap().p.as_str()).unwrap();
    let encrypt_data = rsa_util::encrypt(
        &public_key,
        format!("{},{}", SDK_CONFIG.sdk.app_id, SDK_CONFIG.sdk.im_token).as_bytes(),
    );
    let app_id_key = base64_util::encode(encrypt_data.unwrap());

    let mut data = HashMap::new();
    data.insert("appIdKey", app_id_key);
    let resp = IM_HTTP_CLIENT
        .c
        .post(SDK_CONFIG.im_server.prefix_url.clone().add("/serverIp"))
        .headers(headers)
        .json(&data)
        .send()
        .await?
        .json::<IMHttpResult<String>>()
        .await?;

    if 1000 != resp.code {
        return Err(Error::msg(resp.message));
    }

    Ok(resp.data.unwrap())
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn get_server_socket_url_test() {
        let resp = get_server_socket_url().await;
        assert!(resp.is_ok())
    }
}
