use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{Error, Result};
use bytes::BufMut;
use image::EncodableLayout;
use log::debug;

use crate::util;
use rsa::{
    pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding},
    Pkcs1v15Encrypt, PublicKey, RsaPrivateKey, RsaPublicKey,
};

static PAIR_AES_KEY: &str = "ShVmYq3t6w9z$C&E";

trait RsaEncodePrivateKey {
    fn to_pkcs8_encrypted_pem_file(&self, path: impl AsRef<Path>) -> Result<()>;
    fn from_pkcs8_encrypted_pem_file(path: impl AsRef<Path>) -> Result<RsaPrivateKey>;
}

impl RsaEncodePrivateKey for RsaPrivateKey {
    fn to_pkcs8_encrypted_pem_file(&self, path: impl AsRef<Path>) -> Result<()> {
        let _priv_key = self.to_pkcs8_pem(rsa::pkcs8::LineEnding::default())?;
        let aes_encrypt_data =
            util::aes_util::encrypt(_priv_key.as_bytes(), PAIR_AES_KEY.as_bytes()).unwrap();
        let mut file = File::create(path).expect("create public.pem failed");
        file.write_all(&aes_encrypt_data).expect("write failed");
        Ok(())
    }

    fn from_pkcs8_encrypted_pem_file(path: impl AsRef<Path>) -> Result<RsaPrivateKey> {
        let content = fs::read(path)?;
        let aes_decrypt_data = util::aes_util::decrypt(&content, PAIR_AES_KEY.as_bytes()).unwrap();
        Ok(
            RsaPrivateKey::from_pkcs8_pem(String::from_utf8(aes_decrypt_data).unwrap().as_str())
                .unwrap(),
        )
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RsaKeyPair {
    pub priv_key: RsaPrivateKey,
    pub pub_key: RsaPublicKey,
}

pub trait PrintKeyStr {
    fn get_key_str(&self) -> Result<String>;
    fn get_java_key_str(&self) -> Result<String>;
}

impl PrintKeyStr for RsaPrivateKey {
    fn get_key_str(&self) -> Result<String> {
        Ok(String::from(
            self.to_pkcs8_pem(LineEnding::default())?.as_str(),
        ))
    }

    fn get_java_key_str(&self) -> Result<String> {
        todo!()
    }
}

impl PrintKeyStr for RsaPublicKey {
    fn get_key_str(&self) -> Result<String> {
        Ok(String::from(
            self.to_public_key_pem(LineEnding::default())?.as_str(),
        ))
    }

    fn get_java_key_str(&self) -> Result<String> {
        Ok(
            String::from(self.to_public_key_pem(LineEnding::default())?.as_str())
                .replace("-----BEGIN PUBLIC KEY-----", "")
                .replace("-----END PUBLIC KEY-----", "")
                .replace("\n", ""),
        )
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

    priv_key.to_pkcs8_encrypted_pem_file(key_pair_path.join("private.pem").to_str().unwrap())?;

    Ok(RsaKeyPair { priv_key, pub_key })
}

fn load_key_pair(base_path: &str) -> Result<RsaKeyPair> {
    let key_pair_path = PathBuf::from(base_path).join("keyPair");
    let priv_key = RsaPrivateKey::from_pkcs8_encrypted_pem_file(
        key_pair_path.join("private.pem").to_str().unwrap(),
    )
    .unwrap();
    let pub_key = RsaPublicKey::from(&priv_key);
    Ok(RsaKeyPair { priv_key, pub_key })
}

fn key_pair_exist(base_path: &str) -> Result<bool> {
    let key_pair_path = PathBuf::from(base_path).join("keyPair");
    Ok(Path::new(key_pair_path.join("private.pem").to_str().unwrap()).exists())
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
    match pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, msg) {
        Ok(v) => Ok(v),
        Err(e) => Err(Error::from(e)),
    }
}

pub fn decrypt(priv_key: &RsaPrivateKey, data: &[u8]) -> Result<Vec<u8>> {
    let mut decrypt_result: Vec<u8> = Vec::new();
    let mut decrypt_message = data.clone();
    let max_len = 256;

    while decrypt_message.len() > 0 {
        let input_data = if decrypt_message.len() >= max_len {
            &decrypt_message[..max_len]
        } else {
            &decrypt_message[..]
        };

        decrypt_message = if decrypt_message.len() >= max_len {
            &decrypt_message[max_len..]
        } else {
            &decrypt_message[..]
        };

        match priv_key.decrypt(Pkcs1v15Encrypt, input_data) {
            Ok(v) => decrypt_result.put(v.as_bytes()),
            Err(e) => {
                return Err(Error::from(e));
            }
        }
    }
    Ok(decrypt_result)
}

#[cfg(test)]
mod test {
    use crate::util;
    use crate::util::rsa_util::{decrypt, encrypt, get_or_gen_key_pair, PrintKeyStr};
    use log::debug;
    use rsa::{
        pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding},
        Pkcs1v15Encrypt, PublicKey, RsaPrivateKey, RsaPublicKey,
    };
    use std::env;
    use std::io::Read;

    #[test]
    fn test_encrypt_and_decrypt() {
        let temp_dir = env::temp_dir().join("_rsa_util_test_");
        let key_pair = get_or_gen_key_pair(temp_dir.to_str().unwrap()).unwrap();
        // println!("private_key: {:?}", key_pair.priv_key.get_key_str());
        // println!("public_key: {:?}", key_pair.pub_key.get_key_str());

        let data = "abc123456".as_bytes();
        let encrypt_data = encrypt(&key_pair.pub_key, data).unwrap();
        println!("{:?}", util::base64_util::encode(&encrypt_data));
        assert_eq!(data, decrypt(&key_pair.priv_key, &encrypt_data).unwrap());
    }
}
