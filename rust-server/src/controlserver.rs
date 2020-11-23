use std::boxed::Box;
use std::collections::{HashMap, HashSet};
use std::result::Result;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, UdpSocket};
use tokio::sync::mpsc::{Sender, channel, Receiver};
use tracing::{info, Instrument, Level, span, event};
use uuid::Uuid;

use crate::clienthandler::{ClientHandle, run_client, ByteMutWrite};
use quick_protobuf::{BytesReader, MessageWrite, BytesWriter, Writer};
use crate::protos::updmessage::HanUdpMessage;
use std::net::SocketAddr;
use tokio::stream::StreamExt;
use crate::util::Error;
use tokio_util::codec::{Decoder, Encoder};
use bytes::{BytesMut, Buf};

pub struct ControlServer {}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ServerState {
    pub channels: HashMap<Uuid, ChannelState>,
    pub clients: HashMap<Uuid, Arc<Mutex<ClientHandle>>>,
    pub udp_sender: Option<Sender<UdpMessage>>,
}

#[allow(dead_code)]
pub enum InternalUdpMsg {
    DISCONNECT,
    SENDVOICE(Arc<HanUdpMessage>, Vec<SocketAddr>),
}

#[derive(Debug)]
pub struct ChannelState {
    pub name: String,
    pub users: HashSet<Uuid>,
}

#[allow(dead_code)]
pub enum UdpMessage {
    AudioPacket
}

pub struct UdpMessageParser {}


impl ServerState {
    fn new() -> ServerState {
        let mut state = ServerState {
            channels: HashMap::new(),
            clients: HashMap::new(),
            udp_sender: None,
        };
        state.channels.insert(Uuid::new_v4(), ChannelState {
            users: HashSet::new(),
            name: "testchannel".to_string()
        });
        state
    }
}

impl ControlServer {
    pub fn new() -> ControlServer {
        ControlServer {}
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let state = Arc::new(Mutex::new(ServerState::new()));
        let udp = tokio::spawn(listen_udp(state.clone()));
        let tcp = tokio::spawn(listen_tcp(state.clone()));
        match tokio::join!(tcp, udp) {
            (Ok(Ok(())), Ok(Ok(()))) => Ok(()),
            (Err(e), _) => Err(Box::new(e)),
            (_, Err(e)) => Err(Box::new(e)),
            (Ok(Err(e)), _) => Err(Box::new(e)),
            (_, Ok(Err(e))) => Err(Box::new(e))
        }
    }
}

async fn listen_udp(state: Arc<Mutex<ServerState>>) -> Result<(), std::io::Error> {
    let addr = "0.0.0.0:9876".to_string();
    let socket = Arc::new(UdpSocket::bind(&addr).await?);
    let (sender, receiver) = channel(100);
    info!("Starting UDP Listener on {}", &addr);
    tokio::join!(
        udp_client_read(state.clone(), socket.clone(), sender),
        udp_client_write(state, socket, receiver)
    );
    Ok(())
}

async fn udp_client_read(state: Arc<Mutex<ServerState>>, socket: Arc<UdpSocket>, sender: Sender<InternalUdpMsg>) {
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
                        process_udp_message(&addr, &state, Arc::new(message), &sender).await;
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

async fn udp_client_write(_state: Arc<Mutex<ServerState>>, socket: Arc<UdpSocket>, mut receiver: Receiver<InternalUdpMsg>) {
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

async fn listen_tcp(state: Arc<Mutex<ServerState>>) -> Result<(), std::io::Error> {
    let addr = "0.0.0.0:9876".to_string();
    let listener = TcpListener::bind(&addr).await?;
    info!("Starting Listener on {}", &addr);
    // Bind a TCP listener to the socket address.
    //
    // Note that this is the Tokio TcpListener, which is fully async.
    loop {
        let (stream, addr) = listener.accept().await?;
        let local_state = Arc::clone(&state);
        info!("Acception Connection from {}", &addr);
        tokio::spawn(async move {
            let uuid = Uuid::new_v4();
            run_client(uuid, stream, local_state)
                .instrument(span!(Level::ERROR, "Connection", "{}", &uuid))
                .await;
        });
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


#[cfg(test)]
mod tests {}