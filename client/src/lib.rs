pub mod user;
pub mod server_endpoint;
mod server_connection;
mod util;

#[macro_use]
extern crate log;

pub struct DistronetClient {
    server_connection: ServerConnection,
    user: DistronetUser,
}

impl DistronetClient {
    pub fn new_connection(user: DistronetUser) -> Result<DistronetClient> {
        for &endpoint in KNOWN_ENDPOINTS {
            match ServerConnection::new(endpoint, &user) {
                Some(con) => {
                    info!("Successfully authorized on server!");
                    return Ok(DistronetClient {
                        server_connection: con,
                        user,
                    });
                }
                None => {
                    warn!("Failed to connect to server.");
                }
            }
        }
        error!("No servers available!");

        Err(NoServersAvailable)
    }

    // Send message to connected server
    pub fn send(&mut self, data: &str) -> Result<()> {
        info!("Sending data: {}", data);


        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    NoServersAvailable,
    UserBlocked,
    UserNotFound,
}
use Error::*;
use crate::server_connection::ServerConnection;
use crate::server_endpoint::KNOWN_ENDPOINTS;
use crate::user::DistronetUser;


pub type Result<T> = std::result::Result<T, Error>;