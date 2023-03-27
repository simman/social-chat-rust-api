use crate::socket::packs::codec;
use anyhow::Result;
use bytes::{Buf, BufMut, BytesMut};
use image::EncodableLayout;
use log::{debug, error, info};
use parking_lot::Mutex;
use std::collections::VecDeque;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::time::sleep;

pub struct SocketClient {
    pub messages_in: Arc<Mutex<BytesMut>>,
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
                            return;
                        }
                        Ok(n) => n,
                        Err(e) => {
                            eprintln!("[Connection] failed to read from socket; err = {:?}", e);
                            match e.kind() {
                                std::io::ErrorKind::BrokenPipe => {
                                    *is_connected.lock() = false;
                                }
                                std::io::ErrorKind::ConnectionReset => {
                                    *is_connected.lock() = false;
                                }
                                _ => {
                                    unimplemented!()
                                }
                            }
                            return;
                        }
                    };
                    debug!("读取到的数据长度：{:?}", byte_count);
                    let b = &buf[0..byte_count];
                    messages_in.lock().put_slice(b);
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
                    if !*is_connected.lock() {
                        eprintln!("[Connection] connection closed but still alive?");
                        return;
                    }

                    if messages_out.lock().len() > 0 {
                        let next = {
                            let mut write = messages_out.lock();
                            write.pop_front()
                        };
                        if let Some(msg) = next {
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
                                    std::io::ErrorKind::ConnectionReset => {
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
        *self.is_connected.lock() = true;
        debug!("===> disconnect");
        debug!("===> disconnect write_stream");
        debug!("{:?}", self.write_stream);
        // let x = self.write_stream.clone();
    }

    pub fn is_connected(&self) -> bool {
        *self.is_connected.lock()
    }

    pub fn send(&mut self, msg: Vec<u8>) {
        debug!("===> send: {:?}", msg);
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

    #[test]
    fn test_xx() {
        // tokio::Future::
    }
}
