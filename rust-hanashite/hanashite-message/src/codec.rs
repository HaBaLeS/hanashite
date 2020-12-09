use bytes::{Buf, BytesMut};
use prost::Message;
use crate::protos::{HEADER_LENGTH, MAGIC_HEADER};
use crate::protos::hanmessage::*;
use tokio_util::codec::{Encoder, Decoder};
use std::io::ErrorKind;

pub struct HanMessageCodec();


impl Decoder for HanMessageCodec {
    type Item = Box<HanMessage>;
    type Error = std::io::Error;

    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> Result<Option<Box<HanMessage>>, Self::Error> {
        //  skip magic bytes
        if src.len() < HEADER_LENGTH {
            return Ok(None);
        }
        let header = StreamHeader::decode(&src[0..HEADER_LENGTH])?;
        if header.magic != MAGIC_HEADER {
            return Err(std::io::Error::new(ErrorKind::InvalidInput, "Invalid Magic Number !"));
        }
        if src.len() < header.length as usize + HEADER_LENGTH {
            return Ok(None);
        }
        src.advance(HEADER_LENGTH);
        let msg = HanMessage::decode(src)?;
        Ok(Some(Box::new(msg)))
    }
}

impl Encoder<Box<HanMessage>> for HanMessageCodec {
    type Error = std::io::Error;

    fn encode(&mut self, message: Box<HanMessage>, dst: &mut BytesMut) -> Result<(), std::io::Error> {
        (StreamHeader {
            magic: MAGIC_HEADER,
            length: message.encoded_len() as u32,
        }).encode(dst).expect("Message encoder broken");
        message.encode(dst).expect("Message encoder broken");
        Ok(())
    }
}

