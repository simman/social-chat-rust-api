use crate::config::constant::SDK_CONFIG;
use crate::util;
use crate::util::logger_util::init_log;
use crate::util::rsa_util::RsaKeyPair;

#[derive(Debug)]
pub struct ChatSdk {
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

        // init rsa pair
        ChatSdk {
            rsa_key_pair: util::rsa_util::get_or_gen_key_pair(data_dir).unwrap(),
        }
    }
}
