use uuid::Uuid;

pub mod hanmessage;
pub mod udpmessage;

pub const HEADER_LENGTH: usize = 10;

pub const MAGIC_HEADER: u32 = 0x00008A71;

pub fn try_uuid(bytes: &Vec<u8>) -> Option<Uuid> {
    match Uuid::from_slice(&bytes[..]) {
        Err(_) => None,
        Ok(uuid) => Some(uuid)
    }
}

#[allow(dead_code)]
pub fn to_data_opt(id: &Option<Uuid>) -> Vec<u8> {
    match id {
        None => vec![],
        Some(uuid) => Vec::from(&uuid.as_bytes()[..])
    }
}

#[allow(dead_code)]
pub fn to_data(id: &Uuid) -> Vec<u8> {
    Vec::from(&id.as_bytes()[..])
}
