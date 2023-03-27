use crate::api::client::{ApiClient, ClientTypeEnum};
use crate::chat_sdk::ChatSdk;
use crate::config::config::{load_config, ChatSdkConfig};
use crate::util::device_util::serial;
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::rc::Rc;

thread_local! {
    pub static CHAT_SDK: Rc<RefCell<ChatSdk>> = Rc::new(RefCell::new(ChatSdk::new()));
}

lazy_static! {
    // 全局配置
    pub static ref SDK_CONFIG: ChatSdkConfig = load_config().unwrap();
    // sdk
    pub static ref SDKKKKK: ChatSdk = ChatSdk::new();
    // global http client
    pub static ref HTTP_CLIENT: ApiClient = ApiClient::new_client(ClientTypeEnum::COMMON);
    // IM HTTP Client
    pub static ref IM_HTTP_CLIENT: ApiClient = ApiClient::new_client(ClientTypeEnum::IM);
    // machine id
    pub static ref MACHINE_ID: String = serial();
}
