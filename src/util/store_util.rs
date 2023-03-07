use crate::util::safe_store::{SafeStore, StoreConfig};
use once_cell::sync::Lazy;
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::{Arc, Mutex, MutexGuard};

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

static PWD_PATH: Lazy<Arc<Mutex<Option<String>>>> =
    Lazy::new(|| Arc::new(Mutex::new(Option::None)));
static CURRENT_USER_ID: Lazy<Arc<Mutex<Option<String>>>> =
    Lazy::new(|| Arc::new(Mutex::new(Option::None)));
static STORE_MAPS: Lazy<Arc<Mutex<BTreeMap<StoreName, Arc<Mutex<SafeStore>>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(BTreeMap::new())));

pub(crate) fn init(pwd: &str) -> () {
    match PWD_PATH.lock() {
        Ok(mut i) => *i = Some(pwd.to_string()),
        Err(_) => panic!("pwd path is not empty!"),
    }
    let mut store_map = STORE_MAPS.lock().unwrap();
    match store_map.get(&StoreName::COMMON) {
        Some(_) => (),
        None => {
            let path = get_store_config(pwd, None);
            let config = StoreConfig {
                pwd: path,
                config_name: StoreName::COMMON.str().to_string(),
            };
            let store = SafeStore::from_config(config);
            store_map.insert(StoreName::COMMON, Arc::new(Mutex::new(store)));
        }
    }
}

pub fn change_user_scope(user_id: &str, common_store: MutexGuard<SafeStore>) {
    restore();

    match CURRENT_USER_ID.lock() {
        Ok(mut i) => *i = Some(user_id.to_string()),
        Err(_) => (),
    }
    common_store.set("last_login_user_id", user_id);
}

pub fn get(store_name: StoreName) -> Option<Arc<Mutex<SafeStore>>> {
    let mut store_map = STORE_MAPS.lock().unwrap();
    match store_map.get(&store_name) {
        Some(store) => Some(store.clone()),
        None => match PWD_PATH.lock() {
            Ok(pwd) => {
                let uid = CURRENT_USER_ID.lock().unwrap().clone().unwrap();
                let path = get_store_config(pwd.as_ref().unwrap().as_str(), Some(uid.as_str()));
                let config = StoreConfig {
                    pwd: path,
                    config_name: store_name.str().to_string(),
                };
                let store = Arc::new(Mutex::new(SafeStore::from_config(config)));
                store_map.insert(store_name, store.clone());
                return Some(store);
            }
            Err(_) => None,
        },
    }
}

pub fn restore() {
    match CURRENT_USER_ID.lock() {
        Ok(mut i) => *i = None,
        Err(_) => (),
    }

    match STORE_MAPS.lock() {
        Ok(mut store_map) => store_map.retain(|k, _| *k == StoreName::COMMON),
        Err(_) => (),
    }
}

pub fn get_store_config(pwd: &str, user_id: Option<&str>) -> String {
    let mut binding = Path::new(pwd).join("Profiles");
    if let Some(uid) = user_id {
        binding = binding.join(uid);
    }
    let path = Path::new(binding.to_str().unwrap());
    return path.to_str().unwrap().to_string();
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::util::store_util;
//     use crate::util::store_util::StoreName;
//     use std::thread;

//     #[test]
//     fn test_store_write_and_read() {
//         let idx = 1000;
//         let key = format!("user_name_{}", idx);
//         store_util::get(StoreName::EMOTICONS)
//             .unwrap()
//             .lock()
//             .unwrap()
//             .set(key.clone(), key.clone().as_str());

//         let val = store_util::get(StoreName::EMOTICONS)
//             .unwrap()
//             .lock()
//             .unwrap()
//             .get(key.clone())
//             .unwrap();
//         println!(
//             "threadId: {:?}, idx: {}, val: {:?}",
//             thread::current().id(),
//             idx,
//             String::from_utf8(val.to_vec())
//         );
//     }
// }
