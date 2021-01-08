use std::ops::DerefMut;

use std::ops::Deref;
use crate::server::*;

pub trait ChannelServer {
    fn channel_join_request(&self, connection_id: &Uuid, channel_name: &str, audio: bool) -> Result<JoinStatus, Error>;
    fn channel_part_request(&self, connection_id: &Uuid, channel_name: &str) -> Result<PartStatus, Error>;
}


#[derive(PartialEq, Eq, Debug, Clone)]
#[allow(dead_code)]
pub enum JoinStatus {
    //Changed Channel Substribtions
    Joined(HashSet<String>),
    NotFound,
    PermissionDenied,
    Unknown,
}

#[derive(PartialEq, Eq, Debug, Clone)]
#[allow(dead_code)]
pub enum PartStatus {
    Parted,
    NotFound,
    NotJoined,
    Unknown,
}

impl ChannelServer for ServerStruct {
    fn channel_join_request(&self, connection_id: &Uuid, channel_name: &str, audio: bool)
                            -> Result<JoinStatus, Error> {
        let mut connections = self.connections.lock().unwrap();
        get_valid_connection(connections.deref_mut(), connection_id)?;
        let mut channels = self.channels.lock().unwrap();
        // Join new channel
        if channels.contains_key(channel_name) {
            // Leave all other voice channels.
            let mut old_channels = HashSet::<String>::new();
            if audio {
                if !is_audio_enabled(channels.deref(), channel_name) {
                    return Ok(JoinStatus::PermissionDenied);
                }
                for channel in channels.values_mut() {
                    if channel.name != channel_name &&
                        channel.audio_connections.contains(connection_id) {
                        old_channels.insert(channel.name.clone());
                    }
                }
            }
            for channel in old_channels.iter() {
                if let Some(chan) = channels.get_mut(channel) {
                    chan.audio_connections.remove(connection_id);
                }
            }
            if let Some(channel) = channels.get_mut(channel_name) {
                if !channel.connections.contains(connection_id) ||
                    audio != channel.audio_connections.contains(connection_id) {
                    old_channels.insert(channel.name.clone());
                }
                channel.connections.insert(connection_id.clone());
                if audio {
                    channel.audio_connections.insert(connection_id.clone());
                }
                Ok(JoinStatus::Joined(old_channels))
            } else {
                unreachable!();
            }
        } else {
            Ok(JoinStatus::NotFound)
        }
    }

    fn channel_part_request(&self, connection_id: &Uuid, channel_name: &str)
                            -> Result<PartStatus, Error> {
        let mut connections = self.connections.lock().unwrap();
        get_valid_connection(connections.deref_mut(), connection_id)?;
        let mut channels = self.channels.lock().unwrap();
        if let Some(channel) = channels.get_mut(channel_name) {
            channel.audio_connections.remove(connection_id);
            if channel.connections.remove(connection_id) {
                Ok(PartStatus::Parted)
            } else {
                Ok(PartStatus::NotJoined)
            }
        } else {
            Ok(PartStatus::NotFound)
        }
    }
}

fn is_audio_enabled(channels: &HashMap<String, Channel>, channel: &str) -> bool {
    channels.get(channel).map(|c| c.audio).unwrap_or(false)
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::server::test::*;
    use assert_matches::assert_matches;

    #[test]
    fn test_channel_join() {
        let data = setup_test();
        let result = data.server.channel_join_request(&data.connections[0],
                                                      "testchannel1", true).unwrap();
        assert_matches!(result, JoinStatus::Joined(old_channels) => {
            assert_eq!(HashSet::from_iter(vec!["testchannel2".to_string()].iter().cloned()), old_channels);
            let vocie_channels = data.server.channels.lock().unwrap();
            let con = vocie_channels.get("testchannel1").unwrap();
            assert!(con.connections.contains(&data.connections[0]));
            let con2 = vocie_channels.get("testchannel2").unwrap();
            assert!(con2.connections.contains(&data.connections[0]));
            assert!(!con2.audio_connections.contains(&data.connections[0]));
        });
    }

    #[test]
    fn test_channel_no_audio() {
        let data = setup_test();
        let result = data.server.channel_join_request(&data.connections[0],
                                                      "testchannel3", true).unwrap();
        assert_matches!(result, JoinStatus::PermissionDenied);
    }

    #[test]
    fn test_channel_no_audio_ok() {
        let data = setup_test();
        let result = data.server.channel_join_request(&data.connections[0],
                                                      "testchannel3", false).unwrap();
        assert_matches!(result, JoinStatus::Joined(_));
    }

    #[test]
    fn test_channel_join_doesnotexist() {
        let data = setup_test();
        let result = data.server.channel_join_request(&data.connections[0],
                                                      "testchannel4", true).unwrap();
        assert_matches!(result, JoinStatus::NotFound);
    }


    #[test]
    fn test_channel_part_doesnotexist() {
        let data = setup_test();
        let result = data.server.channel_part_request(&data.connections[0],
                                                      "testchannel4").unwrap();
        assert_matches!(result, PartStatus::NotFound);
    }

    #[test]
    fn test_channel_part_notjoined() {
        let data = setup_test();
        let result = data.server.channel_part_request(&data.connections[0],
                                                      "testchannel1").unwrap();
        assert_matches!(result, PartStatus::NotJoined);
    }

    #[test]
    fn test_channel_part_ok() {
        let data = setup_test();
        let result = data.server.channel_part_request(&data.connections[0],
                                                      "testchannel2").unwrap();
        assert_matches!(result, PartStatus::Parted);
        let channels = data.server.channels.lock().unwrap();
        let con = channels.get("testchannel2").unwrap();
        assert!(!con.connections.contains(&data.connections[0]));
        assert!(!con.audio_connections.contains(&data.connections[0]));
    }
}