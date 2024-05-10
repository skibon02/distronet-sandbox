mod server;

use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
pub use server::ClientStorageServer;

#[derive(Serialize, Deserialize, Debug)]
pub struct Client {
    ip: String,
    port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientStorage {
    clients: BTreeMap<String, Client>
}

impl ClientStorage {
    fn new_empty() -> Self {
        let res = ClientStorage {
            clients: BTreeMap::new()
        };
        res.save();
        res
    }
    pub fn restore() -> ClientStorage {
        let filename = "clients.bin";
        match std::fs::read(filename) {
            Ok(data) => {
                match bincode::deserialize::<ClientStorage>(&data) {
                    Ok(clients_storage) => {
                        info!("Restored clients: {:?}", clients_storage.clients.len());
                        clients_storage
                    }
                    Err(e) => {
                        error!("Failed to deserialize data: {:?}", e);
                        Self::new_empty()
                    }
                }
            }
            Err(_) => {
                error!("Failed to read file");
                Self::new_empty()
            }
        }
    }

    pub fn save(&self) {
        let filename = "clients.bin";
        match bincode::serialize(&self) {
            Ok(data) => {
                match std::fs::write(filename, data) {
                    Ok(_) => {
                        info!("Saved clients: {:?}", self.clients.len());
                    }
                    Err(e) => {
                        error!("Failed to write data: {:?}", e);
                    }
                }
            }
            Err(e) => {
                error!("Failed to serialize data: {:?}", e);
            }
        }
    }
}