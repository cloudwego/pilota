use std::{
    fmt::{self, Display, Formatter},
    io, string,
};

use tokio::io::AsyncRead;

use super::{
    binary::TAsyncBinaryProtocol, Message, TInputProtocol, TLengthProtocol, TOutputProtocol, TAsyncInputProtocol,
};

#[derive(Debug)]
pub enum Error {
    /// Errors encountered while operating on I/O channels.
    ///
    /// These include *connection closed* and *bind failure*.
    Transport(TransportError),
    /// Errors encountered during runtime-library processing.
    ///
    /// These include *message too large* and *unsupported protocol version*.
    Protocol(ProtocolError),
}

impl From<TransportError> for Error {
    fn from(e: TransportError) -> Self {
        Error::Transport(e)
    }
}

impl From<ProtocolError> for Error {
    fn from(e: ProtocolError) -> Self {
        Error::Protocol(e)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::ConnectionReset
            | io::ErrorKind::ConnectionRefused
            | io::ErrorKind::NotConnected => Error::Transport(TransportError {
                kind: TransportErrorKind::NotOpen,
                message: err.to_string(),
            }),
            io::ErrorKind::AlreadyExists => Error::Transport(TransportError {
                kind: TransportErrorKind::AlreadyOpen,
                message: err.to_string(),
            }),
            io::ErrorKind::TimedOut => Error::Transport(TransportError {
                kind: TransportErrorKind::TimedOut,
                message: err.to_string(),
            }),
            io::ErrorKind::UnexpectedEof => Error::Transport(TransportError {
                kind: TransportErrorKind::EndOfFile,
                message: err.to_string(),
            }),
            _ => {
                Error::Transport(TransportError {
                    kind: TransportErrorKind::Unknown,
                    message: err.to_string(), // FIXME: use io error's debug string
                })
            }
        }
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Self {
        Error::Protocol(ProtocolError {
            kind: ProtocolErrorKind::InvalidData,
            message: err.to_string(), // FIXME: use fmt::Error's debug string
        })
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

/// Information about I/O errors.
#[derive(Debug, Eq, PartialEq)]
pub struct TransportError {
    /// I/O error variant.
    ///
    /// If a specific `TransportErrorKind` does not apply use
    /// `TransportErrorKind::Unknown`.
    pub kind: TransportErrorKind,
    /// Human-readable error message.
    pub message: String,
}

impl TransportError {
    /// Create a new `TransportError`.
    pub fn new<S: Into<String>>(kind: TransportErrorKind, message: S) -> TransportError {
        TransportError {
            kind,
            message: message.into(),
        }
    }
}

/// I/O error categories.
///
/// This list may grow, and it is not recommended to match against it.
#[non_exhaustive]
#[derive(Clone, Copy, Eq, Debug, PartialEq)]
pub enum TransportErrorKind {
    /// Catch-all I/O error.
    Unknown = 0,
    /// An I/O operation was attempted when the transport channel was not open.
    NotOpen = 1,
    /// The transport channel cannot be opened because it was opened previously.
    AlreadyOpen = 2,
    /// An I/O operation timed out.
    TimedOut = 3,
    /// A read could not complete because no bytes were available.
    EndOfFile = 4,
    /// An invalid (buffer/message) size was requested or received.
    NegativeSize = 5,
    /// Too large a buffer or message size was requested or received.
    SizeLimit = 6,
}

impl Display for TransportError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let error_text = match self.kind {
            TransportErrorKind::Unknown => "transport error",
            TransportErrorKind::NotOpen => "not open",
            TransportErrorKind::AlreadyOpen => "already open",
            TransportErrorKind::TimedOut => "timed out",
            TransportErrorKind::EndOfFile => "end of file",
            TransportErrorKind::NegativeSize => "negative size message",
            TransportErrorKind::SizeLimit => "message too long",
        };

        write!(f, "{} because {}", error_text, self.message)
    }
}

impl TryFrom<i32> for TransportErrorKind {
    type Error = Error;
    fn try_from(from: i32) -> Result<Self, Self::Error> {
        match from {
            0 => Ok(TransportErrorKind::Unknown),
            1 => Ok(TransportErrorKind::NotOpen),
            2 => Ok(TransportErrorKind::AlreadyOpen),
            3 => Ok(TransportErrorKind::TimedOut),
            4 => Ok(TransportErrorKind::EndOfFile),
            5 => Ok(TransportErrorKind::NegativeSize),
            6 => Ok(TransportErrorKind::SizeLimit),
            _ => Err(Error::Protocol(ProtocolError {
                kind: ProtocolErrorKind::Unknown,
                message: format!("cannot convert {} to TransportErrorKind", from),
            })),
        }
    }
}

