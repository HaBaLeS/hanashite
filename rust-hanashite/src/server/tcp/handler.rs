use crate::error::Error;
use crate::protos::{to_data_opt, to_data};
use crate::protos::hanmessage::*;
use crate::protos::hanmessage::han_message::Msg;
use std::sync::Arc;
use tracing::{warn};
use uuid::Uuid;
use crate::server::{Role, ControlMessage};
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
        con.check_permission(&Role::PreAuth)?;
        let connections = self.server.connections.lock().unwrap();
        let success = if let None = connections.values()
            //Skip current connection
            .filter(|con| !Arc::ptr_eq(&self.connection, *con))
            .filter(|con| con.lock().unwrap().user_name == msg.username)
            .nth(0) {
            false
        } else {
            con.user_name = msg.username.clone();
            con.roles.insert(Role::User);
            con.roles.remove(&Role::PreAuth);
            true
        };
        let msg = Box::new(HanMessage {
            message_id: to_data_opt(message_id),
            msg: Some(Msg::AuthResult(AuthResult {
                success,
                connection_id: to_data(&con.connection_id),
            })),
        });
        let sender = con.sender.clone();
        send_logged(sender, ControlMessage::SENDCTRL(msg));
        Ok(())
    }

    pub async fn handle_chan_crea(&self, _message_id: &Option<Uuid>, _msg: &ChannelCreate) -> Result<(), Error> {
        Ok(())
    }

    pub async fn handle_chan_del(&self, _message_id: &Option<Uuid>, _msg: &ChannelDelete) -> Result<(), Error> {
        Ok(())
    }

    pub async fn handle_chan_join(&self, _message_id: &Option<Uuid>, _msg: &ChannelJoin) -> Result<(), Error> {
        Ok(())
    }

    pub async fn handle_chan_part(&self, _message_id: &Option<Uuid>, _msg: &ChannelPart) -> Result<(), Error> {
        Ok(())
    }

    pub async fn handle_chan_lst(&self, _message_id: &Option<Uuid>, _msg: &ChannelList) -> Result<(), Error> {
        Ok(())
    }

    pub async fn handle_chan_status(&self, _message_id: &Option<Uuid>, _msg: &ChannelStatus) -> Result<(), Error> {
        Ok(())
    }

    pub async fn handle_status(&self, _message_id: &Option<Uuid>, _msg: &Status) -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::server::{Server, Connection, Channel};
    use std::sync::{Arc, Mutex};
    use std::collections::HashMap;
    use std::path::Path;
    use tokio::sync::oneshot::channel;
    use std::collections::HashSet;
    use std::iter::FromIterator;
    use std::net::SocketAddr;
    use std::str::FromStr;

    struct TestData {
        user_name: [String; 3],
        user_id: [Uuid; 3],
        user_sender: [tokio::sync::mpsc::Sender<ControlMessage>; 3],
        user_receiver: [tokio::sync::mpsc::Receiver<ControlMessage>; 3],
        channel_name: [String; 2],
        channel_id: [Uuid; 2],
        server: Server,
    }

    fn setup_test_data() -> TestData {
        let (s, _) = tokio::sync::broadcast::channel(10);
        let (s1, r1) = tokio::sync::mpsc::channel(10);
        let (s2, r2) = tokio::sync::mpsc::channel(10);
        let (s3, r3) = tokio::sync::mpsc::channel(10);
        let data = TestData {
            user_name: ["testuser1".to_string(), "testuser2".to_string(), "testuser3".to_string()],
            user_id: [Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()],
            user_sender: [s1, s2, s3],
            user_receiver: [r1, r2, r3],
            channel_name: ["testchannel1".to_string(), "testchannel2".to_string()],
            channel_id: [Uuid::new_v4(), Uuid::new_v4()],
            server: Server {
                connections: Mutex::new(HashMap::new()),
                channels: Mutex::new(HashMap::new()),
                config: crate::configuration::init(Path::new("")),
                shutdown_sender: s,
            },
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
                addr: SocketAddr::from_str("127.0.0.1:1").unwrap()
            })));
            con.insert(data.user_id[1].clone(), Arc::new(Mutex::new(Connection {
                user_name: data.user_name[1].clone(),
                sender: data.user_sender[1].clone(),
                connection_id: data.user_id[1].clone(),
                roles: vec![Role::User].into_iter().collect(),
                public_key: None,
                addr: SocketAddr::from_str("127.0.0.1:2").unwrap()
            })));
            con.insert(data.user_id[2].clone(), Arc::new(Mutex::new(Connection {
                user_name: data.user_name[2].clone(),
                sender: data.user_sender[2].clone(),
                connection_id: data.user_id[2].clone(),
                roles: vec![Role::PreAuth].into_iter().collect(),
                public_key: None,
                addr: SocketAddr::from_str("127.0.0.1:3").unwrap()
            })));
        }
        data
    }

    #[tokio::test]
}