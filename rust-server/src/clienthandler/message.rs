use std::io;
use bytes::{Bytes, BytesMut, BufMut};
use byteorder::{BigEndian, ByteOrder};

pub struct RawHanMessage {
    pub bytes: BytesMut
}

#[allow(dead_code)]
pub enum HanMessage {
    // Client Messages
    Auth { cmduuid: String, user: String },
    Join { cmduuid: String, channel: String },
    // Server Messages
    LoginOk { cmduuid: String, uuid: String },
    LoginFail { cmduuid: String, message: String },
    JoinOk { cmduuid: String, channel: String },
    JoinFail { cmduuid: String, channel: String },
}

#[allow(dead_code)]
pub mod han_message_id {
    // Client Messages
    pub const AUTH: u16 = 1;
    pub const JOIN: u16 = 2;
    pub const LOGIN_OK: u16 = 101;
    pub const LOGIN_FAIL: u16 = 102;
    pub const JOIN_OK: u16 = 103;
    pub const JOIN_FAIL: u16 = 104;
}


const MAGIC_BYTES: Bytes = Bytes::from_static(&[0x8A, 0x71]);

impl RawHanMessage {
    pub fn new(msg_type: u16) -> RawHanMessage {
        let mut data: BytesMut = BytesMut::with_capacity(8);
        data.put_slice(&MAGIC_BYTES[..]);
        data.put_u32(8);
        data.put_u16(msg_type);
        return RawHanMessage {
            bytes: data
        };
    }


    pub fn validate(data: &BytesMut) -> Result<bool, io::Error> {
        if data.len() >= 2 && data[0..1] != MAGIC_BYTES {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Magic Bytes were wrong !"));
        }
        if data.len() >= 8 {
            let msg_size = BigEndian::read_u32(&data[2..6]) as usize;
            return Ok(data.len() >= msg_size);
        }
        return Ok(false);
    }

    #[allow(dead_code)]
    pub fn add_u8(&mut self, data: u8) {
        self.bytes.put_u8(data);
        self.update_size();
    }

    #[allow(dead_code)]
    pub fn add_i8(&mut self, data: i8) {
        self.bytes.put_i8(data);
        self.update_size();
    }

    #[allow(dead_code)]
    pub fn add_u16(&mut self, data: u16) {
        self.bytes.put_u16(data);
        self.update_size();
    }

    #[allow(dead_code)]
    pub fn add_i16(&mut self, data: i16) {
        self.bytes.put_i16(data);
        self.update_size();
    }

    #[allow(dead_code)]
    pub fn add_u32(&mut self, data: u32) {
        self.bytes.put_u32(data);
        self.update_size();
    }

    #[allow(dead_code)]
    pub fn add_i32(&mut self, data: i32) {
        self.bytes.put_i32(data);
        self.update_size();
    }

    #[allow(dead_code)]
    pub fn add_u64(&mut self, data: u64) {
        self.bytes.put_u64(data);
        self.update_size();
    }

    #[allow(dead_code)]
    pub fn add_i64(&mut self, data: i64) {
        self.bytes.put_i64(data);
        self.update_size();
    }

    #[allow(dead_code)]
    pub fn add_string(&mut self, data: &String) {
        self.bytes.put_u16(data.len() as u16);
        self.bytes.put_slice(data.as_bytes());
        self.update_size();
    }

    #[allow(dead_code)]
    pub fn add_blob(&mut self, data: &Bytes) {
        self.bytes.put_u16(data.len() as u16);
        self.bytes.put_slice(&data[..]);
        self.update_size();
    }

    fn update_size(&mut self) {
        let len = self.bytes.len() as u32;
        self.bytes[2..6].copy_from_slice(&len.to_be_bytes());
    }
}
