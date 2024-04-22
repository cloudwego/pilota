pub mod unknown_fields {
    #![allow(warnings, clippy::all)]

    pub mod must_gen_items {
        #[derive(
            PartialOrd,
            Hash,
            Eq,
            Ord,
            Debug,
            Default,
            ::pilota::serde::Serialize,
            ::pilota::serde::Deserialize,
            Clone,
            PartialEq,
        )]
        pub struct A {
            pub a: ::std::option::Option<i32>,
            pub _unknown_fields: ::pilota::LinkedBytes,
        }
        impl ::pilota::thrift::Message for A {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "A" };

                protocol.write_struct_begin(&struct_ident)?;
                if let Some(value) = self.a.as_ref() {
                    protocol.write_i32_field(1, *value)?;
                }
                for bytes in self._unknown_fields.list.iter() {
                    protocol.write_bytes_without_len(bytes.clone());
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut a = None;
                let mut _unknown_fields = ::pilota::LinkedBytes::new();

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(mut err) = (|| {
                    loop {
                        let mut __pilota_offset = 0;
                        let __pilota_begin_ptr = protocol.buf().chunk().as_ptr();
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            __pilota_offset += protocol.field_stop_len();
                            break;
                        } else {
                            __pilota_offset +=
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1) if field_ident.field_type == ::pilota::thrift::TType::I32 => {
                                a = Some(protocol.read_i32()?);
                            }
                            _ => {
                                __pilota_offset += protocol.skip(field_ident.field_type)?;
                                _unknown_fields.push_back(
                                    protocol
                                        .get_bytes(Some(__pilota_begin_ptr), __pilota_offset)?,
                                );
                            }
                        }

                        protocol.read_field_end()?;
                        __pilota_offset += protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::ThriftException>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!(
                            "decode struct `A` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return Err(err);
                };
                protocol.read_struct_end()?;

                let data = Self { a, _unknown_fields };
                Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut a = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(mut err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type == ::pilota::thrift::TType::I32 =>
                                {
                                    a = Some(protocol.read_i32().await?);
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!(
                                "decode struct `A` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return Err(err);
                    };
                    protocol.read_struct_end().await?;

                    let data = Self {
                        a,
                        _unknown_fields: ::pilota::LinkedBytes::new(),
                    };
                    Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "A" })
                    + self
                        .a
                        .as_ref()
                        .map_or(0, |value| protocol.i32_field_len(Some(1), *value))
                    + self._unknown_fields.size()
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(
            PartialOrd,
            Hash,
            Eq,
            Ord,
            Debug,
            Default,
            ::pilota::serde::Serialize,
            ::pilota::serde::Deserialize,
            Clone,
            PartialEq,
        )]
        pub struct B {
            pub a: ::std::option::Option<A>,
            pub _unknown_fields: ::pilota::LinkedBytes,
        }
        impl ::pilota::thrift::Message for B {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "b" };

                protocol.write_struct_begin(&struct_ident)?;
                if let Some(value) = self.a.as_ref() {
                    protocol.write_struct_field(2, value, ::pilota::thrift::TType::Struct)?;
                }
                for bytes in self._unknown_fields.list.iter() {
                    protocol.write_bytes_without_len(bytes.clone());
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut a = None;
                let mut _unknown_fields = ::pilota::LinkedBytes::new();

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(mut err) = (|| {
                    loop {
                        let mut __pilota_offset = 0;
                        let __pilota_begin_ptr = protocol.buf().chunk().as_ptr();
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            __pilota_offset += protocol.field_stop_len();
                            break;
                        } else {
                            __pilota_offset +=
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(2)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                a = Some(::pilota::thrift::Message::decode(protocol)?);
                            }
                            _ => {
                                __pilota_offset += protocol.skip(field_ident.field_type)?;
                                _unknown_fields.push_back(
                                    protocol
                                        .get_bytes(Some(__pilota_begin_ptr), __pilota_offset)?,
                                );
                            }
                        }

                        protocol.read_field_end()?;
                        __pilota_offset += protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::ThriftException>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!(
                            "decode struct `b` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return Err(err);
                };
                protocol.read_struct_end()?;

                let data = Self { a, _unknown_fields };
                Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut a = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(mut err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(2)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    a = Some(
                                        <A as ::pilota::thrift::Message>::decode_async(protocol)
                                            .await?,
                                    );
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!(
                                "decode struct `b` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return Err(err);
                    };
                    protocol.read_struct_end().await?;

                    let data = Self {
                        a,
                        _unknown_fields: ::pilota::LinkedBytes::new(),
                    };
                    Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "b" })
                    + self
                        .a
                        .as_ref()
                        .map_or(0, |value| protocol.struct_field_len(Some(2), value))
                    + self._unknown_fields.size()
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
    }

    pub mod unknown_fields {
        #[derive(
            PartialOrd,
            Hash,
            Eq,
            Ord,
            Debug,
            Default,
            ::pilota::serde::Serialize,
            ::pilota::serde::Deserialize,
            Clone,
            PartialEq,
        )]
        pub struct A {
            pub bytes: ::pilota::Bytes,

            pub vec: ::std::vec::Vec<u8>,
            pub _unknown_fields: ::pilota::LinkedBytes,
        }
        impl ::pilota::thrift::Message for A {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "A" };

                protocol.write_struct_begin(&struct_ident)?;
                protocol.write_bytes_field(1, (&self.bytes).clone())?;
                protocol.write_bytes_vec_field(2, &self.vec)?;
                for bytes in self._unknown_fields.list.iter() {
                    protocol.write_bytes_without_len(bytes.clone());
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut bytes = None;
                let mut vec = None;
                let mut _unknown_fields = ::pilota::LinkedBytes::new();

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(mut err) = (|| {
                    loop {
                        let mut __pilota_offset = 0;
                        let __pilota_begin_ptr = protocol.buf().chunk().as_ptr();
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            __pilota_offset += protocol.field_stop_len();
                            break;
                        } else {
                            __pilota_offset +=
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                bytes = Some(protocol.read_bytes()?);
                            }
                            Some(2)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                vec = Some(protocol.read_bytes_vec()?);
                            }
                            _ => {
                                __pilota_offset += protocol.skip(field_ident.field_type)?;
                                _unknown_fields.push_back(
                                    protocol
                                        .get_bytes(Some(__pilota_begin_ptr), __pilota_offset)?,
                                );
                            }
                        }

                        protocol.read_field_end()?;
                        __pilota_offset += protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::ThriftException>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!(
                            "decode struct `A` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return Err(err);
                };
                protocol.read_struct_end()?;

                let Some(bytes) = bytes else {
                    return Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field bytes is required".to_string(),
                    ));
                };
                let Some(vec) = vec else {
                    return Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field vec is required".to_string(),
                    ));
                };

                let data = Self {
                    bytes,
                    vec,
                    _unknown_fields,
                };
                Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut bytes = None;
                    let mut vec = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(mut err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    bytes = Some(protocol.read_bytes().await?);
                                }
                                Some(2)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    vec = Some(protocol.read_bytes_vec().await?);
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!(
                                "decode struct `A` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return Err(err);
                    };
                    protocol.read_struct_end().await?;

                    let Some(bytes) = bytes else {
                        return Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "field bytes is required".to_string(),
                        ));
                    };
                    let Some(vec) = vec else {
                        return Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "field vec is required".to_string(),
                        ));
                    };

                    let data = Self {
                        bytes,
                        vec,
                        _unknown_fields: ::pilota::LinkedBytes::new(),
                    };
                    Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "A" })
                    + protocol.bytes_field_len(Some(1), &self.bytes)
                    + protocol.bytes_vec_field_len(Some(2), &self.vec)
                    + self._unknown_fields.size()
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(
            PartialOrd,
            Hash,
            Eq,
            Ord,
            Debug,
            Default,
            ::pilota::serde::Serialize,
            ::pilota::serde::Deserialize,
            Clone,
            PartialEq,
        )]
        pub struct D {
            pub td: Td,
            pub _unknown_fields: ::pilota::LinkedBytes,
        }
        impl ::pilota::thrift::Message for D {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "D" };

                protocol.write_struct_begin(&struct_ident)?;
                protocol.write_struct_field(1, &self.td, ::pilota::thrift::TType::List)?;
                for bytes in self._unknown_fields.list.iter() {
                    protocol.write_bytes_without_len(bytes.clone());
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut td = None;
                let mut _unknown_fields = ::pilota::LinkedBytes::new();

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(mut err) = (|| {
                    loop {
                        let mut __pilota_offset = 0;
                        let __pilota_begin_ptr = protocol.buf().chunk().as_ptr();
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            __pilota_offset += protocol.field_stop_len();
                            break;
                        } else {
                            __pilota_offset +=
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1) if field_ident.field_type == ::pilota::thrift::TType::List => {
                                td = Some(::pilota::thrift::Message::decode(protocol)?);
                            }
                            _ => {
                                __pilota_offset += protocol.skip(field_ident.field_type)?;
                                _unknown_fields.push_back(
                                    protocol
                                        .get_bytes(Some(__pilota_begin_ptr), __pilota_offset)?,
                                );
                            }
                        }

                        protocol.read_field_end()?;
                        __pilota_offset += protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::ThriftException>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!(
                            "decode struct `D` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return Err(err);
                };
                protocol.read_struct_end()?;

                let Some(td) = td else {
                    return Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field td is required".to_string(),
                    ));
                };

                let data = Self {
                    td,
                    _unknown_fields,
                };
                Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut td = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(mut err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type == ::pilota::thrift::TType::List =>
                                {
                                    td = Some(
                                        <Td as ::pilota::thrift::Message>::decode_async(protocol)
                                            .await?,
                                    );
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!(
                                "decode struct `D` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return Err(err);
                    };
                    protocol.read_struct_end().await?;

                    let Some(td) = td else {
                        return Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "field td is required".to_string(),
                        ));
                    };

                    let data = Self {
                        td,
                        _unknown_fields: ::pilota::LinkedBytes::new(),
                    };
                    Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "D" })
                    + protocol.struct_field_len(Some(1), &self.td)
                    + self._unknown_fields.size()
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        pub trait Test {}
        #[derive(
            PartialOrd,
            Hash,
            Eq,
            Ord,
            Debug,
            Default,
            ::pilota::serde::Serialize,
            ::pilota::serde::Deserialize,
            Clone,
            PartialEq,
        )]
        pub struct TestTest123ArgsSend {}
        impl ::pilota::thrift::Message for TestTest123ArgsSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TestTest123ArgsSend",
                };

                protocol.write_struct_begin(&struct_ident)?;

                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(mut err) = (|| {
                    loop {
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            protocol.field_stop_len();
                            break;
                        } else {
                            protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }

                        protocol.read_field_end()?;
                        protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::ThriftException>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!(
                            "decode struct `TestTest123ArgsSend` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return Err(err);
                };
                protocol.read_struct_end()?;

                let data = Self {};
                Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(mut err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!("decode struct `TestTest123ArgsSend` field(#{}) failed, caused by: ", field_id));
                        }
                        return Err(err);
                    };
                    protocol.read_struct_end().await?;

                    let data = Self {};
                    Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestTest123ArgsSend",
                }) + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        pub const TEST_LIST: [&'static str; 2] = ["hello", "world"];
        #[derive(
            PartialOrd,
            Hash,
            Eq,
            Ord,
            Debug,
            Default,
            ::pilota::serde::Serialize,
            ::pilota::serde::Deserialize,
            Clone,
            PartialEq,
        )]
        pub struct B {
            pub faststr: ::pilota::FastStr,

            pub string: ::std::string::String,

            pub list: ::std::vec::Vec<::std::vec::Vec<::pilota::FastStr>>,
            pub _unknown_fields: ::pilota::LinkedBytes,
        }
        impl ::pilota::thrift::Message for B {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "B" };

                protocol.write_struct_begin(&struct_ident)?;
                protocol.write_faststr_field(1, (&self.faststr).clone())?;
                protocol.write_string_field(2, &self.string)?;
                protocol.write_list_field(
                    3,
                    ::pilota::thrift::TType::List,
                    &&self.list,
                    |protocol, val| {
                        protocol.write_list(
                            ::pilota::thrift::TType::Binary,
                            &val,
                            |protocol, val| {
                                protocol.write_faststr((val).clone())?;
                                Ok(())
                            },
                        )?;
                        Ok(())
                    },
                )?;
                for bytes in self._unknown_fields.list.iter() {
                    protocol.write_bytes_without_len(bytes.clone());
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut faststr = None;
                let mut string = None;
                let mut list = None;
                let mut _unknown_fields = ::pilota::LinkedBytes::new();

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(mut err) = (|| {
                    loop {
                        let mut __pilota_offset = 0;
                        let __pilota_begin_ptr = protocol.buf().chunk().as_ptr();
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            __pilota_offset += protocol.field_stop_len();
                            break;
                        } else {
                            __pilota_offset +=
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                faststr = Some(protocol.read_faststr()?);
                            }
                            Some(2)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                string = Some(protocol.read_string()?);
                            }
                            Some(3) if field_ident.field_type == ::pilota::thrift::TType::List => {
                                list = Some(unsafe {
                                    let list_ident = protocol.read_list_begin()?;
                                    let mut val: Vec<::std::vec::Vec<::pilota::FastStr>> =
                                        Vec::with_capacity(list_ident.size);
                                    for i in 0..list_ident.size {
                                        val.as_mut_ptr().offset(i as isize).write(unsafe {
                                            let list_ident = protocol.read_list_begin()?;
                                            let mut val: Vec<::pilota::FastStr> =
                                                Vec::with_capacity(list_ident.size);
                                            for i in 0..list_ident.size {
                                                val.as_mut_ptr()
                                                    .offset(i as isize)
                                                    .write(protocol.read_faststr()?);
                                            }
                                            val.set_len(list_ident.size);
                                            protocol.read_list_end()?;
                                            val
                                        });
                                    }
                                    val.set_len(list_ident.size);
                                    protocol.read_list_end()?;
                                    val
                                });
                            }
                            _ => {
                                __pilota_offset += protocol.skip(field_ident.field_type)?;
                                _unknown_fields.push_back(
                                    protocol
                                        .get_bytes(Some(__pilota_begin_ptr), __pilota_offset)?,
                                );
                            }
                        }

                        protocol.read_field_end()?;
                        __pilota_offset += protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::ThriftException>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!(
                            "decode struct `B` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return Err(err);
                };
                protocol.read_struct_end()?;

                let Some(faststr) = faststr else {
                    return Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field faststr is required".to_string(),
                    ));
                };
                let Some(string) = string else {
                    return Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field string is required".to_string(),
                    ));
                };
                let Some(list) = list else {
                    return Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field list is required".to_string(),
                    ));
                };

                let data = Self {
                    faststr,
                    string,
                    list,
                    _unknown_fields,
                };
                Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut faststr = None;
                    let mut string = None;
                    let mut list = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(mut err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    faststr = Some(protocol.read_faststr().await?);
                                }
                                Some(2)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    string = Some(protocol.read_string().await?);
                                }
                                Some(3)
                                    if field_ident.field_type == ::pilota::thrift::TType::List =>
                                {
                                    list = Some({
                                        let list_ident = protocol.read_list_begin().await?;
                                        let mut val = Vec::with_capacity(list_ident.size);
                                        for _ in 0..list_ident.size {
                                            val.push({
                                                let list_ident = protocol.read_list_begin().await?;
                                                let mut val = Vec::with_capacity(list_ident.size);
                                                for _ in 0..list_ident.size {
                                                    val.push(protocol.read_faststr().await?);
                                                }
                                                protocol.read_list_end().await?;
                                                val
                                            });
                                        }
                                        protocol.read_list_end().await?;
                                        val
                                    });
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!(
                                "decode struct `B` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return Err(err);
                    };
                    protocol.read_struct_end().await?;

                    let Some(faststr) = faststr else {
                        return Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "field faststr is required".to_string(),
                        ));
                    };
                    let Some(string) = string else {
                        return Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "field string is required".to_string(),
                        ));
                    };
                    let Some(list) = list else {
                        return Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "field list is required".to_string(),
                        ));
                    };

                    let data = Self {
                        faststr,
                        string,
                        list,
                        _unknown_fields: ::pilota::LinkedBytes::new(),
                    };
                    Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "B" })
                    + protocol.faststr_field_len(Some(1), &self.faststr)
                    + protocol.string_field_len(Some(2), &&self.string)
                    + protocol.list_field_len(
                        Some(3),
                        ::pilota::thrift::TType::List,
                        &self.list,
                        |protocol, el| {
                            protocol.list_len(
                                ::pilota::thrift::TType::Binary,
                                el,
                                |protocol, el| protocol.faststr_len(el),
                            )
                        },
                    )
                    + self._unknown_fields.size()
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(
            PartialOrd,
            Hash,
            Eq,
            Ord,
            Debug,
            Default,
            ::pilota::serde::Serialize,
            ::pilota::serde::Deserialize,
            Clone,
            PartialEq,
        )]
        pub struct SubMessage {
            pub _unknown_fields: ::pilota::LinkedBytes,
        }
        impl ::pilota::thrift::Message for SubMessage {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "SubMessage" };

                protocol.write_struct_begin(&struct_ident)?;
                for bytes in self._unknown_fields.list.iter() {
                    protocol.write_bytes_without_len(bytes.clone());
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut _unknown_fields = ::pilota::LinkedBytes::new();

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(mut err) = (|| {
                    loop {
                        let mut __pilota_offset = 0;
                        let __pilota_begin_ptr = protocol.buf().chunk().as_ptr();
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            __pilota_offset += protocol.field_stop_len();
                            break;
                        } else {
                            __pilota_offset +=
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            _ => {
                                __pilota_offset += protocol.skip(field_ident.field_type)?;
                                _unknown_fields.push_back(
                                    protocol
                                        .get_bytes(Some(__pilota_begin_ptr), __pilota_offset)?,
                                );
                            }
                        }

                        protocol.read_field_end()?;
                        __pilota_offset += protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::ThriftException>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!(
                            "decode struct `SubMessage` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return Err(err);
                };
                protocol.read_struct_end()?;

                let data = Self { _unknown_fields };
                Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(mut err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!(
                                "decode struct `SubMessage` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return Err(err);
                    };
                    protocol.read_struct_end().await?;

                    let data = Self {
                        _unknown_fields: ::pilota::LinkedBytes::new(),
                    };
                    Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol
                    .struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "SubMessage" })
                    + self._unknown_fields.size()
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(
            PartialOrd,
            Hash,
            Eq,
            Ord,
            Debug,
            Default,
            ::pilota::serde::Serialize,
            ::pilota::serde::Deserialize,
            Clone,
            PartialEq,
        )]
        pub struct StException {
            pub message: ::std::option::Option<::pilota::FastStr>,
            pub _unknown_fields: ::pilota::LinkedBytes,
        }
        impl ::pilota::thrift::Message for StException {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "STException",
                };

                protocol.write_struct_begin(&struct_ident)?;
                if let Some(value) = self.message.as_ref() {
                    protocol.write_faststr_field(1, (value).clone())?;
                }
                for bytes in self._unknown_fields.list.iter() {
                    protocol.write_bytes_without_len(bytes.clone());
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut message = None;
                let mut _unknown_fields = ::pilota::LinkedBytes::new();

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(mut err) = (|| {
                    loop {
                        let mut __pilota_offset = 0;
                        let __pilota_begin_ptr = protocol.buf().chunk().as_ptr();
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            __pilota_offset += protocol.field_stop_len();
                            break;
                        } else {
                            __pilota_offset +=
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                message = Some(protocol.read_faststr()?);
                            }
                            _ => {
                                __pilota_offset += protocol.skip(field_ident.field_type)?;
                                _unknown_fields.push_back(
                                    protocol
                                        .get_bytes(Some(__pilota_begin_ptr), __pilota_offset)?,
                                );
                            }
                        }

                        protocol.read_field_end()?;
                        __pilota_offset += protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::ThriftException>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!(
                            "decode struct `STException` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return Err(err);
                };
                protocol.read_struct_end()?;

                let data = Self {
                    message,
                    _unknown_fields,
                };
                Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut message = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(mut err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Binary =>
                                {
                                    message = Some(protocol.read_faststr().await?);
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!(
                                "decode struct `STException` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return Err(err);
                    };
                    protocol.read_struct_end().await?;

                    let data = Self {
                        message,
                        _unknown_fields: ::pilota::LinkedBytes::new(),
                    };
                    Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "STException",
                }) + self
                    .message
                    .as_ref()
                    .map_or(0, |value| protocol.faststr_field_len(Some(1), value))
                    + self._unknown_fields.size()
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(
            PartialOrd,
            Hash,
            Eq,
            Ord,
            Debug,
            Default,
            ::pilota::serde::Serialize,
            ::pilota::serde::Deserialize,
            Clone,
            PartialEq,
        )]
        pub struct TestTest123ArgsRecv {}
        impl ::pilota::thrift::Message for TestTest123ArgsRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TestTest123ArgsRecv",
                };

                protocol.write_struct_begin(&struct_ident)?;

                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(mut err) = (|| {
                    loop {
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            protocol.field_stop_len();
                            break;
                        } else {
                            protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }

                        protocol.read_field_end()?;
                        protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::ThriftException>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!(
                            "decode struct `TestTest123ArgsRecv` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return Err(err);
                };
                protocol.read_struct_end()?;

                let data = Self {};
                Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(mut err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!("decode struct `TestTest123ArgsRecv` field(#{}) failed, caused by: ", field_id));
                        }
                        return Err(err);
                    };
                    protocol.read_struct_end().await?;

                    let data = Self {};
                    Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestTest123ArgsRecv",
                }) + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        ::pilota::lazy_static::lazy_static! {
            pub static ref TEST_MAP_LIST: ::pilota::AHashMap<i32, ::std::vec::Vec<&'static str>> = {
            let mut map = ::pilota::AHashMap::with_capacity(1);
            map.insert(1i32, ::std::vec!["hello"]);
            map
        };
        }
        #[derive(
            PartialOrd,
            Hash,
            Eq,
            Ord,
            Debug,
            Default,
            ::pilota::serde::Serialize,
            ::pilota::serde::Deserialize,
            Clone,
            PartialEq,
        )]
        pub struct C {
            pub a: super::must_gen_items::A,
            pub _unknown_fields: ::pilota::LinkedBytes,
        }
        impl ::pilota::thrift::Message for C {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "C" };

                protocol.write_struct_begin(&struct_ident)?;
                protocol.write_struct_field(1, &self.a, ::pilota::thrift::TType::Struct)?;
                for bytes in self._unknown_fields.list.iter() {
                    protocol.write_bytes_without_len(bytes.clone());
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut a = None;
                let mut _unknown_fields = ::pilota::LinkedBytes::new();

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(mut err) = (|| {
                    loop {
                        let mut __pilota_offset = 0;
                        let __pilota_begin_ptr = protocol.buf().chunk().as_ptr();
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            __pilota_offset += protocol.field_stop_len();
                            break;
                        } else {
                            __pilota_offset +=
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                a = Some(::pilota::thrift::Message::decode(protocol)?);
                            }
                            _ => {
                                __pilota_offset += protocol.skip(field_ident.field_type)?;
                                _unknown_fields.push_back(
                                    protocol
                                        .get_bytes(Some(__pilota_begin_ptr), __pilota_offset)?,
                                );
                            }
                        }

                        protocol.read_field_end()?;
                        __pilota_offset += protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::ThriftException>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!(
                            "decode struct `C` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return Err(err);
                };
                protocol.read_struct_end()?;

                let Some(a) = a else {
                    return Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field a is required".to_string(),
                    ));
                };

                let data = Self { a, _unknown_fields };
                Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut a = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(mut err) = async {
                    loop {


                let field_ident = protocol.read_field_begin().await?;
                if field_ident.field_type == ::pilota::thrift::TType::Stop {

                    break;
                } else {

                }
                __pilota_decoding_field_id = field_ident.id;
                match field_ident.id {
                    Some(1) if field_ident.field_type == ::pilota::thrift::TType::Struct  => {
                    a = Some(<super::must_gen_items::A as ::pilota::thrift::Message>::decode_async(protocol).await?);

                },
                    _ => {
                        protocol.skip(field_ident.field_type).await?;

                    },
                }

                protocol.read_field_end().await?;


            };
                    Ok::<_, ::pilota::thrift::ThriftException>(())
                }.await {
                if let Some(field_id) = __pilota_decoding_field_id {
                    err.prepend_msg(&format!("decode struct `C` field(#{}) failed, caused by: ", field_id));
                }
                return Err(err);
            };
                    protocol.read_struct_end().await?;

                    let Some(a) = a else {
                        return Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "field a is required".to_string(),
                        ));
                    };

                    let data = Self {
                        a,
                        _unknown_fields: ::pilota::LinkedBytes::new(),
                    };
                    Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "C" })
                    + protocol.struct_field_len(Some(1), &self.a)
                    + self._unknown_fields.size()
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(
            PartialOrd,
            Hash,
            Eq,
            Ord,
            Debug,
            Default,
            ::pilota::serde::Serialize,
            ::pilota::serde::Deserialize,
            Clone,
            PartialEq,
        )]
        pub struct Message {
            pub _unknown_fields: ::pilota::LinkedBytes,
        }
        impl ::pilota::thrift::Message for Message {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "Message" };

                protocol.write_struct_begin(&struct_ident)?;
                for bytes in self._unknown_fields.list.iter() {
                    protocol.write_bytes_without_len(bytes.clone());
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut _unknown_fields = ::pilota::LinkedBytes::new();

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(mut err) = (|| {
                    loop {
                        let mut __pilota_offset = 0;
                        let __pilota_begin_ptr = protocol.buf().chunk().as_ptr();
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            __pilota_offset += protocol.field_stop_len();
                            break;
                        } else {
                            __pilota_offset +=
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            _ => {
                                __pilota_offset += protocol.skip(field_ident.field_type)?;
                                _unknown_fields.push_back(
                                    protocol
                                        .get_bytes(Some(__pilota_begin_ptr), __pilota_offset)?,
                                );
                            }
                        }

                        protocol.read_field_end()?;
                        __pilota_offset += protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::ThriftException>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!(
                            "decode struct `Message` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return Err(err);
                };
                protocol.read_struct_end()?;

                let data = Self { _unknown_fields };
                Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(mut err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!(
                                "decode struct `Message` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return Err(err);
                    };
                    protocol.read_struct_end().await?;

                    let data = Self {
                        _unknown_fields: ::pilota::LinkedBytes::new(),
                    };
                    Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "Message" })
                    + self._unknown_fields.size()
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
        #[derivative(Default)]
        #[derive(::pilota::serde::Serialize, ::pilota::serde::Deserialize, Clone, PartialEq)]
        pub enum TestUnion {
            #[derivative(Default)]
            A(A),

            B(B),
            _UnknownFields(::pilota::LinkedBytes),
        }

        impl ::pilota::thrift::Message for TestUnion {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TestUnion",
                })?;
                match self {
                    TestUnion::A(ref value) => {
                        protocol.write_struct_field(1, value, ::pilota::thrift::TType::Struct)?;
                    }
                    TestUnion::B(ref value) => {
                        protocol.write_struct_field(2, value, ::pilota::thrift::TType::Struct)?;
                    }
                    TestUnion::_UnknownFields(ref value) => {
                        for bytes in value.list.iter() {
                            protocol.write_bytes_without_len(bytes.clone());
                        }
                    }
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};
                let mut ret = None;
                protocol.read_struct_begin()?;
                loop {
                    let mut __pilota_offset = 0;
                    let __pilota_begin_ptr = protocol.buf().chunk().as_ptr();
                    let field_ident = protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        __pilota_offset += protocol.field_stop_len();
                        break;
                    } else {
                        __pilota_offset +=
                            protocol.field_begin_len(field_ident.field_type, field_ident.id);
                    }
                    match field_ident.id {
                        Some(1) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(protocol)?;
                                __pilota_offset += protocol.struct_len(&field_ident);
                                ret = Some(TestUnion::A(field_ident));
                            } else {
                                return Err(::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ));
                            }
                        }
                        Some(2) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(protocol)?;
                                __pilota_offset += protocol.struct_len(&field_ident);
                                ret = Some(TestUnion::B(field_ident));
                            } else {
                                return Err(::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ));
                            }
                        }
                        _ => {
                            __pilota_offset += protocol.skip(field_ident.field_type)?;
                            if ret.is_none() {
                                unsafe {
                                    let mut __pilota_linked_bytes = ::pilota::LinkedBytes::new();
                                    __pilota_linked_bytes.push_back(
                                        protocol
                                            .get_bytes(Some(__pilota_begin_ptr), __pilota_offset)?,
                                    );
                                    ret = Some(TestUnion::_UnknownFields(__pilota_linked_bytes));
                                }
                            } else {
                                return Err(::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ));
                            }
                        }
                    }
                }
                protocol.read_field_end()?;
                protocol.read_struct_end()?;
                if let Some(ret) = ret {
                    Ok(ret)
                } else {
                    Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "received empty union from remote Message",
                    ))
                }
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut ret = None;
                    protocol.read_struct_begin().await?;
                    loop {
                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        match field_ident.id {
                            Some(1) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <A as ::pilota::thrift::Message>::decode_async(protocol)
                                            .await?;

                                    ret = Some(TestUnion::A(field_ident));
                                } else {
                                    return Err(::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ));
                                }
                            }
                            Some(2) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <B as ::pilota::thrift::Message>::decode_async(protocol)
                                            .await?;

                                    ret = Some(TestUnion::B(field_ident));
                                } else {
                                    return Err(::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ));
                                }
                            }
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }
                    }
                    protocol.read_field_end().await?;
                    protocol.read_struct_end().await?;
                    if let Some(ret) = ret {
                        Ok(ret)
                    } else {
                        Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol
                    .struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "TestUnion" })
                    + match self {
                        TestUnion::A(ref value) => protocol.struct_field_len(Some(1), value),
                        TestUnion::B(ref value) => protocol.struct_field_len(Some(2), value),
                        TestUnion::_UnknownFields(ref value) => value.size(),
                    }
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
        #[derivative(Default)]
        #[derive(::pilota::serde::Serialize, ::pilota::serde::Deserialize, Clone, PartialEq)]
        pub enum TestTestExceptionException {
            #[derivative(Default)]
            StException(StException),
        }

        impl ::pilota::thrift::Message for TestTestExceptionException {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TestTestExceptionException",
                })?;
                match self {
                    TestTestExceptionException::StException(ref value) => {
                        protocol.write_struct_field(1, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};
                let mut ret = None;
                protocol.read_struct_begin()?;
                loop {
                    let field_ident = protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        protocol.field_stop_len();
                        break;
                    } else {
                        protocol.field_begin_len(field_ident.field_type, field_ident.id);
                    }
                    match field_ident.id {
                        Some(1) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(protocol)?;
                                protocol.struct_len(&field_ident);
                                ret = Some(TestTestExceptionException::StException(field_ident));
                            } else {
                                return Err(::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ));
                            }
                        }
                        _ => {
                            protocol.skip(field_ident.field_type)?;
                        }
                    }
                }
                protocol.read_field_end()?;
                protocol.read_struct_end()?;
                if let Some(ret) = ret {
                    Ok(ret)
                } else {
                    Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "received empty union from remote Message",
                    ))
                }
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut ret = None;
                    protocol.read_struct_begin().await?;
                    loop {
                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        match field_ident.id {
                            Some(1) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <StException as ::pilota::thrift::Message>::decode_async(
                                            protocol,
                                        )
                                        .await?;

                                    ret =
                                        Some(TestTestExceptionException::StException(field_ident));
                                } else {
                                    return Err(::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ));
                                }
                            }
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }
                    }
                    protocol.read_field_end().await?;
                    protocol.read_struct_end().await?;
                    if let Some(ret) = ret {
                        Ok(ret)
                    } else {
                        Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestTestExceptionException",
                }) + match self {
                    TestTestExceptionException::StException(ref value) => {
                        protocol.struct_field_len(Some(1), value)
                    }
                } + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
        #[derivative(Default)]
        #[derive(::pilota::serde::Serialize, ::pilota::serde::Deserialize, Clone, PartialEq)]
        pub enum TestTestExceptionResultRecv {
            #[derivative(Default)]
            Ok(ObjReq),

            StException(StException),
        }

        impl ::pilota::thrift::Message for TestTestExceptionResultRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TestTestExceptionResultRecv",
                })?;
                match self {
                    TestTestExceptionResultRecv::Ok(ref value) => {
                        protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                    TestTestExceptionResultRecv::StException(ref value) => {
                        protocol.write_struct_field(1, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};
                let mut ret = None;
                protocol.read_struct_begin()?;
                loop {
                    let field_ident = protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        protocol.field_stop_len();
                        break;
                    } else {
                        protocol.field_begin_len(field_ident.field_type, field_ident.id);
                    }
                    match field_ident.id {
                        Some(0) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(protocol)?;
                                protocol.struct_len(&field_ident);
                                ret = Some(TestTestExceptionResultRecv::Ok(field_ident));
                            } else {
                                return Err(::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ));
                            }
                        }
                        Some(1) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(protocol)?;
                                protocol.struct_len(&field_ident);
                                ret = Some(TestTestExceptionResultRecv::StException(field_ident));
                            } else {
                                return Err(::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ));
                            }
                        }
                        _ => {
                            protocol.skip(field_ident.field_type)?;
                        }
                    }
                }
                protocol.read_field_end()?;
                protocol.read_struct_end()?;
                if let Some(ret) = ret {
                    Ok(ret)
                } else {
                    Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "received empty union from remote Message",
                    ))
                }
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut ret = None;
                    protocol.read_struct_begin().await?;
                    loop {
                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        match field_ident.id {
                            Some(0) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <ObjReq as ::pilota::thrift::Message>::decode_async(
                                            protocol,
                                        )
                                        .await?;

                                    ret = Some(TestTestExceptionResultRecv::Ok(field_ident));
                                } else {
                                    return Err(::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ));
                                }
                            }
                            Some(1) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <StException as ::pilota::thrift::Message>::decode_async(
                                            protocol,
                                        )
                                        .await?;

                                    ret =
                                        Some(TestTestExceptionResultRecv::StException(field_ident));
                                } else {
                                    return Err(::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ));
                                }
                            }
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }
                    }
                    protocol.read_field_end().await?;
                    protocol.read_struct_end().await?;
                    if let Some(ret) = ret {
                        Ok(ret)
                    } else {
                        Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestTestExceptionResultRecv",
                }) + match self {
                    TestTestExceptionResultRecv::Ok(ref value) => {
                        protocol.struct_field_len(Some(0), value)
                    }
                    TestTestExceptionResultRecv::StException(ref value) => {
                        protocol.struct_field_len(Some(1), value)
                    }
                } + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
        #[derivative(Default)]
        #[derive(::pilota::serde::Serialize, ::pilota::serde::Deserialize, Clone, PartialEq)]
        pub enum TestTest123ResultRecv {
            #[derivative(Default)]
            Ok(()),
        }

        impl ::pilota::thrift::Message for TestTest123ResultRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TestTest123ResultRecv",
                })?;
                match self {
                    TestTest123ResultRecv::Ok(ref value) => {}
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};
                let mut ret = None;
                protocol.read_struct_begin()?;
                loop {
                    let field_ident = protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        protocol.field_stop_len();
                        break;
                    } else {
                        protocol.field_begin_len(field_ident.field_type, field_ident.id);
                    }
                    match field_ident.id {
                        _ => {
                            protocol.skip(field_ident.field_type)?;
                        }
                    }
                }
                protocol.read_field_end()?;
                protocol.read_struct_end()?;
                if let Some(ret) = ret {
                    Ok(ret)
                } else {
                    Ok(TestTest123ResultRecv::Ok(()))
                }
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut ret = None;
                    protocol.read_struct_begin().await?;
                    loop {
                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        match field_ident.id {
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }
                    }
                    protocol.read_field_end().await?;
                    protocol.read_struct_end().await?;
                    if let Some(ret) = ret {
                        Ok(ret)
                    } else {
                        Ok(TestTest123ResultRecv::Ok(()))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestTest123ResultRecv",
                }) + match self {
                    TestTest123ResultRecv::Ok(ref value) => 0,
                } + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
        #[derivative(Default)]
        #[derive(::pilota::serde::Serialize, ::pilota::serde::Deserialize)]
        #[serde(transparent)]
        #[derive(Clone, PartialEq, Copy)]
        #[repr(transparent)]
        pub struct Index(i32);

        impl Index {
            pub const A: Self = Self(0);
            pub const B: Self = Self(1);

            pub fn inner(&self) -> i32 {
                self.0
            }

            pub fn as_str(&self) -> &'static str {
                match self {
                    Self(0) => "A",
                    Self(1) => "B",
                    Self(val) => Box::leak(val.to_string().into_boxed_str()),
                }
            }
        }

        impl ::std::convert::From<i32> for Index {
            fn from(value: i32) -> Self {
                Self(value)
            }
        }

        impl ::std::convert::From<Index> for i32 {
            fn from(value: Index) -> i32 {
                value.0
            }
        }

        impl ::pilota::thrift::Message for Index {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                protocol.write_i32(self.inner())?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};
                let value = protocol.read_i32()?;
                Ok(::std::convert::TryFrom::try_from(value).map_err(|err| {
                    ::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        format!("invalid enum value for Index, value: {}", value),
                    )
                })?)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let value = protocol.read_i32().await?;
                    Ok(::std::convert::TryFrom::try_from(value).map_err(|err| {
                        ::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            format!("invalid enum value for Index, value: {}", value),
                        )
                    })?)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.i32_len(self.inner())
            }
        }
        #[derive(
            PartialOrd,
            Hash,
            Eq,
            Ord,
            Debug,
            Default,
            ::pilota::serde::Serialize,
            ::pilota::serde::Deserialize,
            Clone,
            PartialEq,
        )]
        pub struct ObjReq {
            pub _unknown_fields: ::pilota::LinkedBytes,
        }
        impl ::pilota::thrift::Message for ObjReq {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "ObjReq" };

                protocol.write_struct_begin(&struct_ident)?;
                for bytes in self._unknown_fields.list.iter() {
                    protocol.write_bytes_without_len(bytes.clone());
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut __pilota_fields_num = 0;
                let mut _unknown_fields = ::pilota::LinkedBytes::new();

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(mut err) = (|| {
                    loop {
                        if __pilota_fields_num == 0 {
                            let __pilota_remaining = protocol.buf().remaining();
                            _unknown_fields
                                .push_back(protocol.get_bytes(None, __pilota_remaining - 2)?);
                            break;
                        }
                        let mut __pilota_offset = 0;
                        let __pilota_begin_ptr = protocol.buf().chunk().as_ptr();
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            __pilota_offset += protocol.field_stop_len();
                            break;
                        } else {
                            __pilota_offset +=
                                protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            _ => {
                                __pilota_offset += protocol.skip(field_ident.field_type)?;
                                _unknown_fields.push_back(
                                    protocol
                                        .get_bytes(Some(__pilota_begin_ptr), __pilota_offset)?,
                                );
                            }
                        }

                        protocol.read_field_end()?;
                        __pilota_offset += protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::ThriftException>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!(
                            "decode struct `ObjReq` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return Err(err);
                };
                protocol.read_struct_end()?;

                let data = Self { _unknown_fields };
                Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(mut err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!(
                                "decode struct `ObjReq` field(#{}) failed, caused by: ",
                                field_id
                            ));
                        }
                        return Err(err);
                    };
                    protocol.read_struct_end().await?;

                    let data = Self {
                        _unknown_fields: ::pilota::LinkedBytes::new(),
                    };
                    Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "ObjReq" })
                    + self._unknown_fields.size()
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(
            PartialOrd,
            Hash,
            Eq,
            Ord,
            Debug,
            Default,
            ::pilota::serde::Serialize,
            ::pilota::serde::Deserialize,
            Clone,
            PartialEq,
        )]
        pub struct TestTestExceptionArgsSend {
            pub req: ObjReq,
        }
        impl ::pilota::thrift::Message for TestTestExceptionArgsSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TestTestExceptionArgsSend",
                };

                protocol.write_struct_begin(&struct_ident)?;
                protocol.write_struct_field(1, &self.req, ::pilota::thrift::TType::Struct)?;
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut req = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(mut err) = (|| {
                    loop {
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            protocol.field_stop_len();
                            break;
                        } else {
                            protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                req = Some(::pilota::thrift::Message::decode(protocol)?);
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }

                        protocol.read_field_end()?;
                        protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::ThriftException>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!("decode struct `TestTestExceptionArgsSend` field(#{}) failed, caused by: ", field_id));
                    }
                    return Err(err);
                };
                protocol.read_struct_end()?;

                let Some(req) = req else {
                    return Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field req is required".to_string(),
                    ));
                };

                let data = Self { req };
                Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut req = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(mut err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    req = Some(
                                        <ObjReq as ::pilota::thrift::Message>::decode_async(
                                            protocol,
                                        )
                                        .await?,
                                    );
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!("decode struct `TestTestExceptionArgsSend` field(#{}) failed, caused by: ", field_id));
                        }
                        return Err(err);
                    };
                    protocol.read_struct_end().await?;

                    let Some(req) = req else {
                        return Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "field req is required".to_string(),
                        ));
                    };

                    let data = Self { req };
                    Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestTestExceptionArgsSend",
                }) + protocol.struct_field_len(Some(1), &self.req)
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
        #[derivative(Default)]
        #[derive(::pilota::serde::Serialize, ::pilota::serde::Deserialize, Clone, PartialEq)]
        pub enum TestTest123ResultSend {
            #[derivative(Default)]
            Ok(()),
        }

        impl ::pilota::thrift::Message for TestTest123ResultSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TestTest123ResultSend",
                })?;
                match self {
                    TestTest123ResultSend::Ok(ref value) => {}
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};
                let mut ret = None;
                protocol.read_struct_begin()?;
                loop {
                    let field_ident = protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        protocol.field_stop_len();
                        break;
                    } else {
                        protocol.field_begin_len(field_ident.field_type, field_ident.id);
                    }
                    match field_ident.id {
                        _ => {
                            protocol.skip(field_ident.field_type)?;
                        }
                    }
                }
                protocol.read_field_end()?;
                protocol.read_struct_end()?;
                if let Some(ret) = ret {
                    Ok(ret)
                } else {
                    Ok(TestTest123ResultSend::Ok(()))
                }
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut ret = None;
                    protocol.read_struct_begin().await?;
                    loop {
                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        match field_ident.id {
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }
                    }
                    protocol.read_field_end().await?;
                    protocol.read_struct_end().await?;
                    if let Some(ret) = ret {
                        Ok(ret)
                    } else {
                        Ok(TestTest123ResultSend::Ok(()))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestTest123ResultSend",
                }) + match self {
                    TestTest123ResultSend::Ok(ref value) => 0,
                } + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(
            PartialOrd,
            Hash,
            Eq,
            Ord,
            Debug,
            Default,
            ::pilota::serde::Serialize,
            ::pilota::serde::Deserialize,
            Clone,
            PartialEq,
        )]
        pub struct Td(pub ::std::vec::Vec<::std::vec::Vec<::pilota::FastStr>>);

        impl ::std::ops::Deref for Td {
            type Target = ::std::vec::Vec<::std::vec::Vec<::pilota::FastStr>>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<::std::vec::Vec<::std::vec::Vec<::pilota::FastStr>>> for Td {
            fn from(v: ::std::vec::Vec<::std::vec::Vec<::pilota::FastStr>>) -> Self {
                Self(v)
            }
        }

        impl ::pilota::thrift::Message for Td {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                protocol.write_list(
                    ::pilota::thrift::TType::List,
                    &(&**self),
                    |protocol, val| {
                        protocol.write_list(
                            ::pilota::thrift::TType::Binary,
                            &val,
                            |protocol, val| {
                                protocol.write_faststr((val).clone())?;
                                Ok(())
                            },
                        )?;
                        Ok(())
                    },
                )?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};
                Ok(Td(unsafe {
                    let list_ident = protocol.read_list_begin()?;
                    let mut val: Vec<::std::vec::Vec<::pilota::FastStr>> =
                        Vec::with_capacity(list_ident.size);
                    for i in 0..list_ident.size {
                        val.as_mut_ptr().offset(i as isize).write(unsafe {
                            let list_ident = protocol.read_list_begin()?;
                            let mut val: Vec<::pilota::FastStr> =
                                Vec::with_capacity(list_ident.size);
                            for i in 0..list_ident.size {
                                val.as_mut_ptr()
                                    .offset(i as isize)
                                    .write(protocol.read_faststr()?);
                            }
                            val.set_len(list_ident.size);
                            protocol.read_list_end()?;
                            val
                        });
                    }
                    val.set_len(list_ident.size);
                    protocol.read_list_end()?;
                    val
                }))
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    Ok(Td({
                        let list_ident = protocol.read_list_begin().await?;
                        let mut val = Vec::with_capacity(list_ident.size);
                        for _ in 0..list_ident.size {
                            val.push({
                                let list_ident = protocol.read_list_begin().await?;
                                let mut val = Vec::with_capacity(list_ident.size);
                                for _ in 0..list_ident.size {
                                    val.push(protocol.read_faststr().await?);
                                }
                                protocol.read_list_end().await?;
                                val
                            });
                        }
                        protocol.read_list_end().await?;
                        val
                    }))
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.list_len(::pilota::thrift::TType::List, &**self, |protocol, el| {
                    protocol.list_len(::pilota::thrift::TType::Binary, el, |protocol, el| {
                        protocol.faststr_len(el)
                    })
                })
            }
        }
        #[derive(
            PartialOrd,
            Hash,
            Eq,
            Ord,
            Debug,
            Default,
            ::pilota::serde::Serialize,
            ::pilota::serde::Deserialize,
            Clone,
            PartialEq,
        )]
        pub struct TestTestExceptionArgsRecv {
            pub req: ObjReq,
        }
        impl ::pilota::thrift::Message for TestTestExceptionArgsRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TestTestExceptionArgsRecv",
                };

                protocol.write_struct_begin(&struct_ident)?;
                protocol.write_struct_field(1, &self.req, ::pilota::thrift::TType::Struct)?;
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut req = None;

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(mut err) = (|| {
                    loop {
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            protocol.field_stop_len();
                            break;
                        } else {
                            protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            Some(1)
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                req = Some(::pilota::thrift::Message::decode(protocol)?);
                            }
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }

                        protocol.read_field_end()?;
                        protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::ThriftException>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!("decode struct `TestTestExceptionArgsRecv` field(#{}) failed, caused by: ", field_id));
                    }
                    return Err(err);
                };
                protocol.read_struct_end()?;

                let Some(req) = req else {
                    return Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "field req is required".to_string(),
                    ));
                };

                let data = Self { req };
                Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut req = None;

                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(mut err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                Some(1)
                                    if field_ident.field_type
                                        == ::pilota::thrift::TType::Struct =>
                                {
                                    req = Some(
                                        <ObjReq as ::pilota::thrift::Message>::decode_async(
                                            protocol,
                                        )
                                        .await?,
                                    );
                                }
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!("decode struct `TestTestExceptionArgsRecv` field(#{}) failed, caused by: ", field_id));
                        }
                        return Err(err);
                    };
                    protocol.read_struct_end().await?;

                    let Some(req) = req else {
                        return Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "field req is required".to_string(),
                        ));
                    };

                    let data = Self { req };
                    Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestTestExceptionArgsRecv",
                }) + protocol.struct_field_len(Some(1), &self.req)
                    + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
        #[derivative(Default)]
        #[derive(::pilota::serde::Serialize, ::pilota::serde::Deserialize, Clone, PartialEq)]
        pub enum TestTestExceptionResultSend {
            #[derivative(Default)]
            Ok(ObjReq),

            StException(StException),
        }

        impl ::pilota::thrift::Message for TestTestExceptionResultSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TestTestExceptionResultSend",
                })?;
                match self {
                    TestTestExceptionResultSend::Ok(ref value) => {
                        protocol.write_struct_field(0, value, ::pilota::thrift::TType::Struct)?;
                    }
                    TestTestExceptionResultSend::StException(ref value) => {
                        protocol.write_struct_field(1, value, ::pilota::thrift::TType::Struct)?;
                    }
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};
                let mut ret = None;
                protocol.read_struct_begin()?;
                loop {
                    let field_ident = protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        protocol.field_stop_len();
                        break;
                    } else {
                        protocol.field_begin_len(field_ident.field_type, field_ident.id);
                    }
                    match field_ident.id {
                        Some(0) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(protocol)?;
                                protocol.struct_len(&field_ident);
                                ret = Some(TestTestExceptionResultSend::Ok(field_ident));
                            } else {
                                return Err(::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ));
                            }
                        }
                        Some(1) => {
                            if ret.is_none() {
                                let field_ident = ::pilota::thrift::Message::decode(protocol)?;
                                protocol.struct_len(&field_ident);
                                ret = Some(TestTestExceptionResultSend::StException(field_ident));
                            } else {
                                return Err(::pilota::thrift::new_protocol_exception(
                                    ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                    "received multiple fields for union from remote Message",
                                ));
                            }
                        }
                        _ => {
                            protocol.skip(field_ident.field_type)?;
                        }
                    }
                }
                protocol.read_field_end()?;
                protocol.read_struct_end()?;
                if let Some(ret) = ret {
                    Ok(ret)
                } else {
                    Err(::pilota::thrift::new_protocol_exception(
                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                        "received empty union from remote Message",
                    ))
                }
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut ret = None;
                    protocol.read_struct_begin().await?;
                    loop {
                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        match field_ident.id {
                            Some(0) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <ObjReq as ::pilota::thrift::Message>::decode_async(
                                            protocol,
                                        )
                                        .await?;

                                    ret = Some(TestTestExceptionResultSend::Ok(field_ident));
                                } else {
                                    return Err(::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ));
                                }
                            }
                            Some(1) => {
                                if ret.is_none() {
                                    let field_ident =
                                        <StException as ::pilota::thrift::Message>::decode_async(
                                            protocol,
                                        )
                                        .await?;

                                    ret =
                                        Some(TestTestExceptionResultSend::StException(field_ident));
                                } else {
                                    return Err(::pilota::thrift::new_protocol_exception(
                                        ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                                        "received multiple fields for union from remote Message",
                                    ));
                                }
                            }
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }
                    }
                    protocol.read_field_end().await?;
                    protocol.read_struct_end().await?;
                    if let Some(ret) = ret {
                        Ok(ret)
                    } else {
                        Err(::pilota::thrift::new_protocol_exception(
                            ::pilota::thrift::ProtocolExceptionKind::InvalidData,
                            "received empty union from remote Message",
                        ))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestTestExceptionResultSend",
                }) + match self {
                    TestTestExceptionResultSend::Ok(ref value) => {
                        protocol.struct_field_len(Some(0), value)
                    }
                    TestTestExceptionResultSend::StException(ref value) => {
                        protocol.struct_field_len(Some(1), value)
                    }
                } + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        ::pilota::lazy_static::lazy_static! {
            pub static ref TEST_MAP: ::pilota::AHashMap<Index, &'static str> = {
            let mut map = ::pilota::AHashMap::with_capacity(2);
            map.insert(Index::A, "hello");map.insert(Index::B, "world");
            map
        };
        }
    }

    pub mod void {

        pub trait Test {}
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
        #[derivative(Default)]
        #[derive(::pilota::serde::Serialize, ::pilota::serde::Deserialize, Clone, PartialEq)]
        pub enum TestTest123ResultRecv {
            #[derivative(Default)]
            Ok(()),
        }

        impl ::pilota::thrift::Message for TestTest123ResultRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TestTest123ResultRecv",
                })?;
                match self {
                    TestTest123ResultRecv::Ok(ref value) => {}
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};
                let mut ret = None;
                protocol.read_struct_begin()?;
                loop {
                    let field_ident = protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        protocol.field_stop_len();
                        break;
                    } else {
                        protocol.field_begin_len(field_ident.field_type, field_ident.id);
                    }
                    match field_ident.id {
                        _ => {
                            protocol.skip(field_ident.field_type)?;
                        }
                    }
                }
                protocol.read_field_end()?;
                protocol.read_struct_end()?;
                if let Some(ret) = ret {
                    Ok(ret)
                } else {
                    Ok(TestTest123ResultRecv::Ok(()))
                }
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut ret = None;
                    protocol.read_struct_begin().await?;
                    loop {
                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        match field_ident.id {
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }
                    }
                    protocol.read_field_end().await?;
                    protocol.read_struct_end().await?;
                    if let Some(ret) = ret {
                        Ok(ret)
                    } else {
                        Ok(TestTest123ResultRecv::Ok(()))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestTest123ResultRecv",
                }) + match self {
                    TestTest123ResultRecv::Ok(ref value) => 0,
                } + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, ::pilota::derivative::Derivative)]
        #[derivative(Default)]
        #[derive(::pilota::serde::Serialize, ::pilota::serde::Deserialize, Clone, PartialEq)]
        pub enum TestTest123ResultSend {
            #[derivative(Default)]
            Ok(()),
        }

        impl ::pilota::thrift::Message for TestTest123ResultSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TestTest123ResultSend",
                })?;
                match self {
                    TestTest123ResultSend::Ok(ref value) => {}
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};
                let mut ret = None;
                protocol.read_struct_begin()?;
                loop {
                    let field_ident = protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        protocol.field_stop_len();
                        break;
                    } else {
                        protocol.field_begin_len(field_ident.field_type, field_ident.id);
                    }
                    match field_ident.id {
                        _ => {
                            protocol.skip(field_ident.field_type)?;
                        }
                    }
                }
                protocol.read_field_end()?;
                protocol.read_struct_end()?;
                if let Some(ret) = ret {
                    Ok(ret)
                } else {
                    Ok(TestTest123ResultSend::Ok(()))
                }
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut ret = None;
                    protocol.read_struct_begin().await?;
                    loop {
                        let field_ident = protocol.read_field_begin().await?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            break;
                        } else {
                        }
                        match field_ident.id {
                            _ => {
                                protocol.skip(field_ident.field_type).await?;
                            }
                        }
                    }
                    protocol.read_field_end().await?;
                    protocol.read_struct_end().await?;
                    if let Some(ret) = ret {
                        Ok(ret)
                    } else {
                        Ok(TestTest123ResultSend::Ok(()))
                    }
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestTest123ResultSend",
                }) + match self {
                    TestTest123ResultSend::Ok(ref value) => 0,
                } + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(
            PartialOrd,
            Hash,
            Eq,
            Ord,
            Debug,
            Default,
            ::pilota::serde::Serialize,
            ::pilota::serde::Deserialize,
            Clone,
            PartialEq,
        )]
        pub struct TestTest123ArgsSend {}
        impl ::pilota::thrift::Message for TestTest123ArgsSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TestTest123ArgsSend",
                };

                protocol.write_struct_begin(&struct_ident)?;

                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(mut err) = (|| {
                    loop {
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            protocol.field_stop_len();
                            break;
                        } else {
                            protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }

                        protocol.read_field_end()?;
                        protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::ThriftException>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!(
                            "decode struct `TestTest123ArgsSend` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return Err(err);
                };
                protocol.read_struct_end()?;

                let data = Self {};
                Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(mut err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!("decode struct `TestTest123ArgsSend` field(#{}) failed, caused by: ", field_id));
                        }
                        return Err(err);
                    };
                    protocol.read_struct_end().await?;

                    let data = Self {};
                    Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestTest123ArgsSend",
                }) + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
        #[derive(
            PartialOrd,
            Hash,
            Eq,
            Ord,
            Debug,
            Default,
            ::pilota::serde::Serialize,
            ::pilota::serde::Deserialize,
            Clone,
            PartialEq,
        )]
        pub struct TestTest123ArgsRecv {}
        impl ::pilota::thrift::Message for TestTest123ArgsRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TestTest123ArgsRecv",
                };

                protocol.write_struct_begin(&struct_ident)?;

                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }

            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::ThriftException> {
                #[allow(unused_imports)]
                use ::pilota::{thrift::TLengthProtocolExt, Buf};

                let mut __pilota_decoding_field_id = None;

                protocol.read_struct_begin()?;
                if let Err(mut err) = (|| {
                    loop {
                        let field_ident = protocol.read_field_begin()?;
                        if field_ident.field_type == ::pilota::thrift::TType::Stop {
                            protocol.field_stop_len();
                            break;
                        } else {
                            protocol.field_begin_len(field_ident.field_type, field_ident.id);
                        }
                        __pilota_decoding_field_id = field_ident.id;
                        match field_ident.id {
                            _ => {
                                protocol.skip(field_ident.field_type)?;
                            }
                        }

                        protocol.read_field_end()?;
                        protocol.field_end_len();
                    }
                    Ok::<_, ::pilota::thrift::ThriftException>(())
                })() {
                    if let Some(field_id) = __pilota_decoding_field_id {
                        err.prepend_msg(&format!(
                            "decode struct `TestTest123ArgsRecv` field(#{}) failed, caused by: ",
                            field_id
                        ));
                    }
                    return Err(err);
                };
                protocol.read_struct_end()?;

                let data = Self {};
                Ok(data)
            }

            fn decode_async<'a, T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &'a mut T,
            ) -> ::std::pin::Pin<
                ::std::boxed::Box<
                    dyn ::std::future::Future<
                            Output = ::std::result::Result<Self, ::pilota::thrift::ThriftException>,
                        > + Send
                        + 'a,
                >,
            > {
                ::std::boxed::Box::pin(async move {
                    let mut __pilota_decoding_field_id = None;

                    protocol.read_struct_begin().await?;
                    if let Err(mut err) = async {
                        loop {
                            let field_ident = protocol.read_field_begin().await?;
                            if field_ident.field_type == ::pilota::thrift::TType::Stop {
                                break;
                            } else {
                            }
                            __pilota_decoding_field_id = field_ident.id;
                            match field_ident.id {
                                _ => {
                                    protocol.skip(field_ident.field_type).await?;
                                }
                            }

                            protocol.read_field_end().await?;
                        }
                        Ok::<_, ::pilota::thrift::ThriftException>(())
                    }
                    .await
                    {
                        if let Some(field_id) = __pilota_decoding_field_id {
                            err.prepend_msg(&format!("decode struct `TestTest123ArgsRecv` field(#{}) failed, caused by: ", field_id));
                        }
                        return Err(err);
                    };
                    protocol.read_struct_end().await?;

                    let data = Self {};
                    Ok(data)
                })
            }

            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                #[allow(unused_imports)]
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestTest123ArgsRecv",
                }) + protocol.field_stop_len()
                    + protocol.struct_end_len()
            }
        }
    }
}
