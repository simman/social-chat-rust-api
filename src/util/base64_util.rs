use base64::{Engine, engine::general_purpose, DecodeError};

pub fn encode<T: AsRef<[u8]>>(input: T) -> String {
    general_purpose::STANDARD.encode(input)
}

pub fn decode<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, DecodeError> {
    general_purpose::STANDARD.decode(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(encode("hello".as_bytes()), "aGVsbG8=");
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode("aGVsbG8=").unwrap(), "hello".as_bytes());
    }
}