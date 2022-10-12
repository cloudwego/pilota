pub mod const_val {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::unused_unit,
        clippy::needless_borrow,
        unused_mut
    )]
    pub mod const_val {
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, :: pilota :: derivative :: Derivative)]
        #[derivative(Default)]
        #[derive(
            :: pilota :: num_enum :: IntoPrimitive,
            :: pilota :: num_enum :: TryFromPrimitive,
            Clone,
            PartialEq,
        )]
        #[repr(i32)]
        #[derive(Copy)]
        pub enum Index {
            #[derivative(Default)]
            A = 0i32,
            B = 1i32,
        }
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for Index {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::Error> {
                protocol.write_i32(*self as i32)?;
                Ok(())
            }
            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::Error> {
                Ok(Self::try_from(protocol.read_i32()?).map_err(|err| {
                    ::pilota::thrift::new_protocol_error(
                        ::pilota::thrift::ProtocolErrorKind::InvalidData,
                        format!("invalid enum value for Index, value: {}", value),
                    )
                })?)
            }
            async fn decode_async<C: ::tokio::io::AsyncRead + Unpin + Send>(
                protocol: &mut ::pilota::thrift::TAsyncBinaryProtocol<C>,
            ) -> ::std::result::Result<Self, ::pilota::thrift::Error> {
                Ok(Self::try_from(protocol.read_i32().await?).map_err(|err| {
                    ::pilota::thrift::new_protocol_error(
                        ::pilota::thrift::ProtocolErrorKind::InvalidData,
                        format!("invalid enum value for Index, value: {}", value),
                    )
                })?)
            }
            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &T) -> usize {
                protocol.write_i32_len(*self as i32)
            }
        }
        ::pilota::lazy_static::lazy_static! { pub static ref TEST_MAP : :: std :: collections :: HashMap < Index , & 'static str > = { let mut map = :: std :: collections :: HashMap :: with_capacity (2usize) ; map . insert (Index :: A , "hello") ; map . insert (Index :: B , "world") ; map } ; }
    }
}
