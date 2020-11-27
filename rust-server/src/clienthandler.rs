use bytes::{BytesMut, Buf};
use futures::SinkExt;
use crate::clienthandler::ClientState::{LOGGEDIN};
use crate::controlserver::ServerState;
use crate::protos::hanmessage::{HanMessage, Auth, StreamHeader, AuthResult, ChannelList, ChannelPart, ChannelJoin, ChannelStatus, ChannelJoinResult, ChannelListResult, ChannelListentry, Status, StatusResult, UserEntry, ChannelStatusResult};
use crate::protos::hanmessage::han_message::Msg;
use crate::util::Error;
use prost::Message;
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;
use tokio::net::{TcpStream};
use tokio::net::tcp::{ReadHalf, WriteHalf};
use tokio::stream::StreamExt;
use tokio::sync::mpsc::{Sender, channel, Receiver};
use tokio_util::codec::{FramedWrite, FramedRead};
use tokio_util::codec::{Encoder, Decoder};
use tracing::{event, Level};
use uuid::Uuid;

#[allow(dead_code)]
pub enum InternalMsg {
    DISCONNECT,
    SENDCTRL(Box<HanMessage>),
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
                process_message(&client, &result).await
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

async fn process_message(client: &Mutex<ClientHandle>, data: &HanMessage) {
    event!(Level::INFO, "Nessage: {:?}", &data);
    match multiplex(client, &data).await {
        Err(e) => {
            event!(Level::WARN, "{}", e.to_string());
            disconnect(e.to_string(), client).await
        }
        _ => ()
    }
}

async fn disconnect(error: String, client: &Mutex<ClientHandle>) {
    let sender = client.lock().unwrap().sender.clone();
    match sender.send(InternalMsg::DISCONNECT).await {
        Err(e) => event!(Level::WARN, "Cleanup Failed {}", e.to_string()),
        _ => event!(Level::TRACE, "Cleanup Message sent: {}", error)
    }
}

async fn multiplex(client: &Mutex<ClientHandle>, data: &HanMessage) -> Result<(), Error> {
    let uuid = match Uuid::from_slice(&data.message_id[..]) {
        Err(_) => return Err(Error::ProtocolError("Illegal UUID".to_string())),
        Ok(id) => id
    };
    match &data.msg {
        Some(Msg::Auth(msg)) => handle_auth(client, &uuid, &msg).await,
        Some(Msg::AuthResult(_)) => handle_illegal_msg(client, &uuid, "Illegal Message AuthResult").await,
        Some(Msg::ChanJoin(msg)) => handle_chan_join(client, &uuid, &msg).await,
        Some(Msg::ChanJoinResult(_)) => handle_illegal_msg(client, &uuid, "Illegal Message ChanJoinResult").await,
        Some(Msg::ChanPart(msg)) => handle_chan_part(client, &uuid, &msg).await,
        Some(Msg::ChanPartResult(_)) => handle_illegal_msg(client, &uuid, "Illegal Message ChanPartResult").await,
        Some(Msg::ChanLst(msg)) => handle_chan_lst(client, &uuid, &msg).await,
        Some(Msg::ChanLstResult(_)) => handle_illegal_msg(client, &uuid, "Illegal Message ChanLstResult").await,
        Some(Msg::ChanStatus(msg)) => handle_chan_status(client, &uuid, &msg).await,
        Some(Msg::ChanStatusResult(_)) => handle_illegal_msg(client, &uuid, "Illegal Message ChanStatusResult").await,
        Some(Msg::Status(msg)) => handle_status(client, &uuid, &msg).await,
        Some(Msg::StatusResult(_)) => handle_illegal_msg(client, &uuid, "Illegal Message StatusResult").await,
        None => handle_illegal_msg(client, &uuid, "Empty message").await
    }
}

/////////////////////
// Message Handler //
/////////////////////


async fn handle_status(client: &Mutex<ClientHandle>, uuid: &Uuid, _msg: &Status) -> Result<(), Error> {
    let (result, sender) = {
        let client = client.lock().unwrap();
        let server = client.server.lock().unwrap();
        let chan = server.channels.values().filter(|x| x.users.contains(&client.uuid))
            .map(|x| x.name.clone())
            .nth(0);
        (Box::new(HanMessage {
            message_id: Vec::from(&uuid.as_bytes()[..]),
            msg: Some(Msg::StatusResult(StatusResult {
                name: client.username.clone(),
                connection_id: Vec::from(&client.uuid.as_bytes()[..]),
                channel: chan.unwrap_or("".to_string()),
            })),
        }), client.sender.clone())
    };
    sender.send(InternalMsg::SENDCTRL(result)).await?;
    Ok(())
}

async fn handle_chan_status(client: &Mutex<ClientHandle>, uuid: &Uuid, msg: &ChannelStatus) -> Result<(), Error> {
    let (result, sender) = {
        let client = client.lock().unwrap();
        let server = client.server.lock().unwrap();
        let client_id = &client.uuid;
        fn create_user(client_id: &Uuid, id: &Uuid, handle: &Arc<Mutex<ClientHandle>>) -> Option<UserEntry> {
            if client_id == id {
                return None;
            }
            let handle = handle.lock().unwrap();
            Some(UserEntry {
                name: handle.username.clone(),
                user_id: Vec::from(&handle.uuid.as_bytes()[..]),
            })
        }
        let channel = server.channels.iter().filter(|(_, x)| &x.name == &msg.name).nth(0);
        if let Some((channel_id, channel)) = channel {
            let chan: Vec<UserEntry> = channel.users.iter()
                .map(|x|
                    server.clients.get(x).map(|y| create_user(&client_id, x, y)
                    ).unwrap_or(None))
                .filter(|x| x.is_some())
                .map(|x| x.unwrap())
                .collect();
            (Box::new(HanMessage {
                message_id: Vec::from(&uuid.as_bytes()[..]),
                msg: Some(Msg::ChanStatusResult(ChannelStatusResult {
                    user: chan,
                    name: channel.name.clone(),
                    channel_id: Vec::from(&channel_id.as_bytes()[..]),
                })),
            }), client.sender.clone())
        } else {
            (Box::new(HanMessage {
                message_id: Vec::from(&uuid.as_bytes()[..]),
                msg: Some(Msg::ChanStatusResult(ChannelStatusResult {
                    user: vec![],
                    name: "".to_string(),
                    channel_id: vec![],
                })),
            }), client.sender.clone())
        }
    };
    sender.send(InternalMsg::SENDCTRL(result)).await?;
    Ok(())
}

async fn handle_chan_part(_client: &Mutex<ClientHandle>, _uuid: &Uuid, _msg: &ChannelPart) -> Result<(), Error> {
    todo!()
}

async fn handle_chan_lst(client: &Mutex<ClientHandle>, uuid: &Uuid, _msg: &ChannelList) -> Result<(), Error> {
    let (result, sender) = {
        let client = client.lock().unwrap();
        let server = client.server.lock().unwrap();
        (Box::new(HanMessage {
            message_id: Vec::from(&uuid.as_bytes()[..]),
            msg: Some(Msg::ChanLstResult(ChannelListResult {
                channel: server.channels.iter()
                    .map(|c| ChannelListentry { channel_id: Vec::from(&c.0.as_bytes()[..]), name: c.1.name.clone() })
                    .collect()
            })),
        }), client.sender.clone())
    };
    sender.send(InternalMsg::SENDCTRL(result)).await?;
    Ok(())
}

async fn handle_chan_join(client: &Mutex<ClientHandle>, uuid: &Uuid, msg: &ChannelJoin) -> Result<(), Error> {
    let (sender, message) = || -> Result<(Sender<InternalMsg>, ChannelJoinResult), Error>{
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
    sender.send(InternalMsg::SENDCTRL(Box::new(HanMessage {
        message_id: Vec::from(&uuid.as_bytes()[..]),
        msg: Some(Msg::ChanJoinResult(message)),
    }))).await?;
    Ok(())
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
    sender.send(InternalMsg::SENDCTRL(Box::new(HanMessage {
        message_id: Vec::from(&uuid.as_bytes()[..]),
        msg: Some(Msg::AuthResult(
            AuthResult {
                success: true,
                connection_id: Vec::from(&state_uuid.as_bytes()[..]),
            }
        )),
    }))).await?;
    Ok(())
}

async fn handle_illegal_msg(_client: &Mutex<ClientHandle>, _uuid: &Uuid, _message: &str) -> Result<(), Error> {
    Err(Error::ProtocolError("Illegal Message".to_string()))
}


const HEADER_LENGTH: usize = 10;

impl MessageParser {}


impl Decoder for MessageParser {
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
        if header.magic != 0x0008a71 {
            return Err(Error::ProtocolError("MAGIC is gone !".to_string()));
        }
        if src.len() < header.length as usize + HEADER_LENGTH {
            return Ok(None);
        }
        src.advance(HEADER_LENGTH);
        let msg = HanMessage::decode(src)?;
        Ok(Some(Box::new(msg)))
    }
}

impl Encoder<Box<HanMessage>> for MessageParser {
    type Error = Error;

    fn encode(&mut self, message: Box<HanMessage>, dst: &mut BytesMut) -> Result<(), Error> {
        (StreamHeader {
            magic: 0x00008A71,
            length: message.encoded_len() as u32,
        }).encode(dst).expect("Message encoder broken");
        message.encode(dst).expect("Message encoder broken");
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use crate::protos::hanmessage::StreamHeader;
    use prost::Message;

    #[test]
    fn testheader() {
        let header = StreamHeader {
            magic: 0x00008A71,
            length: 12345,
        };
        println!("Header size: {}", header.encoded_len());
    }
}