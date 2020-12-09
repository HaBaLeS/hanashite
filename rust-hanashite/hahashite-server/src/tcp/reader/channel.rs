use crate::server::auth::AuthServer;
use crate::server::{AuthStatus, Server, ControlMessage};
use uuid::Uuid;
use hanashite_message::protos::hanmessage::VoiceChannelJoin;
use crate::error::Error;


impl<T: Server + AuthServer> super::Reader<T> {
    pub async fn handle_voice_channel_join(&self, message_id: &Option<Uuid>, msg: &VoiceChannelJoin) -> Result<(), Error> {
        let sender = self.server.connection_sender(&self.connection_id)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
}