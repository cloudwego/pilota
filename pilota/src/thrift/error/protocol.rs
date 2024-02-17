use std::fmt::{self, Display, Formatter};

use faststr::FastStr;

#[deprecated(
    since = "0.11.0",
    note = "Please use the `ProtocolException` instead. This type will be removed in the next release."
)]
pub type ProtocolError = ProtocolException;

/// Information about errors that occur in the runtime library.
///
/// This exception does not send across endpoints, so seems that it is
/// not necessary to keep it in sync with other languages.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ProtocolException {
    /// Protocol error variant.
    ///
    /// If a specific `ProtocolExceptionKind` does not apply use
    /// `ProtocolExceptionKind::Unknown`.
    kind: ProtocolExceptionKind,
    /// Human-readable error message.
    message: FastStr,
}

impl ProtocolException {
    /// Create a new `ProtocolError`.
    pub fn new<S: Into<FastStr>>(kind: ProtocolExceptionKind, message: S) -> ProtocolException {
        ProtocolException {
            kind,
            message: message.into(),
        }
    }

    /// Get the error kind.
    #[inline]
    pub fn kind(&self) -> ProtocolExceptionKind {
        self.kind
    }

    /// Get the error message.
    #[inline]
    pub fn message(&self) -> &FastStr {
        &self.message
    }

    /// Append a message to the existing error message.
    ///
    /// That means, the new message will be: `old_message` + `message`.
    pub fn append_msg(&mut self, message: &str) {
        let mut s = String::with_capacity(self.message.len() + message.len());
        s.push_str(self.message.as_str());
        s.push_str(message);
        self.message = s.into();
    }

    /// Prepend a message to the existing error message.
    ///
    /// That means, the new message will be: `message` + `old_message`.
    pub fn prepend_msg(&mut self, message: &str) {
        let mut s = String::with_capacity(self.message.len() + message.len());
        s.push_str(message);
        s.push_str(self.message.as_str());
        self.message = s.into();
    }
}

impl Display for ProtocolException {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let error_text = match self.kind {
            ProtocolExceptionKind::Unknown => "protocol error",
            ProtocolExceptionKind::InvalidData => "bad data",
            ProtocolExceptionKind::NegativeSize => "negative message size",
            ProtocolExceptionKind::SizeLimit => "message too long",
            ProtocolExceptionKind::BadVersion => "invalid thrift version",
            ProtocolExceptionKind::NotImplemented => "not implemented",
            ProtocolExceptionKind::DepthLimit => "maximum skip depth reached",
        };

        write!(f, "{}, {}", error_text, self.message)
    }
}

impl std::error::Error for ProtocolException {}

#[deprecated(
    since = "0.11.0",
    note = "Please use the `ProtocolExceptionKind` instead. This type will be removed in the next release."
)]
pub type ProtocolErrorKind = ProtocolExceptionKind;

/// Runtime library error categories.
///
/// This list may grow, and it is not recommended to match against it.
#[non_exhaustive]
#[derive(Clone, Copy, Eq, Debug, PartialEq)]
pub enum ProtocolExceptionKind {
    /// Catch-all runtime-library error.
    Unknown,
    /// An invalid argument was supplied to a library function, or invalid data
    /// was received from a Thrift endpoint.
    InvalidData,
    /// An invalid size was received in an encoded field.
    NegativeSize,
    /// Thrift message or field was too long.
    SizeLimit,
    /// Unsupported or unknown Thrift protocol version.
    BadVersion,
    /// Unsupported Thrift protocol, server or field type.
    NotImplemented,
    /// Reached the maximum nested depth to which an encoded Thrift field could
    /// be skipped.
    DepthLimit,
}
