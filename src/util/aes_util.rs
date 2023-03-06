extern crate crypto;

use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::{aes, blockmodes, buffer, symmetriccipher};

static IV: &str = "0123456789012345";

pub fn encrypt(data: &[u8], key: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut encryptor = aes::cbc_encryptor(
        aes::KeySize::KeySize128,
        key,
        IV.as_bytes(),
        blockmodes::PkcsPadding,
    );

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = encryptor
            .encrypt(&mut read_buffer, &mut write_buffer, true)
            .unwrap();
        final_result.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&i| i),
        );

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}

pub fn decrypt(
    encrypted_data: &[u8],
    key: &[u8],
) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor = aes::cbc_decryptor(
        aes::KeySize::KeySize128,
        key,
        IV.as_bytes(),
        blockmodes::PkcsPadding,
    );

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor
            .decrypt(&mut read_buffer, &mut write_buffer, true)
            .unwrap();
        final_result.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&i| i),
        );
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(final_result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;
    use crate::util::base64_util::decode;

    #[test]
    fn test_encode() {
        let key = "ShVmYq3t6w9z$C&E";
        assert_eq!(
            encrypt("hello".as_bytes(), key.as_bytes()).unwrap(),
            vec![70, 187, 90, 156, 151, 198, 246, 224, 184, 153, 170, 232, 91, 104, 248, 4]
        );
    }

    #[test]
    fn test_decode() {
        let key = "ShVmYq3t6w9z$C&E";
        let encrypt_data = vec![
            70, 187, 90, 156, 151, 198, 246, 224, 184, 153, 170, 232, 91, 104, 248, 4,
        ];

        let decrypt_data = decrypt(&encrypt_data, key.as_bytes()).unwrap();
        assert_eq!(decrypt_data, "hello".as_bytes());
    }
}
