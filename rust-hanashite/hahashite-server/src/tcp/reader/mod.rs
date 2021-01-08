#[cfg(test)]
mod test;

use hanashite_message::codec::HanMessageCodec;
use crate::error::Error;
use hanashite_message::protos::*;
use hanashite_message::protos::hanmessage::*;
use std::sync::Arc;
use tokio::net::tcp::ReadHalf;
use tokio_util::codec::FramedRead;
use tokio_stream::StreamExt;
use tracing::{trace, info, warn};
use crate::server::{Server, ServerBusEndpoint, ControlMessage, ConnectionContext};
use uuid::Uuid;
use crate::bus::{MessagePredicate, BusMessage};

pub struct Reader<T>
{
    connection_id: Uuid,
    server: Arc<T>,
    endpoint: ServerBusEndpoint,
}

struct ReaderMessagePredicate {
    connection_id: Uuid,
}

impl MessagePredicate<ControlMessage, ConnectionContext> for ReaderMessagePredicate {
    fn relevant(&self, message: &BusMessage<ControlMessage, ConnectionContext>,
                context: &ConnectionContext) -> bool {
        match message.msg {
            ControlMessage::DISCONNECT(connection_id) => connection_id == self.connection_id,
            _ => false
        }
    }
}

impl<T> Reader<T>
    where T: Server {
    pub fn new(server: &Arc<T>, connection_id: &Uuid, endpoint: ServerBusEndpoint) -> Reader<T> {
        Reader {
            server: server.clone(),
            connection_id: connection_id.clone(),
            endpoint,
        }
    }


    pub async fn client_reader(&mut self,
                               tcp_reader: ReadHalf<'_>)
                               -> Result<(), Error> {
        let predicate = ReaderMessagePredicate { connection_id: self.connection_id };
        let mut reader = FramedRead::new(tcp_reader, HanMessageCodec());
        loop {
            tokio::select!(
                msg = reader.next() => self.network_message(msg).await?,
                msg = self.endpoint.recv(&predicate) => self.bus_message(msg?).await?,
            )
        }
    }

    async fn bus_message(&self, msg: Arc<BusMessage<ControlMessage, ConnectionContext>>)
                         -> Result<(), Error> {
        Ok(())
    }

    async fn network_message(&self, msg: Option<Result<Box<HanMessage>, std::io::Error>>) -> Result<(), Error> {
        match msg {
            Some(Ok(result)) => {
                trace!("Message received");
                self.process_message(&result).await?;
                Ok(())
            }
            Some(Err(e)) => {
                trace!("Error {}", e.to_string());
                self.server.terminate_connection(&self.connection_id)?;
                Err(Error::from(e))
            }
            None => {
                info!("Connection closed");
                self.server.terminate_connection(&self.connection_id)?;
                Ok(())
            }
        }
    }

    async fn process_message(&self, msg: &Box<HanMessage>) -> Result<(), Error> {
        let message_id = try_uuid(&msg.message_id);
        match &msg.msg {
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