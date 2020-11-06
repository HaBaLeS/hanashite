pub struct ControlServer {

}
use std::result::Result;
use std::error::Error;
use tokio::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::boxed::Box;
use std::net::SocketAddr;
use tokio::sync::{Mutex};
use tokio_util::codec::{Framed, LinesCodec};
struct ServerState {

}

impl ControlServer {
    pub fn new() -> ControlServer {
        ControlServer {
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let addr = "0.0.0.0:9876".to_string();
        let state = Arc::new(Mutex::new(ServerState {}));
        // Bind a TCP listener to the socket address.
        //
        // Note that this is the Tokio TcpListener, which is fully async.
        let listener = TcpListener::bind(&addr).await?;

        loop {
            let (stream, addr) = listener.accept().await?;
            let state = Arc::clone(&state);
            // Spawn our handler to be run asynchronously.
            tokio::spawn(async move {
                
            });
        }
     }

     async fn run_client(
        state: Arc<Mutex<ServerState>>,
        stream: TcpStream,
        addr: SocketAddr,
    ) -> Result<(), Box<dyn Error>> {
        let mut clientHandler = Framed::new(stream, LinesCodec::new());

     }
}

#[cfg(test)]
mod tests {

}