use std::net::Ipv4Addr;

#[derive(Debug, Copy, Clone)]
pub struct ServerEndpoint {
    pub ip: Ipv4Addr,
    pub port: u16,
    pub pubkey: &'static str,
}

pub static KNOWN_ENDPOINTS: &'static [ServerEndpoint] = &[
    ServerEndpoint::new([185,104,249,231]),
    ServerEndpoint::new([127, 0, 0, 1]),
];

impl ServerEndpoint {
    pub const fn new(octets: [u8; 4]) -> Self {
        Self {
            ip: Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]),
            port: 12345,
            pubkey: "",
        }
    }
}