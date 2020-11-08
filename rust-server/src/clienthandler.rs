mod message;

use crate::controlserver::ServerState;
use bytes::{BytesMut, BufMut, Buf};
use std::io;
use std::sync::Arc;
use std::str::from_utf8;
use std::net::SocketAddr;

use tokio::net::{TcpStream};
use tokio::sync::{Mutex};
use tokio_util::codec::Framed;
use tokio_util::codec::{Encoder, Decoder};
use tokio::stream::StreamExt;

use message::{RawHanMessage, HanMessage, han_message_id};
use tokio::io::ErrorKind;
use crate::clienthandler::message::HanMessage::Auth;

#[allow(dead_code)]
pub struct ClientHandler {
    state: Arc<Mutex<ServerState>>,
    addr: SocketAddr,
}

struct MessageParser {}

impl ClientHandler {
    pub fn new(state: Arc<Mutex<ServerState>>,
               addr: SocketAddr) -> ClientHandler {
        ClientHandler {
            state,
            addr,
        }
    }
}


impl ClientHandler {
    pub async fn run(&self,
                     stream: TcpStream) {
        let mut messages = Framed::new(stream, MessageParser {});
        while let Some(result) = messages.next().await {
            match result {
                Ok(msg) => self.process_message(msg, &messages),
                Err(_) => return
            }
        }
    }

    fn process_message(&self, _data: HanMessage, _control: &Framed<TcpStream, MessageParser>) {}
}

fn validate_size(size: &mut usize, reduction: usize) {
    if reduction > *size { panic!("Illegal Message !"); }
    *size -= reduction;
}

fn parse_uuid()

impl MessageParser {
    fn encode_login_ok(cmduuid: &String, uuid: &String, output: &mut BytesMut) -> Result<(), io::Error> {
        let mut message = RawHanMessage::new(han_message_id::LOGIN_OK);
        message.add_string(cmduuid);
        message.add_string(uuid);
        output.put_slice(&message.bytes[..]);
        Ok(())
    }

    fn encode_login_fail(cmduuid: &String, msg: &String, output: &mut BytesMut) -> Result<(), io::Error> {
        let mut message = RawHanMessage::new(han_message_id::LOGIN_FAIL);
        message.add_string(cmduuid);
        message.add_string(msg);
        output.put_slice(&message.bytes[..]);
        Ok(())
    }

    fn encode_join_ok(cmduuid: &String, channel: &String, output: &mut BytesMut) -> Result<(), io::Error> {
        let mut message = RawHanMessage::new(han_message_id::JOIN_OK);
        message.add_string(cmduuid);
        message.add_string(channel);
        output.put_slice(&message.bytes[..]);
        Ok(())
    }

    fn encode_join_fail(cmduuid: &String, channel: &String, output: &mut BytesMut) -> Result<(), io::Error> {
        let mut message = RawHanMessage::new(han_message_id::JOIN_FAIL);
        message.add_string(cmduuid);
        message.add_string(channel);
        output.put_slice(&message.bytes[..]);
        Ok(())
    }

    fn decode_auth(psize: u32, src: &mut BytesMut) -> Result<Option<HanMessage>, io::Error> {
        let mut size = psize as usize;
        validate_size(&mut size, 2);
        let uuidlen = src.get_u16() as usize;
        validate_size(&mut size, uuidlen);
        let uuid = String::from(from_utf8(&src[..uuidlen]).unwrap());
        src.advance(uuidlen);
        validate_size(&mut size, 2);
        let userlen = src.get_u16() as usize;
        validate_size(&mut size, userlen);
        let username =  String::from(from_utf8(&src[..userlen]).unwrap());
        src.advance(userlen);
        return Ok(Some(Auth { cmduuid: uuid, user: username }))
    }


    fn parse_message(src: &mut BytesMut) -> Result<Option<HanMessage>, io::Error> {
        //  skip magic bytes
        src.advance(2);
        let mut size = src.get_u32();
        let id = src.get_u16();
        size -= 6;
        return match id {
            han_message_id::AUTH => MessageParser::decode_auth(size, src),
            _ => Err(io::Error::new(ErrorKind::InvalidData, "Unsupported Message"))
        };
    }
}


impl Decoder for MessageParser {
    type Item = HanMessage;
    type Error = io::Error;

    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> Result<Option<HanMessage>, Self::Error> {
        return match RawHanMessage::validate(&src) {
            Ok(result) => if result { MessageParser::parse_message(src) } else { Ok(None) },
            Err(e) => Err(e)
        };
    }
}

impl Encoder<HanMessage> for MessageParser {
    type Error = io::Error;

    fn encode(&mut self, message: HanMessage, dst: &mut BytesMut) -> Result<(), io::Error> {
        return match message {
            HanMessage::LoginOk { cmduuid, uuid } => MessageParser::encode_login_ok(&cmduuid, &uuid, dst),
            HanMessage::LoginFail { cmduuid, message } => MessageParser::encode_login_fail(&cmduuid, &message, dst),
            HanMessage::JoinOk { cmduuid, channel } => MessageParser::encode_join_ok(&cmduuid, &channel, dst),
            HanMessage::JoinFail { cmduuid, channel } => MessageParser::encode_join_fail(&cmduuid, &channel, dst),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Trying to send Illegal Message"))
        };
    }
}