mod application;
mod protocol;
mod transport;

pub use application::*;
pub use protocol::*;
pub use transport::*;

use std::{
    fmt::{self, Debug, Display, Formatter},
    string,
};

use faststr::FastStr;

#[derive(Debug)]
pub enum ThriftException {
    /// Errors encountered within auto-generated code, or when incoming
    /// or outgoing messages violate the Thrift spec.
    ///
    /// These include *out-of-order messages* and *missing required struct
    /// fields*.
    ///
    /// This variant also functions as a catch-all: errors from handler
    /// functions are automatically returned as an `ApplicationException`.
    Application(ApplicationException),
    /// Errors encountered during runtime-library processing.
    ///
    /// These include *message too large* and *unsupported protocol version*.
    Protocol(ProtocolException),
    /// Errors encountered while operating on I/O channels.
    ///
    /// These include *connection closed* and *bind failure*.
    Transport(TransportException),
}

impl From<TransportException> for ThriftException {
    fn from(e: TransportException) -> Self {
        ThriftException::Transport(e)
    }
}

impl From<std::io::Error> for ThriftException {
    fn from(e: std::io::Error) -> Self {
        e.into()
    }
}

impl From<ProtocolException> for ThriftException {
    fn from(e: ProtocolException) -> Self {
        ThriftException::Protocol(e)
    }
}

impl From<string::FromUtf8Error> for ThriftException {
    fn from(err: string::FromUtf8Error) -> Self {
        ThriftException::Protocol(ProtocolException::new(
            ProtocolExceptionKind::InvalidData,
            format!("{:?}", err),
        ))
    }
}

impl Display for ThriftException {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ThriftException {}

impl From<ApplicationException> for DecodeError {
    fn from(value: ApplicationException) -> Self {
        DecodeError::new(DecodeErrorKind::Application(value), "")
    }
}

impl From<ProtocolException> for DecodeError {
    fn from(value: ProtocolException) -> Self {
        DecodeError::new(DecodeErrorKind::Protocol(value), "")
    }
}

impl From<std::io::Error> for DecodeError {
    fn from(value: std::io::Error) -> Self {
        DecodeError::new(DecodeErrorKind::Transport(value.into()), "")
    }
}

impl From<ProtocolException> for EncodeError {
    fn from(value: ProtocolException) -> Self {
        EncodeError::new(EncodeErrorKind::Protocol(value), "")
    }
}

#[derive(Debug)]
pub struct DecodeError {
    kind: DecodeErrorKind,
    message: FastStr,
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)?;
        if matches!(self.kind, DecodeErrorKind::WithContext(_)) {
            write!(f, ", caused by {}", self.kind)?;
        } else {
            write!(f, "{}", self.kind)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum DecodeErrorKind {
    Application(ApplicationException),
    Protocol(ProtocolException),
    Transport(TransportException),
    WithContext(Box<DecodeError>),
}

impl Display for DecodeErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DecodeErrorKind::Application(e) => write!(f, "ApplicationException: {}", e),
            DecodeErrorKind::Protocol(e) => write!(f, "ProtocolException: {}", e),
            DecodeErrorKind::Transport(e) => write!(f, "TransportException: {}", e),
            DecodeErrorKind::WithContext(e) => write!(f, "{}", e),
        }
    }
}

#[derive(Debug)]
pub struct EncodeError {
    kind: EncodeErrorKind,
    message: FastStr,
}

#[derive(Debug)]
pub enum EncodeErrorKind {
    Protocol(ProtocolException),
    WithContext(Box<EncodeError>),
}

impl Display for EncodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)?;
        if matches!(self.kind, EncodeErrorKind::WithContext(_)) {
            write!(f, ", caused by {}", self.kind)?;
        } else {
            write!(f, "{}", self.kind)?;
        }
        Ok(())
    }
}

impl std::error::Error for EncodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            EncodeErrorKind::Protocol(e) => Some(e),
            EncodeErrorKind::WithContext(e) => Some(e.as_ref()),
        }
    }
}

