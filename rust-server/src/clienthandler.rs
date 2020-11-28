use bytes::{BytesMut, Buf};
use futures::SinkExt;
use crate::clienthandler::ClientState::{LOGGEDIN};
use crate::controlserver::{ServerState, ChannelState};
use crate::protos::hanmessage::*;
use crate::protos::hanmessage::han_message::Msg;
use crate::util::Error;
use prost::Message;
use std::collections::{HashSet};
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
#[derive(PartialEq)]
pub enum InternalMsg {
    DISCONNECT,
    SENDCTRL(Box<HanMessage>),
}

#[derive(Debug)]
#[derive(PartialEq)]
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
        Some(Msg::ChanCrea(msg)) => handle_chan_crea(client, &uuid, &msg).await,
        Some(Msg::ChanCreaResult(_)) => handle_illegal_msg(client, &uuid, "Illegal Message ChanCreaResult").await,
        Some(Msg::ChanDel(msg)) => handle_chan_del(client, &uuid, msg).await,
        Some(Msg::ChanDelResult(_)) => handle_illegal_msg(client, &uuid, "Illegal Message ChanDelResult").await,
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
        Some(Msg::ChanJoinEv(_)) => handle_illegal_msg(client, &uuid, "Illegal Message ChanJoinEv").await,
        Some(Msg::ChanPartEv(_)) => handle_illegal_msg(client, &uuid, "Illegal Message ChanPartEv").await,
        None => handle_illegal_msg(client, &uuid, "Empty message").await
    }
}

/////////////////////
// Message Handler //
/////////////////////

async fn handle_auth(client: &Mutex<ClientHandle>, uuid: &Uuid, msg: &Auth) -> Result<(), Error> {
    let (result, sender) = {
        let mut state = client.lock().unwrap();
        if let LOGGEDIN = state.client_state {
            return Err(Error::ProtocolError("Relogin after login !".to_string()));
        }
        state.username = msg.username.clone();
        state.client_state = ClientState::LOGGEDIN;
        event!(Level::TRACE, "Received Auth UUID: {}, user: {}", &uuid, &msg.username);
        (Box::new(HanMessage {
            message_id: Vec::from(&uuid.as_bytes()[..]),
            msg: Some(Msg::AuthResult(
                AuthResult {
                    success: true,
                    connection_id: Vec::from(&state.uuid.as_bytes()[..]),
                }
            )),
        }), state.sender.clone())
    };
    tokio::spawn(async move { sender.send(InternalMsg::SENDCTRL(result)).await.unwrap_or(()) });
    Ok(())
}

async fn handle_chan_crea(client: &Mutex<ClientHandle>, uuid: &Uuid, msg: &ChannelCreate) -> Result<(), Error> {
    let (result, sender) = {
        let client = client.lock().unwrap();
        let mut server = client.server.lock().unwrap();
        let (succ, channel_id) =
            match server.channels.iter()
                .filter(|(_, val)| val.name == msg.name)
                .nth(0) {
                None => {
                    let c_id = Uuid::new_v4();
                    server.channels.insert(c_id.clone(), ChannelState {
                        name: msg.name.clone(),
                        users: HashSet::new(),
                    });
                    (true, c_id)
                }
                Some((u, _)) => (false, u.clone())
            };
        (Box::new(HanMessage {
            message_id: Vec::from(&uuid.as_bytes()[..]),
            msg: Some(Msg::ChanCreaResult(ChannelCreateResult {
                name: msg.name.clone(),
                success: succ,
                channel_id: Vec::from(&channel_id.as_bytes()[..]),
            })),
        }), client.sender.clone())
    };
    tokio::spawn(async move { sender.send(InternalMsg::SENDCTRL(result)).await.unwrap_or(()); });
    Ok(())
}

