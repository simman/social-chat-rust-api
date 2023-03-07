use sled::{self, Db, IVec};
use std::path::Path;

pub struct StoreConfig {
    pub pwd: String,
    pub config_name: String,
}

pub struct SafeStore {
    db: Db,
}

impl SafeStore {
    pub fn from_config(config: StoreConfig) -> SafeStore {
        let binding = Path::new(&config.pwd).join(config.config_name);
        let path = Path::new(binding.to_str().unwrap());
        let database = match sled::open(path) {
            Ok(db) => db,
            Err(e) => panic!("failed to open database: {:?}", e),
        };
        SafeStore { db: database }
    }

    pub fn set<K: AsRef<[u8]>, V: Into<IVec>>(&self, key: K, value: V) -> bool {
        match self.db.insert(key, value) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn get<K: AsRef<[u8]>>(&self, key: K) -> Option<IVec> {
        return self.db.get(key).unwrap();
    }

    pub fn del<K: AsRef<[u8]>>(&self, key: K) -> bool {
        match self.db.remove(key) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn has<K: AsRef<[u8]>>(&self, key: K) -> bool {
        match self.db.contains_key(key) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn clear(&self) {
        let _ = self.db.clear();
    }

    pub fn len(&self) -> usize {
        self.db.len()
    }
}

pub trait IVecConvert {
    fn to_string(&self) -> String;
}

impl IVecConvert for IVec {
    fn to_string(&self) -> String {
        String::from_utf8(self.to_vec()).unwrap()
    }
}
