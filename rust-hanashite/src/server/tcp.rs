mod handler;
mod reader;
mod writer;

use bytes::{Buf, BytesMut};
use crate::error::Error;
use crate::protos::{HEADER_LENGTH, MAGIC_HEADER};
use crate::protos::hanmessage::*;
use crate::server::{Server, Connection, Role, User};
use prost::Message;
use std::collections::HashSet;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tracing::{error_span, info, Instrument};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{channel};
use tokio::sync::broadcast;
use tokio_util::codec::{Encoder, Decoder};
use uuid::Uuid;

struct HanMessageCodec();

#[allow(dead_code)]
struct ClientConnection {
    server: Arc<Server>,
    connection: Arc<Mutex<Connection>>,
    user: Option<Arc<Mutex<User>>>,
    term_sender: broadcast::Sender<()>,
}

pub async fn run(server: Arc<Server>) -> Result<(), Error> {
    info!("Entering TCP Loop");
    let config = &server.config.server;
    let addr = format!("{}:{}", &config.tcp_bind_ip, &config.tcp_port);
    info!("Binding TCP port to {}", &addr);
    let listener = TcpListener::bind(&addr).await?;
    let mut shutdown_receiver = server.shutdown_sender.subscribe();
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
            let uuid = Uuid::new_v4();
            run_client(server, &uuid, stream, addr)
                .instrument(error_span!("Connection", "{}", &uuid))
                .await
        });
    }
    Ok(())
}


async fn run_client(server: Arc<Server>, uuid: &Uuid, mut stream: TcpStream, addr: SocketAddr) {
    let (sender, receiver) = channel(5);
    let connection = {
        let mut connection = Connection {
            addr,
            connection_id: uuid.clone(),
            roles: HashSet::new(),
            public_key: None,
            user_name: String::new(),
            sender,
        };
        connection.roles.insert(Role::PreAuth);
        let (term_sender, _) = broadcast::channel(2);
        let con = ClientConnection {
            connection: Arc::new(Mutex::new(connection)),
            server,
            user: None,
            term_sender,
        };
        con.server.connections.lock().unwrap().insert(uuid.clone(), con.connection.clone());
        con
    };
    let (tcp_reader, tcp_writer) = stream.split();
    match tokio::join!(
        connection.client_reader(tcp_reader),
        connection.client_writer(tcp_writer, receiver)
    ) {
        (Err(er), Err(ew)) => info!("Client closed with errors {} - {}", &er, &ew),
        (Err(e), _) => info!("Client closed with read error {}", &e),
        (_, Err(e)) => info!("Client closed with write error {}", &e),
        _ => ()
    }
}

impl ClientConnection {
}

impl Decoder for HanMessageCodec {
    type Item = Box<HanMessage>;
    type Error = Error;

    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> Result<Option<Box<HanMessage>>, Self::Error> {
        //  skip magic bytes
        if src.len() < HEADER_LENGTH {
            return Ok(None);
        }
        let header = StreamHeader::decode(&src[0..HEADER_LENGTH])?;
        if header.magic != MAGIC_HEADER {
            return Err(Error::InternalError("MAGIC is gone !".to_string()));
        }
        if src.len() < header.length as usize + HEADER_LENGTH {
            return Ok(None);
        }
        src.advance(HEADER_LENGTH);
        let msg = HanMessage::decode(src)?;
        Ok(Some(Box::new(msg)))
    }
}

impl Encoder<Box<HanMessage>> for HanMessageCodec {
    type Error = Error;

    fn encode(&mut self, message: Box<HanMessage>, dst: &mut BytesMut) -> Result<(), Error> {
        (StreamHeader {
            magic: MAGIC_HEADER,
            length: message.encoded_len() as u32,
        }).encode(dst).expect("Message encoder broken");
        message.encode(dst).expect("Message encoder broken");
        Ok(())
    }
}
