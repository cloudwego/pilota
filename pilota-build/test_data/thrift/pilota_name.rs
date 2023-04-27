pub mod pilota_name {
    #![allow(warnings, clippy::all)]
    pub mod pilota_name {
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct Test2 {
            pub id: ::pilota::FastStr,
        }
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for Test2 {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "TEST" };
                protocol.write_struct_begin(&struct_ident)?;
                protocol.write_faststr_field(1i16, (&self.id).clone())?;
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }
            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut id = None;
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
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                id = Some(protocol.read_faststr()?);
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
                            format!("decode struct `TEST` field(#{}) failed", field_id),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end()?;
                let Some (id) = id else { return Err (:: pilota :: thrift :: DecodeError :: new (:: pilota :: thrift :: DecodeErrorKind :: InvalidData , "field id is required" . to_string ())) } ;
                let data = Self { id };
                Ok(data)
            }
            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut id = None;
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
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                id = Some(protocol.read_faststr().await?);
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
                            format!("decode struct `TEST` field(#{}) failed", field_id),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end().await?;
                let Some (id) = id else { return Err (:: pilota :: thrift :: DecodeError :: new (:: pilota :: thrift :: DecodeErrorKind :: InvalidData , "field id is required" . to_string ())) } ;
                let data = Self { id };
                Ok(data)
            }
            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                use ::pilota::thrift::TLengthProtocolExt;
                protocol
                    .write_struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "TEST" })
                    + protocol.write_faststr_field_len(Some(1i16), &self.id)
                    + protocol.write_field_stop_len()
                    + protocol.write_struct_end_len()
            }
        }
        pub const LANG_ID: &'static str = "id";
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct Test1 {
            pub id: ::pilota::FastStr,
            pub hello: ::pilota::FastStr,
        }
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for Test1 {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier { name: "Test" };
                protocol.write_struct_begin(&struct_ident)?;
                protocol.write_faststr_field(1i16, (&self.id).clone())?;
                protocol.write_faststr_field(2i16, (&self.hello).clone())?;
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }
            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut id = None;
                let mut hello = None;
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
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                id = Some(protocol.read_faststr()?);
                            }
                            Some(2i16)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                hello = Some(protocol.read_faststr()?);
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
                            format!("decode struct `Test` field(#{}) failed", field_id),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end()?;
                let Some (id) = id else { return Err (:: pilota :: thrift :: DecodeError :: new (:: pilota :: thrift :: DecodeErrorKind :: InvalidData , "field id is required" . to_string ())) } ;
                let Some (hello) = hello else { return Err (:: pilota :: thrift :: DecodeError :: new (:: pilota :: thrift :: DecodeErrorKind :: InvalidData , "field hello is required" . to_string ())) } ;
                let data = Self { id, hello };
                Ok(data)
            }
            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut id = None;
                let mut hello = None;
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
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                id = Some(protocol.read_faststr().await?);
                            }
                            Some(2i16)
                                if field_ident.field_type == ::pilota::thrift::TType::Binary =>
                            {
                                hello = Some(protocol.read_faststr().await?);
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
                            format!("decode struct `Test` field(#{}) failed", field_id),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end().await?;
                let Some (id) = id else { return Err (:: pilota :: thrift :: DecodeError :: new (:: pilota :: thrift :: DecodeErrorKind :: InvalidData , "field id is required" . to_string ())) } ;
                let Some (hello) = hello else { return Err (:: pilota :: thrift :: DecodeError :: new (:: pilota :: thrift :: DecodeErrorKind :: InvalidData , "field hello is required" . to_string ())) } ;
                let data = Self { id, hello };
                Ok(data)
            }
            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                use ::pilota::thrift::TLengthProtocolExt;
                protocol
                    .write_struct_begin_len(&::pilota::thrift::TStructIdentifier { name: "Test" })
                    + protocol.write_faststr_field_len(Some(1i16), &self.id)
                    + protocol.write_faststr_field_len(Some(2i16), &self.hello)
                    + protocol.write_field_stop_len()
                    + protocol.write_struct_end_len()
            }
        }
        impl ::std::convert::From<Index> for i32 {
            fn from(e: Index) -> Self {
                e as _
            }
        }
        impl ::std::convert::TryFrom<i32> for Index {
            type Error = ::pilota::EnumConvertError<i32>;
            #[allow(non_upper_case_globals)]
            fn try_from(v: i32) -> ::std::result::Result<Self, ::pilota::EnumConvertError<i32>> {
                const AA: i32 = Index::AA as i32;
                const B: i32 = Index::B as i32;
                match v {
                    AA => ::std::result::Result::Ok(Index::AA),
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
            AA = 0i32,
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
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, :: pilota :: derivative :: Derivative)]
        #[derivative(Default)]
        #[derive(Clone, PartialEq)]
        pub enum TestServiceTestResult {
            #[derivative(Default)]
            Ok(Test1),
        }
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for TestServiceTestResult {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                use ::pilota::thrift::TOutputProtocolExt;
                protocol.write_struct_begin(&::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTestResult",
                })?;
                match self {
                    TestServiceTestResult::Ok(ref value) => {
                        protocol.write_struct_field(0i16, value)?;
                    }
                }
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }
            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut ret = None;
                protocol.read_struct_begin()?;
                loop {
                    let field_ident = protocol.read_field_begin()?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    }
                    let field_id = field_ident.id;
                    match field_id {
                        Some(0i16) => {
                            if ret.is_none() {
                                ret = Some(TestServiceTestResult::Ok(
                                    ::pilota::thrift::Message::decode(protocol)?,
                                ));
                            } else {
                                return Err(::pilota::thrift::DecodeError::new(
                                    ::pilota::thrift::DecodeErrorKind::InvalidData,
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
                    Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        "received empty union from remote Message",
                    ))
                }
            }
            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut ret = None;
                protocol.read_struct_begin().await?;
                loop {
                    let field_ident = protocol.read_field_begin().await?;
                    if field_ident.field_type == ::pilota::thrift::TType::Stop {
                        break;
                    }
                    let field_id = field_ident.id;
                    match field_id {
                        Some(0i16) => {
                            if ret.is_none() {
                                ret = Some(TestServiceTestResult::Ok(
                                    ::pilota::thrift::Message::decode_async(protocol).await?,
                                ));
                            } else {
                                return Err(::pilota::thrift::DecodeError::new(
                                    ::pilota::thrift::DecodeErrorKind::InvalidData,
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
                    Err(::pilota::thrift::DecodeError::new(
                        ::pilota::thrift::DecodeErrorKind::InvalidData,
                        "received empty union from remote Message",
                    ))
                }
            }
            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.write_struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTestResult",
                }) + match self {
                    TestServiceTestResult::Ok(ref value) => {
                        protocol.write_struct_field_len(Some(0i16), value)
                    }
                } + protocol.write_field_stop_len()
                    + protocol.write_struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct TestServiceTestArgsSend {
            pub req: Test2,
        }
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for TestServiceTestArgsSend {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTestArgsSend",
                };
                protocol.write_struct_begin(&struct_ident)?;
                protocol.write_struct_field(1i16, &self.req)?;
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }
            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut req = None;
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
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                req = Some(::pilota::thrift::Message::decode(protocol)?);
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
                            format!(
                                "decode struct `TestServiceTestArgsSend` field(#{}) failed",
                                field_id
                            ),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end()?;
                let Some (req) = req else { return Err (:: pilota :: thrift :: DecodeError :: new (:: pilota :: thrift :: DecodeErrorKind :: InvalidData , "field req is required" . to_string ())) } ;
                let data = Self { req };
                Ok(data)
            }
            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut req = None;
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
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                req =
                                    Some(::pilota::thrift::Message::decode_async(protocol).await?);
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
                            format!(
                                "decode struct `TestServiceTestArgsSend` field(#{}) failed",
                                field_id
                            ),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end().await?;
                let Some (req) = req else { return Err (:: pilota :: thrift :: DecodeError :: new (:: pilota :: thrift :: DecodeErrorKind :: InvalidData , "field req is required" . to_string ())) } ;
                let data = Self { req };
                Ok(data)
            }
            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.write_struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTestArgsSend",
                }) + protocol.write_struct_field_len(Some(1i16), &self.req)
                    + protocol.write_field_stop_len()
                    + protocol.write_struct_end_len()
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct TestServiceTestArgsRecv {
            pub req: Test2,
        }
        #[::async_trait::async_trait]
        impl ::pilota::thrift::Message for TestServiceTestArgsRecv {
            fn encode<T: ::pilota::thrift::TOutputProtocol>(
                &self,
                protocol: &mut T,
            ) -> ::std::result::Result<(), ::pilota::thrift::EncodeError> {
                use ::pilota::thrift::TOutputProtocolExt;
                let struct_ident = ::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTestArgsRecv",
                };
                protocol.write_struct_begin(&struct_ident)?;
                protocol.write_struct_field(1i16, &self.req)?;
                protocol.write_field_stop()?;
                protocol.write_struct_end()?;
                Ok(())
            }
            fn decode<T: ::pilota::thrift::TInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut req = None;
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
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                req = Some(::pilota::thrift::Message::decode(protocol)?);
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
                            format!(
                                "decode struct `TestServiceTestArgsRecv` field(#{}) failed",
                                field_id
                            ),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end()?;
                let Some (req) = req else { return Err (:: pilota :: thrift :: DecodeError :: new (:: pilota :: thrift :: DecodeErrorKind :: InvalidData , "field req is required" . to_string ())) } ;
                let data = Self { req };
                Ok(data)
            }
            async fn decode_async<T: ::pilota::thrift::TAsyncInputProtocol>(
                protocol: &mut T,
            ) -> ::std::result::Result<Self, ::pilota::thrift::DecodeError> {
                let mut req = None;
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
                                if field_ident.field_type == ::pilota::thrift::TType::Struct =>
                            {
                                req =
                                    Some(::pilota::thrift::Message::decode_async(protocol).await?);
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
                            format!(
                                "decode struct `TestServiceTestArgsRecv` field(#{}) failed",
                                field_id
                            ),
                        ));
                    } else {
                        return Err(err);
                    }
                };
                protocol.read_struct_end().await?;
                let Some (req) = req else { return Err (:: pilota :: thrift :: DecodeError :: new (:: pilota :: thrift :: DecodeErrorKind :: InvalidData , "field req is required" . to_string ())) } ;
                let data = Self { req };
                Ok(data)
            }
            fn size<T: ::pilota::thrift::TLengthProtocol>(&self, protocol: &mut T) -> usize {
                use ::pilota::thrift::TLengthProtocolExt;
                protocol.write_struct_begin_len(&::pilota::thrift::TStructIdentifier {
                    name: "TestServiceTestArgsRecv",
                }) + protocol.write_struct_field_len(Some(1i16), &self.req)
                    + protocol.write_field_stop_len()
                    + protocol.write_struct_end_len()
            }
        }
        #[::async_trait::async_trait]
        pub trait TestService {}
    }
}
