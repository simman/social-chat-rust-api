use anyhow::{anyhow, Result};
use prost::Message;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Clone)]
pub struct ReqPack<T> {
    pub cmd: u16,
    pub sequence: u64,
    pub data: Vec<u8>,
    pub protocol_data: Option<Box<T>>,
}

impl<T: Message> ReqPack<T> {
    pub fn default() -> ReqPack<T> {
        ReqPack {
            cmd: 0,
            sequence: 0,
            data: vec![],
            protocol_data: None,
        }
    }

    pub fn set_cmd(&mut self, cmd: u16) -> &mut ReqPack<T> {
        self.cmd = cmd;
        self
    }

    pub fn set_sequence(&mut self, sequence: u64) -> &mut ReqPack<T> {
        self.sequence = sequence;
        self
    }

    pub fn set_data(&mut self, protocol_data: T) -> &mut ReqPack<T> {
        self.protocol_data = Some(Box::new(protocol_data));
        self
    }

    pub fn encode(&mut self) -> Result<Vec<u8>> {
        match self.protocol_data.as_ref() {
            Some(data) => {
                let v = data.encode_to_vec();
                self.data = v.clone();
                Ok(v)
            }
            None => Err(anyhow!("encode failed")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::socket::protocols::auth_success::AuthSuccess;

    #[test]
    fn test_encode() {
        let mut auth = AuthSuccess::default();
        auth.user_id = String::from("10000000");

        let mut p = ReqPack::default();
        let data = p.set_cmd(100).set_sequence(5000).set_data(auth).encode();
        assert_eq!(vec![26, 8, 49, 48, 48, 48, 48, 48, 48, 48], data.unwrap())
    }
}
