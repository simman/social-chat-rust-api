use crate::socket::packs::codec;
use anyhow::Result;
use bytes::{Buf, BufMut, BytesMut};
use image::EncodableLayout;
use log::{debug, error, info};
use parking_lot::Mutex;
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::time::sleep;

pub struct SocketClient {
    messages_in: Arc<Mutex<BytesMut>>,
    messages_out: Arc<Mutex<VecDeque<Vec<u8>>>>,

    is_connected: Arc<Mutex<bool>>,
    pub peer_addr: Option<std::net::SocketAddr>,

    read_stream: Option<ReadHalf<tokio::net::TcpStream>>,
    write_stream: Option<WriteHalf<tokio::net::TcpStream>>,
}

impl SocketClient {
    pub fn new() -> Self {
        let messages_in = Arc::new(Mutex::new(BytesMut::new()));
        let messages_out = Arc::new(Mutex::new(VecDeque::new()));
        SocketClient {
            messages_in,
            messages_out,
            is_connected: Arc::new(Mutex::new(false)),
            peer_addr: None,
            read_stream: None,
            write_stream: None,
        }
    }

    pub async fn connect(&mut self, addr: &str) -> Result<()> {
        match TcpStream::connect(addr).await {
            Ok(stream) => {
                let peer_addr = Some(stream.peer_addr().unwrap());
                self.peer_addr = peer_addr;
                // self.stream = Some(stream);

                let (read_stream, write_stream) = tokio::io::split(stream);
                self.read_stream = Some(read_stream);
                self.write_stream = Some(write_stream);
                *self.is_connected.lock() = true;
                Ok(())
            }
            Err(err) => Err(err.into()),
        }
    }

    pub fn start_read_loop(&mut self) {
        debug!("===> start_read_loop");
        if let Some(mut stream) = self.read_stream.take() {
            let messages_in = self.messages_in.clone();
            debug!("messages_in_len: {:?}", messages_in.lock().len());
            let is_connected = Arc::clone(&self.is_connected);
            tokio::spawn(async move {
                let mut buf = [0; 1024];
                loop {
                    if !*is_connected.lock() {
                        eprintln!("[Connection] connection closed but still alive?");
                        return;
                    }
                    let byte_count = match stream.read(&mut buf).await {
                        Ok(n) if n == 0 => {
                            debug!("read byte count: {}", n);
                            return;
                        }
                        Ok(n) => n,
                        Err(e) => {
                            eprintln!("[Connection] failed to read from socket; err = {:?}", e);
                            match e.kind() {
                                std::io::ErrorKind::BrokenPipe => {
                                    *is_connected.lock() = false;
                                }
                                _ => {
                                    unimplemented!()
                                }
                            }

                            return;
                        }
                    };

                    debug!("收到消息, len: {:?}", byte_count);

                    let b = &buf[0..byte_count];
                    messages_in.lock().put_slice(b);

                    debug!("收到的包: {:?}", b);

                    // 解包
                    // contentLen有4位，本身不占有长度
                    let content_len: u32 = 4;
                    let msg_in = messages_in.lock();
                    let mut copy_bytes = msg_in.as_bytes();
                    if copy_bytes.len() > 4 {
                        // 读取body长度
                        let body_len = copy_bytes.get_u32();
                        debug!(
                            "messages_in_len: {:?}, body_len: {:?}",
                            msg_in.len(),
                            body_len
                        );

                        if body_len + content_len <= msg_in.len() as u32 {
                            debug!("开始解包 ===>>>");
                            codec::decode(body_len, copy_bytes);
                        }
                    }
                }
            });
        }
    }

