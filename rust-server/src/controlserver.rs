use crate::clienthandler::{ InternalMsg, run_client};

use std::boxed::Box;
use std::collections::{HashMap,HashSet};
use std::error::Error;
use std::result::Result;
use std::sync::{Arc,Mutex};
use tokio::net::{TcpListener};
use tokio::sync::mpsc::Sender;
use uuid::Uuid;

pub struct ControlServer {

}

#[allow(dead_code)]
pub struct ServerState {
    channels: HashMap<String, HashSet<Uuid>>,
    clients: HashMap<Uuid, Mutex<Sender<InternalMsg>>>
}

impl ServerState {
    fn new() -> ServerState {
        ServerState {
            channels: HashMap::new(),
            clients: HashMap::new()
        }
    }
}

impl ControlServer {
    pub fn new() -> ControlServer {
        ControlServer {
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let addr = "0.0.0.0:9876".to_string();
        let listener = TcpListener::bind(&addr).await?;

        let state = Arc::new(Mutex::new(ServerState::new()));
        // Bind a TCP listener to the socket address.
        //
        // Note that this is the Tokio TcpListener, which is fully async.

        loop {
            let (stream, _addr) = listener.accept().await?;
            let local_state = Arc::clone(&state);
            tokio::spawn(async move {
                run_client(stream, local_state).await;
            });
        }
     }
}

#[cfg(test)]
mod tests {

}