
#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ProtobufError(prost::DecodeError),
    ProtocolError(String)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IoError(err) => err.fmt(f),
            Error::ProtobufError(err) => err.fmt(f),
            Error::ProtocolError(msg) => write!(f, "Protocol Error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

impl From<prost::DecodeError> for Error {
    fn from(err: prost::DecodeError) -> Self {
        Error::ProtobufError(err)
    }
}


impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}
