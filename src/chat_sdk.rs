use crate::config::constant::SDK_CONFIG;
use crate::util;
use crate::util::logger_util::init_log;
use crate::util::rsa_util::RsaKeyPair;
use crate::util::safe_store::IVecConvert;
use crate::util::store_util::StoreName;

#[derive(Debug)]
pub struct ChatSdk {
    pub version: String,
    rsa_key_pair: RsaKeyPair,
}

impl ChatSdk {
    pub(crate) fn new() -> ChatSdk {
        let data_dir = SDK_CONFIG.sdk.data_dir.as_str();
        // init config
        // init log4rs
        init_log(data_dir);

        // init safe store
        util::store_util::init(data_dir);

        // 如果有有登录的用户, 则直接加载
        if let Some(common_store) = util::store_util::get(StoreName::COMMON) {
            match common_store.lock() {
                Ok(store) => {
                    if let Some(uid) = store.get("last_login_user_id") {
                        util::store_util::change_user_scope(uid.to_string().as_str(), store);
                    }
                }
                Err(_) => (),
            }
        }
        // init rsa pair
        ChatSdk {
            version: String::from("1.0.0"),
            rsa_key_pair: util::rsa_util::get_or_gen_key_pair(data_dir).unwrap(),
        }
    }
}
