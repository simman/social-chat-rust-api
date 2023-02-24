use std::error::Error;
use crypto::aes::KeySize::KeySize128;
use crypto::blockmodes::{PkcsPadding};
use crypto::buffer::{RefReadBuffer, RefWriteBuffer};

static IV: &str = "0123456789012345";

/// aes 加密
pub fn encrypt(key: &[u8], text: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut encrypt = crypto::aes::cbc_encryptor(
        KeySize128,
        key,
        IV.as_bytes(),
        PkcsPadding,
    );
    let mut read_buffer = RefReadBuffer::new(text);
    let mut result = vec![0; text.len() * 4];
    let mut write_buffer = RefWriteBuffer::new(&mut result);
    encrypt.encrypt(&mut read_buffer, &mut write_buffer, true).unwrap();
    Ok(result.into_iter().filter(|v| *v != 0).collect())
}


/// aes 解密
pub fn decrypt(key: &[u8], text: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut decrypt = crypto::aes::cbc_decryptor(
        KeySize128,
        key,
        IV.as_bytes(),
        PkcsPadding,
    );
    let mut read_buffer = RefReadBuffer::new(text);
    let mut result = vec![0; text.len()];
    let mut write_buffer = RefWriteBuffer::new(&mut result);
    decrypt.decrypt(&mut read_buffer, &mut write_buffer, true).unwrap();
    Ok(result)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let key = "ShVmYq3t6w9z$C&E";
        assert_eq!(encrypt(key.as_bytes(), "hello".as_bytes()).unwrap(), vec![70, 187, 90, 156, 151, 198, 246, 224, 184, 153, 170, 232, 91, 104, 248, 4]);
    }

    #[test]
    fn test_decode() {
        let key = "ShVmYq3t6w9z$C&E";
        let data: Vec<u8> = vec![70, 187, 90, 156, 151, 198, 246, 224, 184, 153, 170, 232, 91, 104, 248, 4];
        let mut de_data = decrypt(key.as_bytes(), &data).unwrap();
        de_data.retain(|&f| f != 0u8);
        assert_eq!(de_data, "hello".as_bytes().to_vec());
    }
}