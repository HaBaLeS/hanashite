use crate::controlserver::ServerState;
use bytes::{BytesMut, Buf};
use futures::SinkExt;
use crate::protos::hanmessage::{HanMessage, Auth, StreamHeader};
use crate::protos::hanmessage::mod_HanMessage::OneOfmsg;
use crate::util::Error;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpStream};
use tokio::stream::StreamExt;
use tokio::time::timeout;
use tokio_util::codec::{FramedWrite, FramedRead};
use tokio_util::codec::{Encoder, Decoder};
use quick_protobuf::{BytesReader, Writer, MessageWrite};
use uuid::Uuid;
use crate::clienthandler::ClientState::LOGGEDIN;
use tokio::sync::mpsc::{Sender, channel};
use tokio::time::Duration;
use tokio::sync::mpsc::error::TryRecvError;

#[allow(dead_code)]
pub enum InternalMsg {
    DISCONNECT,
    SENDCTRL(HanMessage),
    SENDVOICE,
}

pub enum ClientState {
    CONNECTED,
    LOGGEDIN,
}

#[allow(dead_code)]
pub struct ClientHandle {
    pub server: Arc<Mutex<ServerState>>,
    username: String,
    uuid: Uuid,
    client_state: ClientState,
    sender: Sender<InternalMsg>,
}

pub struct MessageParser {}

pub async fn run_client(pstream: TcpStream, server: Arc<Mutex<ServerState>>) {
    let mut stream = pstream;
    let uuid = Uuid::new_v4();
    {
        let (sender, mut receiver) = channel(100);
        let (sender2, mut receiver2) = channel::<()>(1);
        let client = Mutex::new(ClientHandle {
            server,
            username: "".to_string(),
            uuid,
            client_state: ClientState::CONNECTED,
            sender,
        });
        {
            let client_state = client.lock().unwrap();
            println!("CONNECTION: {} Connected", &client_state.uuid);
        }
        let (read, write) = stream.split();
        tokio::join!(
            async move {
                let mut messages = FramedRead::new(read, MessageParser {});
                loop {
                    let select = timeout(Duration::from_secs(1), messages.next()).await;
                    match receiver2.try_recv() {
                        Err(TryRecvError::Empty) => (),
                        _ => { println!("Disconnect !!"); break }
                    }
                    match select {
                        Err(_) =>{ println!("Timeout .. next"); },
                        Ok(Some(Ok(result))) =>{ println!("proc"); process_message(&client, result).await },
                        Ok(None)=> { println!("NONE"); break },
                        _ => { println!("unknown"); break }
                    }

                }
            },
            async move {
                let mut messages = FramedWrite::new(write, MessageParser {});
                async fn disconnect(sender :&Sender<()>) {
                     match sender.send(()).await {
                        _ => ()
                     };
                }
                while let Some(result) = receiver.next().await {
                    match result {
                        InternalMsg::DISCONNECT => disconnect(&sender2).await,
                        InternalMsg::SENDCTRL(msg) => match messages.send(msg).await {
                           Err(_) => disconnect(&sender2).await,
                            _ => ()
                        },
                        InternalMsg::SENDVOICE => ()
                    };
                }

            }
        );
        println!("Joined");
    }
}

async fn process_message(client: &Mutex<ClientHandle>, data: HanMessage) {
    println!("MSG: {:?}", &data);
    match multiplex(client, &data) {
        Err(e) => disconnect(e.to_string(), client).await,
        _ => ()
    }
}

async fn disconnect(_error: String, client: &Mutex<ClientHandle>) {
    let local_sender = client.lock().unwrap().sender.clone();
    match local_sender.send(InternalMsg::DISCONNECT).await {
        _ => ()
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
    println!("CONNECTION: {} Received Auth UUID: {}, user: {}", &state.uuid, &uuid, &msg.username);
    Ok(())
}

fn handle_illegal_msg(_client: &Mutex<ClientHandle>, _uuid: &Uuid, _message: &str) -> Result<(), Error> {
    unimplemented!()
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