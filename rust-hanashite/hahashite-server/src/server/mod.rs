use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::AtomicU32;
use std::sync::Mutex;

use sodiumoxide::crypto::sign::PublicKey;
use tracing::{Level, span};
use tracing_futures::Instrument;
use uuid::Uuid;

use crate::{tcp, udp};
use crate::configuration::{Config};
use crate::error::Error;
use crate::bus::{Bus, BusEndpoint, BusMessage, MessagePredicate};
use std::collections::HashSet;
use crate::server::ConnectionState::Defunct;
use hanashite_message::protos::hanmessage::han_message::Msg;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Permission {
    Illegal
}

#[cfg(test)]
pub mod test;


#[allow(dead_code)]
#[derive(Debug)]
pub enum ControlMessage {
    DISCONNECT(Uuid),
    EVENT(Msg)
}

#[derive(PartialEq, Debug, Clone)]
#[allow(dead_code)]
pub enum ConnectionState {
    Connected,
    Challenged(Vec<u8>),
    Authenticated,
    Defunct,
}

impl Default for ConnectionState {
    fn default() -> Self {
        Defunct
    }
}

#[derive(Clone, Default)]
pub struct ConnectionContext {
    pub name: String,
    pub state: ConnectionState,
    pub permissions: HashSet<Permission>,
}

pub type ServerBus = Bus<ControlMessage, ConnectionContext>;
pub type ServerBusEndpoint = BusEndpoint<ControlMessage, ConnectionContext>;
pub type ServerBusMessage = BusMessage<ControlMessage, ConnectionContext>;

#[allow(dead_code)]
pub struct ServerStruct {
    pub config: Box<Config>,
    connections: Mutex<HashMap<Uuid, Connection>>,
    users: Mutex<HashMap<u32, User>>,
    next_user_id: AtomicU32,
    bus: Mutex<ServerBus>,
}


#[derive(Debug)]
pub struct Connection {
    pub user_id: u32,
    pub name: String,
    pub peer_addr: SocketAddr,
    pub state: ConnectionState,
}


#[derive(PartialEq, Eq, Debug)]
pub struct User {
    pub id: u32,
    pub public_key: PublicKey,
}


pub trait Server {
    fn terminate_connection(&self, connection_id: &Uuid)-> Result<(), Error>;
}

impl ServerStruct {
    pub fn init(config: Box<Config>) -> Arc<ServerStruct> {
        Arc::new(ServerStruct {
            config,
            next_user_id: AtomicU32::new(1),
            connections: Mutex::new(HashMap::new()),
            users: Mutex::new(HashMap::new()),
            bus: Mutex::new(Bus::new()),
        })
    }

    pub async fn run(server: Arc<ServerStruct>) -> Result<(), Error> {
        ::tokio::select!(
            r = tokio::spawn(tcp::run(server.clone()).instrument(span!(Level::ERROR,"tcp"))) => r,
            r = tokio::spawn(udp::run(server.clone()).instrument(span!(Level::ERROR,"udp"))) => r
        )?
    }

    pub fn new_connection(&self, addr: &SocketAddr) -> Uuid {
        let connection_id = Uuid::new_v4();
        let mut connections = self.connections.lock().unwrap();
        connections.insert(connection_id.clone(), Connection {
            state: ConnectionState::Connected,
            user_id: 0,
            name: String::new(),
            peer_addr: addr.clone(),
        });
        connection_id
    }

    pub fn create_endpoint(&self) -> ServerBusEndpoint {
        self.bus.lock().unwrap().create_endpoint()
    }
}

struct TerminationPredicate {}

/*
TODO: Not all should get the disconnect message.
 */
impl MessagePredicate<ControlMessage, ConnectionContext> for TerminationPredicate {
    fn relevant(&self, message: &ServerBusMessage, context: &ConnectionContext) -> bool {
        true
    }
}

impl Server for ServerStruct {
    fn terminate_connection(&self, connection_id: &Uuid) -> Result<(), Error>{
        let mut connections = self.connections.lock().unwrap();
        if connections.remove(connection_id).is_some() {
            self.bus.lock().unwrap().send(Arc::new(BusMessage {
                predicate: Arc::new(TerminationPredicate {}),
                msg: ControlMessage::DISCONNECT(connection_id.clone()),
            }))?;
        }
        Ok(())
    }
}
