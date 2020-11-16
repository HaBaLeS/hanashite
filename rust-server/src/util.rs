#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ProtoBufError(quick_protobuf::Error),
    ProtocolError(String),
    MiscError(String)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IoError(err) => err.fmt(f),
            Error::ProtoBufError(err) => err.fmt(f),
            Error::ProtocolError(msg) => write!(f, "Protocol Error: {}", msg),
            Error::MiscError(msg) => write!(f, "Protocol Error: {}", msg)
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