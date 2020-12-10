use crate::error::Error;
use uuid::Uuid;
use crate::server::{AuthStatus, Server, ControlMessage};
use hanashite_message::protos::hanmessage::*;
use hanashite_message::protos::hanmessage::auth_response::ResultState;
use sodiumoxide::crypto::sign::PublicKey;
use hanashite_message::protos::build_message;
use hanashite_message::protos::hanmessage::han_message::Msg;
use crate::server::auth::AuthServer;

impl<T: Server + AuthServer> super::Reader<T> {
    pub async fn handle_auth(&self, message_id: &Option<Uuid>, msg: &AuthCmd) -> Result<(), Error> {
        let sender = self.server.connection_sender(&self.connection_id)?;
        if let Some(key) = PublicKey::from_slice(msg.public_key.as_slice()) {
            let result = self.server.auth_request(&self.connection_id, &msg.username, &key)?;
            sender.send(ControlMessage::SENDCTRL(match result {
                AuthStatus::KeyChallenge(challenge) =>
                    build_message(message_id, Msg::Challenge(ChallengeCmd {
                        chellange: challenge
                    })),
                _ => return Err(Error::InternalError("Impossible Error".to_string()))
            })).await?;
        } else {
            sender.send(ControlMessage::SENDCTRL(build_message(message_id, Msg::AuthResponse(AuthResponse {
                result: ResultState::BrokenKey as i32,
                user_id: 0,
                message: "Illegal Public Key".to_string(),
            })),
            )).await?;
        }
        Ok(())
    }

    pub async fn handle_challenge_response(&self, message_id: &Option<Uuid>, msg: &ChallengeResponse) -> Result<(), Error> {
        let result = self.server.challenge_result_request(&self.connection_id, &msg.signature)?;
        let sender = self.server.connection_sender(&self.connection_id)?;
        sender.send(ControlMessage::SENDCTRL(match result {
            AuthStatus::Success(user_id, _user_name) => build_message(message_id, Msg::AuthResponse(AuthResponse {
                result: ResultState::Success as i32,
                user_id,
                message: "".to_string(),
            })),
            AuthStatus::Failed => build_message(message_id, Msg::AuthResponse(AuthResponse {
                result: ResultState::InvalidCredentials as i32,
                user_id: 0,
                message: "Illegal Signature".to_string(),
            })),
            _ => return Err(Error::InternalError("Impossible Error".to_string()))
        })).await?;
        // TODO New User Event.
        Ok(())
    }

}

#[cfg(test)]
mod test {
    use super::*;
    use tokio;
    use tokio::sync::mpsc;
    use mockall::*;
    use prost::alloc::sync::Arc;
    use tokio::sync::mpsc::channel;
    use assert_matches::assert_matches;

    mock! {
        Srv {}

        pub  trait Server {
            fn connection_sender(&self, connection_id: &Uuid) -> Result<mpsc::Sender<ControlMessage>, Error>;
            fn terminate_connection(&self, connection_id: &Uuid);
            fn voice_channel_connections(&self, channel_name: &String) -> Result<Vec<Uuid>, Error>;
        }

        pub trait AuthServer {
            fn auth_request(&self, connection_id: &Uuid, user_name: &str, public_key: &PublicKey) -> Result<AuthStatus, Error>;
            fn challenge_result_request(&self, connection_id: &Uuid, signature: &Vec<u8>) -> Result<AuthStatus, Error>;
        }
    }

    #[tokio::test]
    async fn test_handle_auth_fail() {
        let mut server = MockSrv::new();
        let (sender, mut receiver) = channel::<ControlMessage>(100);
        server.expect_connection_sender()
            .returning(move |_| Ok(sender.clone()));
        let reader = crate::tcp::reader::Reader {
            server: Arc::new(server),
            connection_id: Uuid::new_v4(),
        };
        reader.handle_auth(&None, &AuthCmd {
            username: "name".to_string(),
            public_key: vec![],
        }).await.unwrap();

        let msg = receiver.try_recv().unwrap();
        println!("{:?}", &msg);
        assert_matches!(msg,  ControlMessage::SENDCTRL(msg) => {
            assert_matches!(msg.msg,  Some(Msg::AuthResponse(data)) if  data.result == ResultState::BrokenKey as i32);
        });
    }

    #[tokio::test]
    async fn test_handle_auth() {
        let mut server = MockSrv::new();
        let (public, _) = sodiumoxide::crypto::sign::gen_keypair();
        let (sender, mut receiver) = channel::<ControlMessage>(100);
        server.expect_connection_sender()
            .returning(move |_| Ok(sender.clone()));
        server.expect_auth_request()
            .returning(|_, _, _| Ok(AuthStatus::KeyChallenge(vec![13; 16])));
        let reader = crate::tcp::reader::Reader {
            server: Arc::new(server),
            connection_id: Uuid::new_v4(),
        };
        reader.handle_auth(&None, &AuthCmd {
            username: "name".to_string(),
            public_key: Vec::from(&public.0[..]),
        }).await.unwrap();
        let msg = receiver.try_recv().unwrap();
        assert_matches!(msg,  ControlMessage::SENDCTRL(msg) => {
            assert_matches!(msg.msg,  Some(Msg::Challenge(data)) if  &data.chellange == &vec![13;16]);
        });
    }

    #[tokio::test]
    async fn test_handle_challenge_response() {
        let mut server = MockSrv::new();
        let (sender, mut receiver) = channel::<ControlMessage>(100);
        server.expect_connection_sender()
            .returning(move |_| Ok(sender.clone()));
        server.expect_challenge_result_request()
            .returning(|_,_| Ok(AuthStatus::Success(42, "_name".to_string())));
        let reader = crate::tcp::reader::Reader {
            server: Arc::new(server),
            connection_id: Uuid::new_v4(),
        };
        reader.handle_challenge_response(&None, &ChallengeResponse {
            signature: vec![1,2,3,4]
        }).await.unwrap();
        let msg = receiver.try_recv().unwrap();
        assert_matches!(msg,  ControlMessage::SENDCTRL(msg) => {
            assert_matches!(msg.msg,  Some(Msg::AuthResponse(data)) if  data.result == ResultState::Success as i32);
        });
    }

    #[tokio::test]
    async fn test_handle_challenge_response_fail() {
        let mut server = MockSrv::new();
        let (sender, mut receiver) = channel::<ControlMessage>(100);
        server.expect_connection_sender()
            .returning(move |_| Ok(sender.clone()));
        server.expect_challenge_result_request()
            .returning(|_,_| Ok(AuthStatus::Failed));
        let reader = crate::tcp::reader::Reader {
            server: Arc::new(server),
            connection_id: Uuid::new_v4(),
        };
        reader.handle_challenge_response(&None, &ChallengeResponse {
            signature: vec![1,2,3,4]
        }).await.unwrap();
        let msg = receiver.try_recv().unwrap();
        assert_matches!(msg,  ControlMessage::SENDCTRL(msg) => {
            assert_matches!(msg.msg,  Some(Msg::AuthResponse(data)) if  data.result == ResultState::InvalidCredentials as i32);
        });
    }
}