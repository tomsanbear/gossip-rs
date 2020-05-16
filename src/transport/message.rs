use derive_new::new;
use getset::Getters;
use std::fmt;
use std::net::SocketAddr;

pub type Payload = Vec<u8>;

#[derive(new, Getters, Debug, Clone, PartialEq)]
pub struct Message {
    #[getset(get = "pub")]
    addr: SocketAddr,
    #[getset(get = "pub")]
    payload: Payload,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} bytes to {}", self.payload.len(), self.addr)
    }
}
