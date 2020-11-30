use crate::error::Error;
use crate::protos::{to_data_opt, to_data};
use crate::protos::hanmessage::*;
use crate::protos::hanmessage::han_message::Msg;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::sync::{Mutex, Arc};
use tracing::{warn};
use uuid::Uuid;
use crate::server::{Role, ControlMessage, Channel};
use tokio::sync::mpsc::Sender;


fn send_logged(sender: Sender<ControlMessage>, msg: ControlMessage) {
    tokio::spawn(async move {
        if let Err(err) = sender.send(msg).await {
            warn!("Error Sending Controlmessage: {}", err);
        }
    });
}

impl super::ClientConnection {
    pub async fn handle_illegal_msg(&self, _message_id: &Option<Uuid>, message: &str) -> Result<(), Error> {
        let msg = format!("Illegal message: {}", message);
        warn!("{}", &msg);
        Err(Error::InternalError(msg))
    }

    pub async fn handle_auth(&self, message_id: &Option<Uuid>, msg: &Auth) -> Result<(), Error> {
        let mut con = self.connection.lock().unwrap();
        if !con.roles.contains(&Role::PreAuth) {
            return Err(Error::PermissionDenied);
        }
        let connections = self.server.connections.lock().unwrap();
        let success = if let None = connections.values()
            //Skip current connection
            .filter(|con| !Arc::ptr_eq(&self.connection, *con))
            .filter(|con| con.lock().unwrap().user_name == msg.username)
            .nth(0) {
            con.user_name = msg.username.clone();
            con.roles.insert(Role::User);
            con.roles.remove(&Role::PreAuth);
            true
        } else {
            false
        };
        let result = Box::new(HanMessage {
            message_id: to_data_opt(message_id),
            msg: Some(Msg::AuthResult(AuthResult {
                success,
                connection_id: to_data(&con.connection_id),
            })),
        });
        send_logged(con.sender.clone(), ControlMessage::SENDCTRL(result));
        Ok(())
    }

    pub async fn handle_chan_crea(&self, message_id: &Option<Uuid>, msg: &ChannelCreate) -> Result<(), Error> {
        let con = self.connection.lock().unwrap();
        con.check_permission(&Role::User)?;
        let mut channels = self.server.channels.lock().unwrap();
        let (succ, channel_id) = if channels.contains_key(&msg.name) {
            (false, Some(channels.get(&msg.name).unwrap().lock().unwrap().channel_id.clone()))
        } else {
            let channel_id = Uuid::new_v4();
            channels.insert(msg.name.clone(), Arc::new(Mutex::new(Channel {
                channel_id: channel_id.clone(),
                connections: HashSet::new(),
                silent: false,
                private: false,
                channel_name: msg.name.clone(),
            })));
            // TODO new Channel Event
            (true, Some(channel_id))
        };
        let result = Box::new(HanMessage {
            message_id: to_data_opt(message_id),
            msg: Some(Msg::ChanCreaResult(ChannelCreateResult {
                name: msg.name.clone(),
                success: succ,
                channel_id: to_data_opt(&channel_id),
            })),
        });
        send_logged(con.sender.clone(), ControlMessage::SENDCTRL(result));
        Ok(())
    }

    pub async fn handle_chan_del(&self, message_id: &Option<Uuid>, msg: &ChannelDelete) -> Result<(), Error> {
        let con = self.connection.lock().unwrap();
        con.check_permission(&Role::User)?;
        let mut channels = self.server.channels.lock().unwrap();
        let (succ, channel_id) = if let Some(chan) = channels.remove(&msg.name) {
            let channel = chan.lock().unwrap();
            let _connections =
                Vec::from_iter(channel.connections.iter().map(|uuid| uuid.clone()));
            // TODO EVENT to connections
            (true, Some(channel.channel_id.clone()))
        } else {
            (false, None)
        };
        let result = Box::new(HanMessage {
            message_id: to_data_opt(message_id),
            msg: Some(Msg::ChanDelResult(ChannelDeleteResult {
                name: msg.name.clone(),
                success: succ,
                channel_id: to_data_opt(&channel_id),
            })),
        });
        send_logged(con.sender.clone(), ControlMessage::SENDCTRL(result));
        Ok(())
    }

    pub async fn handle_chan_join(&self, _message_id: &Option<Uuid>, _msg: &ChannelJoin) -> Result<(), Error> {
        todo!()
    }

    pub async fn handle_chan_part(&self, _message_id: &Option<Uuid>, _msg: &ChannelPart) -> Result<(), Error> {
        todo!()
    }

    pub async fn handle_chan_lst(&self, _message_id: &Option<Uuid>, _msg: &ChannelList) -> Result<(), Error> {
        todo!()
    }

    pub async fn handle_chan_status(&self, _message_id: &Option<Uuid>, _msg: &ChannelStatus) -> Result<(), Error> {
        todo!()
    }

