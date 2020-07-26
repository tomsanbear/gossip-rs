use crate::message::*;
use crate::nodestore::Nodestore;
use crate::transport::message::Payload;
use crate::types::{Receiver, Result};
use anyhow::anyhow;
use derive_new::new;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, error, info};
use tracing_attributes::instrument;

#[derive(new)]
pub struct Configuration {}

pub async fn start(
    config: Configuration,
    nodestore: Arc<Nodestore>,
    mut gossip_rx: Receiver<Payload>,
) -> Result<()> {
    // Receive payload events, that then trigger
    while let Some(payload) = gossip_rx.recv().await {

    }
    Ok(())
}

#[instrument]
async fn handle_payload(node_store: Arc<Nodestore>, payload: Payload) -> Result<()> {
    debug!("entering handle_payload");
    match rmp_serde::from_read(&*payload) {
        Ok(payload) => {
            match payload {
                GossipMessage::Ping(ping) => {
                    debug!("Ping: {:?}", ping);

                    // Pass to the handler
                    debug!("handle ping");
                    let ack = handle_ping(node_store.clone(), ping).await?;

                    // Serialize the ack
                    debug!("serialize ack");
                    let payload = ser::to_vec(&GossipMessage::Ack(ack))?;

                    // Send it
                    debug!("passing ack back on the channel");
                    if let Err(_) = oneshot_tx.send(payload) {
                        return Err(anyhow!("failed response over oneshot"));
                    }
                }
                GossipMessage::JoinReq(join_req) => {
                    debug!("JoinReq: {:?}", join_req);

                    // Pass to the handler
                    let join_resp = handle_join_req(node_store.clone(), join_req).await?;

                    // Serialize the ack
                    let payload = ser::to_vec(&GossipMessage::JoinResp(join_resp))?;

                    // Pass response to the oneshot
                    debug!("passing response back to the server");
                    if let Err(_) = oneshot_tx.send(payload) {
                        return Err(anyhow!("failed response over oneshot"));
                    }
                }
                _ => return Err(anyhow!("invalid message received: {:?}", payload)),
            };
        }
        Err(e) => {
            return Err(anyhow!(
                "failed to deserialize: {}\n{}",
                e,
                std::str::from_utf8(&payload).unwrap_or("[could not parse]")
            ))
        }
    }
    Ok(())
}

// handles business logic upon receiving a Ping, receiving a ping generates an ack event
pub async fn handle_ping(node_store: Arc<Nodestore>, ping: Ping) -> Result<Ack> {
    // Handle Gossip
    handle_gossip(node_store.clone(), ping.nodes().clone()).await?;

    // Create a new Ack
    debug!("creating ack response");
    // Get a lock on the nodestore
    debug!("acquiring lock");
    let lock = &mut *node_store.lock().await;
    let payload = Ack::new(*ping.id(), lock.values().map(|i| *i).collect());

    // Done with the lock
    drop(lock);
    Ok(payload)
}

// handles business logic upon receiving an Ack, receiving an ack does not generate any further events
pub async fn handle_ack(node_store: Arc<Mutex<BTreeMap<usize, Node>>>, ack: Ack) -> Result<()> {
    // Handle Gossip, ack sender node is bundled in this gossip (TODO: split it out?)
    handle_gossip(node_store.clone(), ack.nodes().clone()).await?;
    Ok(())
}

// handles business logic upon receiving an JoinReq, receiving a join req generates a join resp event
pub async fn handle_join_req(
    node_store: Arc<Mutex<BTreeMap<usize, Node>>>,
    join_req: JoinReq,
) -> Result<JoinResp> {
    // Get the lock
    let lock = &mut *node_store.lock().await;

    // Add node to our store
    debug!(
        "inserting source node {} to local nodestore",
        join_req.src().id()
    );
    lock.insert(*join_req.src().id(), join_req.src().clone());

    // Create the join response
    debug!("creating payload");
    let payload = JoinResp::new(lock.values().map(|i| *i).collect());

    // Drop the lock
    drop(lock);

    Ok(payload)
}

// common method for handling gossip
pub async fn handle_gossip(
    node_store: Arc<Mutex<BTreeMap<usize, Node>>>,
    nodes: Vec<Node>,
) -> Result<()> {
    // Get a lock on the nodestore
    debug!("acquiring lock");
    let lock = &mut *node_store.lock().await;
    // Update nodes in our nodestore
    debug!("updating nodes passed via gossip");
    for node in nodes {
        if let Some(i) = lock.get(node.id()) {
            if i.incarnation() < node.incarnation() {
                debug!("updating entry for node: {} -> {}", i, node);
                lock.insert(*node.id(), node);
            }
        } else {
            debug!("node did not exist, inserting node: {}", node);
            lock.insert(*node.id(), node);
        }
    }
    Ok(())
}
