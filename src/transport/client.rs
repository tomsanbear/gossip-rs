use super::message::{Message, Payload};
use crate::types::Result;
use bytes::Bytes;
use derive_new::new;
use futures::SinkExt;
use getset::Getters;
use tokio::net::UdpSocket;
use tokio::stream::StreamExt;
use tokio_util::codec::BytesCodec;
use tokio_util::udp::UdpFramed;
use tracing::debug;
use tracing_attributes::instrument;

// ClientConfig: masks values to be passed to internal client config
#[derive(new, Getters, Debug, Clone, PartialEq)]
pub struct ClientConfig {}

// Sends the message, message response
#[instrument]
pub async fn dispatch(msg: Message) -> Result<()> {
    debug!("dispatching: {}", msg);
    let socket = UdpSocket::bind("0.0.0.0:0".to_string()).await?;
    let mut socket_framed = UdpFramed::new(socket, BytesCodec::new());

    // Write the message to the tcp_stream'
    debug!("writing to the stream");
    socket_framed
        .send((Bytes::from(msg.payload().clone()), *msg.addr()))
        .await?;

    debug!("done with the client");
    Ok(())
}
