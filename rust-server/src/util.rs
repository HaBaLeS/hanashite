use bytes::BytesMut;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ProtoBufError(quick_protobuf::Error),
    ProtocolError(String)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IoError(err) => err.fmt(f),
            Error::ProtoBufError(err) => err.fmt(f),
            Error::ProtocolError(msg) => write!(f, "Protocol Error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}


impl From<quick_protobuf::Error> for Error {
    fn from(err: quick_protobuf::Error) -> Self {
        Error::ProtoBufError(err)
    }
}

pub struct ByteMutWrite<'a> {
    pub delegate: &'a mut BytesMut
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
