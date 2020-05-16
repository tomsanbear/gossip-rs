use getset::Getters;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::net::SocketAddr;

#[derive(Serialize, Deserialize, Debug, Hash, Copy, Clone, Eq, PartialEq)]
pub enum State {
    ALIVE,
    DEAD,
}

#[derive(Getters, Serialize, Deserialize, Debug, Hash, Copy, Clone, Eq, PartialEq)]
pub struct Node {
    #[getset(get = "pub")]
    id: usize,
    #[getset(get = "pub")]
    addr: SocketAddr,
    #[getset(get = "pub")]
    state: State,
    #[getset(get = "pub")]
    incarnation: usize,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{id = {}, incarnation = {}}}",
            self.id, self.incarnation
        )
    }
}

impl Node {
    pub fn new(id: usize, addr: SocketAddr) -> Self {
        Node {
            id: id,
            addr: addr,
            state: State::ALIVE,
            incarnation: 0,
        }
    }

    pub fn inc(&mut self) {
        self.incarnation += 1;
    }
}
