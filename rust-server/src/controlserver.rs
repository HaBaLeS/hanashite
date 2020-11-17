use std::boxed::Box;
use std::collections::{HashMap, HashSet};
use std::result::Result;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, UdpSocket};
use tracing::{info, Instrument, Level, span};
use uuid::Uuid;
use crate::clienthandler::{ClientHandle, run_client};
use tokio::sync::mpsc::Sender;

pub struct ControlServer {}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ServerState {
    pub channels: HashMap<String, HashSet<Uuid>>,
    pub clients: HashMap<Uuid, Arc<Mutex<ClientHandle>>>,
    pub udp_sender: Option<Sender<UdpMessage>>

}

#[allow(dead_code)]
pub enum UdpMessage {
    AudioPacket
}

impl ServerState {
    fn new() -> ServerState {
        ServerState {
            channels: HashMap::new(),
            clients: HashMap::new(),
            udp_sender:None
        }
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
            (Ok(Ok(())), Ok(Ok(()))) =>Ok(()),
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
    info!("Starting UDP Listener on {}", &addr);
    tokio::join!(
        udp_client_read(state.clone(), socket.clone()),
        udp_client_write(state.clone(), socket)
    );
    Ok(())
}

async fn udp_client_read(_state: Arc<Mutex<ServerState>>, _socket: Arc<UdpSocket>) {

}

async fn udp_client_write(_state: Arc<Mutex<ServerState>>, _socket: Arc<UdpSocket>) {

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

#[cfg(test)]
mod tests {}