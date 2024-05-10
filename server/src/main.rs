mod client_storage;

use futures::future::join_all;
use simple_logger::SimpleLogger;
use client_storage::ClientStorageServer;
use crate::client_storage::ClientStorage;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();

    let mut tasks = Vec::new();

    let tcp_server = ClientStorageServer::new(ClientStorage::restore()).await;
    tasks.push(tokio::spawn(async move {
        tcp_server.run().await
    }));

    join_all(tasks).await;
}