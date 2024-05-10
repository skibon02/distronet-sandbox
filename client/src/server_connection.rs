use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;
use log::info;
use crate::server_endpoint::ServerEndpoint;
use crate::user::DistronetUser;

pub struct ServerConnection {
    pub endpoint: ServerEndpoint,
    pub tcp_stream: TcpStream
}

impl ServerConnection {
    pub fn new(server_endpoint: ServerEndpoint, user: &DistronetUser) -> Option<Self> {
        // Connect to server
        match TcpStream::connect((server_endpoint.ip, server_endpoint.port)) {
            Ok(mut stream) => {
                debug!("Tcp connection to server: {:?}:{}", server_endpoint.ip, server_endpoint.port);

                stream.set_read_timeout(Some(Duration::from_millis(3000))).ok()?;

                // C2S connection
                // 1.1) Send user's keychain + nonce
                // 1.2) sign with auth key
                let keychain = user.keychain();
                let nonce = distronet::assym_crypto::gen_nonce();



                Some(ServerConnection {
                    endpoint: server_endpoint,
                    tcp_stream: stream
                })
            },
            Err(e) => {
                return None;
            }
        }
    }
}