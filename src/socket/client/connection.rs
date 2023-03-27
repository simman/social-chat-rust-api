use crate::api;
use crate::config::constant::CHAT_SDK;
use crate::socket::client::client::SocketClient;
use crate::socket::cmd::command::S2C_RSAKEY;
use crate::socket::packs::codec;
use crate::socket::packs::req_pack::ReqPack;
use crate::socket::protocols::rsa_key::RsaKeyRes;
use crate::util::logger_util::SOCKET_LOG;
use anyhow::{Error, Result};
use bytes::Buf;
use image::EncodableLayout;
use log::debug;
use parking_lot::Mutex;
use prost::Message;
use std::ops::Deref;
use std::thread;
use std::time::Duration;

pub struct Connection {
    client: SocketClient,
}

impl Connection {
    pub fn new() -> Self {
        Self {
            client: SocketClient::new(),
        }
    }

    /// 连接服务器
    /// addr: 服务器地址
    pub async fn connect(&mut self, addr: Option<String>) -> Result<()> {
        // 监测是否已经连接状态
        if self.client.is_connected() {
            return Err(Error::msg("socket client is connected!"));
        }

        // 如果外部传入addr则直接连接
        let tcp_addr = if let Some(tcp_addr) = addr {
            tcp_addr
        } else {
            api::common::get_server_socket_url().await?
        };

        debug!("tcp connect to: {:?}", tcp_addr);

        self.client.connect(tcp_addr.as_str()).await?;
        self.start_process_loop();
        Ok(())
    }

    /// 启动消息处理循环
    fn start_process_loop(&mut self) {
        self.start_parse_message();
        self.client.start_read_loop();
        self.client.start_write_loop();
    }

    pub fn start_parse_message(&mut self) {
        let messages_in = self.client.messages_in.clone();

        thread::spawn(move || {
            loop {
                // 解包
                // contentLen有4位，本身不占有长度
                let content_len: u32 = 4;
                let mut msg_in = messages_in.lock();
                let mut copy_bytes = msg_in.clone(); // bytesMut
                if copy_bytes.len() as u32 > content_len {
                    // 读取body长度
                    let body_len = copy_bytes.get_u32();
                    if body_len + content_len <= msg_in.len() as u32 {
                        debug!("开始解包 ===>>>");
                        let mut ret = codec::decode(body_len, copy_bytes.as_bytes()).unwrap();
                        if ret.cmd == S2C_RSAKEY {
                            let ret = ret.decode::<RsaKeyRes>().unwrap();
                            // CHAT_SDK.set_server_public_key(ret.server_pub_key);
                        }
                        let _ = msg_in.split_to((body_len + content_len) as usize);
                    }
                }

                std::thread::sleep(Duration::from_millis(100));
            }
        });
    }

    /// 发送消息
    /// [pack] 消息包
    pub async fn send<T: Message>(&mut self, pack: ReqPack<T>) -> Result<()> {
        debug!(
            target: SOCKET_LOG,
            "[sequence: {} - cmd: {}]: =====>>>> sendMessage", pack.sequence, pack.cmd
        );
        let b = codec::encode(pack);
        self.client.send(b.to_vec());
        Ok(())
    }

    /// 断开连接
    pub async fn disconnect(&mut self) {
        self.client.disconnect().await;
    }
}
