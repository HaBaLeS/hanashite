use crate::controlserver::ServerState;
use bytes::{BytesMut, Buf};

use crate::protos::hanmessage::{HanMessage, Auth, StreamHeader};
use crate::protos::hanmessage::mod_HanMessage::OneOfmsg;
use std::sync::{Arc,Mutex};
use std::net::{SocketAddr, Shutdown};
use tokio::net::{TcpStream};
use tokio::stream::StreamExt;
use tokio_util::codec::Framed;
use tokio_util::codec::{Encoder, Decoder};
use quick_protobuf::{BytesReader,  Error, Writer, MessageWrite};
use quick_protobuf::errors::Error::Message;
use uuid::Uuid;


#[allow(dead_code)]
pub struct ClientHandler {
    state: Arc<Mutex<ServerState>>,
    addr: SocketAddr,
}

pub struct MessageParser {}

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
                Ok(msg) => self.process_message(msg, &mut messages),
                Err(_) => return
            }
        }
    }

    fn disconnect(&self, control: &mut Framed<TcpStream, MessageParser>) {
        match control.get_mut().shutdown(Shutdown::Both) {
            _ => ()
        }
    }

    fn process_message(&self, data: HanMessage, control: &mut Framed<TcpStream, MessageParser>) {
        match self.multiplex(&data) {
            Err(_) => self.disconnect(control),
            _ => ()
        }
    }

    fn multiplex(&self, data: &HanMessage) -> Result<(), Error> {
        let uuid = match Uuid::from_slice(&data.uuid[..]) {
            Err(_) => return Err(Error::Message("Illegal UUID".to_string())),
            Ok(id) => id
        };
        match &data.msg {
            OneOfmsg::auth(msg) => self.handle_auth(&uuid, &msg),
            OneOfmsg::auth_result(_) => self.handle_illegal_msg(&uuid, "auth_result"),
            _ => self.handle_illegal_msg(&uuid, "unknown_message")
        }
    }

    fn handle_auth(&self, uuid: &Uuid, msg: &Auth) -> Result<(), Error> {
        println!("Received Auth UUID: {}, user: {}", &uuid, &msg.username);
        Ok(())
    }

    fn handle_illegal_msg(&self, _uuid: &Uuid, _message: &str) -> Result<(), Error> {
        unimplemented!()
    }

}


const HEADER_LENGTH : usize = 10;

impl MessageParser {

    fn read_header(src: &mut BytesMut) -> Result<usize, Error> {
        let mut reader = BytesReader::from_bytes(src.bytes());
        let header: StreamHeader = match reader.read_message_by_len(src.bytes(), HEADER_LENGTH) {
            Err(e) => return Err(e),
            Ok(val) => val
        };
        if header.magic != 0x0008a71 {
            return  Err(Message("MAGIC is gone !".to_string()));
        }
        return Ok(header.length as usize);
    }
}


impl Decoder for MessageParser {
    type Item = HanMessage;
    type Error = quick_protobuf::Error;

    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> Result<Option<HanMessage>, Self::Error> {
        //  skip magic bytes
        if src.len() < HEADER_LENGTH {
            return Ok(None);
        }
        let size = match MessageParser::read_header(src) {
            Ok(val) => val,
            Err(e) => return Err(e)
        };
        if src.len() < size + HEADER_LENGTH {
            return Ok(None);
        }
        src.advance(HEADER_LENGTH);
        let mut reader = BytesReader::from_bytes(src.bytes());
        let result = reader.read_message_by_len(src.bytes(), size);
        src.advance(size);
        match result {
            Ok(msg) => Ok(Some(msg)),
            Err(e) => Err(e)
        }
    }
}

impl Encoder<HanMessage> for MessageParser {
    type Error = quick_protobuf::Error;

    fn encode(&mut self, message: HanMessage, dst: &mut BytesMut) -> Result<(), quick_protobuf::Error> {
        let mut writer = Writer::new(ByteMutWrite { delegate: dst });
        match (StreamHeader {
            magic: 0x00008A71,
            length: message.get_size() as u32
        }).write_message(&mut writer) {
            Err(e) => return Err(e),
            _ => ()
        }
        message.write_message(&mut writer)
    }
}

pub struct ByteMutWrite<'a> {
    delegate: &'a mut BytesMut
}

impl std::io::Write for ByteMutWrite<'_> {

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.delegate.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use quick_protobuf::{Writer, MessageWrite};
    use crate::protos::hanmessage::StreamHeader;

    #[test]
    fn testheader() {
        let header = StreamHeader {
            magic: 0x00008A71,
            length: 12345
        };
        println!("Header size: {}", header.get_size());
    }

    #[test]
    fn testencode() {
        let mut r = Vec::new();
        let mut writer = Writer::new(&mut r);
        writer.write_fixed32(0x00008A71).unwrap();
        println!("Length: {}", r.len());
    }
}