async fn handle_chan_del(client: &Mutex<ClientHandle>, uuid: &Uuid, msg: &ChannelDelete) -> Result<(), Error> {
    // TODO ChannelPartEv
    let (result, sender) = {
        let client = client.lock().unwrap();
        let mut server = client.server.lock().unwrap();
        let (succ, channel_id) = {
            let channel_id = server.channels.iter()
                .filter(|(_, val)| val.name == msg.name)
                .map(|(u, _)| u.clone())
                .nth(0);
            match channel_id {
                Some(u) => {
                    server.channels.remove(&u);
                    (true, Vec::from(&u.as_bytes()[..]))
                }
                None => (false, vec![])
            }
        };
        (Box::new(HanMessage {
            message_id: Vec::from(&uuid.as_bytes()[..]),
            msg: Some(Msg::ChanDelResult(ChannelDeleteResult {
                name: msg.name.clone(),
                success: succ,
                channel_id,
            })),
        }), client.sender.clone())
    };

    tokio::spawn(async move { sender.send(InternalMsg::SENDCTRL(result)).await.unwrap_or(()); });
    Ok(())
}


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
    tokio::spawn(async move { sender.send(InternalMsg::SENDCTRL(result)).await.unwrap_or(()); });
    Ok(())
}

async fn handle_chan_status(client: &Mutex<ClientHandle>, uuid: &Uuid, msg: &ChannelStatus) -> Result<(), Error> {
    let (result, sender) = {
        let client = client.lock().unwrap();
        let server = client.server.lock().unwrap();
        let client_id = &client.uuid;
        fn create_user(client_id: &Uuid, client_name: &String, id: &Uuid, handle: &Arc<Mutex<ClientHandle>>) -> Option<UserEntry> {
            if client_id == id {
                return Some(UserEntry {
                    name: client_name.clone(),
                    user_id: Vec::from(&client_id.as_bytes()[..]),
                });
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
                    server.clients.get(x).map(|y| create_user(&client_id, &client.username, x, y)
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
    tokio::spawn(async move { sender.send(InternalMsg::SENDCTRL(result)).await.unwrap_or(()); });
    Ok(())
}

async fn handle_chan_part(client: &Mutex<ClientHandle>, uuid: &Uuid, _msg: &ChannelPart) -> Result<(), Error> {
    let (result, sender) = {
        let state = client.lock().unwrap();
        let mut server = state.server.lock().unwrap();
        let channel_id = server.channels.iter()
            .filter(|(_, c)| c.users.contains(&state.uuid))
            .map(|(u, _)| u.clone()).nth(0);
        (if let Some(id) = channel_id {
            let name = {
                let channel = server.channels.get_mut(&id).unwrap();
                channel.users.remove(&state.uuid);
                channel.name.clone()
            };
            Box::new(HanMessage {
                message_id: Vec::from(&uuid.as_bytes()[..]),
                msg: Some(Msg::ChanPartResult(ChannelPartResult {
                    name,
                    channel_id: Vec::from(&id.as_bytes()[..]),
                    success: true,
                })),
            })
        } else {
            Box::new(HanMessage {
                message_id: Vec::from(&uuid.as_bytes()[..]),
                msg: Some(Msg::ChanPartResult(ChannelPartResult {
                    name: "".to_string(),
                    channel_id: vec![],
                    success: false,
                })),
            })
        }, state.sender.clone())
    };
    tokio::spawn(async move { sender.send(InternalMsg::SENDCTRL(result)).await.unwrap_or(()); });
    Ok(())
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
    tokio::spawn(async move { sender.send(InternalMsg::SENDCTRL(result)).await.unwrap_or(()); });
    Ok(())
}

async fn handle_chan_join(client: &Mutex<ClientHandle>, uuid: &Uuid, msg: &ChannelJoin) -> Result<(), Error> {
    let (result, sender) = {
        let state = client.lock().unwrap();
        if let ClientState::CONNECTED = state.client_state {
            return Err(Error::ProtocolError("Not Logged in".to_string()));
        }
        let mut server = state.server.lock().unwrap();
        for channel in server.channels.values_mut() {
            channel.users.remove(&state.uuid);
        }
        (Box::new(HanMessage {
            message_id: Vec::from(&uuid.as_bytes()[..]),
            msg: Some(Msg::ChanJoinResult(match server.channels.iter_mut().filter(|(_, c)| c.name == msg.name)
            .nth(0) {
            Some((u, c)) => {
                c.users.insert(state.uuid);
                ChannelJoinResult {
                    success: true,
                    channel_id: Vec::from(&u.as_bytes()[..]),
                }
            }
            None => ChannelJoinResult {
                success: false,
                channel_id: vec![],
            }
        }))}), state.sender.clone())
    };
    tokio::spawn(async move { sender.send(InternalMsg::SENDCTRL(result)).await.unwrap_or(()); });
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
    use crate::protos::hanmessage::{StreamHeader, Auth, Status, ChannelCreate, ChannelDelete, ChannelJoin, ChannelPart, ChannelList, ChannelStatus};
    use prost::Message;
    use crate::controlserver::{ServerState, ChannelState};
    use std::sync::{Arc, Mutex};
    use std::collections::{HashSet, HashMap};
    use std::iter::FromIterator;
    use tokio;
    use tokio::sync::mpsc::{channel, Receiver};
    use uuid::Uuid;
    use crate::clienthandler::{ClientState, ClientHandle, InternalMsg, handle_auth, handle_status, handle_chan_crea, handle_chan_del, handle_chan_join, handle_chan_part, handle_chan_lst, handle_chan_status};
    use crate::protos::hanmessage::han_message::Msg;
    use crate::clienthandler::ClientState::{LOGGEDIN};


    macro_rules! aw {
      ($e:expr) => {
        ::tokio::time::timeout(::std::time::Duration::from_millis(1000), $e).await.unwrap()
      };
    }

    #[allow(dead_code)]
    struct TestSetup {
        uuid1: Uuid,
        uuid2: Uuid,
        uuid3: Uuid,
        receiver1: Receiver<InternalMsg>,
        receiver2: Receiver<InternalMsg>,
        receiver3: Receiver<InternalMsg>,
        server: Arc<Mutex<ServerState>>,
    }

    fn setup_server() -> TestSetup {
        let server = Arc::new(Mutex::new(ServerState {
            udp_sender: None,
            channels: HashMap::new(),
            clients: HashMap::new(),
        }));
        let (sender1, receiver1) = channel(100);
        let (sender2, receiver2) = channel(100);
        let (sender3, receiver3) = channel(100);
        let uuid1 = Uuid::from_slice(&[1; 16][..]).unwrap();
        let uuid2 = Uuid::from_slice(&[2; 16][..]).unwrap();
        let uuid3 = Uuid::from_slice(&[3; 16][..]).unwrap();

        {
            let mut state = server.lock().unwrap();

            state.channels.insert(uuid1.clone(), ChannelState {
                name: "testchannel1".to_string(),
                users: HashSet::new(),
            });
            state.channels.insert(uuid2.clone(), ChannelState {
                name: "testchannel2".to_string(),
                users: HashSet::from_iter(vec![uuid1.clone(), uuid2.clone()]),
            });
            state.clients.insert(uuid1.clone(), Arc::new(Mutex::new(ClientHandle {
                client_state: ClientState::LOGGEDIN,
                uuid: uuid1.clone(),
                username: "testuser1".to_string(),
                server: server.clone(),
                udp_socket: None,
                sender: sender1,
            })));
            state.clients.insert(uuid2.clone(), Arc::new(Mutex::new(ClientHandle {
                client_state: ClientState::LOGGEDIN,
                uuid: uuid2.clone(),
                username: "testuser2".to_string(),
                server: server.clone(),
                udp_socket: None,
                sender: sender2,
            })));
            state.clients.insert(uuid3.clone(), Arc::new(Mutex::new(ClientHandle {
                client_state: ClientState::CONNECTED,
                uuid: uuid3.clone(),
                username: "".to_string(),
                server: server.clone(),
                udp_socket: None,
                sender: sender3,
            })));
        }
        TestSetup {
            uuid1,
            uuid2,
            uuid3,
            receiver1,
            receiver2,
            receiver3,
            server,
        }
    }

    #[tokio::test]
    async fn test_auth() {
        let mut test_setup = setup_server();
        let user3 = {
            test_setup.server.lock().unwrap().clients.get(&test_setup.uuid3).unwrap().clone()
        };
        aw!(handle_auth(&user3, &test_setup.uuid1, &Auth {
            username: "testuser3".to_string()
        })).unwrap();
        let msg = aw!(test_setup.receiver3.recv()).unwrap();
        if let InternalMsg::SENDCTRL(hmsg) = msg {
            if let Some(Msg::AuthResult(r)) = hmsg.msg {
                assert_eq!(true, r.success);
                assert_eq!(&test_setup.uuid3, &Uuid::from_slice(&r.connection_id[..]).unwrap());
            } else {
                assert!(false, "Wrong Message !");
            }
        } else {
            assert!(false, "Wrong Message !");
        }
        {
            let client = user3.lock().unwrap();
            assert_eq!(LOGGEDIN, client.client_state);
            assert_eq!("testuser3", client.username);
        }
    }

    #[tokio::test]
    async fn test_chan_crea_succ() {
        let mut test_setup = setup_server();
        let user1 = {
            test_setup.server.lock().unwrap().clients.get(&test_setup.uuid1).unwrap().clone()
        };
        aw!(handle_chan_crea(&user1, &test_setup.uuid1, &ChannelCreate {
            name: "testchannel3".to_string()
        })).unwrap();
        let msg = aw!(test_setup.receiver1.recv()).unwrap();
        if let InternalMsg::SENDCTRL(hmsg) = msg {
            if let Some(Msg::ChanCreaResult(r)) = hmsg.msg {
                assert!(r.success);
                assert_eq!("testchannel3".to_string(), r.name);
                assert!(r.channel_id.len() == 16);
            } else {
                assert!(false, "Wrong Message !");
            }
        } else {
            assert!(false, "Wrong Message !");
        }
    }

    #[tokio::test]
    async fn test_chan_crea_fail() {
        let mut test_setup = setup_server();
        let user1 = {
            test_setup.server.lock().unwrap().clients.get(&test_setup.uuid1).unwrap().clone()
        };
        aw!(handle_chan_crea(&user1, &test_setup.uuid1, &ChannelCreate {
            name: "testchannel1".to_string()
        })).unwrap();
        let msg = aw!(test_setup.receiver1.recv()).unwrap();
        if let InternalMsg::SENDCTRL(hmsg) = msg {
            if let Some(Msg::ChanCreaResult(r)) = hmsg.msg {
                assert_eq!(false, r.success);
                assert_eq!("testchannel1".to_string(), r.name);
                assert_eq!(&test_setup.uuid1, &Uuid::from_slice(&r.channel_id[..]).unwrap());
            } else {
                assert!(false, "Wrong Message !");
            }
        } else {
            assert!(false, "Wrong Message !");
        }
    }

    #[tokio::test]
    async fn test_chan_del_succ() {
        let mut test_setup = setup_server();
        let user1 = {
            test_setup.server.lock().unwrap().clients.get(&test_setup.uuid1).unwrap().clone()
        };
        aw!(handle_chan_del(&user1, &test_setup.uuid1, &ChannelDelete {
            name: "testchannel2".to_string()
        })).unwrap();
        let msg = aw!(test_setup.receiver1.recv()).unwrap();
        if let InternalMsg::SENDCTRL(hmsg) = msg {
            if let Some(Msg::ChanDelResult(r)) = hmsg.msg {
                assert_eq!(true, r.success);
                assert_eq!("testchannel2".to_string(), r.name);
                assert_eq!(&test_setup.uuid2, &Uuid::from_slice(&r.channel_id[..]).unwrap());
            } else {
                assert!(false, "Wrong Message !");
            }
        } else {
            assert!(false, "Wrong Message !");
        }
        assert_eq!(1, test_setup.server.lock().unwrap().channels.len());
    }

    #[tokio::test]
    async fn test_chan_del_fail() {
        let mut test_setup = setup_server();
        let user1 = {
            test_setup.server.lock().unwrap().clients.get(&test_setup.uuid1).unwrap().clone()
        };
        aw!(handle_chan_del(&user1, &test_setup.uuid1, &ChannelDelete {
            name: "testchannel99".to_string()
        })).unwrap();
        let msg = aw!(test_setup.receiver1.recv()).unwrap();
        if let InternalMsg::SENDCTRL(hmsg) = msg {
            if let Some(Msg::ChanDelResult(r)) = hmsg.msg {
                assert_eq!(false, r.success);
                assert_eq!("testchannel99".to_string(), r.name);
                assert_eq!(&Vec::<u8>::new(), &r.channel_id);
            } else {
                assert!(false, "Wrong Message !");
            }
        } else {
            assert!(false, "Wrong Message !");
        }
        assert_eq!(2, test_setup.server.lock().unwrap().channels.len());
    }

    #[tokio::test]
    async fn test_chan_join_succ() {
        let mut test_setup = setup_server();
        let user1 = {
            test_setup.server.lock().unwrap().clients.get(&test_setup.uuid1).unwrap().clone()
        };
        aw!(handle_chan_join(&user1, &test_setup.uuid1, &ChannelJoin {
            name: "testchannel1".to_string()
        })).unwrap();
        let msg = aw!(test_setup.receiver1.recv()).unwrap();
        if let InternalMsg::SENDCTRL(hmsg) = msg {
            if let Some(Msg::ChanJoinResult(r)) = hmsg.msg {
                assert_eq!(true, r.success);
                assert_eq!(&test_setup.uuid1, &Uuid::from_slice(&r.channel_id[..]).unwrap());
            } else {
                assert!(false, "Wrong Message !");
            }
        } else {
            assert!(false, "Wrong Message !");
        }
        let server = test_setup.server.lock().unwrap();
        assert!(server.channels.get(&test_setup.uuid1).unwrap().users.contains(&test_setup.uuid1));
        assert_eq!(false, server.channels.get(&test_setup.uuid2).unwrap().users.contains(&test_setup.uuid1));
    }

    #[tokio::test]
    async fn test_chan_join_fail() {
        let mut test_setup = setup_server();
        let user1 = {
            test_setup.server.lock().unwrap().clients.get(&test_setup.uuid1).unwrap().clone()
        };
        aw!(handle_chan_join(&user1, &test_setup.uuid1, &ChannelJoin {
            name: "testchannel99".to_string()
        })).unwrap();
        let msg = aw!(test_setup.receiver1.recv()).unwrap();
        if let InternalMsg::SENDCTRL(hmsg) = msg {
            if let Some(Msg::ChanJoinResult(r)) = hmsg.msg {
                assert_eq!(false, r.success);
                assert_eq!(&Vec::<u8>::new(), &r.channel_id);
            } else {
                assert!(false, "Wrong Message !");
            }
        } else {
            assert!(false, "Wrong Message !");
        }
        let server = test_setup.server.lock().unwrap();
        assert_eq!(false, server.channels.get(&test_setup.uuid2).unwrap().users.contains(&test_setup.uuid1));
    }

    #[tokio::test]
    async fn test_chan_part_succ() {
        let mut test_setup = setup_server();
        let user1 = {
            test_setup.server.lock().unwrap().clients.get(&test_setup.uuid1).unwrap().clone()
        };
        aw!(handle_chan_part(&user1, &test_setup.uuid1, &ChannelPart {})).unwrap();
        let msg = aw!(test_setup.receiver1.recv()).unwrap();
        if let InternalMsg::SENDCTRL(hmsg) = msg {
            if let Some(Msg::ChanPartResult(r)) = hmsg.msg {
                assert_eq!(true, r.success);
                assert_eq!(&test_setup.uuid2, &Uuid::from_slice(&r.channel_id[..]).unwrap());
                assert_eq!(&"testchannel2".to_string(), &r.name);
            } else {
                assert!(false, "Wrong Message !");
            }
        } else {
            assert!(false, "Wrong Message !");
        }
        let server = test_setup.server.lock().unwrap();
        assert_eq!(false, server.channels.get(&test_setup.uuid2).unwrap().users.contains(&test_setup.uuid1));
        assert_eq!(false, server.channels.get(&test_setup.uuid1).unwrap().users.contains(&test_setup.uuid1));
    }

    #[tokio::test]
    async fn test_chan_part_fail() {
        let mut test_setup = setup_server();
        let user3 = {
            test_setup.server.lock().unwrap().clients.get(&test_setup.uuid3).unwrap().clone()
        };
        aw!(handle_chan_part(&user3, &test_setup.uuid1, &ChannelPart {})).unwrap();
        let msg = aw!(test_setup.receiver3.recv()).unwrap();
        if let InternalMsg::SENDCTRL(hmsg) = msg {
            if let Some(Msg::ChanPartResult(r)) = hmsg.msg {
                assert_eq!(false, r.success);
                assert_eq!(Vec::<u8>::new(), &r.channel_id[..]);
                assert_eq!(&"".to_string(), &r.name);
            } else {
                assert!(false, "Wrong Message !");
            }
        } else {
            assert!(false, "Wrong Message !");
        }
    }

    #[tokio::test]
    async fn test_chan_lst() {
        let mut test_setup = setup_server();
        let user1 = {
            test_setup.server.lock().unwrap().clients.get(&test_setup.uuid1).unwrap().clone()
        };
        aw!(handle_chan_lst(&user1, &test_setup.uuid1, &ChannelList {})).unwrap();
        let msg = aw!(test_setup.receiver1.recv()).unwrap();
        if let InternalMsg::SENDCTRL(hmsg) = msg {
            if let Some(Msg::ChanLstResult(r)) = hmsg.msg {
                let channels: HashSet<String> = r.channel.iter().map(|e| e.name.clone()).collect();
                let expected: HashSet<String> = ["testchannel2".to_string(), "testchannel1".to_string()].iter().map(|e| e.clone()).collect();
                assert_eq!(&expected, &channels);
            } else {
                assert!(false, "Wrong Message !");
            }
        } else {
            assert!(false, "Wrong Message !");
        }
    }

    #[tokio::test]
    async fn test_chan_status() {
        let mut test_setup = setup_server();
        let user1 = {
            test_setup.server.lock().unwrap().clients.get(&test_setup.uuid1).unwrap().clone()
        };
        aw!(handle_chan_status(&user1, &test_setup.uuid1, &ChannelStatus { name: "testchannel2".to_string() })).unwrap();
        let msg = aw!(test_setup.receiver1.recv()).unwrap();
        if let InternalMsg::SENDCTRL(hmsg) = msg {
            if let Some(Msg::ChanStatusResult(r)) = hmsg.msg {
                assert_eq!(&"testchannel2".to_string(), &r.name);
                let users: HashSet<String> = r.user.iter().map(|e| e.name.clone()).collect();
                let expected: HashSet<String> = ["testuser1".to_string(), "testuser2".to_string()].iter().map(|e| e.clone()).collect();
                assert_eq!(&expected, &users);
            } else {
                assert!(false, "Wrong Message !");
            }
        } else {
            assert!(false, "Wrong Message !");
        }
    }

    #[tokio::test]
    async fn test_status() {
        let mut test_setup = setup_server();
        let user1 = {
            test_setup.server.lock().unwrap().clients.get(&test_setup.uuid1).unwrap().clone()
        };
        aw!(handle_status(&user1, &test_setup.uuid1, &Status {})).unwrap();
        let msg = aw!(test_setup.receiver1.recv()).unwrap();
        let client = user1.lock().unwrap();
        if let InternalMsg::SENDCTRL(hmsg) = msg {
            if let Some(Msg::StatusResult(r)) = hmsg.msg {
                assert_eq!(&test_setup.uuid1, &Uuid::from_slice(&r.connection_id[..]).unwrap());
                assert_eq!(&client.username, &r.name);
                assert_eq!(&"testchannel2".to_string(), &r.channel);
            } else {
                assert!(false, "Wrong Message !");
            }
        } else {
            assert!(false, "Wrong Message !");
        }
    }


    #[test]
    fn testheader() {
        let header = StreamHeader {
            magic: 0x00008A71,
            length: 12345,
        };
        println!("Header size: {}", header.encoded_len());
    }
}