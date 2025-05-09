mod application;
mod macros;
mod protocol;
mod transport;

use std::{
    fmt::{self, Debug, Display, Formatter},
    string,
};

pub use application::*;
use faststr::FastStr;
pub use protocol::*;
pub use transport::*;

/// A Thrift exception.
///
/// This type is used to represent errors that occur during Thrift
/// processing. It is a catch-all for errors that occur in the Thrift
/// runtime, including errors from the protocol, transport, and application
/// layers.
#[derive(Debug, Clone)]
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

impl From<ApplicationException> for ThriftException {
    fn from(e: ApplicationException) -> Self {
        ThriftException::Application(e)
    }
}

impl From<TransportException> for ThriftException {
    fn from(e: TransportException) -> Self {
        ThriftException::Transport(e)
    }
}

impl From<std::io::Error> for ThriftException {
    fn from(e: std::io::Error) -> Self {
        ThriftException::Transport(TransportException::from(e))
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
            format!("{err:?}"),
        ))
    }
}

impl Display for ThriftException {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for ThriftException {}

impl ThriftException {
    /// Get the error message.
    pub fn message(&self) -> &FastStr {
        match self {
            ThriftException::Application(e) => e.message(),
            ThriftException::Protocol(e) => e.message(),
            ThriftException::Transport(e) => e.message(),
        }
    }

    /// Append a message to the existing error message.
    ///
    /// That means, the new message will be: `old_message` + `message`.
    pub fn append_msg(&mut self, message: &str) {
        match self {
            ThriftException::Application(e) => e.append_msg(message),
            ThriftException::Protocol(e) => e.append_msg(message),
            ThriftException::Transport(e) => e.append_msg(message),
        }
    }

    /// Prepend a message to the existing error message.
    ///
    /// That means, the new message will be: `message` + `old_message`.
    pub fn prepend_msg(&mut self, message: &str) {
        match self {
            ThriftException::Application(e) => e.prepend_msg(message),
            ThriftException::Protocol(e) => e.prepend_msg(message),
            ThriftException::Transport(e) => e.prepend_msg(message),
        }
    }
}

/// Create a new `ThriftException` instance of type `Application` that wraps an
/// `ApplicationException`.
pub fn new_application_exception<S: Into<FastStr>>(
    kind: ApplicationExceptionKind,
    message: S,
) -> ThriftException {
    ThriftException::Application(ApplicationException::new(kind, message))
}

/// Create a new `ThriftException` instance of type `Protocol` that wraps a
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
