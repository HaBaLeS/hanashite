///*
/// Envelope to enable easy parsing of multiple types of messages
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamHeader {
    #[prost(fixed32, tag="1")]
    pub magic: u32,
    #[prost(fixed32, tag="2")]
    pub length: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HanMessage {
    #[prost(bytes="vec", tag="1")]
    pub message_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(oneof="han_message::Msg", tags="2, 3, 4, 5, 6, 7, 8, 9")]
    pub msg: ::core::option::Option<han_message::Msg>,
}
/// Nested message and enum types in `HanMessage`.
pub mod han_message {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Msg {
        #[prost(message, tag="2")]
        PubKeyAuth(super::PubKeyAuth),
        #[prost(message, tag="3")]
        AuthResponse(super::AuthResponse),
        #[prost(message, tag="4")]
        Challenge(super::Challenge),
        #[prost(message, tag="5")]
        ChallengeResponse(super::ChallengeResponse),
        #[prost(message, tag="6")]
        ChannelSubscription(super::ChannelSubscription),
        #[prost(message, tag="7")]
        ChannelSubscriptionError(super::ChannelSubscriptionError),
        #[prost(message, tag="8")]
        ChannelSubscriptionEvent(super::ChannelSubscriptionEvent),
        #[prost(message, tag="9")]
        ChannelUnsubscriptionEvent(super::ChannelSubscriptionEvent),
    }
}
///
///Login, nothing works before that.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubKeyAuth {
    #[prost(string, tag="1")]
    pub username: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub public_key: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthResponse {
    #[prost(enumeration="auth_response::ResultState", tag="1")]
    pub result: i32,
    #[prost(uint32, tag="2")]
    pub user_id: u32,
    #[prost(string, tag="3")]
    pub message: ::prost::alloc::string::String,
}
/// Nested message and enum types in `AuthResponse`.
pub mod auth_response {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ResultState {
        Unknown = 0,
        Success = 1,
        BrokenKey = 2,
        InvalidCredentials = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Challenge {
    #[prost(bytes="vec", tag="1")]
    pub chellange: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChallengeResponse {
    #[prost(bytes="vec", tag="1")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSubscription {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub audio: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSubscriptionError {
    #[prost(enumeration="channel_subscription_error::Reason", tag="1")]
    pub reason: i32,
}
/// Nested message and enum types in `ChannelSubscriptionError`.
pub mod channel_subscription_error {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Reason {
        DoesNotExist = 0,
        PermissionDenied = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSubscriptionEvent {
    #[prost(uint32, tag="1")]
    pub user_id: u32,
    #[prost(string, tag="2")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub channel_name: ::prost::alloc::string::String,
    #[prost(enumeration="channel_subscription_event::Flag", repeated, tag="4")]
    pub flags: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `ChannelSubscriptionEvent`.
pub mod channel_subscription_event {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Flag {
        AudioReceive = 0,
        AudioSend = 1,
        ChatSend = 2,
        Moderator = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelUnsubscriptionEvent {
    #[prost(uint32, tag="1")]
    pub user_id: u32,
    #[prost(string, tag="2")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub channel_name: ::prost::alloc::string::String,
}
