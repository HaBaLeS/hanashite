
use crate::controlserver::ServerState;
use crate::protos::updmessage::HanUdpMessage;
use crate::util::{ByteMutWrite,Error};

use bytes::{BytesMut, Buf};
use std::net::SocketAddr;
use std::sync::{Arc,Mutex};
use tokio::net::UdpSocket;
use tokio::stream::StreamExt;
use tokio::sync::mpsc::{Sender, Receiver};
use tokio_util::codec::{Decoder, Encoder};
use quick_protobuf::{BytesReader, MessageWrite, BytesWriter, Writer};
use tracing::{Instrument, Level, span, event};
use uuid::Uuid;


#[allow(dead_code)]
pub enum InternalUdpMsg {
    DISCONNECT,
    SENDVOICE(Arc<HanUdpMessage>, Vec<SocketAddr>),
}

pub struct UdpMessageParser {}



pub async fn udp_client_read(state: Arc<Mutex<ServerState>>, socket: Arc<UdpSocket>, sender: Sender<InternalUdpMsg>) {
    let mut buf = vec![0 as u8; 8152];
    loop {
        match socket.recv_from(buf.as_mut_slice()).await {
            Err(error) => {
                event!(Level::ERROR, "Error with UDP socket: {}", &error);
                panic!("DIE !!!");
            }
            Ok((size, addr)) => {
                buf.resize(size, 0);
                let mut reader = BytesReader::from_bytes(&mut buf);
                match reader.read_message_by_len::<HanUdpMessage>(&mut buf, size) {
                    Err(e) => {
                        panic!("Unable to parse message: {}", &e);
                    }
                    Ok(message) => {
                        let uuid = Uuid::from_slice(message.user_id.as_slice()).unwrap();
                        process_udp_message(&addr, &state, Arc::new(message), &sender)
                            .instrument(span!(Level::ERROR, "Connection", "{}", &uuid))
                            .await;
                    }
                }
            }
        };
    }
}


async fn process_udp_message(addr: &SocketAddr,
                             state_mutex: &Arc<Mutex<ServerState>>,
                             message: Arc<HanUdpMessage>,
                             sender: &Sender<InternalUdpMsg>,
) {
    let user_id = Uuid::from_slice(message.user_id.as_slice()).unwrap();
    // Each package registers the remote for now...
    register_udp_socket(addr, state_mutex, &user_id);
    if message.audio_frame.is_some() {
        send_channel_packet(state_mutex, &user_id, message, sender).await;
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

async fn send_channel_packet(state_mutex: &Arc<Mutex<ServerState>>,
                             user_id: &Uuid,
                             message: Arc<HanUdpMessage>,
                             sender: &Sender<InternalUdpMsg>) {
    event!(Level::INFO, "forwarding Packet");
    let state = state_mutex.lock().unwrap();
    for channel in state.channels.values() {
        if channel.users.contains(user_id) {
            let mut targets = Vec::new();
            for remote_user in channel.users.iter() {
                if *remote_user != *user_id {
                    if let Some(remote_mutex) = state.clients.get(&remote_user) {
                        let remote = remote_mutex.lock().unwrap();
                        if let Some(addr) = remote.udp_socket {
                            targets.push(addr);
                        }
                    }
                }
            }
            let cloned_sender = sender.clone();
            let msg = message.clone();
            tokio::spawn(async move {
                cloned_sender.send(InternalUdpMsg::SENDVOICE(msg, targets)).await
            });
        }
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
        message.write_message(&mut writer)
    }
}

