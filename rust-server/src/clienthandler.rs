use bytes::{BytesMut, Buf};
use futures::SinkExt;
use crate::clienthandler::ClientState::LOGGEDIN;
use crate::controlserver::ServerState;
use crate::protos::hanmessage::{HanMessage, Auth, StreamHeader};
use crate::protos::hanmessage::mod_HanMessage::OneOfmsg;
use crate::util::Error;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpStream};
use tokio::net::tcp::{ReadHalf, WriteHalf};
use tokio::stream::StreamExt;
use tokio::sync::mpsc::{Sender, channel, Receiver};
use tokio::sync::mpsc::error::TryRecvError;
use tokio::time::{timeout, Duration};
use tokio_util::codec::{FramedWrite, FramedRead};
use tokio_util::codec::{Encoder, Decoder};
use tracing::{event, Level};
use quick_protobuf::{BytesReader, Writer, MessageWrite};
use uuid::Uuid;

#[allow(dead_code)]
pub enum InternalMsg {
    DISCONNECT,
    SENDCTRL(HanMessage),
    SENDVOICE,
}

#[derive(Debug)]
pub enum ClientState {
    CONNECTED,
    LOGGEDIN,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ClientHandle {
    pub server: Arc<Mutex<ServerState>>,
    username: String,
    uuid: Uuid,
    client_state: ClientState,
    sender: Sender<InternalMsg>,
}

pub struct MessageParser {}

pub async fn run_client(uuid: Uuid, pstream: TcpStream, server: Arc<Mutex<ServerState>>) {
    let mut stream = pstream;
    {
        let (sender, receiver) = channel(100);
        let (shutdown_sender, shutdown_receiver) = channel::<()>(1);
        let client = Arc::new(Mutex::new(ClientHandle {
            server: server.clone(),
            username: "".to_string(),
            uuid,
            client_state: ClientState::CONNECTED,
            sender,
        }));
        {
            let mut server_state = server.lock().unwrap();
            server_state.clients.insert(uuid, client.clone());
        }
        let (read, write) = stream.split();
        tokio::join!(
            client_reader(read, shutdown_receiver, client.clone()),
            client_writer(write, receiver, shutdown_sender)
        );
        {
            let mut server_state = server.lock().unwrap();
            server_state.clients.remove(&uuid);
        }
        event!(Level::INFO, "Connection closed.");
    }
}

async fn client_reader(read: ReadHalf<'_>, mut shutdown_receiver: Receiver<()>, client: Arc<Mutex<ClientHandle>>) {
    let mut messages = FramedRead::new(read, MessageParser {});
    loop {
        let select = timeout(Duration::from_secs(1), messages.next()).await;
        match shutdown_receiver.try_recv() {
            Err(TryRecvError::Empty) => (),
            _ => {
                event!(Level::INFO, "Disconnect writer !");
                break;
            }
        }
        match select {
            Err(_) => { event!(Level::TRACE, "Wait Timeout"); }
            Ok(Some(Ok(result))) => {
                event!(Level::TRACE, "Message received");
                process_message(&client, result).await
            }
            Ok(None) => {
                event!(Level::INFO, "Writer closed !");
                break;
            }
            _ => {
                event!(Level::WARN, "Unknown State");
                break;
            }
        }
    }
    disconnect("Stream terminated !".to_string(), &client).await;
    event!(Level::TRACE, "Reader closed");
}

async fn client_writer(write: WriteHalf<'_>, mut receiver: Receiver<InternalMsg>, shutdown_sender: Sender<()>) {
    let mut messages = FramedWrite::new(write, MessageParser {});
    async fn disconnect(sender: &Sender<()>) {
        match sender.send(()).await {
            Err(e) => event!(Level::TRACE, "Internal Disconnect msg failed: {}", e.to_string()),
            _ => event!(Level::INFO, "Internal Disconnect msg sent")
        };
    }
    while let Some(result) = receiver.next().await {
        match result {
            InternalMsg::DISCONNECT => { disconnect(&shutdown_sender).await; return; },
            InternalMsg::SENDCTRL(msg) => match messages.send(msg).await {
                Err(_) => disconnect(&shutdown_sender).await,
                _ => ()
            },
            InternalMsg::SENDVOICE => ()
        };
    }
    event!(Level::TRACE, "Writer closed");
}

async fn process_message(client: &Mutex<ClientHandle>, data: HanMessage) {
    event!(Level::INFO, "Nessage: {:?}", &data);
    match multiplex(client, &data) {
        Err(e) => {
            event!(Level::WARN, "{}", e.to_string());
            disconnect(e.to_string(), client).await
        }
        _ => ()
    }
}

async fn disconnect(_error: String, client: &Mutex<ClientHandle>) {
    let local_sender = client.lock().unwrap().sender.clone();
    match local_sender.send(InternalMsg::DISCONNECT).await {
        Err(e) => event!(Level::WARN, "Cleanup Failed {}", e.to_string()),
        _ => event!(Level::TRACE, "Cleanup Message sent !")
    }
}

fn multiplex(client: &Mutex<ClientHandle>, data: &HanMessage) -> Result<(), Error> {
    let uuid = match Uuid::from_slice(&data.uuid[..]) {
        Err(_) => return Err(Error::ProtocolError("Illegal UUID".to_string())),
        Ok(id) => id
    };
    match &data.msg {
        OneOfmsg::auth(msg) => handle_auth(client, &uuid, &msg),
        OneOfmsg::auth_result(_) => handle_illegal_msg(client, &uuid, "auth_result"),
        _ => handle_illegal_msg(client, &uuid, "unknown_message")
    }
}

/////////////////////
// Message Handler //
/////////////////////
fn handle_auth(client: &Mutex<ClientHandle>, uuid: &Uuid, msg: &Auth) -> Result<(), Error> {
    let mut state = client.lock().unwrap();
    if let LOGGEDIN = state.client_state {
        return Err(Error::ProtocolError("Relogin after login !".to_string()));
    }
    state.username = msg.username.clone();
    state.client_state = ClientState::LOGGEDIN;
    event!(Level::TRACE, "Received Auth UUID: {}, user: {}", &uuid, &msg.username);
    Ok(())
}

fn handle_illegal_msg(_client: &Mutex<ClientHandle>, _uuid: &Uuid, _message: &str) -> Result<(), Error> {
    Err(Error::ProtocolError("Illegal Message".to_string()))
}


const HEADER_LENGTH: usize = 10;

impl MessageParser {
    fn read_header(src: &mut BytesMut) -> Result<usize, Error> {
        let mut reader = BytesReader::from_bytes(src.bytes());
        let header: StreamHeader = match reader.read_message_by_len(src.bytes(), HEADER_LENGTH) {
            Err(e) => return Err(Error::from(e)),
            Ok(val) => val
        };
        if header.magic != 0x0008a71 {
            return Err(Error::ProtocolError("MAGIC is gone !".to_string()));
        }
        return Ok(header.length as usize);
    }
}


impl Decoder for MessageParser {
    type Item = HanMessage;
    type Error = Error;

    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> Result<Option<HanMessage>, Self::Error> {
        //  skip magic bytes
        if src.len() < HEADER_LENGTH {
            return Ok(None);
        }
        let size = match MessageParser::read_header(src) {
            Ok(val) => val,
            Err(e) => return Err(e)
        };
        if src.len() < size + HEADER_LENGTH {
            return Ok(None);
        }
        src.advance(HEADER_LENGTH);
        let mut reader = BytesReader::from_bytes(src.bytes());
        let result = reader.read_message_by_len(src.bytes(), size);
        src.advance(size);
        match result {
            Ok(msg) => Ok(Some(msg)),
            Err(e) => Err(Error::from(e))
        }
    }
}

impl Encoder<HanMessage> for MessageParser {
    type Error = quick_protobuf::Error;

    fn encode(&mut self, message: HanMessage, dst: &mut BytesMut) -> Result<(), quick_protobuf::Error> {
        let mut writer = Writer::new(ByteMutWrite { delegate: dst });
        match (StreamHeader {
            magic: 0x00008A71,
            length: message.get_size() as u32,
        }).write_message(&mut writer) {
            Err(e) => return Err(e),
            _ => ()
        }
        message.write_message(&mut writer)
    }
}

pub struct ByteMutWrite<'a> {
    delegate: &'a mut BytesMut
}

impl std::io::Write for ByteMutWrite<'_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.delegate.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use quick_protobuf::{Writer, MessageWrite};
    use crate::protos::hanmessage::StreamHeader;

    #[test]
    fn testheader() {
        let header = StreamHeader {
            magic: 0x00008A71,
            length: 12345,
        };
        println!("Header size: {}", header.get_size());
    }

    #[test]
    fn testencode() {
        let mut r = Vec::new();
        let mut writer = Writer::new(&mut r);
        writer.write_fixed32(0x00008A71).unwrap();
        println!("Length: {}", r.len());
    }
}