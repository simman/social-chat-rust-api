use crate::util;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{env, fs};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatSdkConfig {
    pub sdk: SdkConfig,
    pub app: AppConfig,
    pub socket: SocketConfig,
    pub http: HttpConfig,
    pub im_server: IMServerConfig,
    pub storage: StorageConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SdkConfig {
    pub debug: bool,
    pub app_id: String,
    pub app_key: String,
    pub token: String,
    pub im_token: String,
    pub lang_code: usize,
    pub data_dir: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SocketConfig {
    pub debug: bool,
    pub host: Option<String>,
    pub connection_timeout: usize,
    pub request_timeout: usize,
    pub heartbeat_interval: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpConfig {
    pub debug: bool,
    pub prefix_url: String,
    pub proxy: Option<String>,
    pub user_agent: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IMServerConfig {
    pub debug: bool,
    pub prefix_url: String,
    pub proxy: Option<String>,
    pub user_agent: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageConfig {
    pub s3: S3Config,
    pub cos: CosConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct S3Config {
    pub url: String,
    pub prefix_key: String,
    pub region: String,
    pub identity_pool_id: String,
    pub bucket: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CosConfig {
    pub url: String,
    pub prefix_key: String,
    pub region: String,
    pub bucket: String,
}

fn parse_config_from_env() -> Result<Option<ChatSdkConfig>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return Ok(None);
    }
    let config = &args[1];
    let config_json = &args[2];
    if !config.eq("config") || !config_json.is_empty() {
        return Ok(None);
    }

    let config_json = util::base64_util::decode(config_json)?;
    let sdk_config: ChatSdkConfig = serde_json::from_str(String::from_utf8(config_json)?.as_str())?;
    Ok(Some(sdk_config))
}

// project_root/env.json
fn parse_local_env_file() -> Result<Option<ChatSdkConfig>> {
    let mut root_path = project_root::get_project_root()?;
    root_path = root_path.join("env.json");
    let env = fs::read_to_string(root_path)?;

    let sdk_config: ChatSdkConfig = serde_json::from_str(env.as_str())?;
    Ok(Some(sdk_config))
}

pub(crate) fn load_config() -> Result<ChatSdkConfig> {
    match parse_config_from_env() {
        Ok(config) => {
            if let Some(sdk_config) = config {
                return Ok(sdk_config);
            } else {
                match parse_local_env_file() {
                    Ok(config) => {
                        if let Some(sdk_config) = config {
                            return Ok(sdk_config);
                        }
                        panic!("parse local env file failed!")
                    }
                    Err(e) => panic!("{:?}", e),
                }
            }
        }
        Err(e) => panic!("{:?}", e),
    }
}
