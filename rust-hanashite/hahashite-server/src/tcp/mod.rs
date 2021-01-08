mod reader;
mod writer;


use crate::server::{ServerStruct, ServerBusEndpoint};
use crate::error::Error;
use std::sync::Arc;
use tracing::{error_span, info, Instrument};
use tokio::net::{TcpListener, TcpStream};
use uuid::Uuid;

pub async fn run(server: Arc<ServerStruct>) -> Result<(), Error> {
    info!("Entering TCP Loop");
    let config = &server.config.server;
    let addr = format!("{}:{}", &config.tcp_bind_ip, &config.tcp_port);
    info!("Binding TCP port to {}", &addr);
    let listener = TcpListener::bind(&addr).await?;
    loop {
        let (stream, addr) = tokio::select!(
            r = listener.accept() => r?
            );
        info!("Accepting Connection from {}", &addr);
        let server = server.clone();
        tokio::spawn(async move {
            let endpoint = server.create_endpoint();
            let connection_id = server.new_connection(&addr);
            let span = error_span!("Connection", "{}", &connection_id);
            run_client(server,
                       connection_id,
                       stream,
                       endpoint,
            ).instrument(span)
        });
    }
}


async fn run_client(server: Arc<ServerStruct>,
                    uuid: Uuid,
                    mut stream: TcpStream,
                    endpoint: ServerBusEndpoint,
) {
    let (tcp_reader, tcp_writer) = stream.split();
    let mut reader = reader::Reader::new(&server, &uuid, endpoint.clone());
    let mut writer = writer::Writer::<ServerStruct>::new(&server, &uuid, endpoint);
    match tokio::join!(
        reader.client_reader(tcp_reader),
        writer.client_writer(tcp_writer)
    ) {
        (Err(er), Err(ew)) => info!("Client closed with errors {} - {}", &er, &ew),
        (Err(e), _) => info!("Client closed with read error {}", &e),
        (_, Err(e)) => info!("Client closed with write error {}", &e),
        _ => ()
    }
}
