use uuid::Uuid;

use hanashite_message::protos::build_message;
use hanashite_message::protos::hanmessage::*;
use hanashite_message::protos::hanmessage::han_message::Msg;

use crate::error::Error;
use crate::server::{ControlMessage, Server};
use crate::server::channel::{ChannelServer, JoinStatus};

impl<T: Server + ChannelServer> super::Reader<T> {
    pub async fn handle_voice_channel_join(&self, message_id: &Option<Uuid>, msg: &VoiceChannelJoinCmd) -> Result<(), Error> {
        let result = self.server.voice_channel_join_request(&self.connection_id, &msg.name)?;
        match result {
            JoinStatus::Joined(channel_id, old_channels) =>
                self.voice_join_success(message_id, &msg.name, channel_id, &old_channels).await?,
            JoinStatus::NotFound => self.voice_join_not_found(message_id).await?,
            _ => return Err(Error::InternalError("Impossible Error".to_string()))
        }
        Ok(())
    }

    async fn voice_join_success(&self, message_id: &Option<Uuid>, channel_name: &String, channel_id: Vec<u8>,
                                old_channels: &Vec<String>) -> Result<(), Error> {
        let sender = self.server.connection_sender(&self.connection_id)?;
        sender.send(ControlMessage::SENDCTRL(
            build_message(message_id, Msg::VoiceChannelJoinResponse(VoiceChannelJoinResponse {
                channel_id,
                success: true,
                message: "".to_string(),
            })))).await?;
        println!("{}", &channel_name);
        if let Ok(users) = self.server.voice_channel_connections(channel_name) {
            sender.send(ControlMessage::SENDCTRL(
                build_message(message_id, Msg::VoiceChannelJoinEvent(VoiceChannelJoinEvent {
                    channel_name: channel_name.clone(),
                    user_id: 0,
                    user_name: "".to_string(),
                })))).await?;
        }
        for channel in old_channels {
            if let Ok(connections) = self.server.voice_channel_connections(channel) {
                for old_connection in &connections {
                    println!("cpm: {}", old_connection);
                    if let Ok(old_sender) = self.server.connection_sender(old_connection) {
                        old_sender.send(ControlMessage::SENDCTRL(
                            build_message(message_id, Msg::VoiceChannelLeaveEvent(VoiceChannelLeaveEvent {
                                channel_name: channel.clone(),
                                user_id: 0,
                                user_name: "".to_string(),
                            })))).await?;
                    }
                }
            }
        }
        Ok(())
    }

    async fn voice_join_not_found(&self, message_id: &Option<Uuid>) -> Result<(), Error> {
        let sender = self.server.connection_sender(&self.connection_id)?;
        sender.send(ControlMessage::SENDCTRL(
            build_message(message_id,
                          Msg::VoiceChannelJoinResponse(VoiceChannelJoinResponse {
                              channel_id: vec![],
                              success: false,
                              message: "Channel does not exist.".to_string(),
                          })))).await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use mockall::*;
    use tokio::sync::mpsc;
    use std::sync::Arc;
    use assert_matches::assert_matches;

    mock! {
        Srv {}

        pub  trait Server {
            fn connection_sender(&self, connection_id: &Uuid) -> Result<mpsc::Sender<ControlMessage>, Error>;
            fn terminate_connection(&self, connection_id: &Uuid);
            fn voice_channel_connections(&self, channel_name: &String) -> Result<Vec<Uuid>, Error>;
        }

        pub trait ChannelServer {
            fn voice_channel_join_request(&self, connection_id: &Uuid, channel_name: &str) -> Result<JoinStatus, Error>;
        }
    }

    #[tokio::test]
    async fn test_voice_channel_join() {
        let mut server = MockSrv::new();
        let (sender, mut receiver) = mpsc::channel::<ControlMessage>(100);
        let (sender2, mut receiver2) = mpsc::channel::<ControlMessage>(100);
        let con_id = Uuid::new_v4();
        let con_id_cln = con_id.clone();
        let con_id2 = Uuid::new_v4();
        let con_id2_cln = con_id2.clone();
        println!("1: {}, 2: {}", &con_id, &con_id2);
        server.expect_connection_sender()
            .withf(move |x| &con_id_cln == x)
            .returning(move |_| Ok(sender.clone()));
        server.expect_connection_sender()
            .withf(move |x| &con_id2_cln == x)
            .returning(move |_| Ok(sender2.clone()));
        server.expect_voice_channel_join_request()
            .returning(|_, _| Ok(JoinStatus::Joined(vec![1, 2, 3], vec!["testchannel3".to_string()])));
        server.expect_voice_channel_connections()
            .withf(|n| n == &"testchannel2".to_string())
            .returning(|_| Ok(vec![]));
        server.expect_voice_channel_connections()
            .withf(|n| n == &"testchannel3".to_string())
            .returning(move |_| Ok(vec![con_id2]));
        let reader = crate::tcp::reader::Reader {
            server: Arc::new(server),
            connection_id: con_id,
        };

        reader.handle_voice_channel_join(&None, &VoiceChannelJoinCmd {
            name: "testchannel2".to_string()
        }).await.unwrap();

        let msg = receiver.try_recv().unwrap();
        println!("{:?}", &msg);
        assert_matches!(msg,  ControlMessage::SENDCTRL(msg) => {
            assert_matches!(msg.msg,  Some(Msg::VoiceChannelJoinResponse(data)) => {
                assert_eq!(vec![1,2,3], data.channel_id);
                assert!(data.success);
            });
        });
        let msg = receiver2.try_recv().unwrap();
        println!("{:?}", &msg);
        assert_matches!(msg,  ControlMessage::SENDCTRL(msg) => {
            assert_matches!(msg.msg,  Some(Msg::VoiceChannelLeaveEvent(data)) => {
                assert_eq!(data.);
                assert!(data.success);
            });
        });
    }

    #[tokio::test]
    async fn test_voice_channel_join_fail() {
        let mut server = MockSrv::new();
        let (sender, mut receiver) = mpsc::channel::<ControlMessage>(100);
        server.expect_connection_sender()
            .returning(move |_| Ok(sender.clone()));
        server.expect_voice_channel_join_request()
            .returning(|_, _| Ok(JoinStatus::NotFound));
        let reader = crate::tcp::reader::Reader {
            server: Arc::new(server),
            connection_id: Uuid::new_v4(),
        };

        reader.handle_voice_channel_join(&None, &VoiceChannelJoinCmd {
            name: "testchannel2".to_string()
        }).await.unwrap();

        let msg = receiver.try_recv().unwrap();
        println!("{:?}", &msg);
        assert_matches!(msg,  ControlMessage::SENDCTRL(msg) => {
            assert_matches!(msg.msg,  Some(Msg::VoiceChannelJoinResponse(data)) => {
                assert!(!data.success);
                assert!(data.message.len() > 0);
            });
        });
    }
}