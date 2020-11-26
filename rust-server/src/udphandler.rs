use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use bytes::BytesMut;
use prost::Message;
use tokio::net::UdpSocket;
use tokio::stream::StreamExt;
use tokio::sync::mpsc::Receiver;
use tokio_util::codec::{Decoder, Encoder};
use tracing::{event, Instrument, Level, span};
use uuid::Uuid;
use crate::controlserver::ServerState;
use crate::protos::hanmessage::StreamHeader;
use crate::protos::udpmessage::{HanUdpMessage, PingPacket, han_udp_message::Msg};
use crate::util::Error;

#[allow(dead_code)]
pub enum InternalUdpMsg {
    DISCONNECT,
    SENDPACKAGE(Arc<HanUdpMessage>, Vec<SocketAddr>),
}

pub struct UdpMessageParser {}


pub async fn udp_client_write(_state: Arc<Mutex<ServerState>>, socket: Arc<UdpSocket>, mut receiver: Receiver<InternalUdpMsg>) {
    loop {
        match receiver.next().await {
            None => { return; }
            Some(InternalUdpMsg::SENDPACKAGE(msg, targets)) => {
                let mut buf = BytesMut::new();
                let size = msg.encoded_len();
                StreamHeader {
                    magic: 0x0008a71,
                    length: size as u32,
                }.encode(&mut buf).expect("Message serializer broken");
                msg.encode(&mut buf).expect("Message serializer broken");
                for addr in targets {
                    match socket.send_to(buf.as_ref(), addr).await {
                        Err(e) => event!(Level::INFO, "Unable to send to {:?} - {}", &addr, &e),
                        Ok(size) => event!(Level::TRACE, "Sent packet of zize {} to {:?}", size, &addr)
                    }
                }
            }
            Some(InternalUdpMsg::DISCONNECT) => { return; }
        };
    }
}

pub async fn udp_client_read(state: Arc<Mutex<ServerState>>, socket: Arc<UdpSocket>) {
    let mut buf = BytesMut::with_capacity(8152);
    loop {
        buf.resize(8152, 0);
        match socket.recv_from(buf.as_mut()).await {
            Err(error) => {
                event!(Level::ERROR, "Error with UDP socket: {}", &error);
            }
            Ok((size, addr)) => {
                buf.resize(size, 0);
                let message: HanUdpMessage = parse_msg(&buf).unwrap();
                let user_id = Uuid::from_slice(&message.connection_id.as_slice()).unwrap();
                process_udp_message(&addr, &state, Arc::new(message))
                    .instrument(span!(Level::ERROR, "Connection", "{}", &user_id))
                    .await;
            }
        };
    }
}

async fn process_udp_message(addr: &SocketAddr,
                             state_mutex: &Arc<Mutex<ServerState>>,
                             message: Arc<HanUdpMessage>) {
    let user_id = Uuid::from_slice(message.connection_id.as_slice()).unwrap();
    // Each package registers the remote for now...
    match &message.msg {
        Some(Msg::AudioFrame(_)) => handle_audio_frame(state_mutex, &user_id, message).await,
        Some(Msg::PingPacket(_)) => handle_ping(addr, state_mutex, &user_id).await,
        _ => event!(Level::WARN, "Dropping unknown packet")
    }
}

pub fn parse_msg(bytes: &BytesMut) -> Result<HanUdpMessage, Error> {
    if bytes.len() < 10 {
        return Err(Error::ProtocolError("Udp Packet to small for Header.".to_string()));
    }
    let header = StreamHeader::decode(&bytes.as_ref()[0..10])?;
    if header.magic != 0x0008a71 || header.length as usize != bytes.len() - 10 {
        return Err(Error::ProtocolError("MAGIC is gone !".to_string()));
    }
    Ok(HanUdpMessage::decode(&bytes.as_ref()[10..])?)
}

/// Message Handler

async fn handle_ping(addr: &SocketAddr, state_mutex: &Arc<Mutex<ServerState>>, user_id: &Uuid) {
    event!(Level::INFO, "Got Ping Packet from: {}", &user_id);
    let state = state_mutex.lock().unwrap();
    if let Some(user_mutex) = state.clients.get(&user_id) {
        let mut user = user_mutex.lock().unwrap();
        user.udp_socket = Some(addr.clone());
        event!(Level::INFO, "Registering {:?} for {}({})", &user.udp_socket, &user.username, &user.uuid);
        let sender = state.udp_sender.as_ref().unwrap().clone();
        let addr = addr.clone();
        let user_id = user_id.clone();
        event!(Level::INFO, "Spawn!");
        tokio::spawn(async move {
            event!(Level::WARN, "Ping reply.");
            if let Err(e) =
            sender.send(InternalUdpMsg::SENDPACKAGE(Arc::new(HanUdpMessage {
                connection_id: Vec::from(&user_id.as_bytes()[..]),
                msg: Some(Msg::PingPacket(PingPacket {})),
            }), vec![addr])).await {
                event!(Level::ERROR, "Internal send Failed: {}", e);
            }
        });
    }
}

async fn handle_audio_frame(state_mutex: &Arc<Mutex<ServerState>>,
                            user_id: &Uuid,
                            message: Arc<HanUdpMessage>) {
    let state = state_mutex.lock().unwrap();
    if let Some(channel) = state.channels.values()
        .filter(|c| c.users.contains(user_id))
        .nth(0) {
        let msg = Arc::clone(&message);
        let targets: Vec<SocketAddr> =
            channel.users.iter()
                .filter(|u| u != &user_id)
                .map(|u| state.clients.get(u))
                .filter(|o| o.is_some())
                .map(|o| {
                    o.unwrap().lock().unwrap().udp_socket
                })
                .filter(|o| o.is_some())
                .map(|o| o.unwrap())
                .collect();
        let cloned_sender = state.udp_sender.as_ref().unwrap().clone();
        tokio::spawn(async move {
            event!(Level::WARN, "Forwarding Audio to {} users.", targets.len());
            if let Err(e) = cloned_sender.send(InternalUdpMsg::SENDPACKAGE(msg, targets)).await {
                event!(Level::ERROR, "Internal send Failed: {}", e);
            }
        });
    } else {
        event!(Level::WARN, "Channel for Packet not found.");
    }
}


/////////////////////
/// Helper Decoer ///
/////////////////////

impl Decoder for UdpMessageParser {
    type Item = HanUdpMessage;
    type Error = Error;

    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> Result<Option<HanUdpMessage>, Self::Error> {
        let length = src.len();
        let result = StreamHeader::decode(&src.as_ref()[0..10])?;
        if result.magic != 0x0008a71 || result.length as usize != length - 10 {
            return Err(Error::ProtocolError("Header Missing".to_string()));
        }
        Ok(Some(HanUdpMessage::decode(&src.as_ref()[10..result.length as usize])?))
    }
}

impl Encoder<HanUdpMessage> for UdpMessageParser {
    type Error = Error;

    fn encode(&mut self, message: HanUdpMessage, dst: &mut BytesMut) -> Result<(), Error> {
        StreamHeader {
            magic: 0x0008a71,
            length: message.encoded_len() as u32,
        }.encode(dst).expect("Encoding FAIL !");
        message.encode(dst).expect("Encoder broken");
        Ok(())
    }
}