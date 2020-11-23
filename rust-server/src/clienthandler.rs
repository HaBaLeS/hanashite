use bytes::{BytesMut, Buf};
use futures::SinkExt;
use crate::clienthandler::ClientState::{LOGGEDIN};
use crate::controlserver::ServerState;
use crate::protos::hanmessage::{HanMessage, Auth, StreamHeader, AuthResult, ChannelList, ChannelPart, ChannelJoin, ChannelStatus, ChannelJoinResult};
use crate::protos::hanmessage::mod_HanMessage::OneOfmsg;
use crate::util::{ByteMutWrite,Error};
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;
use tokio::net::{TcpStream};
use tokio::net::tcp::{ReadHalf, WriteHalf};
use tokio::stream::StreamExt;
use tokio::sync::mpsc::{Sender, channel, Receiver};
use tokio_util::codec::{FramedWrite, FramedRead};
use tokio_util::codec::{Encoder, Decoder};
use tracing::{event, Level};
use quick_protobuf::{BytesReader, Writer, MessageWrite};
use uuid::Uuid;

#[allow(dead_code)]
pub enum InternalMsg {
    DISCONNECT,
    SENDCTRL(HanMessage),
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
    pub username: String,
    pub uuid: Uuid,
    pub client_state: ClientState,
    pub udp_socket: Option<SocketAddr>,
    pub sender: Sender<InternalMsg>,
}

pub struct MessageParser {}

