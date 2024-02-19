use std::{
    fmt::{self, Display, Formatter},
    ops::{Deref, DerefMut},
};

use faststr::FastStr;

use super::ThriftException;
use crate::{msg_impl, thrift::{
    Message, TAsyncInputProtocol, TInputProtocol, TLengthProtocol, TOutputProtocol,
    TStructIdentifier, TType,
}};

// use super::{DecodeError, EncodeError};

const TAPPLICATION_EXCEPTION: TStructIdentifier = TStructIdentifier {
    name: "TApplicationException",
};

#[deprecated(
    since = "0.11.0",
    note = "Please use the `ApplicationException` instead. This type will be removed in the next release."
)]
pub type ApplicationError = ApplicationException;

/// Information about errors in auto-generated code or in user-implemented
/// service handlers.
///
/// This exception will transmit across endpoints and languages, so
/// it is important to keep it in sync with the standard.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ApplicationException {
    /// Application error variant.
    ///
    /// If a specific `ApplicationErrorKind` does not apply use
    /// `ApplicationErrorKind::Unknown`.
    kind: ApplicationExceptionKind,
    /// Human-readable error message.
    message: FastStr,
}

impl ApplicationException {
    /// Create a new `ApplicationError`.
    pub fn new<S: Into<FastStr>>(
        kind: ApplicationExceptionKind,
        message: S,
    ) -> ApplicationException {
        ApplicationException {
            kind,
            message: message.into(),
        }
    }

    /// Get the error kind.
    #[inline]
    pub fn kind(&self) -> ApplicationExceptionKind {
        self.kind
    }

    /// Get the error message.
    #[inline]
    pub fn message(&self) -> &FastStr {
        &self.message
    }

    msg_impl!();
}

impl Display for ApplicationException {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let error_text = match self.kind {
            ApplicationExceptionKind::UNKNOWN => "service error",
            ApplicationExceptionKind::UNKNOWN_METHOD => "unknown service method",
            ApplicationExceptionKind::INVALID_MESSAGE_TYPE => "wrong message type received",
            ApplicationExceptionKind::WRONG_METHOD_NAME => "unknown method reply received",
            ApplicationExceptionKind::BAD_SEQUENCE_ID => "out of order sequence id",
            ApplicationExceptionKind::MISSING_RESULT => "missing method result",
            ApplicationExceptionKind::INTERNAL_ERROR => "remote service threw exception",
            ApplicationExceptionKind::PROTOCOL_ERROR => "protocol error",
            ApplicationExceptionKind::INVALID_TRANSFORM => "invalid transform",
            ApplicationExceptionKind::INVALID_PROTOCOL => "invalid protocol requested",
            ApplicationExceptionKind::UNSUPPORTED_CLIENT_TYPE => "unsupported protocol client",
            _ => "other error",
        };

        write!(f, "{}: {}", error_text, self.message)
    }
}

impl std::error::Error for ApplicationException {}

impl Message for ApplicationException {
    /// Convert an `ApplicationError` into its wire representation and write
    /// it to the remote.
    ///
    /// Application code **should never** call this method directly.
    #[inline]
    fn encode<T: TOutputProtocol>(&self, protocol: &mut T) -> Result<(), ThriftException> {
        protocol.write_struct_begin(&TAPPLICATION_EXCEPTION)?;

        protocol.write_field_begin(TType::Binary, 1)?;
        protocol.write_string(&self.message)?;
        protocol.write_field_end()?;

        protocol.write_field_begin(TType::I32, 2)?;
        protocol.write_i32(self.kind.as_i32())?;
        protocol.write_field_end()?;

        protocol.write_field_stop()?;
        protocol.write_struct_end()?;

        protocol.flush()?;
        Ok(())
    }

    #[inline]
    fn decode<T: TInputProtocol>(protocol: &mut T) -> Result<Self, ThriftException> {
        let mut message = "general remote error".into();
        let mut kind = ApplicationExceptionKind::UNKNOWN;

        protocol.read_struct_begin()?;

        loop {
            let field_ident = protocol.read_field_begin()?;

            if field_ident.field_type == TType::Stop {
                break;
            }

            let id = field_ident
                .id
                .expect("sender should always specify id for non-STOP field");

            match id {
                1 => {
                    let remote_message = protocol.read_string()?;
                    protocol.read_field_end()?;
                    message = remote_message.into();
                }
                2 => {
                    let remote_type_as_int = protocol.read_i32()?;
                    let remote_kind: ApplicationExceptionKind = From::from(remote_type_as_int);
                    protocol.read_field_end()?;
                    kind = remote_kind;
                }
                _ => {
                    protocol.skip(field_ident.field_type)?;
                }
            }
        }

        protocol.read_struct_end()?;

        Ok(ApplicationException { kind, message })
    }

