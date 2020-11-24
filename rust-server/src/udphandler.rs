use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use bytes::{Buf, BytesMut};
use quick_protobuf::{BytesReader, MessageWrite, Writer};
use tokio::net::UdpSocket;
use tokio::stream::StreamExt;
use tokio::sync::mpsc::Receiver;
use tokio_util::codec::{Decoder, Encoder};
use tracing::{event, Instrument, Level, span};
use uuid::Uuid;

use crate::controlserver::ServerState;
use crate::protos::hanmessage::StreamHeader;
use crate::protos::updmessage::{HanUdpMessage, PingPacket};
use crate::protos::updmessage::mod_HanUdpMessage::OneOfmsg;
use crate::util::{ByteMutWrite, Error};

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
                let mut writer = Writer::new(ByteMutWrite { delegate: &mut buf });
                let size = msg.get_size();
                StreamHeader {
                    magic: 0x0008a71,
                    length: size as u32,
                }.write_message(&mut writer).expect("Message serializer broken");
                msg.write_message(&mut writer).expect("Message serializer broken");
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
    let mut buf = vec![0 as u8; 8152];
    loop {
        buf.resize(8152, 0);
        match socket.recv_from(buf.as_mut_slice()).await {
            Err(error) => {
                event!(Level::ERROR, "Error with UDP socket: {}", &error);
            }
            Ok((size, addr)) => {
                buf.resize(size, 0);
                let message: HanUdpMessage = parser_msg(&buf).unwrap();
                let user_id = Uuid::from_slice(&message.user_id.as_slice()).unwrap();
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
    let user_id = Uuid::from_slice(message.user_id.as_slice()).unwrap();
    // Each package registers the remote for now...
    match &message.msg {
        OneOfmsg::audio_frame(_) => handle_audio_frame(state_mutex, &user_id, message).await,
        OneOfmsg::ping_packet(_) => handle_ping(addr, state_mutex, &user_id).await,
        _ => event!(Level::WARN, "Dropping unknown packet")
    }
}

fn parser_msg(bytes: &Vec<u8>) -> Result<HanUdpMessage, Error> {
    if bytes.len() < 10 {
        return Err(Error::ProtocolError("Udp Packet to small for Header.".to_string()));
    }
    let mut reader = BytesReader::from_bytes(bytes);
    let header = match reader.read_message_by_len::<StreamHeader>(bytes, 10) {
        Err(e) => return Err(Error::ProtoBufError(e)),
        Ok(val) => val
    };
    if header.magic != 0x0008a71 || header.length as usize != bytes.len() - 10 {
        return Err(Error::ProtocolError("MAGIC is gone !".to_string()));
    }
    match reader.read_message_by_len::<HanUdpMessage>(bytes, header.length as usize) {
        Err(e) => Err(Error::ProtoBufError(e)),
        Ok(val) => Ok(val)
    }
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
                user_id: Vec::from(&user_id.as_bytes()[..]),
                msg: OneOfmsg::ping_packet(PingPacket {}),
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
        let mut reader = BytesReader::from_bytes(src.bytes());
        let result = reader.read_message_by_len::<StreamHeader>(src.bytes(), 10).unwrap();
        if result.magic != 0x0008a71 || result.length as usize != length - 10 {
            return Err(Error::ProtocolError("Header Missing".to_string()));
        }
        let result = reader.read_message_by_len(src.bytes(), result.length as usize);
        match result {
            Ok(msg) => Ok(Some(msg)),
            Err(e) => Err(Error::from(e))
        }
    }
}

impl Encoder<HanUdpMessage> for UdpMessageParser {
    type Error = quick_protobuf::Error;

    fn encode(&mut self, message: HanUdpMessage, dst: &mut BytesMut) -> Result<(), quick_protobuf::Error> {
        let mut writer = Writer::new(ByteMutWrite { delegate: dst });
        StreamHeader {
            magic: 0x0008a71,
            length: message.get_size() as u32,
        }.write_message(&mut writer).expect("Encoding FAIL !");
        message.write_message(&mut writer)
    }
}

