use crate::socket::cmd;
use crate::socket::protocols::auth_success::AuthSuccess;
use crate::socket::protocols::rsa_key::RsaKeyRes;
use anyhow::Result;
use log::debug;
use prost::Message;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug)]
pub struct RetPack {
    pub len: u32,
    pub cmd: u16,
    pub sequence: u64,
    pub data: Vec<u8>,
    pub protocol_data: Option<Box<dyn Message>>,
}

impl RetPack {
    pub fn default() -> RetPack {
        RetPack {
            len: 0,
            cmd: 0,
            sequence: 0,
            data: vec![],
            protocol_data: None,
        }
    }

    pub fn set_len(&mut self, len: u32) -> &mut RetPack {
        self.len = len;
        self
    }

    pub fn set_cmd(&mut self, cmd: u16) -> &mut RetPack {
        self.cmd = cmd;
        self
    }

    pub fn set_sequence(&mut self, sequence: u64) -> &mut RetPack {
        self.sequence = sequence;
        self
    }

    pub fn set_data(&mut self, data: Vec<u8>) -> &mut RetPack {
        self.data = data;
        self
    }

    pub fn decode(&mut self) -> Result<()> {
        if cmd::command::S2C_RSAKEY == self.cmd {
            let decode = RsaKeyRes::decode(&*self.data);
            let data = decode?;
            debug!("protocol_data解析结果: {:?}", data);
            self.protocol_data = Some(Box::new(data))
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        let mut p: RetPack = RetPack::default();
        p.set_data(vec![26, 8, 49, 48, 48, 48, 48, 48, 48, 48]);
        p.set_cmd(cmd::command::S2C_RSAKEY);
        let _ = p.decode();

        let x = p.protocol_data.unwrap();
        println!("{:?}", x);
    }
}
