pub mod auth;
pub mod channel;

#[cfg(test)]
pub mod test;

pub use auth::*;
use crate::{tcp, udp};
use crate::configuration::Config;
use crate::error::Error;
use hanashite_message::protos::hanmessage::HanMessage;
use sodiumoxide::crypto::sign::PublicKey;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::broadcast::{Sender, channel};
use tokio::sync::{broadcast, mpsc};
use tracing::{Level, span};
use tracing_futures::{Instrument};
use uuid::Uuid;
use std::net::SocketAddr;
use std::sync::atomic::AtomicU32;
use std::sync::Mutex;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
pub enum ControlMessage {
    SENDCTRL(Box<HanMessage>),
    DISCONNECT,
}

#[derive(PartialEq, Debug)]
#[allow(dead_code)]
pub enum ConnectionState {
    Connected,
    Challenged(Vec<u8>),
    Authenticated,
    Defunct,
}

#[derive(Debug)]
pub struct Connection {
    pub name: String,
    pub peer_addr: SocketAddr,
    pub udp_addr: Option<SocketAddr>,
    pub user_id: u32,
    pub state: ConnectionState,
    pub sender: mpsc::Sender<ControlMessage>,
    pub termination_sender: broadcast::Sender<()>,
}

#[allow(dead_code)]
enum ChannelRole {
    Moderator(u32),
    Voice(u32),
}

#[allow(dead_code)]
struct VoiceChannel {
    name: String,
    channel_id: Uuid,
    roles: HashSet<ChannelRole>,
    connections: HashSet<Uuid>,
}

#[allow(dead_code)]
struct ChatChannel {
    name: String,
    roles: HashSet<ChannelRole>,
    connections: HashSet<Uuid>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct User {
    pub id: u32,
    pub public_key: PublicKey,
}

#[allow(dead_code)]
pub struct ServerStruct {
    pub config: Box<Config>,
    connections: Mutex<HashMap<Uuid, Connection>>,
    users: Mutex<HashMap<u32, User>>,
    voice_channels: Mutex<HashMap<String, VoiceChannel>>,
    chat_channels: Mutex<HashMap<String, ChatChannel>>,
    next_user_id: AtomicU32,
    shutdown_sender: Sender<()>,
}


pub trait Server {
    fn connection_sender(&self, connection_id: &Uuid) -> Result<mpsc::Sender<ControlMessage>, Error>;
    fn terminate_connection(&self, connection_id: &Uuid);
}

impl ServerStruct {
    pub fn init(config: Box<Config>) -> Arc<ServerStruct> {
        let (sender, _) = channel(10);
        Arc::new(ServerStruct {
            config,
            chat_channels: Mutex::new(HashMap::new()),
            users: Mutex::new(HashMap::new()),
            connections: Mutex::new(HashMap::new()),
            next_user_id: AtomicU32::new(1),
            voice_channels: Mutex::new(HashMap::new()),
            shutdown_sender: sender,
        })
    }

    pub async fn run(server: Arc<ServerStruct>) -> Result<(), Error> {
        ::tokio::select!(
            r = tokio::spawn(tcp::run(server.clone()).instrument(span!(Level::ERROR,"tcp"))) => r,
            r = tokio::spawn(udp::run(server.clone()).instrument(span!(Level::ERROR,"udp"))) => r
        )?
    }

    pub fn new_connection(&self, addr: &SocketAddr) -> (
        Uuid,
        mpsc::Receiver<ControlMessage>,
        broadcast::Sender<()>
    ) {
        let connection_id = Uuid::new_v4();
        let mut connections = self.connections.lock().unwrap();
        let (sender, receiver) = mpsc::channel(100);
        let (termination_sender, _) = broadcast::channel(2);
        connections.insert(connection_id.clone(), Connection {
            state: ConnectionState::Connected,
            user_id: 0,
            name: String::new(),
            peer_addr: addr.clone(),
            udp_addr: None,
            sender,
            termination_sender: termination_sender.clone(),
        });
        (connection_id, receiver, termination_sender)
    }

    pub fn shutdown_receiver(&self) -> broadcast::Receiver<()> {
        self.shutdown_sender.subscribe()
    }
}

pub fn get_valid_connection<'t>(connections: &'t mut HashMap<Uuid, Connection>, connection_id: &Uuid)
                                -> Result<&'t mut Connection, Error> {
    if let Some(con) = connections.get_mut(connection_id) {
        if con.state == ConnectionState::Connected {
            Ok(con)
        } else {
            Err(Error::ProtocolError("Connection not active.".to_string()))
        }
    } else {
        Err(Error::InternalError("Illegal Connection used.".to_string()))
    }
}

impl Server for ServerStruct {
    fn connection_sender(&self, connection_id: &Uuid) -> Result<mpsc::Sender<ControlMessage>, Error> {
        let connections = self.connections.lock().unwrap();
        if let Some(connection) = connections.get(connection_id) {
            return Ok(connection.sender.clone());
        }
        Err(Error::InternalError("Illegal Connection Id".to_string()))
    }

    fn terminate_connection(&self, connection_id: &Uuid) {
        let connections = self.connections.lock().unwrap();
        if let Some(connection) = connections.get(connection_id) {
            connection.termination_sender.send(()).unwrap_or(0);
        }
    }
}
