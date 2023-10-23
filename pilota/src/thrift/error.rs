use std::{
    fmt::{self, Debug, Display, Formatter},
    io, string,
};

use faststr::FastStr;

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

impl From<io::Error> for TransportError {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::ConnectionReset
            | io::ErrorKind::ConnectionRefused
            | io::ErrorKind::NotConnected => TransportError {
                kind: TransportErrorKind::NotOpen,
                message: err.to_string(),
            },
            io::ErrorKind::AlreadyExists => TransportError {
                kind: TransportErrorKind::AlreadyOpen,
                message: err.to_string(),
            },
            io::ErrorKind::TimedOut => TransportError {
                kind: TransportErrorKind::TimedOut,
                message: err.to_string(),
            },
            io::ErrorKind::UnexpectedEof => TransportError {
                kind: TransportErrorKind::EndOfFile,
                message: err.to_string(),
            },
            _ => {
                TransportError {
                    kind: TransportErrorKind::Unknown,
                    message: err.to_string(), // FIXME: use io error's debug string
                }
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

impl std::error::Error for TransportError {}

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

impl From<ProtocolError> for DecodeError {
    fn from(value: ProtocolError) -> Self {
        let kind = match value.kind {
            ProtocolErrorKind::InvalidData => DecodeErrorKind::InvalidData,
            ProtocolErrorKind::NegativeSize => DecodeErrorKind::NegativeSize,
            ProtocolErrorKind::BadVersion => DecodeErrorKind::BadVersion,
            ProtocolErrorKind::NotImplemented => DecodeErrorKind::NotImplemented,
            ProtocolErrorKind::DepthLimit => DecodeErrorKind::DepthLimit,
            ProtocolErrorKind::Unknown => DecodeErrorKind::Unknown,
            ProtocolErrorKind::SizeLimit => DecodeErrorKind::Unknown,
        };
        DecodeError::new(kind, value.message)
    }
}

impl From<ProtocolError> for EncodeError {
    fn from(value: ProtocolError) -> Self {
        EncodeError::new(value.kind, value.message)
    }
}

impl From<std::io::Error> for DecodeError {
    fn from(value: std::io::Error) -> Self {
        DecodeError::new(DecodeErrorKind::IOError(value), "")
    }
}

impl From<std::io::Error> for EncodeError {
    fn from(value: std::io::Error) -> Self {
        EncodeError::new(ProtocolErrorKind::Unknown, value.to_string())
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

impl std::error::Error for ProtocolError {}

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
pub fn new_protocol_error<S: Into<String>>(kind: ProtocolErrorKind, message: S) -> ProtocolError {
    ProtocolError::new(kind, message)
}

#[derive(Debug)]
pub struct DecodeError {
    pub kind: DecodeErrorKind,
    pub message: String,
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use DecodeErrorKind::*;

        write!(f, "{}", self.message)?;
        if !matches!(
            self.kind,
            BadVersion | InvalidData | NegativeSize | NotImplemented | UnknownMethod
        ) {
            write!(f, ", caused by {}", self.kind)?;
        }
        Ok(())
    }
}

impl std::error::Error for DecodeError {}

#[derive(Debug)]
pub enum DecodeErrorKind {
    Unknown,
    InvalidData,
    NegativeSize,
    BadVersion,
    NotImplemented,
    DepthLimit,
    UnknownMethod,
    IOError(std::io::Error),
    WithContext(Box<DecodeError>),
}

impl Display for DecodeErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DecodeErrorKind::IOError(e) => write!(f, "IOError: {}", e),
            DecodeErrorKind::WithContext(e) => write!(f, "{}", e),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct EncodeError {
    pub kind: ProtocolErrorKind,
    pub message: FastStr,
}

impl Display for EncodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "EncodeError: {}", self.message)?;
        writeln!(f, ", {}", self.message)?;
        Ok(())
    }
}

impl std::error::Error for EncodeError {}

pub trait DecodeErrorExt {
    fn with_msg<S: Into<String>>(self, get_msg: impl FnOnce() -> S) -> Self;
}

impl<T> DecodeErrorExt for Result<T, DecodeError> {
    fn with_msg<S: Into<String>>(self, get_msg: impl FnOnce() -> S) -> Self {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(DecodeError {
                kind: DecodeErrorKind::WithContext(Box::new(e)),
                message: get_msg().into(),
            }),
        }
    }
}

impl DecodeError {
    pub fn new<S: Into<String>>(kind: DecodeErrorKind, message: S) -> DecodeError {
        DecodeError {
            message: message.into(),
            kind,
        }
    }
}

impl EncodeError {
    pub fn new<S: Into<FastStr>>(kind: ProtocolErrorKind, message: S) -> EncodeError {
        EncodeError {
            message: message.into(),
            kind,
        }
    }
}
