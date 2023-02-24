use std::path::{Path, PathBuf};

use anyhow::Result;

use rsa::{
    pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding},
    Pkcs1v15Encrypt, PublicKey, RsaPrivateKey, RsaPublicKey,
};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RsaKeyPair {
    pub priv_key: RsaPrivateKey,
    pub pub_key: RsaPublicKey,
}

pub trait PrintKeyStr {
    fn get_key_str(&self) -> Result<String>;
}

impl PrintKeyStr for RsaPrivateKey {
    fn get_key_str(&self) -> Result<String> {
        Ok(String::from(
            self.to_pkcs8_pem(LineEnding::default())?.as_str(),
        ))
    }
}

impl PrintKeyStr for RsaPublicKey {
    fn get_key_str(&self) -> Result<String> {
        Ok(String::from(
            self.to_public_key_pem(LineEnding::default())?.as_str(),
        ))
    }
}

fn generate_key_pair(base_path: &str, pkcs_size: Option<usize>) -> Result<RsaKeyPair> {
    let mut rng = rand::thread_rng();
    let bits = pkcs_size.unwrap_or(2048);
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);
    let key_pair_path = PathBuf::from(base_path).join("keyPair");
    let p = Path::new(key_pair_path.to_str().unwrap());
    if !p.exists() {
        std::fs::create_dir_all(p)?
    }
    priv_key.write_pkcs8_pem_file(
        key_pair_path.join("private.pem").to_str().unwrap(),
        rsa::pkcs8::LineEnding::default(),
    )?;
    pub_key.write_public_key_pem_file(
        key_pair_path.join("public.pem").to_str().unwrap(),
        rsa::pkcs8::LineEnding::default(),
    )?;
    Ok(RsaKeyPair { priv_key, pub_key })
}

fn load_key_pair(base_path: &str) -> Result<RsaKeyPair> {
    let key_pair_path = PathBuf::from(base_path).join("keyPair");
    let priv_key =
        RsaPrivateKey::read_pkcs8_pem_file(key_pair_path.join("private.pem").to_str().unwrap())
            .unwrap();
    let pub_key = RsaPublicKey::from(&priv_key);
    Ok(RsaKeyPair { priv_key, pub_key })
}

fn key_pair_exist(base_path: &str) -> Result<bool> {
    let key_pair_path = PathBuf::from(base_path).join("keyPair");
    Ok(
        Path::new(key_pair_path.join("private.pem").to_str().unwrap()).exists()
            && Path::new(key_pair_path.join("public.pem").to_str().unwrap()).exists(),
    )
}

pub fn get_or_gen_key_pair(base_path: &str) -> Result<RsaKeyPair> {
    return match key_pair_exist(base_path) {
        Result::Ok(exist) => {
            if exist {
                load_key_pair(base_path)
            } else {
                generate_key_pair(base_path, None)
            }
        }
        Err(e) => Err(e),
    };
}

pub fn get_pub_key_pair_with_public_key(public_key: &str) -> Result<RsaPublicKey> {
    if !public_key.starts_with("-----BEGIN PUBLIC KEY-----") {
        let line_len = 64;
        let mut public_key_pem = String::new();
        let len = public_key.len() / line_len;
        for i in 0..len {
            let start = i * line_len;
            let end = (i + 1) * line_len;
            let xx = &public_key[start..end];
            public_key_pem.push_str(&*format!("{}\n", xx));
        }
        public_key_pem.push_str(&public_key[6 * line_len..public_key.len()]);
        public_key_pem.insert_str(0, "-----BEGIN PUBLIC KEY-----\n");
        public_key_pem.push_str("\n-----END PUBLIC KEY-----");
        return Ok(RsaPublicKey::from_public_key_pem(public_key_pem.as_str())?);
    }
    Ok(RsaPublicKey::from_public_key_pem(public_key)?)
}

pub fn encrypt(pub_key: &RsaPublicKey, msg: &[u8]) -> Result<Vec<u8>> {
    let mut rng = rand::thread_rng();
    let o = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, msg);
    Result::Ok(o.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store_util;
    use crate::store_util::StoreName;
    use std::thread;

    #[test]
    fn test_store_write_and_read() {
        let idx = 1000;
        let key = format!("user_name_{}", idx);
        store_util::get(StoreName::EMOTICONS)
            .unwrap()
            .lock()
            .unwrap()
            .set(key.clone(), key.clone().as_str());

        let val = store_util::get(StoreName::EMOTICONS)
            .unwrap()
            .lock()
            .unwrap()
            .get(key.clone())
            .unwrap();
        println!(
            "threadId: {:?}, idx: {}, val: {:?}",
            thread::current().id(),
            idx,
            String::from_utf8(val.to_vec())
        );
    }
}