    async fn decode_async<T: TAsyncInputProtocol>(
        protocol: &mut T,
    ) -> Result<Self, ThriftException> {
        let mut message = "general remote error".into();
        let mut kind = ApplicationExceptionKind::UNKNOWN;

        protocol.read_struct_begin().await?;

        loop {
            let field_ident = protocol.read_field_begin().await?;

            if field_ident.field_type == TType::Stop {
                break;
            }

            let id = field_ident
                .id
                .expect("sender should always specify id for non-STOP field");

            match id {
                1 => {
                    let remote_message = protocol.read_string().await?;
                    protocol.read_field_end().await?;
                    message = remote_message.into();
                }
                2 => {
                    let remote_type_as_int = protocol.read_i32().await?;
                    let remote_kind: ApplicationExceptionKind = From::from(remote_type_as_int);
                    protocol.read_field_end().await?;
                    kind = remote_kind;
                }
                _ => {
                    protocol.skip(field_ident.field_type).await?;
                }
            }
        }

        protocol.read_struct_end().await?;

        Ok(ApplicationException { kind, message })
    }

    #[inline]
    fn size<T: TLengthProtocol>(&self, protocol: &mut T) -> usize {
        protocol.struct_begin_len(&TAPPLICATION_EXCEPTION)
            + protocol.field_begin_len(TType::Binary, Some(1))
            + protocol.string_len(&self.message)
            + protocol.field_end_len()
            + protocol.field_begin_len(TType::I32, Some(2))
            + protocol.i32_len(self.kind.as_i32())
            + protocol.field_end_len()
            + protocol.field_stop_len()
            + protocol.struct_end_len()
    }
}

#[deprecated(
    since = "0.11.0",
    note = "Please use the `ApplicationExceptionKind` instead. This type will be removed in the next release."
)]
pub type ApplicationErrorKind = ApplicationExceptionKind;

/// Auto-generated or user-implemented code error categories.
///
/// This list may grow, and it is not recommended to match against it.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct ApplicationExceptionKind(i32);

impl ApplicationExceptionKind {
    /// Catch-all application error.
    pub const UNKNOWN: Self = Self(0);
    /// Made service call to an unknown service method.
    pub const UNKNOWN_METHOD: Self = Self(1);
    /// Received an unknown Thrift message type. That is, not one of the
    /// `thrift::protocol::TMessageType` variants.
    pub const INVALID_MESSAGE_TYPE: Self = Self(2);
    /// Method name in a service reply does not match the name of the
    /// receiving service method.
    pub const WRONG_METHOD_NAME: Self = Self(3);
    /// Received an out-of-order Thrift message.
    pub const BAD_SEQUENCE_ID: Self = Self(4);
    /// Service reply is missing required fields.
    pub const MISSING_RESULT: Self = Self(5);
    /// User server handler error.
    pub const INTERNAL_ERROR: Self = Self(6);
    /// Thrift protocol error. When possible use `Error::ProtocolError` with a
    /// specific `ProtocolExceptionKind` instead.
    pub const PROTOCOL_ERROR: Self = Self(7);
    /// *Unknown*. Included only for compatibility with existing Thrift
    /// implementations.
    pub const INVALID_TRANSFORM: Self = Self(8);
    /// Thrift endpoint requested, or is using, an unsupported encoding.
    pub const INVALID_PROTOCOL: Self = Self(9);
    /// Thrift endpoint requested, or is using, an unsupported auto-generated
    /// client type.
    pub const UNSUPPORTED_CLIENT_TYPE: Self = Self(10);
    /// validation failed
    pub const VALIDATION_FAILED: Self = Self(11);

    #[inline]
    pub fn as_i32(self) -> i32 {
        self.0
    }

    #[inline]
    pub const fn from_i32(i: i32) -> Self {
        Self(i)
    }
}

impl From<i32> for ApplicationExceptionKind {
    #[inline]
    fn from(from: i32) -> Self {
        Self::from_i32(from)
    }
}

impl From<ApplicationExceptionKind> for i32 {
    #[inline]
    fn from(value: ApplicationExceptionKind) -> Self {
        value.as_i32()
    }
}

impl Deref for ApplicationExceptionKind {
    type Target = i32;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ApplicationExceptionKind {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
