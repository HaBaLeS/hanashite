///*
/// Envelope to enable easy parsing of multiple types of messages
#[derive(Eq,Hash)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamHeader {
    #[prost(fixed32, tag="1")]
    pub magic: u32,
    #[prost(fixed32, tag="2")]
    pub length: u32,
}
#[derive(Eq,Hash)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HanMessage {
    #[prost(bytes="vec", tag="1")]
    pub message_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(oneof="han_message::Msg", tags="2, 3, 14, 15, 16, 17, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 18, 19")]
    pub msg: ::core::option::Option<han_message::Msg>,
}
/// Nested message and enum types in `HanMessage`.
pub mod han_message {
    #[derive(Eq,Hash)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Msg {
        #[prost(message, tag="2")]
        Auth(super::Auth),
        #[prost(message, tag="3")]
        AuthResult(super::AuthResult),
        #[prost(message, tag="14")]
        ChanCrea(super::ChannelCreate),
        #[prost(message, tag="15")]
        ChanCreaResult(super::ChannelCreateResult),
        #[prost(message, tag="16")]
        ChanDel(super::ChannelDelete),
        #[prost(message, tag="17")]
        ChanDelResult(super::ChannelDeleteResult),
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
        #[prost(message, tag="18")]
        ChanJoinEv(super::ChannelJoinEvent),
        #[prost(message, tag="19")]
        ChanPartEv(super::ChannelPartEvent),
    }
}
///
///Login, nothing works before that.
#[derive(Eq,Hash)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Auth {
    #[prost(string, tag="1")]
    pub username: ::prost::alloc::string::String,
}
#[derive(Eq,Hash)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthResult {
    #[prost(bool, tag="1")]
    pub success: bool,
    #[prost(bytes="vec", tag="2")]
    pub connection_id: ::prost::alloc::vec::Vec<u8>,
}
///
///List all channels.
///
/// NIL
#[derive(Eq,Hash)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelList {
}
#[derive(Eq,Hash)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelListentry {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub channel_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Eq,Hash)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelListResult {
    #[prost(message, repeated, tag="1")]
    pub channel: ::prost::alloc::vec::Vec<ChannelListentry>,
}
///
///Create a Channel.
#[derive(Eq,Hash)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelCreate {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Eq,Hash)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelCreateResult {
    #[prost(bool, tag="1")]
    pub success: bool,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="3")]
    pub channel_id: ::prost::alloc::vec::Vec<u8>,
}
///
///Delete a channel.
#[derive(Eq,Hash)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelDelete {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Eq,Hash)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelDeleteResult {
    #[prost(bool, tag="1")]
    pub success: bool,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="3")]
    pub channel_id: ::prost::alloc::vec::Vec<u8>,
}
///
///Join a channel. Automatically leaves current channel.
#[derive(Eq,Hash)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelJoin {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Eq,Hash)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelJoinResult {
    #[prost(bool, tag="1")]
    pub success: bool,
    #[prost(bytes="vec", tag="2")]
    pub channel_id: ::prost::alloc::vec::Vec<u8>,
}
///
///Leave current channel.
///
/// Nil
#[derive(Eq,Hash)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelPart {
}
#[derive(Eq,Hash)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelPartResult {
    #[prost(bool, tag="1")]
    pub success: bool,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="3")]
    pub channel_id: ::prost::alloc::vec::Vec<u8>,
}
///
///List stats for a channel.
#[derive(Eq,Hash)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelStatus {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Eq,Hash)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserEntry {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub user_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Eq,Hash)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelStatusResult {
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="3")]
    pub channel_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="1")]
    pub user: ::prost::alloc::vec::Vec<UserEntry>,
}
///
///List your own status.
///
/// Nil
#[derive(Eq,Hash)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
}
#[derive(Eq,Hash)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusResult {
    #[prost(bytes="vec", tag="1")]
    pub connection_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub channel: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
}
///
///NOT YET WORKING. You should get this if someone joins your current channel.
#[derive(Eq,Hash)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelJoinEvent {
    #[prost(string, tag="1")]
    pub channel_name: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub channel_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub user_id: ::prost::alloc::string::String,
}
///*
///Not YET WORKING. You should get this if someone leaves your channel.
#[derive(Eq,Hash)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelPartEvent {
    #[prost(string, tag="1")]
    pub channel_name: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub channel_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub reason: ::prost::alloc::string::String,
}
