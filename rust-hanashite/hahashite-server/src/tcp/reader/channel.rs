use uuid::Uuid;

use hanashite_message::protos::build_message;
use hanashite_message::protos::hanmessage::{VoiceChannelJoin, VoiceChannelJoinResponse};
use hanashite_message::protos::hanmessage::han_message::Msg;

use crate::error::Error;
use crate::server::{ControlMessage, Server};
use crate::server::channel::{ChannelServer, JoinStatus};

impl<T: Server + ChannelServer> super::Reader<T> {
    // TODO: Channel join event
    pub async fn handle_voice_channel_join(&self, message_id: &Option<Uuid>, msg: &VoiceChannelJoin) -> Result<(), Error> {
        let result = self.server.voice_channel_join_request(&self.connection_id, &msg.name)?;
        match result {
            JoinStatus::Joined(channel_id, old_channels) => self.voice_join_success(message_id, channel_id, &old_channels).await?,
            JoinStatus::NotFound => self.voice_join_not_found(message_id).await?,
            _ => return Err(Error::InternalError("Impossible Error".to_string()))
        }
        /*
        self.server.send_to_connection(&self.connection_id, ControlMessage::SENDCTRL(match result {
            JoinStatus::Joined(channel_id, old_channels) =>
                build_message(message_id, Msg::VoiceChannelJoinResponse(VoiceChannelJoinResponse {
                    channel_id,
                    success: true,
                    message: "".to_string(),
                })),
            JoinStatus::NotFound => build_message(message_id, Msg::VoiceChannelJoinResponse(VoiceChannelJoinResponse {
                channel_id: vec![],
                success: false,
                message: "Channel does not exist.".to_string(),
            })),
            _ => return Err(Error::InternalError("Impossible Error".to_string()))
        })).await?; */
        Ok(())
    }

    async fn voice_join_success(&self, message_id: &Option<Uuid>, channel_id: Vec<u8>, old_channels: &Vec<String>) -> Result<(), Error> {
        let sender = self.server.connection_sender(&self.connection_id)?;
        sender.send(ControlMessage::SENDCTRL(build_message(message_id, Msg::VoiceChannelJoinResponse(VoiceChannelJoinResponse {
            channel_id,
            success: true,
            message: "".to_string(),
        })))).await?;
        Ok(())
    }

    async fn voice_join_not_found(&self, message_id: &Option<Uuid>) {
        let sender = self.server.connection_sender(&self.connection_id)?;
        sender.send(ControlMessage::SENDCTRL(build_message(message_id, Msg::VoiceChannelJoinResponse(VoiceChannelJoinResponse {
            channel_id: vec![],
            success: false,
            message: "Channel does not exist.".to_string(),
        })))).await?
    }
}

#[cfg(test)]
mod test {
    use super::*;
}