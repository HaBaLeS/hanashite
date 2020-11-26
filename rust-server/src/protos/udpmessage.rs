///*
/// Udp Message Envelope
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Eq)]
pub struct HanUdpMessage {
    #[prost(bytes, tag="111")]
    pub connection_id: std::vec::Vec<u8>,
    #[prost(oneof="han_udp_message::Msg", tags="100, 101")]
    pub msg: ::std::option::Option<han_udp_message::Msg>,
}
pub mod han_udp_message {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    #[derive(Eq)]
    pub enum Msg {
        #[prost(message, tag="100")]
        AudioFrame(super::AudioPacket),
        #[prost(message, tag="101")]
        PingPacket(super::PingPacket),
    }
}
///*
/// Keepalive package. Also registeres UDP address of the client to the server
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Eq)]
pub struct PingPacket {
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Eq)]
pub struct AudioPacket {
    #[prost(bytes, tag="110")]
    pub channel_id: std::vec::Vec<u8>,
    #[prost(uint64, tag="112")]
    pub sequence_id: u64,
    #[prost(bytes, tag="113")]
    pub data: std::vec::Vec<u8>,
}
