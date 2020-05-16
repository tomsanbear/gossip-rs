use derive_new::new;
use getset::Getters;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(new, Getters, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Configuration {
    #[getset(get = "pub")]
    id: usize,
    #[getset(get = "pub")]
    bind_addr: SocketAddr,
    #[getset(get = "pub")]
    advertised_addr: SocketAddr,
    #[getset(get = "pub")]
    buff_size: usize,
    #[getset(get = "pub")]
    seed_nodes: Vec<SocketAddr>,
}
