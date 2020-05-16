use super::message::Payload;
use crate::types::{Result, Sender};

use derive_new::new;

use getset::Getters;
use std::net::SocketAddr;

use tokio::net::UdpSocket;
use tokio::stream::StreamExt;
use tokio::sync::oneshot;
use tokio::task;
use tokio_util::codec::BytesCodec;
use tokio_util::udp::UdpFramed;
use tracing::{debug, error};
use tracing_attributes::instrument;

// ServerConfig: masks values to be passed to internal server configs
#[derive(new, Getters, Debug, Clone, PartialEq)]
pub struct ServerConfig {
    #[getset(get = "pub")]
    bind_addr: SocketAddr,
}

// Start method to initialize the server event loop
#[instrument]
pub async fn start(config: ServerConfig, tx: Sender<Payload>) -> Result<()> {
    // Transport config
    debug!("listening on {}/udp", config.bind_addr());
    let udp_sock = UdpSocket::bind(&config.bind_addr()).await?;
    let mut udp_framed = UdpFramed::new(udp_sock, BytesCodec::new());

    // Wait on inbound connections
    while let Some(result) = udp_framed.next().await {
        let (bytes, addr) = result?;
        let tx = tx.clone();
        debug!("received {} bytes from {}", bytes.len(), addr);
        task::spawn(async move {
            if let Err(e) = handle_inbound(&bytes, tx.clone()).await {
                error!("error while handling inbound bytes: {}", e);
            }
        });
    }
    Ok(())
}

// Simple function that passes received bytes along a channel that will handle business logic
async fn handle_inbound(bytes: &[u8], mut tx: Sender<Payload>) -> Result<()> {
    debug!("started handler");
    tx.send(bytes.to_vec()).await?;
    Ok(())
}