    pub async fn handle_status(&self, _message_id: &Option<Uuid>, _msg: &Status) -> Result<(), Error> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::server::{Server, Connection, Channel};
    use std::sync::{Arc, Mutex};
    use std::collections::HashMap;
    use std::path::Path;
    use tokio;
    use std::collections::HashSet;
    use std::iter::FromIterator;
    use std::net::SocketAddr;
    use std::str::FromStr;
    use crate::server::tcp::ClientConnection;

    struct TestData {
        user_name: [String; 3],
        user_id: [Uuid; 3],
        user_sender: [tokio::sync::mpsc::Sender<ControlMessage>; 3],
        user_receiver: [tokio::sync::mpsc::Receiver<ControlMessage>; 3],
        channel_name: [String; 2],
        channel_id: [Uuid; 2],
        server: Arc<Server>,
    }

    fn setup_test_data() -> TestData {
        let (s, _) = tokio::sync::broadcast::channel(10);
        let (s1, r1) = tokio::sync::mpsc::channel(10);
        let (s2, r2) = tokio::sync::mpsc::channel(10);
        let (s3, r3) = tokio::sync::mpsc::channel(10);
        let data = TestData {
            user_name: ["testuser1".to_string(), "testuser2".to_string(), "".to_string()],
            user_id: [Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()],
            user_sender: [s1, s2, s3],
            user_receiver: [r1, r2, r3],
            channel_name: ["testchannel1".to_string(), "testchannel2".to_string()],
            channel_id: [Uuid::new_v4(), Uuid::new_v4()],
            server: Arc::new(Server {
                connections: Mutex::new(HashMap::new()),
                channels: Mutex::new(HashMap::new()),
                config: crate::configuration::init(Path::new("")),
                shutdown_sender: s,
            }),
        };
        {
            let mut chan = data.server.channels.lock().unwrap();
            chan.insert(data.channel_name[0].clone(), Arc::new(Mutex::new(Channel {
                channel_name: data.channel_name[0].clone(),
                channel_id: data.channel_id[0].clone(),
                connections: HashSet::from_iter(data.user_id.iter().map(|u| u.clone())),
                private: false,
                silent: false,
            })));
            chan.insert(data.channel_name[1].clone(), Arc::new(Mutex::new(Channel {
                channel_name: data.channel_name[1].clone(),
                channel_id: data.channel_id[1].clone(),
                connections: HashSet::new(),
                private: false,
                silent: false,
            })));
        }
        {
            let mut con = data.server.connections.lock().unwrap();
            con.insert(data.user_id[0].clone(), Arc::new(Mutex::new(Connection {
                user_name: data.user_name[0].clone(),
                sender: data.user_sender[0].clone(),
                connection_id: data.user_id[0].clone(),
                roles: vec![Role::User].into_iter().collect(),
                public_key: None,
                addr: SocketAddr::from_str("127.0.0.1:1").unwrap(),
            })));
            con.insert(data.user_id[1].clone(), Arc::new(Mutex::new(Connection {
                user_name: data.user_name[1].clone(),
                sender: data.user_sender[1].clone(),
                connection_id: data.user_id[1].clone(),
                roles: vec![Role::User].into_iter().collect(),
                public_key: None,
                addr: SocketAddr::from_str("127.0.0.1:2").unwrap(),
            })));
            con.insert(data.user_id[2].clone(), Arc::new(Mutex::new(Connection {
                user_name: data.user_name[2].clone(),
                sender: data.user_sender[2].clone(),
                connection_id: data.user_id[2].clone(),
                roles: vec![Role::PreAuth].into_iter().collect(),
                public_key: None,
                addr: SocketAddr::from_str("127.0.0.1:3").unwrap(),
            })));
        }
        data
    }

    fn client_connection(user: usize, test_data: &TestData) -> ClientConnection {
        let con = test_data.server.connections.lock().unwrap();
        let user_con = con.get(&test_data.user_id[user]).unwrap().clone();
        ClientConnection {
            server: test_data.server.clone(),
            term_sender: test_data.server.shutdown_sender.clone(),
            connection: user_con,
            user: None,
        }
    }

    #[tokio::test]
    async fn test_auth_succ() {
        let mut test_data = setup_test_data();
        let user_con = client_connection(2, &test_data);
        let msg_id = Some(Uuid::new_v4());
        user_con.handle_auth(&msg_id, &Auth {
            username: "testuser3".to_string()
        }).await.unwrap();
        let answer = test_data.user_receiver[2].recv().await.unwrap();
        if let ControlMessage::SENDCTRL(msg) = answer {
            if let Some(Msg::AuthResult(result)) = msg.msg {
                assert!(result.success);
                assert_eq!(to_data(&test_data.user_id[2]), result.connection_id)
            } else {
                assert!(false, "Wrong Message");
            }
        } else {
            assert!(false, "Wrong Message");
        }
        let conn = user_con.connection.lock().unwrap();
        assert_eq!("testuser3".to_string(), conn.user_name);
        assert!(conn.roles.contains(&Role::User));
        assert_eq!(false, conn.roles.contains(&Role::PreAuth));
    }

