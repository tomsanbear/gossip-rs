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
use tokio::net::{TcpListener, TcpStream};
use tokio::stream::{Stream, StreamExt};
use tokio::sync::oneshot;
use tokio::task;
use tokio_util::codec::{BytesCodec, Decoder, Encoder, Framed, LinesCodec};
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
    let mut listener = TcpListener::bind(&config.bind_addr()).await?;
    debug!("now listening on {}", config.bind_addr());

    // Event loop for the server
    while let Some(socket) = listener.next().await {
        let socket = socket?;
        debug!("handling connection: {:?}", socket);
        let tx = tx.clone();
        task::spawn(async move {
            if let Err(e) = handle_connection(socket, tx.clone()).await {
                error!("error handling connection: {}", e);
            }
        });
    }

    // Exit
    Ok(())
}

// Async task to handle connections from the rx socket
#[instrument]
async fn handle_connection(
    socket: TcpStream,
    mut inbound_tx: Sender<(oneshot::Sender<Payload>, Payload)>,
) -> Result<()> {
    let mut framed = Framed::new(socket, BytesCodec::new());

    if let Some(bytes_read) = framed.next().await {
        let bytes_read = bytes_read?.to_vec();

        // Create a oneshot channel for internal process to send the response back on
        debug!("creating oneshot for response");
        let (handler_oneshot_tx, handler_oneshot_rx) = oneshot::channel::<Payload>();

        // Send it internally for handling
        debug!("pass to internal gossip loop");
        inbound_tx.send((handler_oneshot_tx, bytes_read)).await?;

        // Wait on the oneshot response, and write it back to the user
        debug!("wait for oneshot response");
        let payload = handler_oneshot_rx.await?;

        // Send the response back
        debug!("write the response to the tcp stream");
        framed.send(Bytes::from(payload)).await?;
    }

    // Done with the connection
    Ok(())
}
