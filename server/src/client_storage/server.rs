use std::fs::File;
use std::io::BufReader;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use rustls::internal::msgs::codec::Codec;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::server::{Acceptor, ClientHello};
use rustls::ServerConfig;
use rustls_pemfile::{certs, rsa_private_keys};
use tokio::{fs, io};
use tokio::io::{AsyncWriteExt, copy, sink};
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
use crate::client_storage::ClientStorage;

pub struct ClientStorageServer {
    client_storage: ClientStorage,
    tcp_listener: TcpListener,
    acceptor: TlsAcceptor
}
fn load_certs(path: &Path) -> io::Result<Vec<CertificateDer<'static>>> {
    certs(&mut BufReader::new(File::open(path)?)).collect()
}

fn load_keys(path: &Path) -> io::Result<PrivateKeyDer<'static>> {
    rsa_private_keys(&mut BufReader::new(File::open(path)?))
        .next()
        .unwrap()
        .map(Into::into)
}

async fn process_client(stream: tokio::net::TcpStream, addr: SocketAddr, acceptor: TlsAcceptor) -> Result<(), std::io::Error> {
    let mut stream = acceptor.accept(stream).await?;

    let mut output = sink();
    stream
        .write_all(
            &b"Hello dear client, please provide your credentials (and your soul)\n"[..],
        )
        .await?;
    stream.shutdown().await?;
    copy(&mut stream, &mut output).await?;
    println!("Hello: {}", addr);

    Ok(())

}

impl ClientStorageServer {
    pub async fn new(client_storage: ClientStorage) -> Result<ClientStorageServer, std::io::Error> {
        let addr = "0.0.0.0:12345";
        let tcp_listener = TcpListener::bind(addr).await.unwrap();

        let certs = load_certs(Path::new("certs.der"))?;
        let key = load_keys(Path::new("key.der"))?;

        // For some user-defined choose_server_config:
        let config = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, key)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;
        let acceptor = TlsAcceptor::from(Arc::new(config));

        Ok(ClientStorageServer {
            client_storage,
            tcp_listener,
            acceptor
        })
    }

    pub async fn run(&self) {
        info!("TcpServer is running!");
        loop {
            match self.tcp_listener.accept().await {
                Ok((stream, addr)) => {
                    let acceptor = self.acceptor.clone();
                    tokio::spawn(process_client(stream, addr, acceptor)).await.unwrap();
                }
                Err(e) => {
                    warn!("Error: {:?}", e);
                }
            }
        }
    }
}