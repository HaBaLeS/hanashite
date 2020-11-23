// Automatically generated rust module for 'hanmessage.proto' file

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
pub struct StreamHeader {
    pub magic: u32,
    pub length: u32,
}

impl<'a> MessageRead<'a> for StreamHeader {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(13) => msg.magic = r.read_fixed32(bytes)?,
                Ok(21) => msg.length = r.read_fixed32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for StreamHeader {
    fn get_size(&self) -> usize {
        0
        + if self.magic == 0u32 { 0 } else { 1 + 4 }
        + if self.length == 0u32 { 0 } else { 1 + 4 }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.magic != 0u32 { w.write_with_tag(13, |w| w.write_fixed32(*&self.magic))?; }
        if self.length != 0u32 { w.write_with_tag(21, |w| w.write_fixed32(*&self.length))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct HanMessage {
    pub uuid: Vec<u8>,
    pub msg: mod_HanMessage::OneOfmsg,
}

impl<'a> MessageRead<'a> for HanMessage {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(82) => msg.uuid = r.read_bytes(bytes)?.to_owned(),
                Ok(90) => msg.msg = mod_HanMessage::OneOfmsg::auth(r.read_message::<Auth>(bytes)?),
                Ok(98) => msg.msg = mod_HanMessage::OneOfmsg::auth_result(r.read_message::<AuthResult>(bytes)?),
                Ok(106) => msg.msg = mod_HanMessage::OneOfmsg::chan_lst(r.read_message::<ChannelList>(bytes)?),
                Ok(114) => msg.msg = mod_HanMessage::OneOfmsg::chan_lst_result(r.read_message::<ChannelListResult>(bytes)?),
                Ok(122) => msg.msg = mod_HanMessage::OneOfmsg::chan_join(r.read_message::<ChannelJoin>(bytes)?),
                Ok(130) => msg.msg = mod_HanMessage::OneOfmsg::chan_join_result(r.read_message::<ChannelJoinResult>(bytes)?),
                Ok(138) => msg.msg = mod_HanMessage::OneOfmsg::chan_part(r.read_message::<ChannelPart>(bytes)?),
                Ok(146) => msg.msg = mod_HanMessage::OneOfmsg::chan_part_result(r.read_message::<ChannelPartResult>(bytes)?),
                Ok(154) => msg.msg = mod_HanMessage::OneOfmsg::chan_status(r.read_message::<ChannelStatus>(bytes)?),
                Ok(162) => msg.msg = mod_HanMessage::OneOfmsg::chan_status_result(r.read_message::<ChannelStatusResult>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for HanMessage {
    fn get_size(&self) -> usize {
        0
        + if self.uuid.is_empty() { 0 } else { 1 + sizeof_len((&self.uuid).len()) }
        + match self.msg {
            mod_HanMessage::OneOfmsg::auth(ref m) => 1 + sizeof_len((m).get_size()),
            mod_HanMessage::OneOfmsg::auth_result(ref m) => 1 + sizeof_len((m).get_size()),
            mod_HanMessage::OneOfmsg::chan_lst(ref m) => 1 + sizeof_len((m).get_size()),
            mod_HanMessage::OneOfmsg::chan_lst_result(ref m) => 1 + sizeof_len((m).get_size()),
            mod_HanMessage::OneOfmsg::chan_join(ref m) => 1 + sizeof_len((m).get_size()),
            mod_HanMessage::OneOfmsg::chan_join_result(ref m) => 2 + sizeof_len((m).get_size()),
            mod_HanMessage::OneOfmsg::chan_part(ref m) => 2 + sizeof_len((m).get_size()),
            mod_HanMessage::OneOfmsg::chan_part_result(ref m) => 2 + sizeof_len((m).get_size()),
            mod_HanMessage::OneOfmsg::chan_status(ref m) => 2 + sizeof_len((m).get_size()),
            mod_HanMessage::OneOfmsg::chan_status_result(ref m) => 2 + sizeof_len((m).get_size()),
            mod_HanMessage::OneOfmsg::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if !self.uuid.is_empty() { w.write_with_tag(82, |w| w.write_bytes(&**&self.uuid))?; }
        match self.msg {            mod_HanMessage::OneOfmsg::auth(ref m) => { w.write_with_tag(90, |w| w.write_message(m))? },
            mod_HanMessage::OneOfmsg::auth_result(ref m) => { w.write_with_tag(98, |w| w.write_message(m))? },
            mod_HanMessage::OneOfmsg::chan_lst(ref m) => { w.write_with_tag(106, |w| w.write_message(m))? },
            mod_HanMessage::OneOfmsg::chan_lst_result(ref m) => { w.write_with_tag(114, |w| w.write_message(m))? },
            mod_HanMessage::OneOfmsg::chan_join(ref m) => { w.write_with_tag(122, |w| w.write_message(m))? },
            mod_HanMessage::OneOfmsg::chan_join_result(ref m) => { w.write_with_tag(130, |w| w.write_message(m))? },
            mod_HanMessage::OneOfmsg::chan_part(ref m) => { w.write_with_tag(138, |w| w.write_message(m))? },
            mod_HanMessage::OneOfmsg::chan_part_result(ref m) => { w.write_with_tag(146, |w| w.write_message(m))? },
            mod_HanMessage::OneOfmsg::chan_status(ref m) => { w.write_with_tag(154, |w| w.write_message(m))? },
            mod_HanMessage::OneOfmsg::chan_status_result(ref m) => { w.write_with_tag(162, |w| w.write_message(m))? },
            mod_HanMessage::OneOfmsg::None => {},
    }        Ok(())
    }
}

pub mod mod_HanMessage {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfmsg {
    auth(Auth),
    auth_result(AuthResult),
    chan_lst(ChannelList),
    chan_lst_result(ChannelListResult),
    chan_join(ChannelJoin),
    chan_join_result(ChannelJoinResult),
    chan_part(ChannelPart),
    chan_part_result(ChannelPartResult),
    chan_status(ChannelStatus),
    chan_status_result(ChannelStatusResult),
    None,
}

impl Default for OneOfmsg {
    fn default() -> Self {
        OneOfmsg::None
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Auth {
    pub username: String,
}

impl<'a> MessageRead<'a> for Auth {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(162) => msg.username = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Auth {
    fn get_size(&self) -> usize {
        0
        + if self.username == String::default() { 0 } else { 2 + sizeof_len((&self.username).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.username != String::default() { w.write_with_tag(162, |w| w.write_string(&**&self.username))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AuthResult {
    pub success: bool,
    pub connection_id: Vec<u8>,
}

impl<'a> MessageRead<'a> for AuthResult {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(240) => msg.success = r.read_bool(bytes)?,
                Ok(250) => msg.connection_id = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for AuthResult {
    fn get_size(&self) -> usize {
        0
        + if self.success == false { 0 } else { 2 + sizeof_varint(*(&self.success) as u64) }
        + if self.connection_id.is_empty() { 0 } else { 2 + sizeof_len((&self.connection_id).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.success != false { w.write_with_tag(240, |w| w.write_bool(*&self.success))?; }
        if !self.connection_id.is_empty() { w.write_with_tag(250, |w| w.write_bytes(&**&self.connection_id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelList { }

impl<'a> MessageRead<'a> for ChannelList {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelList { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelListentry {
    pub name: String,
    pub uuid: Vec<u8>,
}

impl<'a> MessageRead<'a> for ChannelListentry {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(322) => msg.name = r.read_string(bytes)?.to_owned(),
                Ok(330) => msg.uuid = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ChannelListentry {
    fn get_size(&self) -> usize {
        0
        + if self.name == String::default() { 0 } else { 2 + sizeof_len((&self.name).len()) }
        + if self.uuid.is_empty() { 0 } else { 2 + sizeof_len((&self.uuid).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.name != String::default() { w.write_with_tag(322, |w| w.write_string(&**&self.name))?; }
        if !self.uuid.is_empty() { w.write_with_tag(330, |w| w.write_bytes(&**&self.uuid))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelListResult {
    pub channel: Vec<ChannelListentry>,
}

impl<'a> MessageRead<'a> for ChannelListResult {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(362) => msg.channel.push(r.read_message::<ChannelListentry>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ChannelListResult {
    fn get_size(&self) -> usize {
        0
        + self.channel.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.channel { w.write_with_tag(362, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelJoin {
    pub name: String,
    pub uuid: Vec<u8>,
}

impl<'a> MessageRead<'a> for ChannelJoin {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(402) => msg.name = r.read_string(bytes)?.to_owned(),
                Ok(410) => msg.uuid = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ChannelJoin {
    fn get_size(&self) -> usize {
        0
        + if self.name == String::default() { 0 } else { 2 + sizeof_len((&self.name).len()) }
        + if self.uuid.is_empty() { 0 } else { 2 + sizeof_len((&self.uuid).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.name != String::default() { w.write_with_tag(402, |w| w.write_string(&**&self.name))?; }
        if !self.uuid.is_empty() { w.write_with_tag(410, |w| w.write_bytes(&**&self.uuid))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelJoinResult {
    pub success: bool,
    pub channel_uuid: Vec<u8>,
}

impl<'a> MessageRead<'a> for ChannelJoinResult {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(480) => msg.success = r.read_bool(bytes)?,
                Ok(490) => msg.channel_uuid = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ChannelJoinResult {
    fn get_size(&self) -> usize {
        0
        + if self.success == false { 0 } else { 2 + sizeof_varint(*(&self.success) as u64) }
        + if self.channel_uuid.is_empty() { 0 } else { 2 + sizeof_len((&self.channel_uuid).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.success != false { w.write_with_tag(480, |w| w.write_bool(*&self.success))?; }
        if !self.channel_uuid.is_empty() { w.write_with_tag(490, |w| w.write_bytes(&**&self.channel_uuid))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelPart { }

impl<'a> MessageRead<'a> for ChannelPart {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelPart { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelPartResult {
    pub success: bool,
}

impl<'a> MessageRead<'a> for ChannelPartResult {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(640) => msg.success = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ChannelPartResult {
    fn get_size(&self) -> usize {
        0
        + if self.success == false { 0 } else { 2 + sizeof_varint(*(&self.success) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.success != false { w.write_with_tag(640, |w| w.write_bool(*&self.success))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelStatus {
    pub uuid: Vec<u8>,
}

impl<'a> MessageRead<'a> for ChannelStatus {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(722) => msg.uuid = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ChannelStatus {
    fn get_size(&self) -> usize {
        0
        + if self.uuid.is_empty() { 0 } else { 2 + sizeof_len((&self.uuid).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if !self.uuid.is_empty() { w.write_with_tag(722, |w| w.write_bytes(&**&self.uuid))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChannelStatusResult { }

impl<'a> MessageRead<'a> for ChannelStatusResult {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ChannelStatusResult { }

