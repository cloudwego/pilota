pub mod binary_bytes {
    #![allow(warnings, clippy::all)]
    pub mod binary_bytes {
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct A {
            pub bytes: ::pilota::Bytes,
            pub vec: ::std::vec::Vec<u8>,
        }
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for A {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::Error> {
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "A" };
                protocol.write_struct_begin(&struct_ident)?;
                protocol.write_bytes_field(1i16, (&self.bytes).clone())?;
                protocol.write_bytes_vec_field(2i16, &&self.vec)?;
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }
            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::Error> {
                let mut bytes = None;
                let mut vec = None;
                protocol.read_struct_begin()?;
                loop {
                    let field_ident = protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    }
                    let field_id = field_ident.id;
                    match field_id {
                        Some(1i16) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            bytes = Some(protocol.read_bytes()?);
                        }
                        Some(2i16) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            vec = Some(protocol.read_bytes_vec()?);
                        }
                        _ => {
                            protocol.skip(field_ident.field_type)?;
                        }
                    }
                    protocol.read_field_end()?;
                }
                protocol.read_struct_end()?;
                let Some (bytes) = bytes else { return Err (:: pilota :: thrift :: Error :: Protocol (:: pilota :: thrift :: ProtocolError :: new (:: pilota :: thrift :: ProtocolErrorKind :: InvalidData , "field bytes is required" . to_string ()))) } ;
                let Some (vec) = vec else { return Err (:: pilota :: thrift :: Error :: Protocol (:: pilota :: thrift :: ProtocolError :: new (:: pilota :: thrift :: ProtocolErrorKind :: InvalidData , "field vec is required" . to_string ()))) } ;
                let data = Self { bytes, vec };
                Ok(data)
            }
            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::Error> {
                let mut bytes = None;
                let mut vec = None;
                protocol.read_struct_begin().await?;
                loop {
                    let field_ident = protocol.read_field_begin().await?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    }
                    let field_id = field_ident.id;
                    match field_id {
                        Some(1i16) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            bytes = Some(protocol.read_bytes().await?);
                        }
                        Some(2i16) if field_ident.field_type == ::pilota::thrift::TType::Binary => {
                            vec = Some(protocol.read_bytes_vec().await?);
                        }
                        _ => {
                            protocol.skip(field_ident.field_type).await?;
                        }
                    }
                    protocol.read_field_end().await?;
                }
                protocol.read_struct_end().await?;
                let Some (bytes) = bytes else { return Err (:: pilota :: thrift :: Error :: Protocol (:: pilota :: thrift :: ProtocolError :: new (:: pilota :: thrift :: ProtocolErrorKind :: InvalidData , "field bytes is required" . to_string ()))) } ;
                let Some (vec) = vec else { return Err (:: pilota :: thrift :: Error :: Protocol (:: pilota :: thrift :: ProtocolError :: new (:: pilota :: thrift :: ProtocolErrorKind :: InvalidData , "field vec is required" . to_string ()))) } ;
                let data = Self { bytes, vec };
                Ok(data)
            }
            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.write_struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "A" })
                    + protocol.write_bytes_field_len(Some(1i16), &self.bytes)
                    + protocol.write_bytes_vec_field_len(Some(2i16), &self.vec)
                    + protocol.write_field_stop_len()
                    + protocol.write_struct_end_len()
            }
        }
    }
}
