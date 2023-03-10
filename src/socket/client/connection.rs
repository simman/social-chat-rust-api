use crate::socket::client::client::SocketClient;

pub struct Connection {
    client: SocketClient,
}

impl Connection {
    //
    fn new() -> Self {
        Self {
            client: SocketClient::new(),
        }
    }

    //
}
