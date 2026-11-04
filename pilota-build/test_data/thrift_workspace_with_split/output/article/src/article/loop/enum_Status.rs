#[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq, Copy)]
#[repr(transparent)]
pub struct Status(i32);

impl Status {
    pub const NORMAL: Self = Self(0);
    pub const DELETED: Self = Self(1);

    pub fn inner(&self) -> i32 {
        self.0
    }

    pub fn to_string(&self) -> ::std::string::String {
        match self {
            Self(0) => ::std::string::String::from("NORMAL"),
            Self(1) => ::std::string::String::from("DELETED"),
            Self(val) => val.to_string(),
        }
    }

    pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
        match value {
            0 => ::std::option::Option::Some(Self::NORMAL),
            1 => ::std::option::Option::Some(Self::DELETED),
            _ => ::std::option::Option::None,
        }
    }
}

impl ::std::convert::From<i32> for Status {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl ::std::convert::From<Status> for i32 {
    fn from(value: Status) -> i32 {
        value.0
    }
}

impl ::pilota::thrift::Message for Status {
    fn encode<__PilotaProtocol: ::pilota::thrift::TOutputProtocol>(
        &self,
        __protocol: &mut __PilotaProtocol,
    ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
        #[allow(unused_imports)]
        use ::pilota::thrift::TOutputProtocolExt;
        __protocol.write_i32(self.inner())?;
        ::std::result::Result::Ok(())
    }

    fn decode<__PilotaProtocol: ::pilota::thrift::TInputProtocol>(
        __protocol: &mut __PilotaProtocol,
    ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
        #[allow(unused_imports)]
        use ::pilota::{Buf, thrift::TLengthProtocolExt};
        let value = __protocol.read_i32()?;
        ::std::result::Result::Ok(::std::convert::TryFrom::try_from(value).map_err(|err| {
            ::pilota::thrift::new_protocol_exception(
                ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                format!("invalid enum value for Status, value: {}", value),
            )
        })?)
    }

    fn decode_async<'a, __PilotaProtocol: ::pilota::thrift::TAsyncInputProtocol>(
        __protocol: &'a mut __PilotaProtocol,
    ) -> ::std::pin::Pin<
        ::std::boxed::Box<
            dyn ::std::future::Future<
                    Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                > + ::std::marker::Send
                + 'a,
        >,
    > {
        ::std::boxed::Box::pin(async move {
            let value = __protocol.read_i32().await?;
            ::std::result::Result::Ok(::std::convert::TryFrom::try_from(value).map_err(|err| {
                ::pilota::thrift::new_protocol_exception(
                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                    format!("invalid enum value for Status, value: {}", value),
                )
            })?)
        })
    }

    fn size<__PilotaProtocol: ::pilota::thrift::TLengthProtocol>(
        &self,
        __protocol: &mut __PilotaProtocol,
    ) -> usize {
        #[allow(unused_imports)]
        use ::pilota::thrift::TLengthProtocolExt;
        __protocol.i32_len(self.inner())
    }
}
