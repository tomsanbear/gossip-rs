use crate::node::Node;
use derive_new::new;
use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(new, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum GossipMessage {
    /// Join Message
    /// Signifies that the sender is a new node and is requesting a full state transfer to the receiver
    JoinReq(JoinReq),
    /// Join Message
    /// Signifies that the sender is a new node and is requesting a full state transfer to the receiver
    JoinResp(JoinResp),
    /// Ping Message
    /// Basic message targeted directly to another node to check health, expects an ack
    Ping(Ping),
    /// Ack Message
    /// Basic message targeted to the sender of a Ping, indicating health of a node
    Ack(Ack),
    /// Indirect Ping Message
    /// Enhanced flow that asks another node to ping the target node on behalf of the sender
    IndirectPing(IndirectPing),
    /// Indirect Ack Message
    /// Proxy node responds back that if can contact the node
    IndirectAct(IndirectAck),
    /// Indirect Nack Message
    /// Proxy node responds back that it too cannot contact the node
    IndirectNack(IndirectNack),
}

#[derive(new, Getters, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JoinReq {
    // This node
    #[getset(get = "pub")]
    src: Node,
}

#[derive(new, Getters, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JoinResp {
    #[getset(get = "pub")]
    nodes: Vec<Node>,
}

#[derive(new, Getters, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Ping {
    #[getset(get = "pub")]
    id: usize,
    #[getset(get = "pub")]
    nodes: Vec<Node>,
}

#[derive(new, Getters, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Ack {
    #[getset(get = "pub")]
    id: usize,
    #[getset(get = "pub")]
    nodes: Vec<Node>,
}

#[derive(new, Getters, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct IndirectPing {
    #[getset(get = "pub")]
    id: usize,
    #[getset(get = "pub")]
    target: Node,
    #[getset(get = "pub")]
    nodes: Vec<Node>,
}

#[derive(new, Getters, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct IndirectAck {
    #[getset(get = "pub")]
    id: usize,
    #[getset(get = "pub")]
    nodes: Vec<Node>,
}

#[derive(new, Getters, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct IndirectNack {
    #[getset(get = "pub")]
    id: usize,
    #[getset(get = "pub")]
    nodes: Vec<Node>,
}
