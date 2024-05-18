use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Write};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use log::info;
use rustls::pki_types;
use rustls::pki_types::IpAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_rustls::{Connect, TlsConnector};
use tokio_rustls::client::TlsStream;
use crate::server_endpoint::ServerEndpoint;
use crate::user::DistronetUser;

pub struct ServerConnection {
    pub endpoint: ServerEndpoint,
    pub tcp_stream: TlsStream<TcpStream>
}

impl ServerConnection {
    pub async fn new(server_endpoint: ServerEndpoint, user: &DistronetUser) -> Option<Self> {
        // Connect to server
        match TcpStream::connect((server_endpoint.ip, server_endpoint.port)).await {
            Ok(stream) => {
                debug!("Tcp connection to server: {:?}:{}", server_endpoint.ip, server_endpoint.port);

                let mut root_cert_store = rustls::RootCertStore::empty();
                let mut pem = BufReader::new(File::open("cert.pem").unwrap());
                for cert in rustls_pemfile::certs(&mut pem) {
                    root_cert_store.add(cert.unwrap()).unwrap();
                }

                let config = rustls::ClientConfig::builder()
                    .with_root_certificates(root_cert_store)
                    .with_no_client_auth(); // i guess this was previously the default?
                let connector = TlsConnector::from(Arc::new(config));

                let domain = pki_types::ServerName::IpAddress(IpAddr::try_from("127.0.0.1").unwrap());

                let stream = match connector.connect(domain, stream).await {
                    Ok(stream) => stream,
                    Err(e) => {
                        error!("{:?}", e);
                        return None;
                    }
                };

                // C2S connection
                // 1.1) Send user's keychain + nonce
                // 1.2) sign with auth key
                // let keychain = user.keychain();
                // let nonce = distronet::assym_crypto::gen_nonce();



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

    pub async fn handle_transaction(&mut self, data: String) -> String {
        // 1. send length
        let data_bytes = data.as_bytes();
        let len = data_bytes.len() as u32;
        self.tcp_stream.write_all(&len.to_be_bytes()).await.unwrap();

        // 2. send data
        self.tcp_stream.write_all(data_bytes).await.unwrap();

        // 3. read response
        let mut len_buf = [0u8; 4];
        self.tcp_stream.read_exact(&mut len_buf).await.unwrap();
        let len = u32::from_be_bytes(len_buf);
        info!("read incoming data length: {}", len);

        let mut buf = vec![0u8; len as usize];
        self.tcp_stream.read_exact(&mut buf).await.unwrap();

        String::from_utf8(buf).unwrap()
    }

    pub async fn finalize(mut self) {
        self.tcp_stream.shutdown().await.unwrap()
    }
}