pub async fn run_client(uuid: Uuid, mut stream: TcpStream, server: Arc<Mutex<ServerState>>) {
    let (sender, receiver) = channel(100);
    let (shutdown_sender, shutdown_receiver) = channel::<()>(1);
    let client = Arc::new(Mutex::new(ClientHandle {
        server: server.clone(),
        username: "".to_string(),
        uuid,
        client_state: ClientState::CONNECTED,
        udp_socket: None,
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
        for channel in server_state.channels.values_mut() {
            channel.users.remove(&uuid);
        }
        server_state.clients.remove(&uuid);
    }
    event!(Level::INFO, "Connection closed.");
}

async fn client_reader(read: ReadHalf<'_>, shutdown_receiver: Receiver<()>, client: Arc<Mutex<ClientHandle>>) {
    let mut messages = FramedRead::new(read, MessageParser {}).fuse();
    let mut fused_receiver = shutdown_receiver.fuse();
    loop {
        let select = tokio::select! {
            msg = messages.next() => msg,
            _ = fused_receiver.next() => { event!(Level::TRACE, "Disconnect event !");  break; }
        };
        match select {
            Some(Ok(result)) => {
                event!(Level::TRACE, "Message received");
                process_message(&client, result).await
            }
            Some(Err(e)) => {
                event!(Level::TRACE, "Error {}", e.to_string());
                break;
            }
            None => {
                event!(Level::INFO, "Connection closed");
                break;
            }
        }
    }
    disconnect("Stream terminated !".to_string(), &client).await;
    event!(Level::INFO, "Reader closed");
}

async fn client_writer(write: WriteHalf<'_>, mut receiver: Receiver<InternalMsg>, shutdown_sender: Sender<()>) {
    let mut messages = FramedWrite::new(write, MessageParser {});
    async fn disconnect(sender: &Sender<()>) {
        match sender.send(()).await {
            Err(e) => event!(Level::TRACE, "Internal Disconnect msg failed: {}", e.to_string()),
            _ => event!(Level::TRACE, "Internal Disconnect msg sent")
        };
    }
    while let Some(result) = receiver.next().await {
        match result {
            InternalMsg::DISCONNECT => {
                disconnect(&shutdown_sender).await;
                break;
            }
            InternalMsg::SENDCTRL(msg) => {
                event!(Level::INFO, "Send Msg: {:?}", &msg);
                match messages.send(msg).await {
                    Err(_) => disconnect(&shutdown_sender).await,
                    _ => event!(Level::TRACE, "Control Message sent !")
                }
            }
        };
    }
    event!(Level::INFO, "Writer closed");
}

async fn process_message(client: &Mutex<ClientHandle>, data: HanMessage) {
    event!(Level::INFO, "Nessage: {:?}", &data);
    match multiplex(client, &data).await {
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

async fn multiplex(client: &Mutex<ClientHandle>, data: &HanMessage) -> Result<(), Error> {
    let uuid = match Uuid::from_slice(&data.message_id[..]) {
        Err(_) => return Err(Error::ProtocolError("Illegal UUID".to_string())),
        Ok(id) => id
    };
    match &data.msg {
        OneOfmsg::auth(msg) => handle_auth(client, &uuid, &msg).await,
        OneOfmsg::chan_join(msg) => handle_chan_join(client, &uuid, &msg).await,
        OneOfmsg::chan_part(msg) => handle_chan_part(client, &uuid, &msg).await,
        OneOfmsg::chan_lst(msg) => handle_chan_lst(client, &uuid, &msg).await,
        OneOfmsg::chan_status(msg) => handle_chan_status(client, &uuid, &msg).await,
        OneOfmsg::auth_result(_) => handle_illegal_msg(client, &uuid, "auth_result").await,
        _ => handle_illegal_msg(client, &uuid, "unknown_message").await
    }
}

/////////////////////
// Message Handler //
/////////////////////

async fn handle_chan_status(_client: &Mutex<ClientHandle>, _uuid: &Uuid, _msg: &ChannelStatus) -> Result<(), Error> {
    unimplemented!()
}

async fn handle_chan_lst(_client: &Mutex<ClientHandle>, _uuid: &Uuid, _msg: &ChannelList) -> Result<(), Error> {
    unimplemented!()
}

async fn handle_chan_part(_client: &Mutex<ClientHandle>, _uuid: &Uuid, _msg: &ChannelPart) -> Result<(), Error> {
    unimplemented!()
}

async fn handle_chan_join(client: &Mutex<ClientHandle>, uuid: &Uuid, msg: &ChannelJoin) -> Result<(), Error> {
    let (sender, message) =  || -> Result<(Sender<InternalMsg>, ChannelJoinResult), Error>{
        let state = client.lock().unwrap();
        if let ClientState::CONNECTED = state.client_state {
            return Err(Error::ProtocolError("Not Logged in".to_string()));
        }
        let mut server = state.server.lock().unwrap();
        for channel in server.channels.values_mut() {
            channel.users.remove(&state.uuid);
        }
        for (channel_id, channel) in server.channels.iter_mut() {
            if channel.name == msg.name {
                channel.users.insert(state.uuid);
                return Ok((state.sender.clone(), ChannelJoinResult {
                    success: true,
                    channel_id: Vec::from(&channel_id.as_bytes()[..]),
                }));
            }
        }
        return Ok((state.sender.clone(), ChannelJoinResult {
            success: false,
            channel_id: vec![],
        }));
    }()?;
    match sender.send(InternalMsg::SENDCTRL(HanMessage {
        message_id: Vec::from(&uuid.as_bytes()[..]),
        msg: OneOfmsg::chan_join_result(message),
    })).await {
        Err(e) => return Err(Error::ProtocolError(e.to_string())),
        _ => return Ok(())
    }
}

async fn handle_auth(client: &Mutex<ClientHandle>, uuid: &Uuid, msg: &Auth) -> Result<(), Error> {
    let (sender, state_uuid) = {
        let mut state = client.lock().unwrap();
        if let LOGGEDIN = state.client_state {
            return Err(Error::ProtocolError("Relogin after login !".to_string()));
        }
        state.username = msg.username.clone();
        state.client_state = ClientState::LOGGEDIN;
        event!(Level::TRACE, "Received Auth UUID: {}, user: {}", &uuid, &msg.username);
        (state.sender.clone(), state.uuid)
    };
    match sender.send(InternalMsg::SENDCTRL(HanMessage {
        message_id: Vec::from(&uuid.as_bytes()[..]),
        msg: OneOfmsg::auth_result(
            AuthResult {
                success: true,
                connection_id: Vec::from(&state_uuid.as_bytes()[..]),
            }
        ),
    })).await {
        Err(e) => Err(Error::ProtocolError(e.to_string())),
        _ => Ok(())
    }
}

async fn handle_illegal_msg(_client: &Mutex<ClientHandle>, _uuid: &Uuid, _message: &str) -> Result<(), Error> {
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