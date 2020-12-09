use hanashite_message::codec::HanMessageCodec;
use crate::error::Error;
use crate::server::{ControlMessage, Server};
use tracing::{info, warn, trace};
use tokio::net::tcp::WriteHalf;
use tokio::sync::{mpsc, broadcast};
use tokio_util::codec::FramedWrite;
use futures::{SinkExt};
use uuid::Uuid;
use std::sync::Arc;

pub struct Writer<T> {
    connection_id: Uuid,
    server: Arc<T>,
}

impl<T: Server> Writer<T> {
    pub fn new(server: &Arc<T>, connection_id: &Uuid) -> Writer<T> {
        Writer {
            connection_id: connection_id.clone(),
            server: server.clone(),
        }
    }

    pub async fn client_writer(&self, tcp_writer: WriteHalf<'_>,
                               mut receiver: mpsc::Receiver<ControlMessage>,
                               mut termination_receiver: broadcast::Receiver<()>)
                               -> Result<(), Error> {
        let mut writer = FramedWrite::new(tcp_writer, HanMessageCodec());
        loop {
            tokio::select!(
                msg = receiver.recv() => self.process_ctrl_message(msg, &mut writer).await?,
                _ = termination_receiver.recv() => {
                    info!("Connection writer terminated");
                    break;
                }
            )
        }
        Ok(())
    }

    async fn process_ctrl_message(&self, message: Option<ControlMessage>, writer: &mut FramedWrite<WriteHalf<'_>, HanMessageCodec>)
                                  -> Result<(), Error> {
        match message {
            None => {
                warn!("Internal receiver died.");
                self.server.terminate_connection(&self.connection_id);
                Err(Error::InternalError("Internal Receiver terminated on Client Connection".to_string()))
            }
            Some(ControlMessage::DISCONNECT) => {
                info!("Disconnect command received.");
                self.server.terminate_connection(&self.connection_id);
                Ok(())
            }
            Some(ControlMessage::SENDCTRL(msg)) => {
                info!("Sending Message to client !");
                trace!("Msg: {:?}", &msg);
                writer.send(msg).await?;
                Ok(())
            }
        }
    }
}
