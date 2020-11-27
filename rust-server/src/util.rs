
#[derive(Debug)]
pub enum Error {
    InternalError(String),
    IoError(std::io::Error),
    ProtobufError(prost::DecodeError),
    ProtocolError(String)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InternalError(err) => err.fmt(f),
            Error::IoError(err) => err.fmt(f),
            Error::ProtobufError(err) => err.fmt(f),
            Error::ProtocolError(msg) => write!(f, "Protocol Error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

impl <T>From<tokio::sync::mpsc::error::SendError<T>> for Error {
    fn from(err: tokio::sync::mpsc::error::SendError<T>) -> Self { Error::InternalError(err.to_string())}
}

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
