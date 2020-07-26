mod config;
mod gossip;
mod message;
mod node;
mod nodestore;
mod transport;
mod types;

use config::Configuration;
use futures::try_join;
use getset::Getters;
use node::Node;
use nodestore::Nodestore;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::task;
use tracing::error;
use transport::server;

// Main struct to interact with this library
#[derive(Getters)]
pub struct Gossip {
    #[getset(get = "pub")]
    config: Configuration,
    #[getset(get = "pub")]
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
 
        // 'main' task 
        { 
            let gossip_internal = gossip.clone();
            task::spawn(async move {
                initialize(gossip_internal).await;
            }); 
        } 

        // Pass back self within an Arc, TODO: this could be split between two methods in the future...
        return gossip;
    }

    // Gets a vector of all member nodes
    pub async fn members(&self) -> Vec<Node> {
        let node_store = &*self.node_store;
        node_store.get_all().await
    }
}

async fn initialize(gossip: Arc<Gossip>) {
    let gossip = &*gossip;

    // Create assorted channels
    let (gossip_inbound_tx, gossip_inbound_rx) = mpsc::channel(*gossip.config().buff_size());

    // Gossip Event Loop: Accepts and handles state changes
    let gossip_config = gossip::Configuration::new();
    let gossip_fut = gossip::start(gossip_config, gossip.node_store.clone(), gossip_inbound_rx);

    // Server Event Loop: Handles inbound network requests
    let server_config = server::ServerConfig::new(*gossip.config().bind_addr());
    let server_fut = server::start(server_config, gossip_inbound_tx);

    // Join all futures, any of them finishing indicates end of program
    // TODO: replace with select!
    if let Err(e) = try_join!(gossip_fut, server_fut) {
        error!("exiting with error: {}", e);
    }
}
