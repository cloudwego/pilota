use std::{
    fmt::{self, Display, Formatter},
    io,
};

use faststr::FastStr;

#[deprecated(
    since = "0.11.0",
    note = "Please use the `TransportException` instead. This type will be removed in the next release."
)]
pub type TransportError = TransportException;

/// Information about I/O errors.
///
/// This exception does not send across endpoints, so seems that it is
/// not necessary to keep it in sync with other languages.
///
/// This exception is used to wrap I/O errors and provide a human-readable
/// error message cache.
#[derive(Debug)]
pub struct TransportException {
    /// Actual I/O error.
    io_error: io::Error,
    /// Human-readable error message.
    message: FastStr,
}

impl PartialEq for TransportException {
    fn eq(&self, other: &Self) -> bool {
        self.io_error.kind() == other.io_error.kind()
    }
}

impl Eq for TransportException {}

impl TransportException {
    #[inline]
    pub fn io_error(&self) -> &io::Error {
        &self.io_error
    }

    #[inline]
    pub fn message(&self) -> &FastStr {
        &self.message
    }

    #[inline]
    pub fn kind(&self) -> io::ErrorKind {
        self.io_error.kind()
    }

    /// Append a message to the existing error message.
    pub fn append_msg(&mut self, message: &str) {
        let mut s = String::with_capacity(self.message.len() + message.len());
        s.push_str(self.message.as_str());
        s.push_str(message);
        self.message = s.into();
    }
}

impl From<io::Error> for TransportException {
    #[inline]
    fn from(err: io::Error) -> Self {
        let message = FastStr::from_string(err.to_string());
        TransportException {
            io_error: err,
            message,
        }
    }
}

impl Display for TransportException {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind(), self.message())
    }
}

impl std::error::Error for TransportException {
    #[inline]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.io_error)
    }
}
