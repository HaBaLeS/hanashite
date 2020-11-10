use crate::controlserver::ServerState;
use bytes::{BytesMut, Buf};

use crate::protos::hanmessage::{HanMessage};

use std::sync::Arc;
use std::net::SocketAddr;

use tokio::net::{TcpStream};
use tokio::stream::StreamExt;
use tokio::sync::{Mutex};
use tokio_util::codec::Framed;
use tokio_util::codec::{Encoder, Decoder};
use quick_protobuf::{BytesReader, MessageRead, Error, MessageWrite, BytesWriter,Writer};
use quick_protobuf::errors::Error::Message;


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

const HEADER_LENGTH : usize = 8;

impl MessageParser {

    fn read_header(src: &&mut BytesMut) -> Result<usize, Error> {
        let data = &src[..8];
        let mut reader = BytesReader::from_bytes(data);
        // Read MAGIC
        match reader.read_fixed32(data) {
            Ok(0x00008A71) => Ok(reader.read_fixed32(data).unwrap() as usize),
            Ok(_) => Err(Message("MAGIC is gone !".to_string())),
            Err(e) => Err(e)
        }
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
        let size = match MessageParser::read_header(&src) {
            Ok(val) => val,
            Err(e) => return Err(e)
        };
        if src.len() < size + HEADER_LENGTH {
            return Ok(None);
        }
        src.advance(HEADER_LENGTH);
        let data = &src[0..size];
        let mut reader = BytesReader::from_bytes(data);
        let result = HanMessage::from_reader(&mut reader, data);
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
        let msg_size = message.get_size() + HEADER_LENGTH;
        let offset = dst.len();
        dst.resize(offset + msg_size, 0);
        let mut writer = Writer::new(BytesWriter::new(&mut dst[offset..]));
        message.write_message(&mut writer)
    }
}

#[cfg(test)]
mod tests {
    use quick_protobuf::Writer;

    #[test]
    fn testencode() {
        let mut r = Vec::new();
        let mut writer = Writer::new(&mut r);
        writer.write_fixed32(0x00008A71).unwrap();
        println!("Length: {}", r.len());
    }
}