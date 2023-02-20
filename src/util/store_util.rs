use std::collections::{BTreeMap};
use std::path::Path;
use crate::safe_store::{SafeStore, StoreConfig};
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub enum StoreName {
    COMMON,
    USER,
    FRIEND,
    GROUP,
    EMOTICONS,
    SESSION,
    ASSETS,
}

impl StoreName {
    fn str(&self) -> &str {
        match self {
            StoreName::COMMON => "common-store",
            StoreName::USER => "user-store",
            StoreName::FRIEND => "user-friends-store",
            StoreName::GROUP => "group-store",
            StoreName::EMOTICONS => "emoticons-store",
            StoreName::SESSION => "session-store",
            StoreName::ASSETS => "assets-store",
        }
    }
}

static PWD_PATH: Lazy<Arc<Mutex<Option<String>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(Option::None))
});
static CURRENT_USER_ID: Lazy<Arc<Mutex<Option<String>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(Option::None))
});
static STORE_MAPS: Lazy<Arc<Mutex<BTreeMap<StoreName, SafeStore>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(BTreeMap::new()))
});

pub fn init(pwd: &str, user_id: &str) -> () {
    match PWD_PATH.lock() {
        Ok(mut i) => *i = Some(pwd.to_string()),
        Err(_) => panic!("pwd path is not empty!"),
    }
    match CURRENT_USER_ID.lock() {
        Ok(mut i) => *i = Some(user_id.to_string()),
        Err(_) => panic!("current user id is not empty!"),
    }
    let mut sl = STORE_MAPS.lock().unwrap();
    match sl.get(&StoreName::COMMON) {
        Some(_) => (),
        None => {
            // let binding = Path::new(pwd).join("Profiles");
            // let path = Path::new(binding.to_str().unwrap());
            let path = get_store_config(pwd, None);
            let config = StoreConfig {
                pwd: path,
                config_name: StoreName::COMMON.str().to_string(),
            };
            let store = SafeStore::from_config(config);
            sl.insert(StoreName::COMMON, store);
        }
    }
}

pub fn change_user_scope(user_id: &str) {
    match CURRENT_USER_ID.lock() {
        Ok(mut i) => *i = Some(user_id.to_string()),
        Err(_) => (),
    }

    restore();

    let sl = STORE_MAPS.lock().unwrap();
    match sl.get(&StoreName::COMMON) {
        Some(ss) => {
            ss.set("lastLoginUser.userId", user_id);
        },
        None => (),
    }
}

pub fn get<'static>(store_name: StoreName) -> Option<&'static SafeStore> {
    let mut sl = STORE_MAPS.lock().unwrap();
    if store_name == StoreName::COMMON {
        let mut common_store = sl.get(&StoreName::COMMON);
        return common_store;
        // return sl.get(&StoreName::COMMON);
        // return Some(sl.get(&StoreName::COMMON).unwrap())
    }

    // match sl.get(&store_name) {
    //     Some(ss) => Some(ss),
    //     None => {
    //         match PWD_PATH.lock() {
    //             Ok(mut pwd) => {
    //                 let path = get_store_config(&pwd.unwrap().to_string(), None);
    //                 // let config = StoreConfig {
    //                 //     pwd: path,
    //                 //     config_name: store_name.str().to_string(),
    //                 // };
    //                 // let store = SafeStore::from_config(config);
    //                 // sl.insert(store_name, store);

    //                 // Some(store);

    //                 None
    //             },
    //             Err(_) => None,
    //         }
    //     },
    // }

//     None
}

pub fn restore() {
    match CURRENT_USER_ID.lock() {
        Ok(mut i) => *i = None,
        Err(_) => (),
    }

    match STORE_MAPS.lock() {
        Ok(mut sl) => sl.retain(|k, _| *k != StoreName::COMMON),
        Err(_) => (),
    }
}

pub fn get_store_config(pwd: &str, user_id: Option<&str>) -> String {
    let mut binding = Path::new(pwd).join("Profiles");
    match user_id {
        Some(uid) => binding = binding.join(uid),
        None => (),
    };
    let path = Path::new(binding.to_str().unwrap());
    return path.to_str().unwrap().to_string()
}