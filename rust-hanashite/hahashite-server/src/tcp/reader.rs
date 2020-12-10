mod auth;
mod channel;

use hanashite_message::codec::HanMessageCodec;
use crate::error::Error;
use hanashite_message::protos::*;
use hanashite_message::protos::hanmessage::*;
use hanashite_message::protos::hanmessage::han_message::Msg;
use std::sync::Arc;
use tokio::net::tcp::ReadHalf;
use tokio::stream::StreamExt;
use tokio::sync::broadcast;
use tokio_util::codec::FramedRead;
use tracing::{trace, info, warn};
use crate::server::Server;
use uuid::Uuid;
use crate::server::auth::AuthServer;
use crate::server::channel::ChannelServer;

pub struct Reader<T>
{
    connection_id: Uuid,
    server: Arc<T>,
}

impl<T> Reader<T>
    where T: Server,
          T: AuthServer,
          T: ChannelServer {
    pub fn new(server: &Arc<T>, connection_id: &Uuid) -> Reader<T> {
        Reader {
            server: server.clone(),
            connection_id: connection_id.clone(),
        }
    }


    pub async fn client_reader(&self,
                               tcp_reader: ReadHalf<'_>,
                               mut termination_receiver: broadcast::Receiver<()>)
                               -> Result<(), Error> {
        let mut reader = FramedRead::new(tcp_reader, HanMessageCodec());
        loop {
            tokio::select!(
                msg = reader.next() => self.receive_message(msg).await?,
                _ = termination_receiver.recv() => {
                    info!("Connection reader terminated");
                    break;
                }
            )
        }
        Ok(())
    }

    async fn receive_message(&self, msg: Option<Result<Box<HanMessage>, std::io::Error>>) -> Result<(), Error> {
        match msg {
            Some(Ok(result)) => {
                trace!("Message received");
                self.process_message(&result).await?;
                Ok(())
            }
            Some(Err(e)) => {
                trace!("Error {}", e.to_string());
                self.server.terminate_connection(&self.connection_id);
                Err(Error::from(e))
            }
            None => {
                info!("Connection closed");
                self.server.terminate_connection(&self.connection_id);
                Ok(())
            }
        }
    }

    async fn process_message(&self, msg: &Box<HanMessage>) -> Result<(), Error> {
        let message_id = try_uuid(&msg.message_id);
        match &msg.msg {
            Some(Msg::Auth(msg)) => self.handle_auth(&message_id, &msg).await,
            Some(Msg::ChallengeResponse(msg)) => self.handle_challenge_response(&message_id, &msg).await,
            Some(Msg::VoiceChannelJoin(msg)) => self.handle_voice_channel_join(&message_id, &msg).await,
            Some(_) => self.handle_illegal_msg(&message_id, "Illegal Message Received").await,
            None => self.handle_illegal_msg(&message_id, "Empty message").await
        }
    }

    pub async fn handle_illegal_msg(&self, _message_id: &Option<Uuid>, message: &str) -> Result<(), Error> {
        let msg = format!("Illegal message: {}", message);
        warn!("{}", &msg);
        Err(Error::InternalError(msg))
    }
}