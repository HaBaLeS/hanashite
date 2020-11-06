use tokio_util::codec::{Encoder,Decoder};
use bytes::{Bytes, BytesMut};
use std::io;
pub struct ClientHandler {

}

impl Decoder for ClientHandler {
    type Item = Bytes;
    type Error = io::Error;
    
    fn decode(
        &mut self,
        src: &mut BytesMut
    ) -> Result<Option<Bytes>, Self::Error> {

    }
}

impl Encoder<Bytes> for ClientHandler {
    type Error = io::Error;

    fn encode(&mut self, bytes: Bytes, dst: &mut BytesMut) -> Result<(), Self::Error> {

    }
}