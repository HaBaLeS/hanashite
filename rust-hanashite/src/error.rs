
#[derive(Debug)]
pub enum Error {
    InternalError(String),
    IoError(std::io::Error),
    JoinError(tokio::task::JoinError),
    ProtobufError(prost::DecodeError),
    PermissionDenied
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InternalError(err) => err.fmt(f),
            Error::IoError(err) => err.fmt(f),
            Error::JoinError(err) => err.fmt(f),
            Error::ProtobufError(err) => err.fmt(f),
            Error::PermissionDenied => "PermissionDenied".fmt(f),
        }
    }
}


impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(err: tokio::task::JoinError) -> Self {
        Error::JoinError(err)
    }
}

impl From<prost::DecodeError> for Error {
    fn from(err: prost::DecodeError) -> Self {
        Error::ProtobufError(err)
    }
}

impl From<tokio::sync::broadcast::error::SendError<()>> for Error {
    fn from(err: tokio::sync::broadcast::error::SendError<()>) -> Self {
        Error::InternalError(err.to_string())
    }
}