pub mod ahash {
    #![allow(warnings, clippy::all)]
    pub mod ahash {
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct A {}
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for A {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "A" };
                protocol.write_struct_begin(&struct_ident)?;
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }
            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut __pilota_decoding_field_id = None;
                protocol.read_struct_begin()?;
                if let Err(err) = (|| {
                    loop {
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        }
                        let field_id = field_ident.id;
                        __pilota_decoding_field_id = field_id;
                        match field_id {
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }
                        protocol.read_field_end()?;
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!("decode struct `A` field(#{}) failed", field_id),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end()?;
                let data = Self {};
                Ok(data)
            }
            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut __pilota_decoding_field_id = None;
                protocol.read_struct_begin().await?;
                if let Err(err) = async {
                    loop {
                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        }
                        let field_id = field_ident.id;
                        __pilota_decoding_field_id = field_id;
                        match field_id {
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }
                        protocol.read_field_end().await?;
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                }
                .await
                {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!("decode struct `A` field(#{}) failed", field_id),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end().await?;
                let data = Self {};
                Ok(data)
            }
            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.write_struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "A" })
                    + protocol.write_field_stop_len()
                    + protocol.write_struct_end_len()
            }
        }
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct B {
            pub m: ::pilota::ahash::AHashMap<i32, ::std::vec::Vec<::std::sync::Arc<A>>>,
            pub s: ::pilota::ahash::AHashSet<i32>,
        }
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for B {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "B" };
                protocol.write_struct_begin(&struct_ident)?;
                protocol.write_ahashmap_field(
                    1i16,
                    ::pilota::thrift::TType::I32,
                    ::pilota::thrift::TType::List,
                    &&self.m,
                    |protocol, key| {
                        protocol.write_i32(*key)?;
                        Ok(())
                    },
                    |protocol, val| {
                        protocol.write_list(
                            ::pilota::thrift::TType::Struct,
                            &val,
                            |protocol, val| {
                                protocol.write_struct(val)?;
                                Ok(())
                            },
                        )?;
                        Ok(())
                    },
                )?;
                protocol.write_ahashset_field(
                    2i16,
                    ::pilota::thrift::TType::I32,
                    &&self.s,
                    |protocol, val| {
                        protocol.write_i32(*val)?;
                        Ok(())
                    },
                )?;
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }
            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut m = None;
                let mut s = None;
                let mut __pilota_decoding_field_id = None;
                protocol.read_struct_begin()?;
                if let Err(err) = (|| {
                    loop {
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        }
                        let field_id = field_ident.id;
                        __pilota_decoding_field_id = field_id;
                        match field_id {
                            Some(1i16)
                                if field_ident.field_type == ::pilota::thrift::TType::Map =>
                            {
                                m = Some({
                                    let map_ident = protocol.read_map_begin()?;
                                    let mut val =
                                        ::pilota::ahash::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        let el_key = protocol.read_i32()?;
                                        let el_val = {
                                            let list_ident = protocol.read_list_begin()?;
                                            let mut val = Vec::with_capacity(list_ident.size);
                                            for _ in 0..list_ident.size {
                                                val.push(::std::sync::Arc::new(
                                                    ::pilota::thrift::Message::decode(protocol)?,
                                                ));
                                            }
                                            protocol.read_list_end()?;
                                            val
                                        };
                                        val.insert(el_key, el_val);
                                    }
                                    protocol.read_map_end()?;
                                    val
                                });
                            }
                            Some(2i16)
                                if field_ident.field_type == ::pilota::thrift::TType::Set =>
                            {
                                s = Some({
                                    let list_ident = protocol.read_set_begin()?;
                                    let mut val =
                                        ::pilota::ahash::AHashSet::with_capacity(list_ident.size);
                                    for _ in 0..list_ident.size {
                                        val.insert(protocol.read_i32()?);
                                    }
                                    protocol.read_set_end()?;
                                    val
                                });
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }
                        protocol.read_field_end()?;
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!("decode struct `B` field(#{}) failed", field_id),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end()?;
                let Some (m) = m else { return Err (:: pilota :: thrift :: DecodeError :: new (:: pilota :: thrift :: DecodeErrorKind :: InvalidData , "field m is required" . to_string ())) } ;
                let Some (s) = s else { return Err (:: pilota :: thrift :: DecodeError :: new (:: pilota :: thrift :: DecodeErrorKind :: InvalidData , "field s is required" . to_string ())) } ;
                let data = Self { m, s };
                Ok(data)
            }
            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut m = None;
                let mut s = None;
                let mut __pilota_decoding_field_id = None;
                protocol.read_struct_begin().await?;
                if let Err(err) = async {
                    loop {
                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        }
                        let field_id = field_ident.id;
                        __pilota_decoding_field_id = field_id;
                        match field_id {
                            Some(1i16)
                                if field_ident.field_type == ::pilota::thrift::TType::Map =>
                            {
                                m = Some({
                                    let map_ident = protocol.read_map_begin().await?;
                                    let mut val =
                                        ::pilota::ahash::AHashMap::with_capacity(map_ident.size);
                                    for _ in 0..map_ident.size {
                                        let el_key = protocol.read_i32().await?;
                                        let el_val = {
                                            let list_ident = protocol.read_list_begin().await?;
                                            let mut val = Vec::with_capacity(list_ident.size);
                                            for _ in 0..list_ident.size {
                                                val.push(::std::sync::Arc::new(
                                                    ::pilota::thrift::Message::decode_async(
                                                        protocol,
                                                    )
                                                    .await?,
                                                ));
                                            }
                                            protocol.read_list_end().await?;
                                            val
                                        };
                                        val.insert(el_key, el_val);
                                    }
                                    protocol.read_map_end().await?;
                                    val
                                });
                            }
                            Some(2i16)
                                if field_ident.field_type == ::pilota::thrift::TType::Set =>
                            {
                                s = Some({
                                    let list_ident = protocol.read_set_begin().await?;
                                    let mut val =
                                        ::pilota::ahash::AHashSet::with_capacity(list_ident.size);
                                    for _ in 0..list_ident.size {
                                        val.insert(protocol.read_i32().await?);
                                    }
                                    protocol.read_set_end().await?;
                                    val
                                });
                            }
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }
                        protocol.read_field_end().await?;
                    }
                    Ok::<_, ::pilota::thrift::DecodeError>(())
                }
                .await
                {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        return Err(::pilota::thrift::DecodeError::new(
                            ::pilota::thrift::DecodeErrorKind::WithContext(::std::boxed::Box::new(
                                err,
                            )),
                            format!("decode struct `B` field(#{}) failed", field_id),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end().await?;
                let Some (m) = m else { return Err (:: pilota :: thrift :: DecodeError :: new (:: pilota :: thrift :: DecodeErrorKind :: InvalidData , "field m is required" . to_string ())) } ;
                let Some (s) = s else { return Err (:: pilota :: thrift :: DecodeError :: new (:: pilota :: thrift :: DecodeErrorKind :: InvalidData , "field s is required" . to_string ())) } ;
                let data = Self { m, s };
                Ok(data)
            }
            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.write_struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "B" })
                    + protocol.write_ahashmap_field_len(
                        Some(1i16),
                        ::pilota::thrift::TType::I32,
                        ::pilota::thrift::TType::List,
                        &self.m,
                        |protocol, key| protocol.write_i32_len(*key),
                        |protocol, val| {
                            protocol.write_list_len(
                                ::pilota::thrift::TType::Struct,
                                val,
                                |protocol, el| protocol.write_struct_len(el),
                            )
                        },
                    )
                    + protocol.write_ahashset_field_len(
                        Some(2i16),
                        ::pilota::thrift::TType::I32,
                        &self.s,
                        |protocol, el| protocol.write_i32_len(*el),
                    )
                    + protocol.write_field_stop_len()
                    + protocol.write_struct_end_len()
            }
        }
        ::pilota::lazy_static::lazy_static! { pub static ref TEST_MAP_LIST : :: pilota :: ahash :: AHashMap < i32 , :: std :: vec :: Vec < & 'static str > > = { let mut map = :: pilota :: ahash :: AHashMap :: with_capacity (1usize) ; map . insert (1i32 , :: std :: vec ! ["hello"]) ; map } ; }
        impl ::std::convert::From<Index> for i32 {
            fn from(e: Index) -> Self {
                e as _
            }
        }
        impl ::std::convert::TryFrom<i32> for Index {
            type Error = ::pilota::EnumConvertError<i32>;
            #[allow(non_upper_case_globals)]
            fn try_from(v: i32) -> Result<Self, ::pilota::EnumConvertError<i32>> {
                const A: i32 = Index::A as i32;
                const B: i32 = Index::B as i32;
                match v {
                    A => ::std::result::Result::Ok(Index::A),
                    B => ::std::result::Result::Ok(Index::B),
                    _ => ::std::result::Result::Err(::pilota::EnumConvertError::InvalidNum(
                        v, "Index",
                    )),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, :: pilota :: derivative :: Derivative)]
        #[derivative(Default)]
        #[derive(Clone, PartialEq)]
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
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                use ::pilota::thrift::TOutputProtocolExt;
                protocol.write_i32(*self as i32)?;
                Ok(())
            }
            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let value = protocol.read_i32()?;
                Ok(Self::try_from(value).map_err(|err| {
                    ::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        format!("invalid enum value for Index, value: {}", value),
                    )
                })?)
            }
            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let value = protocol.read_i32().await?;
                Ok(Self::try_from(value).map_err(|err| {
                    ::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        format!("invalid enum value for Index, value: {}", value),
                    )
                })?)
            }
            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.write_i32_len(*self as i32)
            }
        }
        ::pilota::lazy_static::lazy_static! { pub static ref TEST_MAP : :: pilota :: ahash :: AHashMap < Index , & 'static str > = { let mut map = :: pilota :: ahash :: AHashMap :: with_capacity (2usize) ; map . insert (Index :: A , "hello") ; map . insert (Index :: B , "world") ; map } ; }
    }
}