    pub fn start_write_loop(&mut self) {
        debug!("===> start_write_loop");
        if let Some(mut stream) = self.write_stream.take() {
            let messages_out = Arc::clone(&self.messages_out);
            let is_connected = Arc::clone(&self.is_connected);
            let peer_addr = self.peer_addr.unwrap().clone();
            tokio::spawn(async move {
                loop {
                    // debug!("发送消息....");
                    if !*is_connected.lock() {
                        eprintln!("[Connection] connection closed but still alive?");
                        return;
                    }

                    // let bytes: Vec<u8> = Vec::from("XX");
                    // let e = stream.write(&bytes).await;
                    if messages_out.lock().len() > 0 {
                        debug!("发送消息....");
                        let next = {
                            let mut write = messages_out.lock();
                            write.pop_front()
                        };
                        if let Some(msg) = next {
                            debug!("发送消息::::....");
                            let bytes: Vec<u8> = Vec::from(msg);
                            if let Err(e) = stream.write(&bytes).await {
                                eprintln!(
                                    "[Connection] failed to write to socket; addr:{:?} err = {:?}",
                                    peer_addr, e
                                );
                                match e.kind() {
                                    std::io::ErrorKind::BrokenPipe => {
                                        *is_connected.lock() = false;
                                    }
                                    _ => {
                                        unimplemented!()
                                    }
                                }
                                return;
                            }
                        }
                    }
                    sleep(Duration::from_millis(100)).await;
                }
            });
        }
    }

    pub async fn disconnect(&mut self) {
        todo!()
    }

    pub fn is_connected(&self) -> bool {
        *self.is_connected.lock()
    }

    pub fn send(&mut self, msg: Vec<u8>) {
        self.messages_out.lock().push_back(msg);
    }
}

#[cfg(test)]
mod test {
    use image::EncodableLayout;
    use std::thread::sleep;

    use crate::config::constant::{CHAT_SDK, MACHINE_ID};

    use super::*;
    use tokio::{
        io::{self, Interest},
        net::TcpStream,
    };

    #[test]
    fn test_vec_deque() {
        let mut buf = BytesMut::new();
        buf.put_u32(1);
        // buf.put_i8(1);
        // buf.put_i8(2);
        // buf.put_i8(3);
        // buf.put_u8(b'$');

        println!("===> 初始: {:?}, 长度: {}", buf.to_vec(), buf.len());

        // let mut len_bytes = buf.as_bytes();
        // println!("body_len: {:?}", len_bytes.get_u32());
        //
        // println!("get_i8 -> {}", buf.get_i8());
        // println!("get_i8 -> {}", buf.get_i8());
        //
        // println!("===> 最终: {:?}, 长度: {}", buf.to_vec(), buf.len());
    }

    #[tokio::test]
    async fn test_socket_client() {
        println!("{:?}", CHAT_SDK.version);
        let addr = "qx-im-8.aitdins.com:8700";
        // let addr = "tcpbin.com:4242";
        let mut socket = SocketClient::new();
        let res = socket.connect(addr).await;
        if res.is_ok() {
            socket.start_read_loop();
            socket.start_write_loop();
        }

        // loop {
        //     sleep(std::time::Duration::from_millis(100));
        // }
    }

    #[tokio::test]
    async fn test_socket_clientx() {
        let mut stream = TcpStream::connect("qx-im-8.axxitdins.com:8700")
            .await
            .unwrap();
        println!("created stream success!!!");
        loop {
            let ready = stream
                .ready(Interest::READABLE | Interest::WRITABLE)
                .await
                .unwrap();
            println!("{:?}", ready);
            if ready.is_readable() {
                let mut data = vec![0; 1024];
                // Try to read data, this may still fail with `WouldBlock`
                // if the readiness event is a false positive.
                match stream.try_read(&mut data) {
                    Ok(n) => {
                        println!("read {} bytes", n);
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(e) => {
                        panic!("{}", e);
                    }
                }
            }

            if ready.is_writable() {
                // Try to write data, this may still fail with `WouldBlock`
                // if the readiness event is a false positive.
                match stream.try_write(b"hello world") {
                    Ok(n) => {
                        println!("write {} bytes", n);
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                    Err(e) => {
                        panic!("{}", e);
                    }
                }
            }
        }
    }
}
