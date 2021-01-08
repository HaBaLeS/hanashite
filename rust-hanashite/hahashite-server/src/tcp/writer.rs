use hanashite_message::codec::HanMessageCodec;
use crate::error::Error;
use crate::server::{ControlMessage, Server, ServerBusEndpoint, ConnectionContext};
use tracing::{warn};
use tokio::net::tcp::WriteHalf;
use tokio_util::codec::FramedWrite;
use uuid::Uuid;
use std::sync::Arc;
use crate::bus::{MessagePredicate, BusMessage};

pub struct Writer<T> {
    connection_id: Uuid,
    server: Arc<T>,
    endpoint: ServerBusEndpoint,
}

struct WriterMessagePredicate {
    connection_id: Uuid,
}

impl MessagePredicate<ControlMessage, ConnectionContext> for WriterMessagePredicate {
    fn relevant(&self, message: &BusMessage<ControlMessage, ConnectionContext>,
                context: &ConnectionContext) -> bool {
        match message.msg {
            ControlMessage::DISCONNECT(connection_id) => connection_id == self.connection_id,
            _ => false
        }
    }
}

impl<T: Server> Writer<T> {
    pub fn new(server: &Arc<T>, connection_id: &Uuid, endpoint: ServerBusEndpoint) -> Writer<T> {
        Writer {
            connection_id: connection_id.clone(),
            server: server.clone(),
            endpoint,
        }
    }

    pub async fn client_writer(&mut self, tcp_writer: WriteHalf<'_>)
                               -> Result<(), Error> {
        let predicate = WriterMessagePredicate { connection_id: self.connection_id.clone() };
        let mut writer = FramedWrite::new(tcp_writer, HanMessageCodec());
        loop {
            let msg = self.endpoint.recv(&predicate).await?;
            match msg.msg {
                ControlMessage::DISCONNECT(connection_id) => if connection_id == self.connection_id { break; },
                _ => ()
            }
        }
        Ok(())
    }

    async fn process_ctrl_message(&self, message: Option<ControlMessage>, writer: &mut FramedWrite<WriteHalf<'_>, HanMessageCodec>)
                                  -> Result<(), Error> {
        match message {
            None => {
                warn!("Internal receiver died.");
                self.server.terminate_connection(&self.connection_id)?;
                Err(Error::InternalError("Internal Receiver terminated on Client Connection".to_string()))
            }
            _ => Ok(())
        }
    }
}