impl std::error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            DecodeErrorKind::Application(e) => Some(e),
            DecodeErrorKind::Protocol(e) => Some(e),
            DecodeErrorKind::Transport(e) => Some(e),
            DecodeErrorKind::WithContext(e) => Some(e.as_ref()),
        }
    }
}

impl Display for EncodeErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            EncodeErrorKind::Protocol(e) => write!(f, "Protocol: {}", e),
            EncodeErrorKind::WithContext(e) => write!(f, "{}", e),
        }
    }
}

pub trait DecodeErrorExt {
    fn with_msg<S: Into<FastStr>>(self, get_msg: impl FnOnce() -> S) -> Self;
}

impl<T> DecodeErrorExt for Result<T, DecodeError> {
    fn with_msg<S: Into<FastStr>>(self, get_msg: impl FnOnce() -> S) -> Self {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(DecodeError {
                kind: DecodeErrorKind::WithContext(Box::new(e)),
                message: get_msg().into(),
            }),
        }
    }
}

pub trait EncodeErrorExt {
    fn with_msg<S: Into<FastStr>>(self, get_msg: impl FnOnce() -> S) -> Self;
}

impl<T> EncodeErrorExt for Result<T, EncodeError> {
    fn with_msg<S: Into<FastStr>>(self, get_msg: impl FnOnce() -> S) -> Self {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(EncodeError {
                kind: EncodeErrorKind::WithContext(Box::new(e)),
                message: get_msg().into(),
            }),
        }
    }
}

impl DecodeError {
    pub fn new<S: Into<FastStr>>(kind: DecodeErrorKind, message: S) -> Self {
        Self {
            message: message.into(),
            kind,
        }
    }

    pub fn new_protocol<S: Into<FastStr>>(kind: ProtocolExceptionKind, message: S) -> Self {
        Self::new(
            DecodeErrorKind::Protocol(ProtocolException::new(kind, message)),
            "",
        )
    }

    pub fn new_application<S: Into<FastStr>>(kind: ApplicationExceptionKind, message: S) -> Self {
        Self::new(
            DecodeErrorKind::Application(ApplicationException::new(kind, message)),
            "",
        )
    }

    pub fn kind(&self) -> &DecodeErrorKind {
        &self.kind
    }

    pub fn message(&self) -> &FastStr {
        &self.message
    }
}

impl EncodeError {
    pub fn new<S: Into<FastStr>>(kind: EncodeErrorKind, message: S) -> EncodeError {
        EncodeError {
            message: message.into(),
            kind,
        }
    }

    pub fn new_protocol<S: Into<FastStr>>(kind: ProtocolExceptionKind, message: S) -> EncodeError {
        EncodeError {
            message: "".into(),
            kind: EncodeErrorKind::Protocol(ProtocolException::new(kind, message)),
        }
    }

    pub fn kind(&self) -> &EncodeErrorKind {
        &self.kind
    }

    pub fn message(&self) -> &FastStr {
        &self.message
    }
}

/// Create a new `Error` instance of type `Application` that wraps an
/// `ApplicationError`.
pub fn new_application_exception<S: Into<FastStr>>(
    kind: ApplicationExceptionKind,
    message: S,
) -> ThriftException {
    ThriftException::Application(ApplicationException::new(kind, message))
}

/// Create a new `Error` instance of type `Protocol` that wraps a
/// `ProtocolException`.
pub fn new_protocol_exception<S: Into<FastStr>>(
    kind: ProtocolExceptionKind,
    message: S,
) -> ThriftException {
    ThriftException::Protocol(ProtocolException::new(kind, message))
}

#[deprecated(
    since = "0.11.0",
    note = "Please use `new_protocol_exception` instead. This function will be removed in the next release."
)]
pub fn new_protocol_error<S: Into<FastStr>>(
    kind: ProtocolExceptionKind,
    message: S,
) -> ThriftException {
    new_protocol_exception(kind, message)
}
