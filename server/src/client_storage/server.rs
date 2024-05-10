use std::net::SocketAddr;
use std::sync::Arc;
use rustls::internal::msgs::codec::Codec;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::server::{ClientHello};
use rustls::ServerConfig;
use tokio::fs;
use tokio::io::{AsyncWriteExt, BufReader};
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use crate::client_storage::ClientStorage;

pub struct ClientStorageServer {
    client_storage: ClientStorage,
    tcp_listener: TcpListener,
}

async fn process_client(mut stream: tokio::net::TcpStream, addr: SocketAddr) -> Result<(), std::io::Error> {
    let mut acceptor = Acceptor::default();
    let accepted = loop {
        acceptor.read_tls(&mut stream).unwrap();
        if let Some(accepted) = acceptor.accept().unwrap() {
            break accepted;
        }
    };

    // For some user-defined choose_server_config:
    let config = choose_server_config(accepted.client_hello());
    let mut conn = accepted
        .into_connection(config)
        .unwrap();

    loop {
        if conn.wants_read() {
            match conn.read_tls(&mut stream) {
                Ok(0) => {
                    info!("EOF");
                    break;
                }
                Ok(_) => {
                    if let Err(e) = conn.process_new_packets() {
                        return e.into();
                    }
                    let mut plaintext = Vec::new();
                    conn.reader().read_to_end(&mut plaintext).await?;
                    info!("Received data: {:?}", plaintext);
                }
                Err(e) => {
                    error!("Failed to read from stream: {:?}", e);
                    return Err(e);
                }
            }
        }
        if conn.wants_write() {
            conn.write_tls(&mut stream)?;
        }
    }

    Ok(())

}

async fn choose_server_config(_client_hello: ClientHello) -> Arc<ServerConfig> {
    let cert_chain = vec![
        CertificateDer::read_bytes(&fs::read("cert.der").await.unwrap()).unwrap()
    ];
    let priv_key = PrivateKeyDer::try_from(fs::read("key.der").await.unwrap()).unwrap();
    Arc::new(ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, priv_key).unwrap()
    )
}

impl ClientStorageServer {
    pub async fn new(client_storage: ClientStorage) -> ClientStorageServer {
        let addr = "0.0.0.0:12345";
        let tcp_listener = TcpListener::bind(addr).await.unwrap();
        ClientStorageServer {
            client_storage,
            tcp_listener
        }
    }

    pub async fn run(&self) {
        info!("TcpServer is running!");
        loop {
            match self.tcp_listener.accept().await {
                Ok((stream, addr)) => {
                    tokio::spawn(process_client(stream, addr)).await.unwrap();
                }
                Err(e) => {
                    warn!("Error: {:?}", e);
                }
            }
        }
    }
}