/// Create a new `Error` instance of type `Transport` that wraps a
/// `TransportError`.
pub fn new_transport_error<S: Into<String>>(kind: TransportErrorKind, message: S) -> Error {
    Error::Transport(TransportError::new(kind, message))
}

/// Information about errors that occur in the runtime library.
#[derive(Debug, Eq, PartialEq)]
pub struct ProtocolError {
    /// Protocol error variant.
    ///
    /// If a specific `ProtocolErrorKind` does not apply use
    /// `ProtocolErrorKind::Unknown`.
    pub kind: ProtocolErrorKind,
    /// Human-readable error message.
    pub message: String,
}

impl ProtocolError {
    /// Create a new `ProtocolError`.
    pub fn new<S: Into<String>>(kind: ProtocolErrorKind, message: S) -> ProtocolError {
        ProtocolError {
            kind,
            message: message.into(),
        }
    }
}

impl Display for ProtocolError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let error_text = match self.kind {
            ProtocolErrorKind::Unknown => "protocol error",
            ProtocolErrorKind::InvalidData => "bad data",
            ProtocolErrorKind::NegativeSize => "negative message size",
            ProtocolErrorKind::SizeLimit => "message too long",
            ProtocolErrorKind::BadVersion => "invalid thrift version",
            ProtocolErrorKind::NotImplemented => "not implemented",
            ProtocolErrorKind::DepthLimit => "maximum skip depth reached",
        };

        write!(f, "{}, {}", error_text, self.message)
    }
}

/// Runtime library error categories.
///
/// This list may grow, and it is not recommended to match against it.
#[non_exhaustive]
#[derive(Clone, Copy, Eq, Debug, PartialEq)]
pub enum ProtocolErrorKind {
    /// Catch-all runtime-library error.
    Unknown = 0,
    /// An invalid argument was supplied to a library function, or invalid data
    /// was received from a Thrift endpoint.
    InvalidData = 1,
    /// An invalid size was received in an encoded field.
    NegativeSize = 2,
    /// Thrift message or field was too long.
    SizeLimit = 3,
    /// Unsupported or unknown Thrift protocol version.
    BadVersion = 4,
    /// Unsupported Thrift protocol, server or field type.
    NotImplemented = 5,
    /// Reached the maximum nested depth to which an encoded Thrift field could
    /// be skipped.
    DepthLimit = 6,
}

impl TryFrom<i32> for ProtocolErrorKind {
    type Error = ProtocolError;

    fn try_from(from: i32) -> Result<Self, Self::Error> {
        match from {
            0 => Ok(ProtocolErrorKind::Unknown),
            1 => Ok(ProtocolErrorKind::InvalidData),
            2 => Ok(ProtocolErrorKind::NegativeSize),
            3 => Ok(ProtocolErrorKind::SizeLimit),
            4 => Ok(ProtocolErrorKind::BadVersion),
            5 => Ok(ProtocolErrorKind::NotImplemented),
            6 => Ok(ProtocolErrorKind::DepthLimit),
            _ => Err(ProtocolError {
                kind: ProtocolErrorKind::Unknown,
                message: format!("cannot convert {} to ProtocolErrorKind", from),
            }),
        }
    }
}

/// Create a new `Error` instance of type `Protocol` that wraps a
/// `ProtocolError`.
pub fn new_protocol_error<S: Into<String>>(kind: ProtocolErrorKind, message: S) -> Error {
    Error::Protocol(ProtocolError::new(kind, message))
}

#[derive(Debug, Clone, Copy)]
pub struct DummyError;

#[async_trait::async_trait]
impl Message for DummyError {
    fn encode<T: TOutputProtocol>(&self, _protocol: &mut T) -> Result<(), Error> {
        panic!()
    }

    fn decode<T: TInputProtocol>(_protocol: &mut T) -> Result<Self, Error> {
        panic!()
    }

    async fn decode_async<T: TAsyncInputProtocol>(_protocol: &mut T) -> Result<Self, Error> {
        panic!()
    }

    fn size<T: TLengthProtocol>(&self, _protocol: &mut T) -> usize {
        panic!()
    }
}
