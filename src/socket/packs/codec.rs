use crate::config::constant::CHAT_SDK;
use crate::socket::cmd::command::*;
use crate::socket::packs::req_pack::ReqPack;
use crate::socket::packs::ret_pack::RetPack;
use crate::socket::protocols::rsa_key::RsaKeyRes;
use crate::util;
use anyhow::Result;
use bytes::{Buf, BufMut, BytesMut};
use image::EncodableLayout;
use log::{debug, error};
use prost::Message;
use std::any::Any;

pub fn encode<T: Message>(mut pack: ReqPack<T>) -> Box<Vec<u8>> {
    let mut content_len = 7;
    // 如果不是心跳包
    if pack.cmd != C2S_HEARTBEAT {
        content_len += 8
    }

    let mut body = BytesMut::new();

    // 如果是确认包, 无需处理
    if pack.cmd != S2C_ACK {
        let re = pack.encode().unwrap();
        body.put_slice(&re);
    }

    let have_rsa_encrypt_cmd = vec![C2S_KEY];
    let no_need_encrypt_cmd = vec![C2S_RSAKEY];

    // noNeedEncryptCmd := []int{msg_cmd.C2S_RSAKEY}
    // haveRsaEncryptCmd := []int{msg_cmd.C2S_KEY}
    // 需要rsa加密
    if have_rsa_encrypt_cmd.contains(&pack.cmd) {
        debug!("rsa encrypt cmd: {}", pack.cmd);
        let rsa_public_key = util::rsa_util::get_pub_key_pair_with_public_key("MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAjNPWIwsGqIcjMATBeHH6fjw7gpee1zgISYtZi6KBdUNxbU/s//Toz0KFxc9eHuSCsh/W88XM69eYvsytshveswDPE7f52roPoaR7jwIBgo87eWxOzvqjvad88YAurdvEP9ZFRbNa1OUPl48VFOXI3zQnyNxj9XXX2ombLQcfhbbPyHmtph/VNrNHs5tfTuKIvrje0M4JLWfC20fIgKo8JIqwYS1rVIRdXW6E8feNqd72z8Up2721m/+yzMgx3Sv3IYY0TiHWBB44VYRNc5rgGwPoQkq04B306aJaKvVkAOdU7Flcgv0nhgWx41HFv30mmqzMxNwp4zjG5e38rGjFhQIDAQAB").unwrap();
        let ret = util::rsa_util::encrypt(&rsa_public_key, &body).unwrap();
        body = BytesMut::from(ret.as_bytes());
    } else if no_need_encrypt_cmd.contains(&pack.cmd) && body.len() > 0 {
    } else if body.len() > 0 {
        // 需要使用aes加密的指令
    }

    let mut buf = BytesMut::new();
    buf.put_u32(content_len + body.len() as u32);
    buf.put_u16(1); // version: 1
    buf.put_u8(b'$');
    buf.put_u8(b'T');
    buf.put_u8(b'Q');
    buf.put_u16(pack.cmd);
    buf.put_u64(pack.sequence);
    if body.len() > 0 {
        buf.put(body);
    }
    return Box::new(buf.to_vec());
}

pub fn decode(body_len: u32, mut buf: &[u8]) -> Result<RetPack> {
    let _ = buf.get_u8(); // $
    let _ = buf.get_u8(); // T
    let _ = buf.get_u8(); // Q

    let cmd = buf.get_u16();
    //

    let mut pack = RetPack::default();
    pack.set_len(body_len).set_cmd(cmd);

    let _len: usize = body_len as usize - 13;
    let mut body = vec![];

    // 如果不是心跳包
    if S2C_HEARTBEAT != cmd {
        let sequence = buf.get_u64();
        pack.set_sequence(sequence);
        body = buf[.._len].to_vec();
        // pack.set_data(body.to_vec());
    }

    // 如果有数据
    if body.len() > 0 {
        // 需要使用 rsa 解密 body
        let need_rsa_decrypt_cmd = vec![S2C_KEY, S2C_RSAKEY];
        let need_aes_decrypt_cmd = vec![S2C_ACK, S2C_KICK, S2C_DESTORY_SUCCESS];

        if need_rsa_decrypt_cmd.contains(&pack.cmd) {
            // match util::rsa_util::decrypt(&CHAT_SDK.rsa_key_pair.priv_key, &body) {
            //     Ok(v) => {
            //         pack.set_data(v);
            //     }
            //     Err(e) => {
            //         error!("rsa decrypt error: {:?}", e);
            //         return Err(e);
            //     }
            // }
        } else if need_aes_decrypt_cmd.contains(&pack.cmd) {
            // 需要使用 aes 解密 body
        }

        // let res = pack.decode::<RsaKeyRes>();
        // debug!("======================");
        // debug!("pack: {:?}", res);
        return Ok(pack);
    }

    return Err(anyhow::anyhow!("decode RetPack error!"));
}

fn check_valid_ret_pack() {}

#[cfg(test)]
mod test {
    use crate::socket::packs::codec::encode;
    use crate::socket::packs::req_pack::ReqPack;
    use crate::socket::protocols::auth_success::AuthSuccess;
    use bytes::{BufMut, Bytes, BytesMut};
    use image::EncodableLayout;

    #[test]
    fn test_encode() {
        let mut auth = AuthSuccess::default();
        auth.user_id = String::from("10000000");

        let mut p = ReqPack::default();
        let data = p.set_cmd(100).set_sequence(5000).set_data(auth).encode();

        let b = encode(p);

        println!("{:?}", b);
    }

    #[test]
    fn test_xxx() {
        println!("{:?}", u16::MAX);
    }
}
