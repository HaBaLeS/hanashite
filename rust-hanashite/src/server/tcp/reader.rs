use crate::error::Error;
use crate::protos::*;
use crate::protos::hanmessage::*;
use crate::protos::hanmessage::han_message::Msg;
use super::HanMessageCodec;
use tokio::net::tcp::ReadHalf;
use tokio::stream::StreamExt;
use tokio_util::codec::FramedRead;
use tracing::{trace, info};


impl super::ClientConnection {
    pub async fn client_reader(&self, tcp_reader: ReadHalf<'_>)
                           -> Result<(), Error> {
        let mut reader = FramedRead::new(tcp_reader, HanMessageCodec());
        let mut term_receiver = self.term_sender.subscribe();
        loop {
            tokio::select!(
                msg = reader.next() => self.receive_message(msg).await?,
                _ = term_receiver.recv() => {
                    info!("Connection reader terminated");
                    break;
                }
            )
        }
        Ok(())
    }

    async fn receive_message(&self, msg: Option<Result<Box<HanMessage>, Error>>) -> Result<(), Error> {
        match msg {
            Some(Ok(result)) => {
                trace!("Message received");
                self.process_message(&result).await
            }
            Some(Err(e)) => {
                trace!("Error {}", e.to_string());
                self.term_sender.send(())?;
                Err(e)
            }
            None => {
                info!("Connection closed");
                self.term_sender.send(())?;
                Ok(())
            }
        }
    }

    async fn process_message(&self, msg: &Box<HanMessage>) -> Result<(), Error> {
        let message_id = try_uuid(&msg.message_id);
        match &msg.msg {
            Some(Msg::Auth(msg)) => self.handle_auth(&message_id, &msg).await,
            Some(Msg::ChanCrea(msg)) => self.handle_chan_crea(&message_id, &msg).await,
            Some(Msg::ChanDel(msg)) => self.handle_chan_del(&message_id, msg).await,
            Some(Msg::ChanJoin(msg)) => self.handle_chan_join(&message_id, &msg).await,
            Some(Msg::ChanPart(msg)) => self.handle_chan_part(&message_id, &msg).await,
            Some(Msg::ChanLst(msg)) => self.handle_chan_lst(&message_id, &msg).await,
            Some(Msg::ChanStatus(msg)) => self.handle_chan_status(&message_id, &msg).await,
            Some(Msg::Status(msg)) => self.handle_status(&message_id, &msg).await,
            Some(_) => self.handle_illegal_msg(&message_id, "Illegal Message Received").await,
            None => self.handle_illegal_msg(&message_id, "Empty message").await
        }
    }
}