use super::message::{Message, Payload};
use crate::types::{Result, Sender};
use bytes::{BufMut, Bytes, BytesMut};
use derive_new::new;
use futures::SinkExt;
use getset::Getters;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::io::{BufReader, BufWriter};
use tokio::net::{TcpStream, UdpSocket};
use tokio::stream::{Stream, StreamExt};
use tokio::sync::oneshot;
use tokio::task;
use tokio_util::codec::{BytesCodec, Decoder, Encoder, Framed, LinesCodec};
use tokio_util::udp::UdpFramed;
use tracing::{debug, error, info};
use tracing_attributes::instrument;

// ServerConfig: masks values to be passed to internal server configs
#[derive(new, Getters, Debug, Clone, PartialEq)]
pub struct ServerConfig {
    #[getset(get = "pub")]
    bind_addr: SocketAddr,
}

// Start method to initialize the server event loop
#[instrument]
pub async fn start(
    config: ServerConfig,
    tx: Sender<(oneshot::Sender<Payload>, Payload)>,
) -> Result<()> {
    // Transport config
    debug!("listening on {}/udp", config.bind_addr());
    let udp_sock = UdpSocket::bind(&config.bind_addr()).await?;
    let mut udp_framed = UdpFramed::new(udp_sock, BytesCodec::new());

    // Wait on inbound connections
    while let Some(result) = udp_framed.next().await {
        let (bytes, addr) = result?;
        debug!("received {} bytes from {}", bytes.len(), addr);
        task::spawn(async move {
            if let Err(e) = handle_inbound(&bytes).await {
                error!("error while handling inbound bytes: {}", e);
            }
        });
    }
    Ok(())
}

// Simple function that passes received bytes along a channel that will handle business logic
async fn handle_inbound(bytes: &[u8]) -> Result<()> {
    debug!("started handler");
    Ok(())
}
