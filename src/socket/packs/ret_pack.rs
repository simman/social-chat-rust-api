use crate::cmd;
use crate::protos::ack_status::AckStatus;
use crate::protos::auth_success::AuthSuccess;
use anyhow::{anyhow, Result};
use prost::{DecodeError, Message};

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug)]
pub struct RetPack {
    len: usize,
    cmd: usize,
    sequence: usize,
    data: Vec<u8>,
    proto_data: Option<Box<dyn Message>>,
}

impl RetPack {
    pub fn default() -> RetPack {
        RetPack {
            len: 0,
            cmd: 0,
            sequence: 0,
            data: vec![],
            proto_data: None,
        }
    }

    pub fn set_cmd(&mut self, cmd: usize) -> &mut RetPack {
        self.cmd = cmd;
        self
    }

    pub fn set_sequence(&mut self, sequence: usize) -> &mut RetPack {
        self.sequence = sequence;
        self
    }

    pub fn set_data(&mut self, data: Vec<u8>) -> &mut RetPack {
        self.data = data;
        self
    }

    pub fn decode(&mut self) -> Result<()> {
        if cmd::command::S2C_RSAKEY == self.cmd {
            let decode = AuthSuccess::decode(&*self.data);
            let data = decode?;
            self.proto_data = Some(Box::new(data))
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

        let x = p.proto_data.unwrap();
        println!("{:?}", x);
    }
}
