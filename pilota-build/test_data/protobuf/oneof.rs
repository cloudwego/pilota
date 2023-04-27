pub mod oneof {
    #![allow(warnings, clippy::all)]
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct Test {
        pub r#type: ::std::option::Option<test::Type>,
    }
    impl ::pilota::prost::Message for Test {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + self.r#type.as_ref().map_or(0, |msg| msg.encoded_len())
        }
        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::pilota::prost::bytes::BufMut,
        {
            if let Some(_pilota_inner_value) = self.r#type.as_ref() {
                _pilota_inner_value.encode(buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field<B>(
            &mut self,
            tag: u32,
            wire_type: ::pilota::prost::encoding::WireType,
            buf: &mut B,
            ctx: ::pilota::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::pilota::prost::DecodeError>
        where
            B: ::pilota::prost::bytes::Buf,
        {
            const STRUCT_NAME: &'static str = stringify!(Test);
            match tag {
                2u32 | 4u32 => {
                    let mut _inner_pilota_value = &mut self.r#type;
                    test::Type::merge(&mut _inner_pilota_value, tag, wire_type, buf, ctx).map_err(
                        |mut error| {
                            error.push(STRUCT_NAME, stringify!(r#type));
                            error
                        },
                    )
                }
                _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    pub mod test {
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, :: pilota :: derivative :: Derivative)]
        #[derivative(Default)]
        #[derive(Clone, PartialEq)]
        pub enum Type {
            #[derivative(Default)]
            S(::pilota::FastStr),
            I(i32),
        }
        impl Type {
            pub fn encode<B>(&self, buf: &mut B)
            where
                B: ::pilota::prost::bytes::BufMut,
            {
                match self {
                    Type::S(value) => {
                        ::pilota::prost::encoding::string::encode(2u32, &*value, buf);
                    }
                    Type::I(value) => {
                        ::pilota::prost::encoding::int32::encode(4u32, &*value, buf);
                    }
                }
            }
            #[doc = r" Returns the encoded length of the message without a length delimiter."]
            #[inline]
            pub fn encoded_len(&self) -> usize {
                match self {
                    Type::S(value) => ::pilota::prost::encoding::string::encoded_len(2u32, &*value),
                    Type::I(value) => ::pilota::prost::encoding::int32::encoded_len(4u32, &*value),
                }
            }
            #[doc = r" Decodes an instance of the message from a buffer, and merges it into self."]
            pub fn merge<B>(
                field: &mut ::core::option::Option<Self>,
                tag: u32,
                wire_type: ::pilota::prost::encoding::WireType,
                buf: &mut B,
                ctx: ::pilota::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::pilota::prost::DecodeError>
            where
                B: ::pilota::prost::bytes::Buf,
            {
                match tag {
                    2u32 => match field {
                        ::core::option::Option::Some(Type::S(ref mut value)) => {
                            ::pilota::prost::encoding::string::merge(wire_type, value, buf, ctx)?;
                        }
                        _ => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::prost::encoding::string::merge(wire_type, value, buf, ctx)?;
                            *field = ::core::option::Option::Some(Type::S(owned_value));
                        }
                    },
                    4u32 => match field {
                        ::core::option::Option::Some(Type::I(ref mut value)) => {
                            ::pilota::prost::encoding::int32::merge(wire_type, value, buf, ctx)?;
                        }
                        _ => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::prost::encoding::int32::merge(wire_type, value, buf, ctx)?;
                            *field = ::core::option::Option::Some(Type::I(owned_value));
                        }
                    },
                    _ => unreachable!(concat!("invalid ", stringify!(Type), " tag: {}"), tag),
                };
                Ok(())
            }
        }
    }
}
