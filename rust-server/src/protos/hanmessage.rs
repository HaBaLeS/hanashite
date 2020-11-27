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
    #[prost(bytes, tag="1")]
    pub message_id: std::vec::Vec<u8>,
    #[prost(oneof="han_message::Msg", tags="2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13")]
    pub msg: ::std::option::Option<han_message::Msg>,
}
pub mod han_message {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    #[derive(Eq)]
    pub enum Msg {
        #[prost(message, tag="2")]
        Auth(super::Auth),
        #[prost(message, tag="3")]
        AuthResult(super::AuthResult),
        #[prost(message, tag="4")]
        ChanLst(super::ChannelList),
        #[prost(message, tag="5")]
        ChanLstResult(super::ChannelListResult),
        #[prost(message, tag="6")]
        ChanJoin(super::ChannelJoin),
        #[prost(message, tag="7")]
        ChanJoinResult(super::ChannelJoinResult),
        #[prost(message, tag="8")]
        ChanPart(super::ChannelPart),
        #[prost(message, tag="9")]
        ChanPartResult(super::ChannelPartResult),
        #[prost(message, tag="10")]
        ChanStatus(super::ChannelStatus),
        #[prost(message, tag="11")]
        ChanStatusResult(super::ChannelStatusResult),
        #[prost(message, tag="12")]
        Status(super::Status),
        #[prost(message, tag="13")]
        StatusResult(super::StatusResult),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Eq)]
pub struct Auth {
    #[prost(string, tag="1")]
    pub username: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Eq)]
pub struct AuthResult {
    #[prost(bool, tag="1")]
    pub success: bool,
    #[prost(bytes, tag="2")]
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
    #[prost(string, tag="1")]
    pub name: std::string::String,
    #[prost(bytes, tag="2")]
    pub channel_id: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Eq)]
pub struct ChannelListResult {
    #[prost(message, repeated, tag="1")]
    pub channel: ::std::vec::Vec<ChannelListentry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Eq)]
pub struct ChannelJoin {
    #[prost(string, tag="1")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Eq)]
pub struct ChannelJoinResult {
    #[prost(bool, tag="1")]
    pub success: bool,
    #[prost(bytes, tag="2")]
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
    #[prost(bool, tag="1")]
    pub success: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Eq)]
pub struct ChannelStatus {
    #[prost(string, tag="1")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Eq)]
pub struct UserEntry {
    #[prost(string, tag="1")]
    pub name: std::string::String,
    #[prost(bytes, tag="2")]
    pub user_id: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Eq)]
pub struct ChannelStatusResult {
    #[prost(string, tag="2")]
    pub name: std::string::String,
    #[prost(bytes, tag="3")]
    pub channel_id: std::vec::Vec<u8>,
    #[prost(message, repeated, tag="1")]
    pub user: ::std::vec::Vec<UserEntry>,
}
/// Nil
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Eq)]
pub struct Status {
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Eq)]
pub struct StatusResult {
    #[prost(bytes, tag="1")]
    pub connection_id: std::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub channel: std::string::String,
    #[prost(string, tag="3")]
    pub name: std::string::String,
}
