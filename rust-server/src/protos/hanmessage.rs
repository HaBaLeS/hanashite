///*
/// Envelope to enable easy parsing of multiple types of messages
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Eq)]
pub struct StreamHeader {
    #[prost(fixed32, tag="1")]
    pub magic: u32,
    #[prost(fixed32, tag="2")]
    pub length: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Eq)]
pub struct HanMessage {
    #[prost(bytes, tag="10")]
    pub message_id: std::vec::Vec<u8>,
    #[prost(oneof="han_message::Msg", tags="11, 12, 13, 14, 15, 16, 17, 18, 19, 20")]
    pub msg: ::std::option::Option<han_message::Msg>,
}
pub mod han_message {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    #[derive(Eq)]
    pub enum Msg {
        #[prost(message, tag="11")]
        Auth(super::Auth),
        #[prost(message, tag="12")]
        AuthResult(super::AuthResult),
        #[prost(message, tag="13")]
        ChanLst(super::ChannelList),
        #[prost(message, tag="14")]
        ChanLstResult(super::ChannelListResult),
        #[prost(message, tag="15")]
        ChanJoin(super::ChannelJoin),
        #[prost(message, tag="16")]
        ChanJoinResult(super::ChannelJoinResult),
        #[prost(message, tag="17")]
        ChanPart(super::ChannelPart),
        #[prost(message, tag="18")]
        ChanPartResult(super::ChannelPartResult),
        #[prost(message, tag="19")]
        ChanStatus(super::ChannelStatus),
        #[prost(message, tag="20")]
        ChanStatusResult(super::ChannelStatusResult),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Eq)]
pub struct Auth {
    #[prost(string, tag="20")]
    pub username: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Eq)]
pub struct AuthResult {
    #[prost(bool, tag="30")]
    pub success: bool,
    #[prost(bytes, tag="31")]
    pub connection_id: std::vec::Vec<u8>,
}
/// NIL
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Eq)]
pub struct ChannelList {
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Eq)]
pub struct ChannelListentry {
    #[prost(string, tag="40")]
    pub name: std::string::String,
    #[prost(bytes, tag="41")]
    pub id: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Eq)]
pub struct ChannelListResult {
    #[prost(message, repeated, tag="45")]
    pub channel: ::std::vec::Vec<ChannelListentry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Eq)]
pub struct ChannelJoin {
    #[prost(string, tag="50")]
    pub name: std::string::String,
    #[prost(bytes, tag="51")]
    pub channel_id: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Eq)]
pub struct ChannelJoinResult {
    #[prost(bool, tag="60")]
    pub success: bool,
    #[prost(bytes, tag="61")]
    pub channel_id: std::vec::Vec<u8>,
}
/// Nil
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Eq)]
pub struct ChannelPart {
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Eq)]
pub struct ChannelPartResult {
    #[prost(bool, tag="80")]
    pub success: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Eq)]
pub struct ChannelStatus {
    #[prost(bytes, tag="90")]
    pub channel_id: std::vec::Vec<u8>,
}
/// TODO
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Eq)]
pub struct ChannelStatusResult {
}
