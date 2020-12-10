use std::ops::DerefMut;

use hanashite_message::protos::to_data;

use crate::server::*;

pub trait ChannelServer {
    fn voice_channel_join_request(&self, connection_id: &Uuid, channel_name: &str) -> Result<JoinStatus, Error>;
}


#[derive(PartialEq, Eq, Debug, Clone)]
#[allow(dead_code)]
pub enum JoinStatus {
    Joined(Vec<u8>, Vec<String>),
    NotFound,
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
            if channel.connections.contains(connection_id) {
                old_channels.push(channel.name.clone());
            }
        }
        // Join new channel
        if voice_channels.contains_key(voice_channel_name) {
            for channel in old_channels.iter() {
                if let Some(chan) = voice_channels.get_mut(channel) {
                    chan.connections.remove(connection_id);
                }
            }
            if let Some(channel) = voice_channels.get_mut(voice_channel_name) {
                channel.connections.insert(connection_id.clone());
                Ok(JoinStatus::Joined(to_data(&channel.channel_id), old_channels))
            } else {
                unreachable!();
            }
        } else {
            Ok(JoinStatus::NotFound)
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::server::test::*;
    use assert_matches::assert_matches;

    #[test]
    fn test_voice_channel_join() {
        let data = setup_test();
        let result = data.server.voice_channel_join_request(&data.connections[0],
                                              "testchannel1").unwrap();
        println!("{:?}", &result);
        assert_matches!(result, JoinStatus::Joined(_uuid, _old_channels) => {
            let vocie_channels = data.server.voice_channels.lock().unwrap();
            let con = vocie_channels.get("testchannel1").unwrap();
            assert!(con.connections.contains(&data.connections[0]));
            let con2 = vocie_channels.get("testchannel2").unwrap();
            assert!(!con2.connections.contains(&data.connections[0]));
        });
    }

    #[test]
    fn test_voice_channel_join_doesnotexist() {
        let data = setup_test();
        let result = data.server.voice_channel_join_request(&data.connections[0],
                                                            "testchannel3").unwrap();
        assert_matches!(result, JoinStatus::NotFound);
    }

}