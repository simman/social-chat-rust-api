use crate::chat_sdk::ChatSdk;
use crate::config::config::{load_config, ChatSdkConfig};
use lazy_static::lazy_static;

lazy_static! {
    // 全局配置
    pub static ref SDK_CONFIG: ChatSdkConfig = load_config().unwrap();
    // sdk
    pub static ref CHAT_SDK: ChatSdk = ChatSdk::new();
}
