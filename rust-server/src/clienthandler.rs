use tokio_util::codec::{Encoder,Decoder};
use bytes::{Bytes, BytesMut};
use std::io;


pub struct ClientHandler {

}


struct MessageParser {

}

impl ClientHandler {
    pub fn new() -> ClientHandler {
        ClientHandler  {

        }
    }
}

impl Decoder for MessageParser {
    type Item = Bytes;
    type Error = io::Error;
    
    fn decode(
        &mut self,
        src: &mut BytesMut
    ) -> Result<Option<Bytes>, Self::Error> {
        Ok(None)
    }
}

impl Encoder<Bytes> for MessageParser {
    type Error = io::Error;

    fn encode(&mut self, bytes: Bytes, dst: &mut BytesMut) -> Result<(), Self::Error> {
        Ok(())
    }
}