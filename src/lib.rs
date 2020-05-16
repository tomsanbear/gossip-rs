mod config;
mod node;
mod nodestore;
mod transport;
mod types;

use config::Configuration;
use nodestore::Nodestore;
use std::sync::Arc;
use tokio::task;

// Main struct to interact with this library
struct Gossip {
    config: Configuration,
    node_store: Arc<Nodestore>,
}

impl Gossip {
    // Provides a clean local initialization of the library, spins up required event loops
    pub fn create(config: Configuration) -> Arc<Gossip> {
        // self node
        let self_node = node::Node::new(*config.id(), *config.advertised_addr());

        // initialize the internal node store
        let node_store = Nodestore::new(self_node);

        // this
        let gossip = Arc::new(Gossip {
            config: config,
            node_store: Arc::new(node_store),
        });

        // 'main' thread/task/whatever...
        let gossip_internal = gossip.clone();
        task::spawn(async move {
            initialize(gossip_internal).await;
        });
        return gossip;
    }
}

async fn initialize(gossip: Arc<Gossip>) {}
