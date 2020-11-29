use crate::error::Error;
use crate::server::ControlMessage;
use super::HanMessageCodec;
use tracing::{info, warn, trace};
use tokio::net::tcp::WriteHalf;
use tokio::sync::mpsc::Receiver;
use tokio_util::codec::FramedWrite;
use futures::{SinkExt};

impl super::ClientConnection {
    pub async fn client_writer(&self, tcp_writer: WriteHalf<'_>, mut receiver: Receiver<ControlMessage>)
                               -> Result<(), Error> {
        let mut writer = FramedWrite::new(tcp_writer, HanMessageCodec());
        let mut term_receiver = self.term_sender.subscribe();
        loop {
            tokio::select!(
                msg = receiver.recv() => self.process_ctrl_message(msg, &mut writer).await?,
                _ = term_receiver.recv() => {
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
                self.term_sender.send(()).unwrap_or(0);
                Err(Error::InternalError("Internal Receiver terminated on Client Connection".to_string()))
            }
            Some(ControlMessage::DISCONNECT) => {
                info!("Disconnect command received.");
                self.term_sender.send(()).unwrap_or(0);
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
