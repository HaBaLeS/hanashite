mod tcp;
mod udp;

use crate::configuration::Config;
use crate::error::Error;
use crate::protos::hanmessage::HanMessage;
use sodiumoxide::crypto::sign::PublicKey;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast::{Sender, channel};
use tokio::sync::mpsc;
use tracing::{Level, span};
use tracing_futures::{Instrument};
use uuid::Uuid;
use std::net::SocketAddr;

#[allow(dead_code)]
pub enum ControlMessage {
    SENDCTRL(Box<HanMessage>),
    DISCONNECT,
}

#[derive(PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum Role {
    // Not allowed to do anything
    PreAuth,
    // Valid connected User
    User,
    // All permissions everywhere
    Admin,
    //Can enter this channel
    Member(String),
    // Can Talk in this channel
    Voice(String),
    // Can kick/ban/voice/mute in this channel
    Moderator(String),
}

#[allow(dead_code)]
pub struct User {
    name: String,
    user_id: Uuid,
    roles: HashSet<Role>,
}

#[allow(dead_code)]
pub struct Channel {
    channel_id: Uuid,
    channel_name: String,
    connections: HashSet<Uuid>,
    private: bool,
    silent: bool,
}

#[allow(dead_code)]
pub struct Connection {
    addr: SocketAddr,
    connection_id: Uuid,
    user_name: String,
    roles: HashSet<Role>,
    public_key: Option<PublicKey>,
    sender: mpsc::Sender<ControlMessage>,
}

pub struct Server {
    pub config: Box<Config>,
    pub channels: Mutex<HashMap<String, Arc<Mutex<Channel>>>>,
    pub connections: Mutex<HashMap<Uuid, Arc<Mutex<Connection>>>>,
    pub shutdown_sender: Sender<()>,
}

impl Server {
    pub fn init(config: Box<Config>) -> Arc<Server> {
        let (sender, _) = channel(10);
        Arc::new(Server {
            config,
            channels: Mutex::new(HashMap::new()),
            connections: Mutex::new(HashMap::new()),
            shutdown_sender: sender,
        })
    }

    pub async fn run(server: Arc<Server>) -> Result<(), Error> {
        ::tokio::select!(
            r = tokio::spawn(tcp::run(server.clone()).instrument(span!(Level::ERROR,"tcp"))) => r,
            r = tokio::spawn(udp::run(server.clone()).instrument(span!(Level::ERROR,"udp"))) => r
        )?
    }
}

impl Connection {

    fn check_permission(&self, role: &Role) -> Result<(), Error> {
        if !self.roles.contains(&Role::PreAuth)
            && (self.roles.contains(&Role::Admin) ||
            self.roles.contains(role)) {
            return Ok(());
        }
        return Err(Error::PermissionDenied);
    }
}