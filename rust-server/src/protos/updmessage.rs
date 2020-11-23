// Automatically generated rust module for 'updmessage.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct HanUdpMessage {
    pub user_id: Vec<u8>,
    pub audio_frame: Option<AudioPacket>,
}

impl<'a> MessageRead<'a> for HanUdpMessage {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(890) => msg.user_id = r.read_bytes(bytes)?.to_owned(),
                Ok(802) => msg.audio_frame = Some(r.read_message::<AudioPacket>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for HanUdpMessage {
    fn get_size(&self) -> usize {
        0
        + if self.user_id.is_empty() { 0 } else { 2 + sizeof_len((&self.user_id).len()) }
        + self.audio_frame.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if !self.user_id.is_empty() { w.write_with_tag(890, |w| w.write_bytes(&**&self.user_id))?; }
        if let Some(ref s) = self.audio_frame { w.write_with_tag(802, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AudioPacket {
    pub channel_id: Vec<u8>,
    pub sequernce_id: u64,
    pub data: Vec<u8>,
}

impl<'a> MessageRead<'a> for AudioPacket {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(882) => msg.channel_id = r.read_bytes(bytes)?.to_owned(),
                Ok(896) => msg.sequernce_id = r.read_uint64(bytes)?,
                Ok(906) => msg.data = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for AudioPacket {
    fn get_size(&self) -> usize {
        0
        + if self.channel_id.is_empty() { 0 } else { 2 + sizeof_len((&self.channel_id).len()) }
        + if self.sequernce_id == 0u64 { 0 } else { 2 + sizeof_varint(*(&self.sequernce_id) as u64) }
        + if self.data.is_empty() { 0 } else { 2 + sizeof_len((&self.data).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if !self.channel_id.is_empty() { w.write_with_tag(882, |w| w.write_bytes(&**&self.channel_id))?; }
        if self.sequernce_id != 0u64 { w.write_with_tag(896, |w| w.write_uint64(*&self.sequernce_id))?; }
        if !self.data.is_empty() { w.write_with_tag(906, |w| w.write_bytes(&**&self.data))?; }
        Ok(())
    }
}

