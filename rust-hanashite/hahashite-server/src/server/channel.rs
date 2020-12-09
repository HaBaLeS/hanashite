use crate::server::*;
use std::ops::DerefMut;
use hanashite_message::protos::to_data;

pub trait ChannelServer {
    fn voice_channel_join_request(&self, connection_id: &Uuid, channel_name: &str) -> Result<JoinStatus, Error>;
}


#[derive(PartialEq, Eq, Debug, Clone)]
#[allow(dead_code)]
pub enum JoinStatus {
    Joined(Vec<u8>, Vec<String>),
    Failed,
    Unknown,
}

impl ChannelServer for ServerStruct {
    fn voice_channel_join_request(&self, connection_id: &Uuid, voice_channel_name: &str) -> Result<JoinStatus, Error> {
        let mut connections = self.connections.lock().unwrap();
        get_valid_connection(connections.deref_mut(), connection_id)?;
        let mut voice_channels = self.voice_channels.lock().unwrap();
        // Leave all other voice channels.
        let mut old_channels = vec![];
        for channel in voice_channels.values_mut() {
            if channel.connections.remove(connection_id) {
                old_channels.push(channel.name.clone());
            }
        }
        // Join new channel
        if let Some(channel) = voice_channels.get_mut(voice_channel_name) {
            channel.connections.insert(connection_id.clone());
            Ok(JoinStatus::Joined(to_data(&channel.channel_id), old_channels))
        } else {
            Ok(JoinStatus::Failed)
        }
    }
}



#[cfg(test)]
mod test {

}