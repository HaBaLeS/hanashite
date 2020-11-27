///*
/// Udp Message Envelope
#[derive(Eq,Hash)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HanUdpMessage {
    #[prost(bytes="vec", tag="111")]
    pub connection_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(oneof="han_udp_message::Msg", tags="100, 101")]
    pub msg: ::core::option::Option<han_udp_message::Msg>,
}
/// Nested message and enum types in `HanUdpMessage`.
pub mod han_udp_message {
    #[derive(Eq,Hash)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Msg {
        #[prost(message, tag="100")]
        AudioFrame(super::AudioPacket),
        #[prost(message, tag="101")]
        PingPacket(super::PingPacket),
    }
}
///*
/// Keepalive package. Also registeres UDP address of the client to the server
#[derive(Eq,Hash)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingPacket {
}
#[derive(Eq,Hash)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudioPacket {
    #[prost(bytes="vec", tag="110")]
    pub channel_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="112")]
    pub sequence_id: u64,
    #[prost(bytes="vec", tag="113")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
