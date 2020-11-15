use crate::clienthandler::{run_client, ClientHandle};

use std::boxed::Box;
use std::collections::{HashMap,HashSet};
use std::error::Error;
use std::result::Result;
use std::sync::{Arc,Mutex};
use tokio::net::{TcpListener};
use tracing::{span, info, Instrument, Level};
use uuid::Uuid;

pub struct ControlServer {

}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ServerState {
    pub channels: HashMap<String, HashSet<Uuid>>,
    pub clients: HashMap<Uuid, Arc<Mutex<ClientHandle>>>
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
        info!("Starting Listener on {}", &addr);
        let state = Arc::new(Mutex::new(ServerState::new()));
        // Bind a TCP listener to the socket address.
        //
        // Note that this is the Tokio TcpListener, which is fully async.

        loop {
            let (stream, addr) = listener.accept().await?;
            let local_state = Arc::clone(&state);
            info!("Acception Connection from {}", &addr);
            tokio::spawn(async move {
                let uuid = Uuid::new_v4();
                run_client(uuid,stream, local_state)
                    .instrument(span!(Level::ERROR, "Connection", "{}", &uuid))
                    .await;
            });
        }
     }
}

#[cfg(test)]
mod tests {

}