    #[tokio::test]
    async fn test_auth_alread_loggedin() {
        let test_data = setup_test_data();
        let user_con = client_connection(0, &test_data);
        let msg_id = Some(Uuid::new_v4());
        user_con.handle_auth(&msg_id, &Auth {
            username: "testuser1".to_string()
        }).await.unwrap_err();
        let conn = user_con.connection.lock().unwrap();
        assert_eq!("testuser1".to_string(), conn.user_name);
        assert!(conn.roles.contains(&Role::User));
        assert_eq!(false, conn.roles.contains(&Role::PreAuth));
    }

    #[tokio::test]
    async fn test_auth_fail() {
        let mut test_data = setup_test_data();
        let user_con = client_connection(2, &test_data);
        let msg_id = Some(Uuid::new_v4());
        user_con.handle_auth(&msg_id, &Auth {
            username: "testuser1".to_string()
        }).await.unwrap();
        let answer = test_data.user_receiver[2].recv().await.unwrap();
        if let ControlMessage::SENDCTRL(msg) = answer {
            if let Some(Msg::AuthResult(result)) = msg.msg {
                assert_eq!(false, result.success);
                assert_eq!(to_data(&test_data.user_id[2]), result.connection_id)
            } else {
                assert!(false, "Wrong Message");
            }
        } else {
            assert!(false, "Wrong Message");
        }
        let conn = user_con.connection.lock().unwrap();
        assert_eq!("".to_string(), conn.user_name);
        assert!(conn.roles.contains(&Role::PreAuth));
        assert_eq!(false, conn.roles.contains(&Role::User));
    }

    #[tokio::test]
    async fn test_chan_crea_succ() {
        let mut test_data = setup_test_data();
        let user_con = client_connection(0, &test_data);
        let msg_id = Some(Uuid::new_v4());
        user_con.handle_chan_crea(&msg_id, &ChannelCreate {
            name: "testchannel3".to_string(),
        }).await.unwrap();
        let answer = test_data.user_receiver[0].recv().await.unwrap();
        if let ControlMessage::SENDCTRL(msg) = answer {
            if let Some(Msg::ChanCreaResult(result)) = msg.msg {
                assert!(result.success);
                assert_eq!(16, result.channel_id.len());
                assert_eq!("testchannel3", result.name);
            } else {
                assert!(false, "Wrong Message");
            }
        } else {
            assert!(false, "Wrong Message");
        }
        let channels = test_data.server.channels.lock().unwrap();
        let chan = channels.get("testchannel3").unwrap().lock().unwrap();
        assert_eq!("testchannel3".to_string(), chan.channel_name);
    }

    #[tokio::test]
    async fn test_chan_crea_fail() {
        let mut test_data = setup_test_data();
        let user_con = client_connection(0, &test_data);
        let msg_id = Some(Uuid::new_v4());
        user_con.handle_chan_crea(&msg_id, &ChannelCreate {
            name: "testchannel1".to_string(),
        }).await.unwrap();
        let answer = test_data.user_receiver[0].recv().await.unwrap();
        if let ControlMessage::SENDCTRL(msg) = answer {
            if let Some(Msg::ChanCreaResult(result)) = msg.msg {
                assert_eq!(false, result.success);
                assert_eq!(to_data(&test_data.channel_id[0]), result.channel_id);
                assert_eq!("testchannel1", result.name);
            } else {
                assert!(false, "Wrong Message");
            }
        } else {
            assert!(false, "Wrong Message");
        }
    }

    #[tokio::test]
    async fn test_chan_del_succ() {
        let mut test_data = setup_test_data();
        let user_con = client_connection(0, &test_data);
        let msg_id = Some(Uuid::new_v4());
        user_con.handle_chan_del(&msg_id, &ChannelDelete {
            name: "testchannel1".to_string(),
        }).await.unwrap();
        let answer = test_data.user_receiver[0].recv().await.unwrap();
        if let ControlMessage::SENDCTRL(msg) = answer {
            if let Some(Msg::ChanDelResult(result)) = msg.msg {
                assert!(result.success);
                assert_eq!(to_data(&test_data.channel_id[0]), result.channel_id);
                assert_eq!("testchannel1", result.name);
            } else {
                assert!(false, "Wrong Message");
            }
        } else {
            assert!(false, "Wrong Message");
        }
        let channels = test_data.server.channels.lock().unwrap();
        assert!(channels.get("testchannel1").is_none());
    }

    #[tokio::test]
    async fn test_chan_del_fail() {
        let mut test_data = setup_test_data();
        let user_con = client_connection(0, &test_data);
        let msg_id = Some(Uuid::new_v4());
        user_con.handle_chan_del(&msg_id, &ChannelDelete {
            name: "testchannel4".to_string(),
        }).await.unwrap();
        let answer = test_data.user_receiver[0].recv().await.unwrap();
        if let ControlMessage::SENDCTRL(msg) = answer {
            if let Some(Msg::ChanDelResult(result)) = msg.msg {
                assert_eq!(false, result.success);
                assert_eq!(0, result.channel_id.len());
                assert_eq!("testchannel4", result.name);
            } else {
                assert!(false, "Wrong Message");
            }
        } else {
            assert!(false, "Wrong Message");
        }
    }
}