use crate::controlserver::ServerState;
use crate::protos::updmessage::HanUdpMessage;
use crate::protos::updmessage::mod_HanUdpMessage::OneOfmsg;
use crate::util::{ByteMutWrite, Error};

use bytes::{BytesMut, Buf};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::UdpSocket;
use tokio::stream::StreamExt;
use tokio::sync::mpsc::{Sender, Receiver};
use tokio_util::codec::{Decoder, Encoder};
use quick_protobuf::{BytesReader, MessageWrite, BytesWriter, Writer};
use tracing::{Level, Instrument, span, event};
use uuid::Uuid;
use crate::protos::hanmessage::StreamHeader;


#[allow(dead_code)]
pub enum InternalUdpMsg {
    DISCONNECT,
    SENDVOICE(Arc<HanUdpMessage>, Vec<SocketAddr>),
}

pub struct UdpMessageParser {}


pub async fn udp_client_read(state: Arc<Mutex<ServerState>>, socket: Arc<UdpSocket>, sender: Sender<InternalUdpMsg>) {
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
                process_udp_message(&addr, &state, Arc::new(message), &sender)
                    .instrument(span!(Level::ERROR, "Connection", "{}", &user_id))
                    .await;
            }
        };
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


async fn process_udp_message(addr: &SocketAddr,
                             state_mutex: &Arc<Mutex<ServerState>>,
                             message: Arc<HanUdpMessage>,
                             sender: &Sender<InternalUdpMsg>) {
    let user_id = Uuid::from_slice(message.user_id.as_slice()).unwrap();
    // Each package registers the remote for now...
    match &message.msg {
        OneOfmsg::audio_frame(_) => handle_audio_frame(state_mutex, &user_id, message, sender).await,
        OneOfmsg::ping_packet(_) => handle_ping(addr, state_mutex, &user_id).await,
        _ => event!(Level::WARN, "Dropping unknown packet")
    }
}

async fn handle_ping(addr: &SocketAddr, state_mutex: &Arc<Mutex<ServerState>>, user_id: &Uuid) {
    event!(Level::INFO, "Got Ping Packet from: {}", &user_id);
    register_udp_socket(addr, state_mutex, &user_id);
}

async fn handle_audio_frame(state_mutex: &Arc<Mutex<ServerState>>,
                            user_id: &Uuid,
                            message: Arc<HanUdpMessage>,
                            sender: &Sender<InternalUdpMsg>) {
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
        let cloned_sender = sender.clone();
        tokio::spawn(async move {
            event!(Level::WARN, "Forwarding Audio to {} users.", targets.len());
            if let Err(e) = cloned_sender.send(InternalUdpMsg::SENDVOICE(msg, targets)).await {
                event!(Level::ERROR, "Internal send Failed: {}", e);
            }
        });
    } else {
        event!(Level::WARN, "Channel for Packet not found.");
    }
}


fn register_udp_socket(addr: &SocketAddr, state_mutex: &Arc<Mutex<ServerState>>, user_id: &Uuid) {
    let state = state_mutex.lock().unwrap();
    if let Some(user_mutex) = state.clients.get(&user_id) {
        let mut user = user_mutex.lock().unwrap();
        user.udp_socket = Some(addr.clone());
        event!(Level::INFO, "Registering {:?} for {}({})", &user.udp_socket, &user.username, &user.uuid);
    }
}

pub async fn udp_client_write(_state: Arc<Mutex<ServerState>>, socket: Arc<UdpSocket>, mut receiver: Receiver<InternalUdpMsg>) {
    let mut buf = Vec::new();
    loop {
        match receiver.next().await {
            None => { return; }
            Some(InternalUdpMsg::SENDVOICE(msg, targets)) => {
                let size = msg.get_size();
                buf.resize(size, 0);
                let mut writer = Writer::new(BytesWriter::new(buf.as_mut_slice()));
                msg.write_message(&mut writer).expect("Message serializer broken");
                for addr in targets {
                    match socket.send_to(buf.as_slice(), addr).await {
                        Err(e) => event!(Level::INFO, "Unable to send to {:?} - {}", &addr, &e),
                        Ok(size) => event!(Level::TRACE, "Sent packet of zize {} to {:?}", size, &addr)
                    }
                }
            }
            Some(InternalUdpMsg::DISCONNECT) => { return; }
        };
    }
}


impl Decoder for UdpMessageParser {
    type Item = HanUdpMessage;
    type Error = Error;

    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> Result<Option<HanUdpMessage>, Self::Error> {
        let mut reader = BytesReader::from_bytes(src.bytes());
        let result = reader.read_message(src.bytes());
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

