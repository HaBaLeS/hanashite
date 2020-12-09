mod reader;
mod writer;


use crate::server::{ServerStruct, ControlMessage};
use crate::error::Error;
use std::sync::Arc;
use tracing::{error_span, info, Instrument};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::sync::broadcast;
use uuid::Uuid;

pub async fn run(server: Arc<ServerStruct>) -> Result<(), Error> {
    info!("Entering TCP Loop");
    let config = &server.config.server;
    let addr = format!("{}:{}", &config.tcp_bind_ip, &config.tcp_port);
    info!("Binding TCP port to {}", &addr);
    let listener = TcpListener::bind(&addr).await?;
    let mut shutdown_receiver = server.shutdown_receiver();
    loop {
        let (stream, addr) = tokio::select!(
            r = listener.accept() => r?,
            _ = shutdown_receiver.recv() => {
                info!("Stopping TCP Port !");
                break;
            });
        info!("Accepting Connection from {}", &addr);
        let server = server.clone();
        tokio::spawn(async move {
            let (connection_id, receiver, termination_sender)
                = server.new_connection(&addr);
            let span = error_span!("Connection", "{}", &connection_id);
            run_client(server,
                       connection_id,
                       stream,
                       receiver,
                       termination_sender,
            ).instrument(span)
        });
    }
    Ok(())
}


async fn run_client(server: Arc<ServerStruct>,
                    uuid: Uuid,
                    mut stream: TcpStream,
                    receiver: mpsc::Receiver<ControlMessage>,
                    termination_sender: broadcast::Sender<()>,
) {
    let (tcp_reader, tcp_writer) = stream.split();
    let reader = reader::Reader::new(&server, &uuid);
    let writer = writer::Writer::<ServerStruct>::new(&server, &uuid);
    match tokio::join!(
        reader.client_reader(tcp_reader, termination_sender.subscribe()),
        writer.client_writer(tcp_writer, receiver, termination_sender.subscribe())
    ) {
        (Err(er), Err(ew)) => info!("Client closed with errors {} - {}", &er, &ew),
        (Err(e), _) => info!("Client closed with read error {}", &e),
        (_, Err(e)) => info!("Client closed with write error {}", &e),
        _ => ()
    }
}
