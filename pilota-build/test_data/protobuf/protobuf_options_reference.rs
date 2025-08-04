pub mod protobuf_options_reference {
    #![allow(warnings, clippy::all)]
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct BankTransferInfo {
        pub account_number: ::pilota::FastStr,

        pub bank_name: ::pilota::FastStr,

        pub swift_code: ::pilota::FastStr,
    }
    impl ::pilota::prost::Message for BankTransferInfo {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::pilota::prost::encoding::faststr::encoded_len(1, &self.account_number)
                + ::pilota::prost::encoding::faststr::encoded_len(2, &self.bank_name)
                + ::pilota::prost::encoding::faststr::encoded_len(3, &self.swift_code)
        }

        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::pilota::prost::bytes::BufMut,
        {
            ::pilota::prost::encoding::faststr::encode(1, &self.account_number, buf);
            ::pilota::prost::encoding::faststr::encode(2, &self.bank_name, buf);
            ::pilota::prost::encoding::faststr::encode(3, &self.swift_code, buf);
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
            const STRUCT_NAME: &'static str = stringify!(BankTransferInfo);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.account_number;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(account_number));
                        error
                    })
                }
                2 => {
                    let mut _inner_pilota_value = &mut self.bank_name;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(bank_name));
                        error
                    })
                }
                3 => {
                    let mut _inner_pilota_value = &mut self.swift_code;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(swift_code));
                        error
                    })
                }
                _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct UpdateUserResponse {
        pub user: ::std::option::Option<User>,

        pub message: ::pilota::FastStr,
    }
    impl ::pilota::prost::Message for UpdateUserResponse {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + self.user.as_ref().map_or(0, |msg| {
                ::pilota::prost::encoding::message::encoded_len(1, msg)
            }) + ::pilota::prost::encoding::faststr::encoded_len(2, &self.message)
        }

        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::pilota::prost::bytes::BufMut,
        {
            if let Some(_pilota_inner_value) = self.user.as_ref() {
                ::pilota::prost::encoding::message::encode(1, _pilota_inner_value, buf);
            }
            ::pilota::prost::encoding::faststr::encode(2, &self.message, buf);
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
            const STRUCT_NAME: &'static str = stringify!(UpdateUserResponse);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.user;
                    ::pilota::prost::encoding::message::merge(
                        wire_type,
                        _inner_pilota_value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(user));
                        error
                    })
                }
                2 => {
                    let mut _inner_pilota_value = &mut self.message;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(message));
                        error
                    })
                }
                _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct DatabaseOptions {
        pub table_name: ::std::option::Option<::pilota::FastStr>,

        pub engine: ::std::option::Option<::pilota::FastStr>,

        pub auto_increment: ::std::option::Option<bool>,

        pub indexes: ::std::vec::Vec<database_options::IndexOptions>,
    }
    impl ::pilota::prost::Message for DatabaseOptions {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + self.table_name.as_ref().map_or(0, |value| {
                ::pilota::prost::encoding::faststr::encoded_len(1, value)
            }) + self.engine.as_ref().map_or(0, |value| {
                ::pilota::prost::encoding::faststr::encoded_len(2, value)
            }) + self.auto_increment.as_ref().map_or(0, |value| {
                ::pilota::prost::encoding::bool::encoded_len(3, value)
            }) + ::pilota::prost::encoding::message::encoded_len_repeated(4, &self.indexes)
        }

        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::pilota::prost::bytes::BufMut,
        {
            if let Some(_pilota_inner_value) = self.table_name.as_ref() {
                ::pilota::prost::encoding::faststr::encode(1, _pilota_inner_value, buf);
            };
            if let Some(_pilota_inner_value) = self.engine.as_ref() {
                ::pilota::prost::encoding::faststr::encode(2, _pilota_inner_value, buf);
            };
            if let Some(_pilota_inner_value) = self.auto_increment.as_ref() {
                ::pilota::prost::encoding::bool::encode(3, _pilota_inner_value, buf);
            };
            for msg in &self.indexes {
                ::pilota::prost::encoding::message::encode(4, msg, buf);
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
            const STRUCT_NAME: &'static str = stringify!(DatabaseOptions);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.table_name;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(table_name));
                        error
                    })
                }
                2 => {
                    let mut _inner_pilota_value = &mut self.engine;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(engine));
                        error
                    })
                }
                3 => {
                    let mut _inner_pilota_value = &mut self.auto_increment;
                    ::pilota::prost::encoding::bool::merge(
                        wire_type,
                        _inner_pilota_value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(auto_increment));
                        error
                    })
                }
                4 => {
                    let mut _inner_pilota_value = &mut self.indexes;
                    ::pilota::prost::encoding::message::merge_repeated(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(indexes));
                        error
                    })
                }
                _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct UserProfile {
        pub full_name: ::pilota::FastStr,

        pub avatar_url: ::pilota::FastStr,

        pub bio: ::pilota::FastStr,
    }
    impl ::pilota::prost::Message for UserProfile {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::pilota::prost::encoding::faststr::encoded_len(1, &self.full_name)
                + ::pilota::prost::encoding::faststr::encoded_len(2, &self.avatar_url)
                + ::pilota::prost::encoding::faststr::encoded_len(3, &self.bio)
        }

        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::pilota::prost::bytes::BufMut,
        {
            ::pilota::prost::encoding::faststr::encode(1, &self.full_name, buf);
            ::pilota::prost::encoding::faststr::encode(2, &self.avatar_url, buf);
            ::pilota::prost::encoding::faststr::encode(3, &self.bio, buf);
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
            const STRUCT_NAME: &'static str = stringify!(UserProfile);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.full_name;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(full_name));
                        error
                    })
                }
                2 => {
                    let mut _inner_pilota_value = &mut self.avatar_url;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(avatar_url));
                        error
                    })
                }
                3 => {
                    let mut _inner_pilota_value = &mut self.bio;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(bio));
                        error
                    })
                }
                _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct CreateUserRequest {
        pub user: ::std::option::Option<User>,
    }
    impl ::pilota::prost::Message for CreateUserRequest {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + self.user.as_ref().map_or(0, |msg| {
                ::pilota::prost::encoding::message::encoded_len(1, msg)
            })
        }

        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::pilota::prost::bytes::BufMut,
        {
            if let Some(_pilota_inner_value) = self.user.as_ref() {
                ::pilota::prost::encoding::message::encode(1, _pilota_inner_value, buf);
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
            const STRUCT_NAME: &'static str = stringify!(CreateUserRequest);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.user;
                    ::pilota::prost::encoding::message::merge(
                        wire_type,
                        _inner_pilota_value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(user));
                        error
                    })
                }
                _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct GetOldUserFormatResponse {
        pub user: ::std::option::Option<User>,
    }
    impl ::pilota::prost::Message for GetOldUserFormatResponse {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + self.user.as_ref().map_or(0, |msg| {
                ::pilota::prost::encoding::message::encoded_len(1, msg)
            })
        }

        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::pilota::prost::bytes::BufMut,
        {
            if let Some(_pilota_inner_value) = self.user.as_ref() {
                ::pilota::prost::encoding::message::encode(1, _pilota_inner_value, buf);
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
            const STRUCT_NAME: &'static str = stringify!(GetOldUserFormatResponse);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.user;
                    ::pilota::prost::encoding::message::merge(
                        wire_type,
                        _inner_pilota_value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(user));
                        error
                    })
                }
                _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct CryptoCurrencyInfo {
        pub wallet_address: ::pilota::FastStr,

        pub currency_type: ::pilota::FastStr,
    }
    impl ::pilota::prost::Message for CryptoCurrencyInfo {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::pilota::prost::encoding::faststr::encoded_len(1, &self.wallet_address)
                + ::pilota::prost::encoding::faststr::encoded_len(2, &self.currency_type)
        }

        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::pilota::prost::bytes::BufMut,
        {
            ::pilota::prost::encoding::faststr::encode(1, &self.wallet_address, buf);
            ::pilota::prost::encoding::faststr::encode(2, &self.currency_type, buf);
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
            const STRUCT_NAME: &'static str = stringify!(CryptoCurrencyInfo);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.wallet_address;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(wallet_address));
                        error
                    })
                }
                2 => {
                    let mut _inner_pilota_value = &mut self.currency_type;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(currency_type));
                        error
                    })
                }
                _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct DeleteUserRequest {
        pub id: ::pilota::FastStr,
    }
    impl ::pilota::prost::Message for DeleteUserRequest {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::pilota::prost::encoding::faststr::encoded_len(1, &self.id)
        }

        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::pilota::prost::bytes::BufMut,
        {
            ::pilota::prost::encoding::faststr::encode(1, &self.id, buf);
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
            const STRUCT_NAME: &'static str = stringify!(DeleteUserRequest);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.id;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(id));
                        error
                    })
                }
                _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    #[derive(PartialOrd, Debug, Default, Clone, PartialEq)]
    pub struct PaymentInfo {
        pub id: i32,

        pub user_id: ::pilota::FastStr,

        pub amount: f64,

        pub payment_method: ::std::option::Option<payment_info::PaymentMethod>,
    }
    impl ::pilota::prost::Message for PaymentInfo {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::pilota::prost::encoding::int32::encoded_len(1, &self.id)
                + ::pilota::prost::encoding::faststr::encoded_len(2, &self.user_id)
                + ::pilota::prost::encoding::double::encoded_len(3, &self.amount)
                + self
                    .payment_method
                    .as_ref()
                    .map_or(0, |msg| msg.encoded_len())
        }

        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::pilota::prost::bytes::BufMut,
        {
            ::pilota::prost::encoding::int32::encode(1, &self.id, buf);
            ::pilota::prost::encoding::faststr::encode(2, &self.user_id, buf);
            ::pilota::prost::encoding::double::encode(3, &self.amount, buf);
            if let Some(_pilota_inner_value) = self.payment_method.as_ref() {
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
            const STRUCT_NAME: &'static str = stringify!(PaymentInfo);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.id;
                    ::pilota::prost::encoding::int32::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(id));
                        error
                    })
                }
                2 => {
                    let mut _inner_pilota_value = &mut self.user_id;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(user_id));
                        error
                    })
                }
                3 => {
                    let mut _inner_pilota_value = &mut self.amount;
                    ::pilota::prost::encoding::double::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(amount));
                        error
                    })
                }
                4 | 5 | 6 | 7 => {
                    let mut _inner_pilota_value = &mut self.payment_method;
                    payment_info::PaymentMethod::merge(
                        &mut _inner_pilota_value,
                        tag,
                        wire_type,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(payment_method));
                        error
                    })
                }
                _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct CreditCardInfo {
        pub card_number: ::pilota::FastStr,

        pub cardholder_name: ::pilota::FastStr,

        pub expiration_date: ::pilota::FastStr,
    }
    impl ::pilota::prost::Message for CreditCardInfo {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::pilota::prost::encoding::faststr::encoded_len(1, &self.card_number)
                + ::pilota::prost::encoding::faststr::encoded_len(2, &self.cardholder_name)
                + ::pilota::prost::encoding::faststr::encoded_len(3, &self.expiration_date)
        }

        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::pilota::prost::bytes::BufMut,
        {
            ::pilota::prost::encoding::faststr::encode(1, &self.card_number, buf);
            ::pilota::prost::encoding::faststr::encode(2, &self.cardholder_name, buf);
            ::pilota::prost::encoding::faststr::encode(3, &self.expiration_date, buf);
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
            const STRUCT_NAME: &'static str = stringify!(CreditCardInfo);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.card_number;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(card_number));
                        error
                    })
                }
                2 => {
                    let mut _inner_pilota_value = &mut self.cardholder_name;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(cardholder_name));
                        error
                    })
                }
                3 => {
                    let mut _inner_pilota_value = &mut self.expiration_date;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(expiration_date));
                        error
                    })
                }
                _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct CreateUserResponse {
        pub user: ::std::option::Option<User>,

        pub message: ::pilota::FastStr,
    }
    impl ::pilota::prost::Message for CreateUserResponse {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + self.user.as_ref().map_or(0, |msg| {
                ::pilota::prost::encoding::message::encoded_len(1, msg)
            }) + ::pilota::prost::encoding::faststr::encoded_len(2, &self.message)
        }

        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::pilota::prost::bytes::BufMut,
        {
            if let Some(_pilota_inner_value) = self.user.as_ref() {
                ::pilota::prost::encoding::message::encode(1, _pilota_inner_value, buf);
            }
            ::pilota::prost::encoding::faststr::encode(2, &self.message, buf);
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
            const STRUCT_NAME: &'static str = stringify!(CreateUserResponse);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.user;
                    ::pilota::prost::encoding::message::merge(
                        wire_type,
                        _inner_pilota_value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(user));
                        error
                    })
                }
                2 => {
                    let mut _inner_pilota_value = &mut self.message;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(message));
                        error
                    })
                }
                _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq, Copy)]
    #[repr(transparent)]
    pub struct UserStatus(i32);

    impl UserStatus {
        pub const UNKNOWN: Self = Self(0);
        pub const ACTIVE: Self = Self(1);
        pub const ENABLED: Self = Self(1);
        pub const INACTIVE: Self = Self(2);
        pub const SUSPENDED: Self = Self(3);
        pub const DELETED: Self = Self(4);

        pub fn inner(&self) -> i32 {
            self.0
        }

        pub fn to_string(&self) -> ::std::string::String {
            match self {
                Self(0) => ::std::string::String::from("UNKNOWN"),
                Self(1) => ::std::string::String::from("ACTIVE"),
                Self(1) => ::std::string::String::from("ENABLED"),
                Self(2) => ::std::string::String::from("INACTIVE"),
                Self(3) => ::std::string::String::from("SUSPENDED"),
                Self(4) => ::std::string::String::from("DELETED"),
                Self(val) => val.to_string(),
            }
        }

        pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
            match value {
                0 => Some(Self::UNKNOWN),
                1 => Some(Self::ACTIVE),
                1 => Some(Self::ENABLED),
                2 => Some(Self::INACTIVE),
                3 => Some(Self::SUSPENDED),
                4 => Some(Self::DELETED),
                _ => None,
            }
        }
    }

    impl ::std::convert::From<i32> for UserStatus {
        fn from(value: i32) -> Self {
            Self(value)
        }
    }

    impl ::std::convert::From<UserStatus> for i32 {
        fn from(value: UserStatus) -> i32 {
            value.0
        }
    }

    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq, Copy)]
    #[repr(transparent)]
    pub struct PaymentType(i32);

    impl PaymentType {
        pub const PAYMENT_UNKNOWN: Self = Self(0);
        pub const PAYMENT_CREDIT: Self = Self(1);
        pub const PAYMENT_DEBIT: Self = Self(2);
        pub const PAYMENT_BANK_TRANSFER: Self = Self(3);
        pub const PAYMENT_CRYPTO: Self = Self(4);
        pub const PAYMENT_CHECK: Self = Self(5);

        pub fn inner(&self) -> i32 {
            self.0
        }

        pub fn to_string(&self) -> ::std::string::String {
            match self {
                Self(0) => ::std::string::String::from("PAYMENT_UNKNOWN"),
                Self(1) => ::std::string::String::from("PAYMENT_CREDIT"),
                Self(2) => ::std::string::String::from("PAYMENT_DEBIT"),
                Self(3) => ::std::string::String::from("PAYMENT_BANK_TRANSFER"),
                Self(4) => ::std::string::String::from("PAYMENT_CRYPTO"),
                Self(5) => ::std::string::String::from("PAYMENT_CHECK"),
                Self(val) => val.to_string(),
            }
        }

        pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
            match value {
                0 => Some(Self::PAYMENT_UNKNOWN),
                1 => Some(Self::PAYMENT_CREDIT),
                2 => Some(Self::PAYMENT_DEBIT),
                3 => Some(Self::PAYMENT_BANK_TRANSFER),
                4 => Some(Self::PAYMENT_CRYPTO),
                5 => Some(Self::PAYMENT_CHECK),
                _ => None,
            }
        }
    }

    impl ::std::convert::From<i32> for PaymentType {
        fn from(value: i32) -> Self {
            Self(value)
        }
    }

    impl ::std::convert::From<PaymentType> for i32 {
        fn from(value: PaymentType) -> i32 {
            value.0
        }
    }

    pub trait UserService {}
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct GetUserRequest {
        pub id: ::pilota::FastStr,
    }
    impl ::pilota::prost::Message for GetUserRequest {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::pilota::prost::encoding::faststr::encoded_len(1, &self.id)
        }

        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::pilota::prost::bytes::BufMut,
        {
            ::pilota::prost::encoding::faststr::encode(1, &self.id, buf);
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
            const STRUCT_NAME: &'static str = stringify!(GetUserRequest);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.id;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(id));
                        error
                    })
                }
                _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct DeleteUserResponse {
        pub success: bool,

        pub message: ::pilota::FastStr,
    }
    impl ::pilota::prost::Message for DeleteUserResponse {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::pilota::prost::encoding::bool::encoded_len(1, &self.success)
                + ::pilota::prost::encoding::faststr::encoded_len(2, &self.message)
        }

        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::pilota::prost::bytes::BufMut,
        {
            ::pilota::prost::encoding::bool::encode(1, &self.success, buf);
            ::pilota::prost::encoding::faststr::encode(2, &self.message, buf);
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
            const STRUCT_NAME: &'static str = stringify!(DeleteUserResponse);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.success;
                    ::pilota::prost::encoding::bool::merge(wire_type, _inner_pilota_value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(success));
                            error
                        })
                }
                2 => {
                    let mut _inner_pilota_value = &mut self.message;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(message));
                        error
                    })
                }
                _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct DebitCardInfo {
        pub card_number: ::pilota::FastStr,

        pub holder_name: ::pilota::FastStr,

        pub bank_name: ::pilota::FastStr,
    }
    impl ::pilota::prost::Message for DebitCardInfo {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::pilota::prost::encoding::faststr::encoded_len(1, &self.card_number)
                + ::pilota::prost::encoding::faststr::encoded_len(2, &self.holder_name)
                + ::pilota::prost::encoding::faststr::encoded_len(3, &self.bank_name)
        }

        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::pilota::prost::bytes::BufMut,
        {
            ::pilota::prost::encoding::faststr::encode(1, &self.card_number, buf);
            ::pilota::prost::encoding::faststr::encode(2, &self.holder_name, buf);
            ::pilota::prost::encoding::faststr::encode(3, &self.bank_name, buf);
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
            const STRUCT_NAME: &'static str = stringify!(DebitCardInfo);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.card_number;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(card_number));
                        error
                    })
                }
                2 => {
                    let mut _inner_pilota_value = &mut self.holder_name;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(holder_name));
                        error
                    })
                }
                3 => {
                    let mut _inner_pilota_value = &mut self.bank_name;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(bank_name));
                        error
                    })
                }
                _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct UpdateUserRequest {
        pub id: ::pilota::FastStr,

        pub user: ::std::option::Option<User>,
    }
    impl ::pilota::prost::Message for UpdateUserRequest {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::pilota::prost::encoding::faststr::encoded_len(1, &self.id)
                + self.user.as_ref().map_or(0, |msg| {
                    ::pilota::prost::encoding::message::encoded_len(2, msg)
                })
        }

        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::pilota::prost::bytes::BufMut,
        {
            ::pilota::prost::encoding::faststr::encode(1, &self.id, buf);
            if let Some(_pilota_inner_value) = self.user.as_ref() {
                ::pilota::prost::encoding::message::encode(2, _pilota_inner_value, buf);
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
            const STRUCT_NAME: &'static str = stringify!(UpdateUserRequest);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.id;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(id));
                        error
                    })
                }
                2 => {
                    let mut _inner_pilota_value = &mut self.user;
                    ::pilota::prost::encoding::message::merge(
                        wire_type,
                        _inner_pilota_value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(user));
                        error
                    })
                }
                _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct MessageValidation {
        pub all_fields_required: ::std::option::Option<bool>,

        pub max_nesting_depth: ::std::option::Option<i32>,

        pub validation_message: ::std::option::Option<::pilota::FastStr>,
    }
    impl ::pilota::prost::Message for MessageValidation {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + self.all_fields_required.as_ref().map_or(0, |value| {
                ::pilota::prost::encoding::bool::encoded_len(1, value)
            }) + self.max_nesting_depth.as_ref().map_or(0, |value| {
                ::pilota::prost::encoding::int32::encoded_len(2, value)
            }) + self.validation_message.as_ref().map_or(0, |value| {
                ::pilota::prost::encoding::faststr::encoded_len(3, value)
            })
        }

        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::pilota::prost::bytes::BufMut,
        {
            if let Some(_pilota_inner_value) = self.all_fields_required.as_ref() {
                ::pilota::prost::encoding::bool::encode(1, _pilota_inner_value, buf);
            };
            if let Some(_pilota_inner_value) = self.max_nesting_depth.as_ref() {
                ::pilota::prost::encoding::int32::encode(2, _pilota_inner_value, buf);
            };
            if let Some(_pilota_inner_value) = self.validation_message.as_ref() {
                ::pilota::prost::encoding::faststr::encode(3, _pilota_inner_value, buf);
            };
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
            const STRUCT_NAME: &'static str = stringify!(MessageValidation);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.all_fields_required;
                    ::pilota::prost::encoding::bool::merge(
                        wire_type,
                        _inner_pilota_value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(all_fields_required));
                        error
                    })
                }
                2 => {
                    let mut _inner_pilota_value = &mut self.max_nesting_depth;
                    ::pilota::prost::encoding::int32::merge(
                        wire_type,
                        _inner_pilota_value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(max_nesting_depth));
                        error
                    })
                }
                3 => {
                    let mut _inner_pilota_value = &mut self.validation_message;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(validation_message));
                        error
                    })
                }
                _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct User {
        pub id: i32,

        pub username: ::pilota::FastStr,

        pub password: ::pilota::FastStr,

        pub email: ::pilota::FastStr,

        pub role_ids: ::std::vec::Vec<i32>,

        pub created_at: i64,

        pub profile: ::std::option::Option<UserProfile>,

        pub old_field: ::pilota::FastStr,
    }
    impl ::pilota::prost::Message for User {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::pilota::prost::encoding::int32::encoded_len(1, &self.id)
                + ::pilota::prost::encoding::faststr::encoded_len(2, &self.username)
                + ::pilota::prost::encoding::faststr::encoded_len(3, &self.password)
                + ::pilota::prost::encoding::faststr::encoded_len(4, &self.email)
                + ::pilota::prost::encoding::int32::encoded_len_repeated(5, &self.role_ids)
                + ::pilota::prost::encoding::int64::encoded_len(6, &self.created_at)
                + self.profile.as_ref().map_or(0, |msg| {
                    ::pilota::prost::encoding::message::encoded_len(7, msg)
                })
                + ::pilota::prost::encoding::faststr::encoded_len(8, &self.old_field)
        }

        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::pilota::prost::bytes::BufMut,
        {
            ::pilota::prost::encoding::int32::encode(1, &self.id, buf);
            ::pilota::prost::encoding::faststr::encode(2, &self.username, buf);
            ::pilota::prost::encoding::faststr::encode(3, &self.password, buf);
            ::pilota::prost::encoding::faststr::encode(4, &self.email, buf);
            ::pilota::prost::encoding::int32::encode_repeated(5, &self.role_ids, buf);
            ::pilota::prost::encoding::int64::encode(6, &self.created_at, buf);
            if let Some(_pilota_inner_value) = self.profile.as_ref() {
                ::pilota::prost::encoding::message::encode(7, _pilota_inner_value, buf);
            }
            ::pilota::prost::encoding::faststr::encode(8, &self.old_field, buf);
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
            const STRUCT_NAME: &'static str = stringify!(User);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.id;
                    ::pilota::prost::encoding::int32::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(id));
                        error
                    })
                }
                2 => {
                    let mut _inner_pilota_value = &mut self.username;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(username));
                        error
                    })
                }
                3 => {
                    let mut _inner_pilota_value = &mut self.password;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(password));
                        error
                    })
                }
                4 => {
                    let mut _inner_pilota_value = &mut self.email;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(email));
                        error
                    })
                }
                5 => {
                    let mut _inner_pilota_value = &mut self.role_ids;
                    ::pilota::prost::encoding::int32::merge_repeated(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(role_ids));
                        error
                    })
                }
                6 => {
                    let mut _inner_pilota_value = &mut self.created_at;
                    ::pilota::prost::encoding::int64::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(created_at));
                        error
                    })
                }
                7 => {
                    let mut _inner_pilota_value = &mut self.profile;
                    ::pilota::prost::encoding::message::merge(
                        wire_type,
                        _inner_pilota_value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(profile));
                        error
                    })
                }
                8 => {
                    let mut _inner_pilota_value = &mut self.old_field;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(old_field));
                        error
                    })
                }
                _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct GetUserResponse {
        pub user: ::std::option::Option<User>,
    }
    impl ::pilota::prost::Message for GetUserResponse {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + self.user.as_ref().map_or(0, |msg| {
                ::pilota::prost::encoding::message::encoded_len(1, msg)
            })
        }

        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::pilota::prost::bytes::BufMut,
        {
            if let Some(_pilota_inner_value) = self.user.as_ref() {
                ::pilota::prost::encoding::message::encode(1, _pilota_inner_value, buf);
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
            const STRUCT_NAME: &'static str = stringify!(GetUserResponse);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.user;
                    ::pilota::prost::encoding::message::merge(
                        wire_type,
                        _inner_pilota_value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(user));
                        error
                    })
                }
                _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct GetOldUserFormatRequest {
        pub id: ::pilota::FastStr,
    }
    impl ::pilota::prost::Message for GetOldUserFormatRequest {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::pilota::prost::encoding::faststr::encoded_len(1, &self.id)
        }

        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::pilota::prost::bytes::BufMut,
        {
            ::pilota::prost::encoding::faststr::encode(1, &self.id, buf);
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
            const STRUCT_NAME: &'static str = stringify!(GetOldUserFormatRequest);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.id;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(id));
                        error
                    })
                }
                _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }
    #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
    pub struct ApiMetadata {
        pub endpoint: ::std::option::Option<::pilota::FastStr>,

        pub method: ::std::option::Option<::pilota::FastStr>,

        pub deprecated: ::std::option::Option<bool>,

        pub version: ::std::option::Option<::pilota::FastStr>,

        pub tags: ::std::vec::Vec<::pilota::FastStr>,

        pub examples: ::std::vec::Vec<api_metadata::Example>,
    }
    impl ::pilota::prost::Message for ApiMetadata {
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + self.endpoint.as_ref().map_or(0, |value| {
                ::pilota::prost::encoding::faststr::encoded_len(1, value)
            }) + self.method.as_ref().map_or(0, |value| {
                ::pilota::prost::encoding::faststr::encoded_len(2, value)
            }) + self.deprecated.as_ref().map_or(0, |value| {
                ::pilota::prost::encoding::bool::encoded_len(3, value)
            }) + self.version.as_ref().map_or(0, |value| {
                ::pilota::prost::encoding::faststr::encoded_len(4, value)
            }) + ::pilota::prost::encoding::faststr::encoded_len_repeated(5, &self.tags)
                + ::pilota::prost::encoding::message::encoded_len_repeated(6, &self.examples)
        }

        #[allow(unused_variables)]
        fn encode_raw<B>(&self, buf: &mut B)
        where
            B: ::pilota::prost::bytes::BufMut,
        {
            if let Some(_pilota_inner_value) = self.endpoint.as_ref() {
                ::pilota::prost::encoding::faststr::encode(1, _pilota_inner_value, buf);
            };
            if let Some(_pilota_inner_value) = self.method.as_ref() {
                ::pilota::prost::encoding::faststr::encode(2, _pilota_inner_value, buf);
            };
            if let Some(_pilota_inner_value) = self.deprecated.as_ref() {
                ::pilota::prost::encoding::bool::encode(3, _pilota_inner_value, buf);
            };
            if let Some(_pilota_inner_value) = self.version.as_ref() {
                ::pilota::prost::encoding::faststr::encode(4, _pilota_inner_value, buf);
            };
            ::pilota::prost::encoding::faststr::encode_repeated(5, &self.tags, buf);
            for msg in &self.examples {
                ::pilota::prost::encoding::message::encode(6, msg, buf);
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
            const STRUCT_NAME: &'static str = stringify!(ApiMetadata);

            match tag {
                1 => {
                    let mut _inner_pilota_value = &mut self.endpoint;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(endpoint));
                        error
                    })
                }
                2 => {
                    let mut _inner_pilota_value = &mut self.method;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(method));
                        error
                    })
                }
                3 => {
                    let mut _inner_pilota_value = &mut self.deprecated;
                    ::pilota::prost::encoding::bool::merge(
                        wire_type,
                        _inner_pilota_value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(deprecated));
                        error
                    })
                }
                4 => {
                    let mut _inner_pilota_value = &mut self.version;
                    ::pilota::prost::encoding::faststr::merge(
                        wire_type,
                        _inner_pilota_value.get_or_insert_with(::core::default::Default::default),
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(version));
                        error
                    })
                }
                5 => {
                    let mut _inner_pilota_value = &mut self.tags;
                    ::pilota::prost::encoding::faststr::merge_repeated(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(tags));
                        error
                    })
                }
                6 => {
                    let mut _inner_pilota_value = &mut self.examples;
                    ::pilota::prost::encoding::message::merge_repeated(
                        wire_type,
                        _inner_pilota_value,
                        buf,
                        ctx,
                    )
                    .map_err(|mut error| {
                        error.push(STRUCT_NAME, stringify!(examples));
                        error
                    })
                }
                _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
    }

    pub mod api_metadata {

        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct Example {
            pub name: ::std::option::Option<::pilota::FastStr>,

            pub request: ::std::option::Option<::pilota::FastStr>,

            pub response: ::std::option::Option<::pilota::FastStr>,
        }
        impl ::pilota::prost::Message for Example {
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + self.name.as_ref().map_or(0, |value| {
                    ::pilota::prost::encoding::faststr::encoded_len(1, value)
                }) + self.request.as_ref().map_or(0, |value| {
                    ::pilota::prost::encoding::faststr::encoded_len(2, value)
                }) + self.response.as_ref().map_or(0, |value| {
                    ::pilota::prost::encoding::faststr::encoded_len(3, value)
                })
            }

            #[allow(unused_variables)]
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::pilota::prost::bytes::BufMut,
            {
                if let Some(_pilota_inner_value) = self.name.as_ref() {
                    ::pilota::prost::encoding::faststr::encode(1, _pilota_inner_value, buf);
                };
                if let Some(_pilota_inner_value) = self.request.as_ref() {
                    ::pilota::prost::encoding::faststr::encode(2, _pilota_inner_value, buf);
                };
                if let Some(_pilota_inner_value) = self.response.as_ref() {
                    ::pilota::prost::encoding::faststr::encode(3, _pilota_inner_value, buf);
                };
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
                const STRUCT_NAME: &'static str = stringify!(Example);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.name;
                        ::pilota::prost::encoding::faststr::merge(
                            wire_type,
                            _inner_pilota_value
                                .get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(name));
                            error
                        })
                    }
                    2 => {
                        let mut _inner_pilota_value = &mut self.request;
                        ::pilota::prost::encoding::faststr::merge(
                            wire_type,
                            _inner_pilota_value
                                .get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(request));
                            error
                        })
                    }
                    3 => {
                        let mut _inner_pilota_value = &mut self.response;
                        ::pilota::prost::encoding::faststr::merge(
                            wire_type,
                            _inner_pilota_value
                                .get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(response));
                            error
                        })
                    }
                    _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
    }

    pub mod database_options {

        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct IndexOptions {
            pub index_name: ::std::option::Option<::pilota::FastStr>,

            pub unique: ::std::option::Option<bool>,

            pub fields: ::std::vec::Vec<::pilota::FastStr>,
        }
        impl ::pilota::prost::Message for IndexOptions {
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + self.index_name.as_ref().map_or(0, |value| {
                    ::pilota::prost::encoding::faststr::encoded_len(1, value)
                }) + self.unique.as_ref().map_or(0, |value| {
                    ::pilota::prost::encoding::bool::encoded_len(2, value)
                }) + ::pilota::prost::encoding::faststr::encoded_len_repeated(3, &self.fields)
            }

            #[allow(unused_variables)]
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::pilota::prost::bytes::BufMut,
            {
                if let Some(_pilota_inner_value) = self.index_name.as_ref() {
                    ::pilota::prost::encoding::faststr::encode(1, _pilota_inner_value, buf);
                };
                if let Some(_pilota_inner_value) = self.unique.as_ref() {
                    ::pilota::prost::encoding::bool::encode(2, _pilota_inner_value, buf);
                };
                ::pilota::prost::encoding::faststr::encode_repeated(3, &self.fields, buf);
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
                const STRUCT_NAME: &'static str = stringify!(IndexOptions);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.index_name;
                        ::pilota::prost::encoding::faststr::merge(
                            wire_type,
                            _inner_pilota_value
                                .get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(index_name));
                            error
                        })
                    }
                    2 => {
                        let mut _inner_pilota_value = &mut self.unique;
                        ::pilota::prost::encoding::bool::merge(
                            wire_type,
                            _inner_pilota_value
                                .get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(unique));
                            error
                        })
                    }
                    3 => {
                        let mut _inner_pilota_value = &mut self.fields;
                        ::pilota::prost::encoding::faststr::merge_repeated(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(fields));
                            error
                        })
                    }
                    _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
    }

    pub mod google {

        pub mod protobuf {

            #[derive(PartialOrd, Debug, Default, Clone, PartialEq)]
            pub struct FileDescriptorSet {
                pub file: ::std::vec::Vec<FileDescriptorProto>,
            }
            impl ::pilota::prost::Message for FileDescriptorSet {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + ::pilota::prost::encoding::message::encoded_len_repeated(1, &self.file)
                }

                #[allow(unused_variables)]
                fn encode_raw<B>(&self, buf: &mut B)
                where
                    B: ::pilota::prost::bytes::BufMut,
                {
                    for msg in &self.file {
                        ::pilota::prost::encoding::message::encode(1, msg, buf);
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
                    const STRUCT_NAME: &'static str = stringify!(FileDescriptorSet);

                    match tag {
                        1 => {
                            let mut _inner_pilota_value = &mut self.file;
                            ::pilota::prost::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(file));
                                error
                            })
                        }
                        _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(PartialOrd, Debug, Default, Clone, PartialEq)]
            pub struct FieldDescriptorProto {
                pub name: ::std::option::Option<::pilota::FastStr>,

                pub number: ::std::option::Option<i32>,

                pub label: ::std::option::Option<field_descriptor_proto::Label>,

                pub r#type: ::std::option::Option<field_descriptor_proto::Type>,

                pub type_name: ::std::option::Option<::pilota::FastStr>,

                pub extendee: ::std::option::Option<::pilota::FastStr>,

                pub default_value: ::std::option::Option<::pilota::FastStr>,

                pub oneof_index: ::std::option::Option<i32>,

                pub json_name: ::std::option::Option<::pilota::FastStr>,

                pub options: ::std::option::Option<FieldOptions>,

                pub proto3_optional: ::std::option::Option<bool>,
            }
            impl ::pilota::prost::Message for FieldDescriptorProto {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + self.name.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::faststr::encoded_len(1, value)
                    }) + self.number.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::int32::encoded_len(3, value)
                    }) + self.label.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::int32::encoded_len(4, value)
                    }) + self.r#type.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::int32::encoded_len(5, value)
                    }) + self.type_name.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::faststr::encoded_len(6, value)
                    }) + self.extendee.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::faststr::encoded_len(2, value)
                    }) + self.default_value.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::faststr::encoded_len(7, value)
                    }) + self.oneof_index.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::int32::encoded_len(9, value)
                    }) + self.json_name.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::faststr::encoded_len(10, value)
                    }) + self.options.as_ref().map_or(0, |msg| {
                        ::pilota::prost::encoding::message::encoded_len(8, msg)
                    }) + self.proto3_optional.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::bool::encoded_len(17, value)
                    })
                }

                #[allow(unused_variables)]
                fn encode_raw<B>(&self, buf: &mut B)
                where
                    B: ::pilota::prost::bytes::BufMut,
                {
                    if let Some(_pilota_inner_value) = self.name.as_ref() {
                        ::pilota::prost::encoding::faststr::encode(1, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.number.as_ref() {
                        ::pilota::prost::encoding::int32::encode(3, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.label.as_ref() {
                        ::pilota::prost::encoding::int32::encode(4, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.r#type.as_ref() {
                        ::pilota::prost::encoding::int32::encode(5, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.type_name.as_ref() {
                        ::pilota::prost::encoding::faststr::encode(6, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.extendee.as_ref() {
                        ::pilota::prost::encoding::faststr::encode(2, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.default_value.as_ref() {
                        ::pilota::prost::encoding::faststr::encode(7, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.oneof_index.as_ref() {
                        ::pilota::prost::encoding::int32::encode(9, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.json_name.as_ref() {
                        ::pilota::prost::encoding::faststr::encode(10, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.options.as_ref() {
                        ::pilota::prost::encoding::message::encode(8, _pilota_inner_value, buf);
                    }
                    if let Some(_pilota_inner_value) = self.proto3_optional.as_ref() {
                        ::pilota::prost::encoding::bool::encode(17, _pilota_inner_value, buf);
                    };
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
                    const STRUCT_NAME: &'static str = stringify!(FieldDescriptorProto);

                    match tag {
                        1 => {
                            let mut _inner_pilota_value = &mut self.name;
                            ::pilota::prost::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(name));
                                error
                            })
                        }
                        3 => {
                            let mut _inner_pilota_value = &mut self.number;
                            ::pilota::prost::encoding::int32::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(number));
                                error
                            })
                        }
                        4 => {
                            let mut _inner_pilota_value = &mut self.label;
                            ::pilota::prost::encoding::int32::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(label));
                                error
                            })
                        }
                        5 => {
                            let mut _inner_pilota_value = &mut self.r#type;
                            ::pilota::prost::encoding::int32::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(r#type));
                                error
                            })
                        }
                        6 => {
                            let mut _inner_pilota_value = &mut self.type_name;
                            ::pilota::prost::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(type_name));
                                error
                            })
                        }
                        2 => {
                            let mut _inner_pilota_value = &mut self.extendee;
                            ::pilota::prost::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(extendee));
                                error
                            })
                        }
                        7 => {
                            let mut _inner_pilota_value = &mut self.default_value;
                            ::pilota::prost::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(default_value));
                                error
                            })
                        }
                        9 => {
                            let mut _inner_pilota_value = &mut self.oneof_index;
                            ::pilota::prost::encoding::int32::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(oneof_index));
                                error
                            })
                        }
                        10 => {
                            let mut _inner_pilota_value = &mut self.json_name;
                            ::pilota::prost::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(json_name));
                                error
                            })
                        }
                        8 => {
                            let mut _inner_pilota_value = &mut self.options;
                            ::pilota::prost::encoding::message::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(options));
                                error
                            })
                        }
                        17 => {
                            let mut _inner_pilota_value = &mut self.proto3_optional;
                            ::pilota::prost::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(proto3_optional));
                                error
                            })
                        }
                        _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(PartialOrd, Debug, Default, Clone, PartialEq)]
            pub struct MethodDescriptorProto {
                pub name: ::std::option::Option<::pilota::FastStr>,

                pub input_type: ::std::option::Option<::pilota::FastStr>,

                pub output_type: ::std::option::Option<::pilota::FastStr>,

                pub options: ::std::option::Option<MethodOptions>,

                pub client_streaming: ::std::option::Option<bool>,

                pub server_streaming: ::std::option::Option<bool>,
            }
            impl ::pilota::prost::Message for MethodDescriptorProto {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + self.name.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::faststr::encoded_len(1, value)
                    }) + self.input_type.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::faststr::encoded_len(2, value)
                    }) + self.output_type.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::faststr::encoded_len(3, value)
                    }) + self.options.as_ref().map_or(0, |msg| {
                        ::pilota::prost::encoding::message::encoded_len(4, msg)
                    }) + self.client_streaming.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::bool::encoded_len(5, value)
                    }) + self.server_streaming.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::bool::encoded_len(6, value)
                    })
                }

                #[allow(unused_variables)]
                fn encode_raw<B>(&self, buf: &mut B)
                where
                    B: ::pilota::prost::bytes::BufMut,
                {
                    if let Some(_pilota_inner_value) = self.name.as_ref() {
                        ::pilota::prost::encoding::faststr::encode(1, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.input_type.as_ref() {
                        ::pilota::prost::encoding::faststr::encode(2, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.output_type.as_ref() {
                        ::pilota::prost::encoding::faststr::encode(3, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.options.as_ref() {
                        ::pilota::prost::encoding::message::encode(4, _pilota_inner_value, buf);
                    }
                    if let Some(_pilota_inner_value) = self.client_streaming.as_ref() {
                        ::pilota::prost::encoding::bool::encode(5, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.server_streaming.as_ref() {
                        ::pilota::prost::encoding::bool::encode(6, _pilota_inner_value, buf);
                    };
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
                    const STRUCT_NAME: &'static str = stringify!(MethodDescriptorProto);

                    match tag {
                        1 => {
                            let mut _inner_pilota_value = &mut self.name;
                            ::pilota::prost::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(name));
                                error
                            })
                        }
                        2 => {
                            let mut _inner_pilota_value = &mut self.input_type;
                            ::pilota::prost::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(input_type));
                                error
                            })
                        }
                        3 => {
                            let mut _inner_pilota_value = &mut self.output_type;
                            ::pilota::prost::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(output_type));
                                error
                            })
                        }
                        4 => {
                            let mut _inner_pilota_value = &mut self.options;
                            ::pilota::prost::encoding::message::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(options));
                                error
                            })
                        }
                        5 => {
                            let mut _inner_pilota_value = &mut self.client_streaming;
                            ::pilota::prost::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(client_streaming));
                                error
                            })
                        }
                        6 => {
                            let mut _inner_pilota_value = &mut self.server_streaming;
                            ::pilota::prost::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(server_streaming));
                                error
                            })
                        }
                        _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
            pub struct SourceCodeInfo {
                pub location: ::std::vec::Vec<source_code_info::Location>,
            }
            impl ::pilota::prost::Message for SourceCodeInfo {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + ::pilota::prost::encoding::message::encoded_len_repeated(1, &self.location)
                }

                #[allow(unused_variables)]
                fn encode_raw<B>(&self, buf: &mut B)
                where
                    B: ::pilota::prost::bytes::BufMut,
                {
                    for msg in &self.location {
                        ::pilota::prost::encoding::message::encode(1, msg, buf);
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
                    const STRUCT_NAME: &'static str = stringify!(SourceCodeInfo);

                    match tag {
                        1 => {
                            let mut _inner_pilota_value = &mut self.location;
                            ::pilota::prost::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(location));
                                error
                            })
                        }
                        _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(PartialOrd, Debug, Default, Clone, PartialEq)]
            pub struct MessageOptions {
                pub message_set_wire_format: ::std::option::Option<bool>,

                pub no_standard_descriptor_accessor: ::std::option::Option<bool>,

                pub deprecated: ::std::option::Option<bool>,

                pub map_entry: ::std::option::Option<bool>,

                pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
            }
            impl ::pilota::prost::Message for MessageOptions {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + self.message_set_wire_format.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::bool::encoded_len(1, value)
                    }) + self
                        .no_standard_descriptor_accessor
                        .as_ref()
                        .map_or(0, |value| {
                            ::pilota::prost::encoding::bool::encoded_len(2, value)
                        })
                        + self.deprecated.as_ref().map_or(0, |value| {
                            ::pilota::prost::encoding::bool::encoded_len(3, value)
                        })
                        + self.map_entry.as_ref().map_or(0, |value| {
                            ::pilota::prost::encoding::bool::encoded_len(7, value)
                        })
                        + ::pilota::prost::encoding::message::encoded_len_repeated(
                            999,
                            &self.uninterpreted_option,
                        )
                }

                #[allow(unused_variables)]
                fn encode_raw<B>(&self, buf: &mut B)
                where
                    B: ::pilota::prost::bytes::BufMut,
                {
                    if let Some(_pilota_inner_value) = self.message_set_wire_format.as_ref() {
                        ::pilota::prost::encoding::bool::encode(1, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.no_standard_descriptor_accessor.as_ref()
                    {
                        ::pilota::prost::encoding::bool::encode(2, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.deprecated.as_ref() {
                        ::pilota::prost::encoding::bool::encode(3, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.map_entry.as_ref() {
                        ::pilota::prost::encoding::bool::encode(7, _pilota_inner_value, buf);
                    };
                    for msg in &self.uninterpreted_option {
                        ::pilota::prost::encoding::message::encode(999, msg, buf);
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
                    const STRUCT_NAME: &'static str = stringify!(MessageOptions);

                    match tag {
                        1 => {
                            let mut _inner_pilota_value = &mut self.message_set_wire_format;
                            ::pilota::prost::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(message_set_wire_format));
                                error
                            })
                        }
                        2 => {
                            let mut _inner_pilota_value = &mut self.no_standard_descriptor_accessor;
                            ::pilota::prost::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error
                                    .push(STRUCT_NAME, stringify!(no_standard_descriptor_accessor));
                                error
                            })
                        }
                        3 => {
                            let mut _inner_pilota_value = &mut self.deprecated;
                            ::pilota::prost::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(deprecated));
                                error
                            })
                        }
                        7 => {
                            let mut _inner_pilota_value = &mut self.map_entry;
                            ::pilota::prost::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(map_entry));
                                error
                            })
                        }
                        999 => {
                            let mut _inner_pilota_value = &mut self.uninterpreted_option;
                            ::pilota::prost::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(uninterpreted_option));
                                error
                            })
                        }
                        _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(PartialOrd, Debug, Default, Clone, PartialEq)]
            pub struct ServiceOptions {
                pub deprecated: ::std::option::Option<bool>,

                pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
            }
            impl ::pilota::prost::Message for ServiceOptions {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + self.deprecated.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::bool::encoded_len(33, value)
                    }) + ::pilota::prost::encoding::message::encoded_len_repeated(
                        999,
                        &self.uninterpreted_option,
                    )
                }

                #[allow(unused_variables)]
                fn encode_raw<B>(&self, buf: &mut B)
                where
                    B: ::pilota::prost::bytes::BufMut,
                {
                    if let Some(_pilota_inner_value) = self.deprecated.as_ref() {
                        ::pilota::prost::encoding::bool::encode(33, _pilota_inner_value, buf);
                    };
                    for msg in &self.uninterpreted_option {
                        ::pilota::prost::encoding::message::encode(999, msg, buf);
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
                    const STRUCT_NAME: &'static str = stringify!(ServiceOptions);

                    match tag {
                        33 => {
                            let mut _inner_pilota_value = &mut self.deprecated;
                            ::pilota::prost::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(deprecated));
                                error
                            })
                        }
                        999 => {
                            let mut _inner_pilota_value = &mut self.uninterpreted_option;
                            ::pilota::prost::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(uninterpreted_option));
                                error
                            })
                        }
                        _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(PartialOrd, Debug, Default, Clone, PartialEq)]
            pub struct UninterpretedOption {
                pub name: ::std::vec::Vec<uninterpreted_option::NamePart>,

                pub identifier_value: ::std::option::Option<::pilota::FastStr>,

                pub positive_int_value: ::std::option::Option<u64>,

                pub negative_int_value: ::std::option::Option<i64>,

                pub double_value: ::std::option::Option<f64>,

                pub string_value: ::std::option::Option<::pilota::Bytes>,

                pub aggregate_value: ::std::option::Option<::pilota::FastStr>,
            }
            impl ::pilota::prost::Message for UninterpretedOption {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + ::pilota::prost::encoding::message::encoded_len_repeated(2, &self.name)
                        + self.identifier_value.as_ref().map_or(0, |value| {
                            ::pilota::prost::encoding::faststr::encoded_len(3, value)
                        })
                        + self.positive_int_value.as_ref().map_or(0, |value| {
                            ::pilota::prost::encoding::uint64::encoded_len(4, value)
                        })
                        + self.negative_int_value.as_ref().map_or(0, |value| {
                            ::pilota::prost::encoding::int64::encoded_len(5, value)
                        })
                        + self.double_value.as_ref().map_or(0, |value| {
                            ::pilota::prost::encoding::double::encoded_len(6, value)
                        })
                        + self.string_value.as_ref().map_or(0, |value| {
                            ::pilota::prost::encoding::bytes::encoded_len(7, value)
                        })
                        + self.aggregate_value.as_ref().map_or(0, |value| {
                            ::pilota::prost::encoding::faststr::encoded_len(8, value)
                        })
                }

                #[allow(unused_variables)]
                fn encode_raw<B>(&self, buf: &mut B)
                where
                    B: ::pilota::prost::bytes::BufMut,
                {
                    for msg in &self.name {
                        ::pilota::prost::encoding::message::encode(2, msg, buf);
                    }
                    if let Some(_pilota_inner_value) = self.identifier_value.as_ref() {
                        ::pilota::prost::encoding::faststr::encode(3, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.positive_int_value.as_ref() {
                        ::pilota::prost::encoding::uint64::encode(4, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.negative_int_value.as_ref() {
                        ::pilota::prost::encoding::int64::encode(5, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.double_value.as_ref() {
                        ::pilota::prost::encoding::double::encode(6, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.string_value.as_ref() {
                        ::pilota::prost::encoding::bytes::encode(7, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.aggregate_value.as_ref() {
                        ::pilota::prost::encoding::faststr::encode(8, _pilota_inner_value, buf);
                    };
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
                    const STRUCT_NAME: &'static str = stringify!(UninterpretedOption);

                    match tag {
                        2 => {
                            let mut _inner_pilota_value = &mut self.name;
                            ::pilota::prost::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(name));
                                error
                            })
                        }
                        3 => {
                            let mut _inner_pilota_value = &mut self.identifier_value;
                            ::pilota::prost::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(identifier_value));
                                error
                            })
                        }
                        4 => {
                            let mut _inner_pilota_value = &mut self.positive_int_value;
                            ::pilota::prost::encoding::uint64::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(positive_int_value));
                                error
                            })
                        }
                        5 => {
                            let mut _inner_pilota_value = &mut self.negative_int_value;
                            ::pilota::prost::encoding::int64::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(negative_int_value));
                                error
                            })
                        }
                        6 => {
                            let mut _inner_pilota_value = &mut self.double_value;
                            ::pilota::prost::encoding::double::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(double_value));
                                error
                            })
                        }
                        7 => {
                            let mut _inner_pilota_value = &mut self.string_value;
                            ::pilota::prost::encoding::bytes::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(string_value));
                                error
                            })
                        }
                        8 => {
                            let mut _inner_pilota_value = &mut self.aggregate_value;
                            ::pilota::prost::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(aggregate_value));
                                error
                            })
                        }
                        _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(PartialOrd, Debug, Default, Clone, PartialEq)]
            pub struct FileDescriptorProto {
                pub name: ::std::option::Option<::pilota::FastStr>,

                pub package: ::std::option::Option<::pilota::FastStr>,

                pub dependency: ::std::vec::Vec<::pilota::FastStr>,

                pub public_dependency: ::std::vec::Vec<i32>,

                pub weak_dependency: ::std::vec::Vec<i32>,

                pub message_type: ::std::vec::Vec<DescriptorProto>,

                pub enum_type: ::std::vec::Vec<EnumDescriptorProto>,

                pub service: ::std::vec::Vec<ServiceDescriptorProto>,

                pub extension: ::std::vec::Vec<FieldDescriptorProto>,

                pub options: ::std::option::Option<FileOptions>,

                pub source_code_info: ::std::option::Option<SourceCodeInfo>,

                pub syntax: ::std::option::Option<::pilota::FastStr>,
            }
            impl ::pilota::prost::Message for FileDescriptorProto {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + self.name.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::faststr::encoded_len(1, value)
                    }) + self.package.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::faststr::encoded_len(2, value)
                    }) + ::pilota::prost::encoding::faststr::encoded_len_repeated(
                        3,
                        &self.dependency,
                    ) + ::pilota::prost::encoding::int32::encoded_len_repeated(
                        10,
                        &self.public_dependency,
                    ) + ::pilota::prost::encoding::int32::encoded_len_repeated(
                        11,
                        &self.weak_dependency,
                    ) + ::pilota::prost::encoding::message::encoded_len_repeated(
                        4,
                        &self.message_type,
                    ) + ::pilota::prost::encoding::message::encoded_len_repeated(5, &self.enum_type)
                        + ::pilota::prost::encoding::message::encoded_len_repeated(6, &self.service)
                        + ::pilota::prost::encoding::message::encoded_len_repeated(
                            7,
                            &self.extension,
                        )
                        + self.options.as_ref().map_or(0, |msg| {
                            ::pilota::prost::encoding::message::encoded_len(8, msg)
                        })
                        + self.source_code_info.as_ref().map_or(0, |msg| {
                            ::pilota::prost::encoding::message::encoded_len(9, msg)
                        })
                        + self.syntax.as_ref().map_or(0, |value| {
                            ::pilota::prost::encoding::faststr::encoded_len(12, value)
                        })
                }

                #[allow(unused_variables)]
                fn encode_raw<B>(&self, buf: &mut B)
                where
                    B: ::pilota::prost::bytes::BufMut,
                {
                    if let Some(_pilota_inner_value) = self.name.as_ref() {
                        ::pilota::prost::encoding::faststr::encode(1, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.package.as_ref() {
                        ::pilota::prost::encoding::faststr::encode(2, _pilota_inner_value, buf);
                    };
                    ::pilota::prost::encoding::faststr::encode_repeated(3, &self.dependency, buf);
                    ::pilota::prost::encoding::int32::encode_repeated(
                        10,
                        &self.public_dependency,
                        buf,
                    );
                    ::pilota::prost::encoding::int32::encode_repeated(
                        11,
                        &self.weak_dependency,
                        buf,
                    );
                    for msg in &self.message_type {
                        ::pilota::prost::encoding::message::encode(4, msg, buf);
                    }
                    for msg in &self.enum_type {
                        ::pilota::prost::encoding::message::encode(5, msg, buf);
                    }
                    for msg in &self.service {
                        ::pilota::prost::encoding::message::encode(6, msg, buf);
                    }
                    for msg in &self.extension {
                        ::pilota::prost::encoding::message::encode(7, msg, buf);
                    }
                    if let Some(_pilota_inner_value) = self.options.as_ref() {
                        ::pilota::prost::encoding::message::encode(8, _pilota_inner_value, buf);
                    }
                    if let Some(_pilota_inner_value) = self.source_code_info.as_ref() {
                        ::pilota::prost::encoding::message::encode(9, _pilota_inner_value, buf);
                    }
                    if let Some(_pilota_inner_value) = self.syntax.as_ref() {
                        ::pilota::prost::encoding::faststr::encode(12, _pilota_inner_value, buf);
                    };
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
                    const STRUCT_NAME: &'static str = stringify!(FileDescriptorProto);

                    match tag {
                        1 => {
                            let mut _inner_pilota_value = &mut self.name;
                            ::pilota::prost::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(name));
                                error
                            })
                        }
                        2 => {
                            let mut _inner_pilota_value = &mut self.package;
                            ::pilota::prost::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(package));
                                error
                            })
                        }
                        3 => {
                            let mut _inner_pilota_value = &mut self.dependency;
                            ::pilota::prost::encoding::faststr::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(dependency));
                                error
                            })
                        }
                        10 => {
                            let mut _inner_pilota_value = &mut self.public_dependency;
                            ::pilota::prost::encoding::int32::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(public_dependency));
                                error
                            })
                        }
                        11 => {
                            let mut _inner_pilota_value = &mut self.weak_dependency;
                            ::pilota::prost::encoding::int32::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(weak_dependency));
                                error
                            })
                        }
                        4 => {
                            let mut _inner_pilota_value = &mut self.message_type;
                            ::pilota::prost::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(message_type));
                                error
                            })
                        }
                        5 => {
                            let mut _inner_pilota_value = &mut self.enum_type;
                            ::pilota::prost::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(enum_type));
                                error
                            })
                        }
                        6 => {
                            let mut _inner_pilota_value = &mut self.service;
                            ::pilota::prost::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(service));
                                error
                            })
                        }
                        7 => {
                            let mut _inner_pilota_value = &mut self.extension;
                            ::pilota::prost::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(extension));
                                error
                            })
                        }
                        8 => {
                            let mut _inner_pilota_value = &mut self.options;
                            ::pilota::prost::encoding::message::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(options));
                                error
                            })
                        }
                        9 => {
                            let mut _inner_pilota_value = &mut self.source_code_info;
                            ::pilota::prost::encoding::message::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(source_code_info));
                                error
                            })
                        }
                        12 => {
                            let mut _inner_pilota_value = &mut self.syntax;
                            ::pilota::prost::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(syntax));
                                error
                            })
                        }
                        _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(PartialOrd, Debug, Default, Clone, PartialEq)]
            pub struct OneofOptions {
                pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
            }
            impl ::pilota::prost::Message for OneofOptions {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + ::pilota::prost::encoding::message::encoded_len_repeated(
                        999,
                        &self.uninterpreted_option,
                    )
                }

                #[allow(unused_variables)]
                fn encode_raw<B>(&self, buf: &mut B)
                where
                    B: ::pilota::prost::bytes::BufMut,
                {
                    for msg in &self.uninterpreted_option {
                        ::pilota::prost::encoding::message::encode(999, msg, buf);
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
                    const STRUCT_NAME: &'static str = stringify!(OneofOptions);

                    match tag {
                        999 => {
                            let mut _inner_pilota_value = &mut self.uninterpreted_option;
                            ::pilota::prost::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(uninterpreted_option));
                                error
                            })
                        }
                        _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(PartialOrd, Debug, Default, Clone, PartialEq)]
            pub struct OneofDescriptorProto {
                pub name: ::std::option::Option<::pilota::FastStr>,

                pub options: ::std::option::Option<OneofOptions>,
            }
            impl ::pilota::prost::Message for OneofDescriptorProto {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + self.name.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::faststr::encoded_len(1, value)
                    }) + self.options.as_ref().map_or(0, |msg| {
                        ::pilota::prost::encoding::message::encoded_len(2, msg)
                    })
                }

                #[allow(unused_variables)]
                fn encode_raw<B>(&self, buf: &mut B)
                where
                    B: ::pilota::prost::bytes::BufMut,
                {
                    if let Some(_pilota_inner_value) = self.name.as_ref() {
                        ::pilota::prost::encoding::faststr::encode(1, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.options.as_ref() {
                        ::pilota::prost::encoding::message::encode(2, _pilota_inner_value, buf);
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
                    const STRUCT_NAME: &'static str = stringify!(OneofDescriptorProto);

                    match tag {
                        1 => {
                            let mut _inner_pilota_value = &mut self.name;
                            ::pilota::prost::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(name));
                                error
                            })
                        }
                        2 => {
                            let mut _inner_pilota_value = &mut self.options;
                            ::pilota::prost::encoding::message::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(options));
                                error
                            })
                        }
                        _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(PartialOrd, Debug, Default, Clone, PartialEq)]
            pub struct FileOptions {
                pub java_package: ::std::option::Option<::pilota::FastStr>,

                pub java_outer_classname: ::std::option::Option<::pilota::FastStr>,

                pub java_multiple_files: ::std::option::Option<bool>,

                pub java_generate_equals_and_hash: ::std::option::Option<bool>,

                pub java_string_check_utf8: ::std::option::Option<bool>,

                pub optimize_for: ::std::option::Option<file_options::OptimizeMode>,

                pub go_package: ::std::option::Option<::pilota::FastStr>,

                pub cc_generic_services: ::std::option::Option<bool>,

                pub java_generic_services: ::std::option::Option<bool>,

                pub py_generic_services: ::std::option::Option<bool>,

                pub php_generic_services: ::std::option::Option<bool>,

                pub deprecated: ::std::option::Option<bool>,

                pub cc_enable_arenas: ::std::option::Option<bool>,

                pub objc_class_prefix: ::std::option::Option<::pilota::FastStr>,

                pub csharp_namespace: ::std::option::Option<::pilota::FastStr>,

                pub swift_prefix: ::std::option::Option<::pilota::FastStr>,

                pub php_class_prefix: ::std::option::Option<::pilota::FastStr>,

                pub php_namespace: ::std::option::Option<::pilota::FastStr>,

                pub php_metadata_namespace: ::std::option::Option<::pilota::FastStr>,

                pub ruby_package: ::std::option::Option<::pilota::FastStr>,

                pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
            }
            impl ::pilota::prost::Message for FileOptions {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + self.java_package.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::faststr::encoded_len(1, value)
                    }) + self.java_outer_classname.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::faststr::encoded_len(8, value)
                    }) + self.java_multiple_files.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::bool::encoded_len(10, value)
                    }) + self
                        .java_generate_equals_and_hash
                        .as_ref()
                        .map_or(0, |value| {
                            ::pilota::prost::encoding::bool::encoded_len(20, value)
                        })
                        + self.java_string_check_utf8.as_ref().map_or(0, |value| {
                            ::pilota::prost::encoding::bool::encoded_len(27, value)
                        })
                        + self.optimize_for.as_ref().map_or(0, |value| {
                            ::pilota::prost::encoding::int32::encoded_len(9, value)
                        })
                        + self.go_package.as_ref().map_or(0, |value| {
                            ::pilota::prost::encoding::faststr::encoded_len(11, value)
                        })
                        + self.cc_generic_services.as_ref().map_or(0, |value| {
                            ::pilota::prost::encoding::bool::encoded_len(16, value)
                        })
                        + self.java_generic_services.as_ref().map_or(0, |value| {
                            ::pilota::prost::encoding::bool::encoded_len(17, value)
                        })
                        + self.py_generic_services.as_ref().map_or(0, |value| {
                            ::pilota::prost::encoding::bool::encoded_len(18, value)
                        })
                        + self.php_generic_services.as_ref().map_or(0, |value| {
                            ::pilota::prost::encoding::bool::encoded_len(42, value)
                        })
                        + self.deprecated.as_ref().map_or(0, |value| {
                            ::pilota::prost::encoding::bool::encoded_len(23, value)
                        })
                        + self.cc_enable_arenas.as_ref().map_or(0, |value| {
                            ::pilota::prost::encoding::bool::encoded_len(31, value)
                        })
                        + self.objc_class_prefix.as_ref().map_or(0, |value| {
                            ::pilota::prost::encoding::faststr::encoded_len(36, value)
                        })
                        + self.csharp_namespace.as_ref().map_or(0, |value| {
                            ::pilota::prost::encoding::faststr::encoded_len(37, value)
                        })
                        + self.swift_prefix.as_ref().map_or(0, |value| {
                            ::pilota::prost::encoding::faststr::encoded_len(39, value)
                        })
                        + self.php_class_prefix.as_ref().map_or(0, |value| {
                            ::pilota::prost::encoding::faststr::encoded_len(40, value)
                        })
                        + self.php_namespace.as_ref().map_or(0, |value| {
                            ::pilota::prost::encoding::faststr::encoded_len(41, value)
                        })
                        + self.php_metadata_namespace.as_ref().map_or(0, |value| {
                            ::pilota::prost::encoding::faststr::encoded_len(44, value)
                        })
                        + self.ruby_package.as_ref().map_or(0, |value| {
                            ::pilota::prost::encoding::faststr::encoded_len(45, value)
                        })
                        + ::pilota::prost::encoding::message::encoded_len_repeated(
                            999,
                            &self.uninterpreted_option,
                        )
                }

                #[allow(unused_variables)]
                fn encode_raw<B>(&self, buf: &mut B)
                where
                    B: ::pilota::prost::bytes::BufMut,
                {
                    if let Some(_pilota_inner_value) = self.java_package.as_ref() {
                        ::pilota::prost::encoding::faststr::encode(1, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.java_outer_classname.as_ref() {
                        ::pilota::prost::encoding::faststr::encode(8, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.java_multiple_files.as_ref() {
                        ::pilota::prost::encoding::bool::encode(10, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.java_generate_equals_and_hash.as_ref() {
                        ::pilota::prost::encoding::bool::encode(20, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.java_string_check_utf8.as_ref() {
                        ::pilota::prost::encoding::bool::encode(27, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.optimize_for.as_ref() {
                        ::pilota::prost::encoding::int32::encode(9, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.go_package.as_ref() {
                        ::pilota::prost::encoding::faststr::encode(11, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.cc_generic_services.as_ref() {
                        ::pilota::prost::encoding::bool::encode(16, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.java_generic_services.as_ref() {
                        ::pilota::prost::encoding::bool::encode(17, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.py_generic_services.as_ref() {
                        ::pilota::prost::encoding::bool::encode(18, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.php_generic_services.as_ref() {
                        ::pilota::prost::encoding::bool::encode(42, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.deprecated.as_ref() {
                        ::pilota::prost::encoding::bool::encode(23, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.cc_enable_arenas.as_ref() {
                        ::pilota::prost::encoding::bool::encode(31, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.objc_class_prefix.as_ref() {
                        ::pilota::prost::encoding::faststr::encode(36, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.csharp_namespace.as_ref() {
                        ::pilota::prost::encoding::faststr::encode(37, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.swift_prefix.as_ref() {
                        ::pilota::prost::encoding::faststr::encode(39, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.php_class_prefix.as_ref() {
                        ::pilota::prost::encoding::faststr::encode(40, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.php_namespace.as_ref() {
                        ::pilota::prost::encoding::faststr::encode(41, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.php_metadata_namespace.as_ref() {
                        ::pilota::prost::encoding::faststr::encode(44, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.ruby_package.as_ref() {
                        ::pilota::prost::encoding::faststr::encode(45, _pilota_inner_value, buf);
                    };
                    for msg in &self.uninterpreted_option {
                        ::pilota::prost::encoding::message::encode(999, msg, buf);
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
                    const STRUCT_NAME: &'static str = stringify!(FileOptions);

                    match tag {
                        1 => {
                            let mut _inner_pilota_value = &mut self.java_package;
                            ::pilota::prost::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(java_package));
                                error
                            })
                        }
                        8 => {
                            let mut _inner_pilota_value = &mut self.java_outer_classname;
                            ::pilota::prost::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(java_outer_classname));
                                error
                            })
                        }
                        10 => {
                            let mut _inner_pilota_value = &mut self.java_multiple_files;
                            ::pilota::prost::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(java_multiple_files));
                                error
                            })
                        }
                        20 => {
                            let mut _inner_pilota_value = &mut self.java_generate_equals_and_hash;
                            ::pilota::prost::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(java_generate_equals_and_hash));
                                error
                            })
                        }
                        27 => {
                            let mut _inner_pilota_value = &mut self.java_string_check_utf8;
                            ::pilota::prost::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(java_string_check_utf8));
                                error
                            })
                        }
                        9 => {
                            let mut _inner_pilota_value = &mut self.optimize_for;
                            ::pilota::prost::encoding::int32::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(optimize_for));
                                error
                            })
                        }
                        11 => {
                            let mut _inner_pilota_value = &mut self.go_package;
                            ::pilota::prost::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(go_package));
                                error
                            })
                        }
                        16 => {
                            let mut _inner_pilota_value = &mut self.cc_generic_services;
                            ::pilota::prost::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(cc_generic_services));
                                error
                            })
                        }
                        17 => {
                            let mut _inner_pilota_value = &mut self.java_generic_services;
                            ::pilota::prost::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(java_generic_services));
                                error
                            })
                        }
                        18 => {
                            let mut _inner_pilota_value = &mut self.py_generic_services;
                            ::pilota::prost::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(py_generic_services));
                                error
                            })
                        }
                        42 => {
                            let mut _inner_pilota_value = &mut self.php_generic_services;
                            ::pilota::prost::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(php_generic_services));
                                error
                            })
                        }
                        23 => {
                            let mut _inner_pilota_value = &mut self.deprecated;
                            ::pilota::prost::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(deprecated));
                                error
                            })
                        }
                        31 => {
                            let mut _inner_pilota_value = &mut self.cc_enable_arenas;
                            ::pilota::prost::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(cc_enable_arenas));
                                error
                            })
                        }
                        36 => {
                            let mut _inner_pilota_value = &mut self.objc_class_prefix;
                            ::pilota::prost::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(objc_class_prefix));
                                error
                            })
                        }
                        37 => {
                            let mut _inner_pilota_value = &mut self.csharp_namespace;
                            ::pilota::prost::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(csharp_namespace));
                                error
                            })
                        }
                        39 => {
                            let mut _inner_pilota_value = &mut self.swift_prefix;
                            ::pilota::prost::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(swift_prefix));
                                error
                            })
                        }
                        40 => {
                            let mut _inner_pilota_value = &mut self.php_class_prefix;
                            ::pilota::prost::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(php_class_prefix));
                                error
                            })
                        }
                        41 => {
                            let mut _inner_pilota_value = &mut self.php_namespace;
                            ::pilota::prost::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(php_namespace));
                                error
                            })
                        }
                        44 => {
                            let mut _inner_pilota_value = &mut self.php_metadata_namespace;
                            ::pilota::prost::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(php_metadata_namespace));
                                error
                            })
                        }
                        45 => {
                            let mut _inner_pilota_value = &mut self.ruby_package;
                            ::pilota::prost::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(ruby_package));
                                error
                            })
                        }
                        999 => {
                            let mut _inner_pilota_value = &mut self.uninterpreted_option;
                            ::pilota::prost::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(uninterpreted_option));
                                error
                            })
                        }
                        _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(PartialOrd, Debug, Default, Clone, PartialEq)]
            pub struct FieldOptions {
                pub ctype: ::std::option::Option<field_options::CType>,

                pub packed: ::std::option::Option<bool>,

                pub jstype: ::std::option::Option<field_options::JsType>,

                pub lazy: ::std::option::Option<bool>,

                pub deprecated: ::std::option::Option<bool>,

                pub weak: ::std::option::Option<bool>,

                pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
            }
            impl ::pilota::prost::Message for FieldOptions {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + self.ctype.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::int32::encoded_len(1, value)
                    }) + self.packed.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::bool::encoded_len(2, value)
                    }) + self.jstype.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::int32::encoded_len(6, value)
                    }) + self.lazy.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::bool::encoded_len(5, value)
                    }) + self.deprecated.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::bool::encoded_len(3, value)
                    }) + self.weak.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::bool::encoded_len(10, value)
                    }) + ::pilota::prost::encoding::message::encoded_len_repeated(
                        999,
                        &self.uninterpreted_option,
                    )
                }

                #[allow(unused_variables)]
                fn encode_raw<B>(&self, buf: &mut B)
                where
                    B: ::pilota::prost::bytes::BufMut,
                {
                    if let Some(_pilota_inner_value) = self.ctype.as_ref() {
                        ::pilota::prost::encoding::int32::encode(1, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.packed.as_ref() {
                        ::pilota::prost::encoding::bool::encode(2, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.jstype.as_ref() {
                        ::pilota::prost::encoding::int32::encode(6, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.lazy.as_ref() {
                        ::pilota::prost::encoding::bool::encode(5, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.deprecated.as_ref() {
                        ::pilota::prost::encoding::bool::encode(3, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.weak.as_ref() {
                        ::pilota::prost::encoding::bool::encode(10, _pilota_inner_value, buf);
                    };
                    for msg in &self.uninterpreted_option {
                        ::pilota::prost::encoding::message::encode(999, msg, buf);
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
                    const STRUCT_NAME: &'static str = stringify!(FieldOptions);

                    match tag {
                        1 => {
                            let mut _inner_pilota_value = &mut self.ctype;
                            ::pilota::prost::encoding::int32::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(ctype));
                                error
                            })
                        }
                        2 => {
                            let mut _inner_pilota_value = &mut self.packed;
                            ::pilota::prost::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(packed));
                                error
                            })
                        }
                        6 => {
                            let mut _inner_pilota_value = &mut self.jstype;
                            ::pilota::prost::encoding::int32::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(jstype));
                                error
                            })
                        }
                        5 => {
                            let mut _inner_pilota_value = &mut self.lazy;
                            ::pilota::prost::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(lazy));
                                error
                            })
                        }
                        3 => {
                            let mut _inner_pilota_value = &mut self.deprecated;
                            ::pilota::prost::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(deprecated));
                                error
                            })
                        }
                        10 => {
                            let mut _inner_pilota_value = &mut self.weak;
                            ::pilota::prost::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(weak));
                                error
                            })
                        }
                        999 => {
                            let mut _inner_pilota_value = &mut self.uninterpreted_option;
                            ::pilota::prost::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(uninterpreted_option));
                                error
                            })
                        }
                        _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(PartialOrd, Debug, Default, Clone, PartialEq)]
            pub struct MethodOptions {
                pub deprecated: ::std::option::Option<bool>,

                pub idempotency_level: ::std::option::Option<method_options::IdempotencyLevel>,

                pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
            }
            impl ::pilota::prost::Message for MethodOptions {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + self.deprecated.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::bool::encoded_len(33, value)
                    }) + self.idempotency_level.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::int32::encoded_len(34, value)
                    }) + ::pilota::prost::encoding::message::encoded_len_repeated(
                        999,
                        &self.uninterpreted_option,
                    )
                }

                #[allow(unused_variables)]
                fn encode_raw<B>(&self, buf: &mut B)
                where
                    B: ::pilota::prost::bytes::BufMut,
                {
                    if let Some(_pilota_inner_value) = self.deprecated.as_ref() {
                        ::pilota::prost::encoding::bool::encode(33, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.idempotency_level.as_ref() {
                        ::pilota::prost::encoding::int32::encode(34, _pilota_inner_value, buf);
                    };
                    for msg in &self.uninterpreted_option {
                        ::pilota::prost::encoding::message::encode(999, msg, buf);
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
                    const STRUCT_NAME: &'static str = stringify!(MethodOptions);

                    match tag {
                        33 => {
                            let mut _inner_pilota_value = &mut self.deprecated;
                            ::pilota::prost::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(deprecated));
                                error
                            })
                        }
                        34 => {
                            let mut _inner_pilota_value = &mut self.idempotency_level;
                            ::pilota::prost::encoding::int32::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(idempotency_level));
                                error
                            })
                        }
                        999 => {
                            let mut _inner_pilota_value = &mut self.uninterpreted_option;
                            ::pilota::prost::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(uninterpreted_option));
                                error
                            })
                        }
                        _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(PartialOrd, Debug, Default, Clone, PartialEq)]
            pub struct EnumValueDescriptorProto {
                pub name: ::std::option::Option<::pilota::FastStr>,

                pub number: ::std::option::Option<i32>,

                pub options: ::std::option::Option<EnumValueOptions>,
            }
            impl ::pilota::prost::Message for EnumValueDescriptorProto {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + self.name.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::faststr::encoded_len(1, value)
                    }) + self.number.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::int32::encoded_len(2, value)
                    }) + self.options.as_ref().map_or(0, |msg| {
                        ::pilota::prost::encoding::message::encoded_len(3, msg)
                    })
                }

                #[allow(unused_variables)]
                fn encode_raw<B>(&self, buf: &mut B)
                where
                    B: ::pilota::prost::bytes::BufMut,
                {
                    if let Some(_pilota_inner_value) = self.name.as_ref() {
                        ::pilota::prost::encoding::faststr::encode(1, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.number.as_ref() {
                        ::pilota::prost::encoding::int32::encode(2, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.options.as_ref() {
                        ::pilota::prost::encoding::message::encode(3, _pilota_inner_value, buf);
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
                    const STRUCT_NAME: &'static str = stringify!(EnumValueDescriptorProto);

                    match tag {
                        1 => {
                            let mut _inner_pilota_value = &mut self.name;
                            ::pilota::prost::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(name));
                                error
                            })
                        }
                        2 => {
                            let mut _inner_pilota_value = &mut self.number;
                            ::pilota::prost::encoding::int32::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(number));
                                error
                            })
                        }
                        3 => {
                            let mut _inner_pilota_value = &mut self.options;
                            ::pilota::prost::encoding::message::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(options));
                                error
                            })
                        }
                        _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(PartialOrd, Debug, Default, Clone, PartialEq)]
            pub struct DescriptorProto {
                pub name: ::std::option::Option<::pilota::FastStr>,

                pub field: ::std::vec::Vec<FieldDescriptorProto>,

                pub extension: ::std::vec::Vec<FieldDescriptorProto>,

                pub nested_type: ::std::vec::Vec<DescriptorProto>,

                pub enum_type: ::std::vec::Vec<EnumDescriptorProto>,

                pub extension_range: ::std::vec::Vec<descriptor_proto::ExtensionRange>,

                pub oneof_decl: ::std::vec::Vec<OneofDescriptorProto>,

                pub options: ::std::option::Option<MessageOptions>,

                pub reserved_range: ::std::vec::Vec<descriptor_proto::ReservedRange>,

                pub reserved_name: ::std::vec::Vec<::pilota::FastStr>,
            }
            impl ::pilota::prost::Message for DescriptorProto {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + self.name.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::faststr::encoded_len(1, value)
                    }) + ::pilota::prost::encoding::message::encoded_len_repeated(2, &self.field)
                        + ::pilota::prost::encoding::message::encoded_len_repeated(
                            6,
                            &self.extension,
                        )
                        + ::pilota::prost::encoding::message::encoded_len_repeated(
                            3,
                            &self.nested_type,
                        )
                        + ::pilota::prost::encoding::message::encoded_len_repeated(
                            4,
                            &self.enum_type,
                        )
                        + ::pilota::prost::encoding::message::encoded_len_repeated(
                            5,
                            &self.extension_range,
                        )
                        + ::pilota::prost::encoding::message::encoded_len_repeated(
                            8,
                            &self.oneof_decl,
                        )
                        + self.options.as_ref().map_or(0, |msg| {
                            ::pilota::prost::encoding::message::encoded_len(7, msg)
                        })
                        + ::pilota::prost::encoding::message::encoded_len_repeated(
                            9,
                            &self.reserved_range,
                        )
                        + ::pilota::prost::encoding::faststr::encoded_len_repeated(
                            10,
                            &self.reserved_name,
                        )
                }

                #[allow(unused_variables)]
                fn encode_raw<B>(&self, buf: &mut B)
                where
                    B: ::pilota::prost::bytes::BufMut,
                {
                    if let Some(_pilota_inner_value) = self.name.as_ref() {
                        ::pilota::prost::encoding::faststr::encode(1, _pilota_inner_value, buf);
                    };
                    for msg in &self.field {
                        ::pilota::prost::encoding::message::encode(2, msg, buf);
                    }
                    for msg in &self.extension {
                        ::pilota::prost::encoding::message::encode(6, msg, buf);
                    }
                    for msg in &self.nested_type {
                        ::pilota::prost::encoding::message::encode(3, msg, buf);
                    }
                    for msg in &self.enum_type {
                        ::pilota::prost::encoding::message::encode(4, msg, buf);
                    }
                    for msg in &self.extension_range {
                        ::pilota::prost::encoding::message::encode(5, msg, buf);
                    }
                    for msg in &self.oneof_decl {
                        ::pilota::prost::encoding::message::encode(8, msg, buf);
                    }
                    if let Some(_pilota_inner_value) = self.options.as_ref() {
                        ::pilota::prost::encoding::message::encode(7, _pilota_inner_value, buf);
                    }
                    for msg in &self.reserved_range {
                        ::pilota::prost::encoding::message::encode(9, msg, buf);
                    }
                    ::pilota::prost::encoding::faststr::encode_repeated(
                        10,
                        &self.reserved_name,
                        buf,
                    );
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
                    const STRUCT_NAME: &'static str = stringify!(DescriptorProto);

                    match tag {
                        1 => {
                            let mut _inner_pilota_value = &mut self.name;
                            ::pilota::prost::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(name));
                                error
                            })
                        }
                        2 => {
                            let mut _inner_pilota_value = &mut self.field;
                            ::pilota::prost::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(field));
                                error
                            })
                        }
                        6 => {
                            let mut _inner_pilota_value = &mut self.extension;
                            ::pilota::prost::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(extension));
                                error
                            })
                        }
                        3 => {
                            let mut _inner_pilota_value = &mut self.nested_type;
                            ::pilota::prost::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(nested_type));
                                error
                            })
                        }
                        4 => {
                            let mut _inner_pilota_value = &mut self.enum_type;
                            ::pilota::prost::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(enum_type));
                                error
                            })
                        }
                        5 => {
                            let mut _inner_pilota_value = &mut self.extension_range;
                            ::pilota::prost::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(extension_range));
                                error
                            })
                        }
                        8 => {
                            let mut _inner_pilota_value = &mut self.oneof_decl;
                            ::pilota::prost::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(oneof_decl));
                                error
                            })
                        }
                        7 => {
                            let mut _inner_pilota_value = &mut self.options;
                            ::pilota::prost::encoding::message::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(options));
                                error
                            })
                        }
                        9 => {
                            let mut _inner_pilota_value = &mut self.reserved_range;
                            ::pilota::prost::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(reserved_range));
                                error
                            })
                        }
                        10 => {
                            let mut _inner_pilota_value = &mut self.reserved_name;
                            ::pilota::prost::encoding::faststr::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(reserved_name));
                                error
                            })
                        }
                        _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(PartialOrd, Debug, Default, Clone, PartialEq)]
            pub struct EnumOptions {
                pub allow_alias: ::std::option::Option<bool>,

                pub deprecated: ::std::option::Option<bool>,

                pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
            }
            impl ::pilota::prost::Message for EnumOptions {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + self.allow_alias.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::bool::encoded_len(2, value)
                    }) + self.deprecated.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::bool::encoded_len(3, value)
                    }) + ::pilota::prost::encoding::message::encoded_len_repeated(
                        999,
                        &self.uninterpreted_option,
                    )
                }

                #[allow(unused_variables)]
                fn encode_raw<B>(&self, buf: &mut B)
                where
                    B: ::pilota::prost::bytes::BufMut,
                {
                    if let Some(_pilota_inner_value) = self.allow_alias.as_ref() {
                        ::pilota::prost::encoding::bool::encode(2, _pilota_inner_value, buf);
                    };
                    if let Some(_pilota_inner_value) = self.deprecated.as_ref() {
                        ::pilota::prost::encoding::bool::encode(3, _pilota_inner_value, buf);
                    };
                    for msg in &self.uninterpreted_option {
                        ::pilota::prost::encoding::message::encode(999, msg, buf);
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
                    const STRUCT_NAME: &'static str = stringify!(EnumOptions);

                    match tag {
                        2 => {
                            let mut _inner_pilota_value = &mut self.allow_alias;
                            ::pilota::prost::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(allow_alias));
                                error
                            })
                        }
                        3 => {
                            let mut _inner_pilota_value = &mut self.deprecated;
                            ::pilota::prost::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(deprecated));
                                error
                            })
                        }
                        999 => {
                            let mut _inner_pilota_value = &mut self.uninterpreted_option;
                            ::pilota::prost::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(uninterpreted_option));
                                error
                            })
                        }
                        _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(PartialOrd, Debug, Default, Clone, PartialEq)]
            pub struct EnumDescriptorProto {
                pub name: ::std::option::Option<::pilota::FastStr>,

                pub value: ::std::vec::Vec<EnumValueDescriptorProto>,

                pub options: ::std::option::Option<EnumOptions>,

                pub reserved_range: ::std::vec::Vec<enum_descriptor_proto::EnumReservedRange>,

                pub reserved_name: ::std::vec::Vec<::pilota::FastStr>,
            }
            impl ::pilota::prost::Message for EnumDescriptorProto {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + self.name.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::faststr::encoded_len(1, value)
                    }) + ::pilota::prost::encoding::message::encoded_len_repeated(2, &self.value)
                        + self.options.as_ref().map_or(0, |msg| {
                            ::pilota::prost::encoding::message::encoded_len(3, msg)
                        })
                        + ::pilota::prost::encoding::message::encoded_len_repeated(
                            4,
                            &self.reserved_range,
                        )
                        + ::pilota::prost::encoding::faststr::encoded_len_repeated(
                            5,
                            &self.reserved_name,
                        )
                }

                #[allow(unused_variables)]
                fn encode_raw<B>(&self, buf: &mut B)
                where
                    B: ::pilota::prost::bytes::BufMut,
                {
                    if let Some(_pilota_inner_value) = self.name.as_ref() {
                        ::pilota::prost::encoding::faststr::encode(1, _pilota_inner_value, buf);
                    };
                    for msg in &self.value {
                        ::pilota::prost::encoding::message::encode(2, msg, buf);
                    }
                    if let Some(_pilota_inner_value) = self.options.as_ref() {
                        ::pilota::prost::encoding::message::encode(3, _pilota_inner_value, buf);
                    }
                    for msg in &self.reserved_range {
                        ::pilota::prost::encoding::message::encode(4, msg, buf);
                    }
                    ::pilota::prost::encoding::faststr::encode_repeated(
                        5,
                        &self.reserved_name,
                        buf,
                    );
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
                    const STRUCT_NAME: &'static str = stringify!(EnumDescriptorProto);

                    match tag {
                        1 => {
                            let mut _inner_pilota_value = &mut self.name;
                            ::pilota::prost::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(name));
                                error
                            })
                        }
                        2 => {
                            let mut _inner_pilota_value = &mut self.value;
                            ::pilota::prost::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(value));
                                error
                            })
                        }
                        3 => {
                            let mut _inner_pilota_value = &mut self.options;
                            ::pilota::prost::encoding::message::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(options));
                                error
                            })
                        }
                        4 => {
                            let mut _inner_pilota_value = &mut self.reserved_range;
                            ::pilota::prost::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(reserved_range));
                                error
                            })
                        }
                        5 => {
                            let mut _inner_pilota_value = &mut self.reserved_name;
                            ::pilota::prost::encoding::faststr::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(reserved_name));
                                error
                            })
                        }
                        _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(PartialOrd, Debug, Default, Clone, PartialEq)]
            pub struct ExtensionRangeOptions {
                pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
            }
            impl ::pilota::prost::Message for ExtensionRangeOptions {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + ::pilota::prost::encoding::message::encoded_len_repeated(
                        999,
                        &self.uninterpreted_option,
                    )
                }

                #[allow(unused_variables)]
                fn encode_raw<B>(&self, buf: &mut B)
                where
                    B: ::pilota::prost::bytes::BufMut,
                {
                    for msg in &self.uninterpreted_option {
                        ::pilota::prost::encoding::message::encode(999, msg, buf);
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
                    const STRUCT_NAME: &'static str = stringify!(ExtensionRangeOptions);

                    match tag {
                        999 => {
                            let mut _inner_pilota_value = &mut self.uninterpreted_option;
                            ::pilota::prost::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(uninterpreted_option));
                                error
                            })
                        }
                        _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(PartialOrd, Debug, Default, Clone, PartialEq)]
            pub struct ServiceDescriptorProto {
                pub name: ::std::option::Option<::pilota::FastStr>,

                pub method: ::std::vec::Vec<MethodDescriptorProto>,

                pub options: ::std::option::Option<ServiceOptions>,
            }
            impl ::pilota::prost::Message for ServiceDescriptorProto {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + self.name.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::faststr::encoded_len(1, value)
                    }) + ::pilota::prost::encoding::message::encoded_len_repeated(2, &self.method)
                        + self.options.as_ref().map_or(0, |msg| {
                            ::pilota::prost::encoding::message::encoded_len(3, msg)
                        })
                }

                #[allow(unused_variables)]
                fn encode_raw<B>(&self, buf: &mut B)
                where
                    B: ::pilota::prost::bytes::BufMut,
                {
                    if let Some(_pilota_inner_value) = self.name.as_ref() {
                        ::pilota::prost::encoding::faststr::encode(1, _pilota_inner_value, buf);
                    };
                    for msg in &self.method {
                        ::pilota::prost::encoding::message::encode(2, msg, buf);
                    }
                    if let Some(_pilota_inner_value) = self.options.as_ref() {
                        ::pilota::prost::encoding::message::encode(3, _pilota_inner_value, buf);
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
                    const STRUCT_NAME: &'static str = stringify!(ServiceDescriptorProto);

                    match tag {
                        1 => {
                            let mut _inner_pilota_value = &mut self.name;
                            ::pilota::prost::encoding::faststr::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(name));
                                error
                            })
                        }
                        2 => {
                            let mut _inner_pilota_value = &mut self.method;
                            ::pilota::prost::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(method));
                                error
                            })
                        }
                        3 => {
                            let mut _inner_pilota_value = &mut self.options;
                            ::pilota::prost::encoding::message::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(options));
                                error
                            })
                        }
                        _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(PartialOrd, Debug, Default, Clone, PartialEq)]
            pub struct EnumValueOptions {
                pub deprecated: ::std::option::Option<bool>,

                pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
            }
            impl ::pilota::prost::Message for EnumValueOptions {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + self.deprecated.as_ref().map_or(0, |value| {
                        ::pilota::prost::encoding::bool::encoded_len(1, value)
                    }) + ::pilota::prost::encoding::message::encoded_len_repeated(
                        999,
                        &self.uninterpreted_option,
                    )
                }

                #[allow(unused_variables)]
                fn encode_raw<B>(&self, buf: &mut B)
                where
                    B: ::pilota::prost::bytes::BufMut,
                {
                    if let Some(_pilota_inner_value) = self.deprecated.as_ref() {
                        ::pilota::prost::encoding::bool::encode(1, _pilota_inner_value, buf);
                    };
                    for msg in &self.uninterpreted_option {
                        ::pilota::prost::encoding::message::encode(999, msg, buf);
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
                    const STRUCT_NAME: &'static str = stringify!(EnumValueOptions);

                    match tag {
                        1 => {
                            let mut _inner_pilota_value = &mut self.deprecated;
                            ::pilota::prost::encoding::bool::merge(
                                wire_type,
                                _inner_pilota_value
                                    .get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(deprecated));
                                error
                            })
                        }
                        999 => {
                            let mut _inner_pilota_value = &mut self.uninterpreted_option;
                            ::pilota::prost::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(uninterpreted_option));
                                error
                            })
                        }
                        _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
            pub struct GeneratedCodeInfo {
                pub annotation: ::std::vec::Vec<generated_code_info::Annotation>,
            }
            impl ::pilota::prost::Message for GeneratedCodeInfo {
                #[inline]
                fn encoded_len(&self) -> usize {
                    0 + ::pilota::prost::encoding::message::encoded_len_repeated(
                        1,
                        &self.annotation,
                    )
                }

                #[allow(unused_variables)]
                fn encode_raw<B>(&self, buf: &mut B)
                where
                    B: ::pilota::prost::bytes::BufMut,
                {
                    for msg in &self.annotation {
                        ::pilota::prost::encoding::message::encode(1, msg, buf);
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
                    const STRUCT_NAME: &'static str = stringify!(GeneratedCodeInfo);

                    match tag {
                        1 => {
                            let mut _inner_pilota_value = &mut self.annotation;
                            ::pilota::prost::encoding::message::merge_repeated(
                                wire_type,
                                _inner_pilota_value,
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, stringify!(annotation));
                                error
                            })
                        }
                        _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                    }
                }
            }

            pub mod descriptor_proto {

                #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
                pub struct ReservedRange {
                    pub start: ::std::option::Option<i32>,

                    pub end: ::std::option::Option<i32>,
                }
                impl ::pilota::prost::Message for ReservedRange {
                    #[inline]
                    fn encoded_len(&self) -> usize {
                        0 + self.start.as_ref().map_or(0, |value| {
                            ::pilota::prost::encoding::int32::encoded_len(1, value)
                        }) + self.end.as_ref().map_or(0, |value| {
                            ::pilota::prost::encoding::int32::encoded_len(2, value)
                        })
                    }

                    #[allow(unused_variables)]
                    fn encode_raw<B>(&self, buf: &mut B)
                    where
                        B: ::pilota::prost::bytes::BufMut,
                    {
                        if let Some(_pilota_inner_value) = self.start.as_ref() {
                            ::pilota::prost::encoding::int32::encode(1, _pilota_inner_value, buf);
                        };
                        if let Some(_pilota_inner_value) = self.end.as_ref() {
                            ::pilota::prost::encoding::int32::encode(2, _pilota_inner_value, buf);
                        };
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
                        const STRUCT_NAME: &'static str = stringify!(ReservedRange);

                        match tag {
                            1 => {
                                let mut _inner_pilota_value = &mut self.start;
                                ::pilota::prost::encoding::int32::merge(
                                    wire_type,
                                    _inner_pilota_value
                                        .get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(start));
                                    error
                                })
                            }
                            2 => {
                                let mut _inner_pilota_value = &mut self.end;
                                ::pilota::prost::encoding::int32::merge(
                                    wire_type,
                                    _inner_pilota_value
                                        .get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(end));
                                    error
                                })
                            }
                            _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                        }
                    }
                }
                #[derive(PartialOrd, Debug, Default, Clone, PartialEq)]
                pub struct ExtensionRange {
                    pub start: ::std::option::Option<i32>,

                    pub end: ::std::option::Option<i32>,

                    pub options: ::std::option::Option<super::ExtensionRangeOptions>,
                }
                impl ::pilota::prost::Message for ExtensionRange {
                    #[inline]
                    fn encoded_len(&self) -> usize {
                        0 + self.start.as_ref().map_or(0, |value| {
                            ::pilota::prost::encoding::int32::encoded_len(1, value)
                        }) + self.end.as_ref().map_or(0, |value| {
                            ::pilota::prost::encoding::int32::encoded_len(2, value)
                        }) + self.options.as_ref().map_or(0, |msg| {
                            ::pilota::prost::encoding::message::encoded_len(3, msg)
                        })
                    }

                    #[allow(unused_variables)]
                    fn encode_raw<B>(&self, buf: &mut B)
                    where
                        B: ::pilota::prost::bytes::BufMut,
                    {
                        if let Some(_pilota_inner_value) = self.start.as_ref() {
                            ::pilota::prost::encoding::int32::encode(1, _pilota_inner_value, buf);
                        };
                        if let Some(_pilota_inner_value) = self.end.as_ref() {
                            ::pilota::prost::encoding::int32::encode(2, _pilota_inner_value, buf);
                        };
                        if let Some(_pilota_inner_value) = self.options.as_ref() {
                            ::pilota::prost::encoding::message::encode(3, _pilota_inner_value, buf);
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
                        const STRUCT_NAME: &'static str = stringify!(ExtensionRange);

                        match tag {
                            1 => {
                                let mut _inner_pilota_value = &mut self.start;
                                ::pilota::prost::encoding::int32::merge(
                                    wire_type,
                                    _inner_pilota_value
                                        .get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(start));
                                    error
                                })
                            }
                            2 => {
                                let mut _inner_pilota_value = &mut self.end;
                                ::pilota::prost::encoding::int32::merge(
                                    wire_type,
                                    _inner_pilota_value
                                        .get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(end));
                                    error
                                })
                            }
                            3 => {
                                let mut _inner_pilota_value = &mut self.options;
                                ::pilota::prost::encoding::message::merge(
                                    wire_type,
                                    _inner_pilota_value
                                        .get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(options));
                                    error
                                })
                            }
                            _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                        }
                    }
                }
            }

            pub mod enum_descriptor_proto {

                #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
                pub struct EnumReservedRange {
                    pub start: ::std::option::Option<i32>,

                    pub end: ::std::option::Option<i32>,
                }
                impl ::pilota::prost::Message for EnumReservedRange {
                    #[inline]
                    fn encoded_len(&self) -> usize {
                        0 + self.start.as_ref().map_or(0, |value| {
                            ::pilota::prost::encoding::int32::encoded_len(1, value)
                        }) + self.end.as_ref().map_or(0, |value| {
                            ::pilota::prost::encoding::int32::encoded_len(2, value)
                        })
                    }

                    #[allow(unused_variables)]
                    fn encode_raw<B>(&self, buf: &mut B)
                    where
                        B: ::pilota::prost::bytes::BufMut,
                    {
                        if let Some(_pilota_inner_value) = self.start.as_ref() {
                            ::pilota::prost::encoding::int32::encode(1, _pilota_inner_value, buf);
                        };
                        if let Some(_pilota_inner_value) = self.end.as_ref() {
                            ::pilota::prost::encoding::int32::encode(2, _pilota_inner_value, buf);
                        };
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
                        const STRUCT_NAME: &'static str = stringify!(EnumReservedRange);

                        match tag {
                            1 => {
                                let mut _inner_pilota_value = &mut self.start;
                                ::pilota::prost::encoding::int32::merge(
                                    wire_type,
                                    _inner_pilota_value
                                        .get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(start));
                                    error
                                })
                            }
                            2 => {
                                let mut _inner_pilota_value = &mut self.end;
                                ::pilota::prost::encoding::int32::merge(
                                    wire_type,
                                    _inner_pilota_value
                                        .get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(end));
                                    error
                                })
                            }
                            _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                        }
                    }
                }
            }

            pub mod field_descriptor_proto {

                #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq, Copy)]
                #[repr(transparent)]
                pub struct Label(i32);

                impl Label {
                    pub const LABEL_OPTIONAL: Self = Self(1);
                    pub const LABEL_REQUIRED: Self = Self(2);
                    pub const LABEL_REPEATED: Self = Self(3);

                    pub fn inner(&self) -> i32 {
                        self.0
                    }

                    pub fn to_string(&self) -> ::std::string::String {
                        match self {
                            Self(1) => ::std::string::String::from("LABEL_OPTIONAL"),
                            Self(2) => ::std::string::String::from("LABEL_REQUIRED"),
                            Self(3) => ::std::string::String::from("LABEL_REPEATED"),
                            Self(val) => val.to_string(),
                        }
                    }

                    pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
                        match value {
                            1 => Some(Self::LABEL_OPTIONAL),
                            2 => Some(Self::LABEL_REQUIRED),
                            3 => Some(Self::LABEL_REPEATED),
                            _ => None,
                        }
                    }
                }

                impl ::std::convert::From<i32> for Label {
                    fn from(value: i32) -> Self {
                        Self(value)
                    }
                }

                impl ::std::convert::From<Label> for i32 {
                    fn from(value: Label) -> i32 {
                        value.0
                    }
                }

                #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq, Copy)]
                #[repr(transparent)]
                pub struct Type(i32);

                impl Type {
                    pub const TYPE_DOUBLE: Self = Self(1);
                    pub const TYPE_FLOAT: Self = Self(2);
                    pub const TYPE_INT64: Self = Self(3);
                    pub const TYPE_UINT64: Self = Self(4);
                    pub const TYPE_INT32: Self = Self(5);
                    pub const TYPE_FIXED64: Self = Self(6);
                    pub const TYPE_FIXED32: Self = Self(7);
                    pub const TYPE_BOOL: Self = Self(8);
                    pub const TYPE_STRING: Self = Self(9);
                    pub const TYPE_GROUP: Self = Self(10);
                    pub const TYPE_MESSAGE: Self = Self(11);
                    pub const TYPE_BYTES: Self = Self(12);
                    pub const TYPE_UINT32: Self = Self(13);
                    pub const TYPE_ENUM: Self = Self(14);
                    pub const TYPE_SFIXED32: Self = Self(15);
                    pub const TYPE_SFIXED64: Self = Self(16);
                    pub const TYPE_SINT32: Self = Self(17);
                    pub const TYPE_SINT64: Self = Self(18);

                    pub fn inner(&self) -> i32 {
                        self.0
                    }

                    pub fn to_string(&self) -> ::std::string::String {
                        match self {
                            Self(1) => ::std::string::String::from("TYPE_DOUBLE"),
                            Self(2) => ::std::string::String::from("TYPE_FLOAT"),
                            Self(3) => ::std::string::String::from("TYPE_INT64"),
                            Self(4) => ::std::string::String::from("TYPE_UINT64"),
                            Self(5) => ::std::string::String::from("TYPE_INT32"),
                            Self(6) => ::std::string::String::from("TYPE_FIXED64"),
                            Self(7) => ::std::string::String::from("TYPE_FIXED32"),
                            Self(8) => ::std::string::String::from("TYPE_BOOL"),
                            Self(9) => ::std::string::String::from("TYPE_STRING"),
                            Self(10) => ::std::string::String::from("TYPE_GROUP"),
                            Self(11) => ::std::string::String::from("TYPE_MESSAGE"),
                            Self(12) => ::std::string::String::from("TYPE_BYTES"),
                            Self(13) => ::std::string::String::from("TYPE_UINT32"),
                            Self(14) => ::std::string::String::from("TYPE_ENUM"),
                            Self(15) => ::std::string::String::from("TYPE_SFIXED32"),
                            Self(16) => ::std::string::String::from("TYPE_SFIXED64"),
                            Self(17) => ::std::string::String::from("TYPE_SINT32"),
                            Self(18) => ::std::string::String::from("TYPE_SINT64"),
                            Self(val) => val.to_string(),
                        }
                    }

                    pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
                        match value {
                            1 => Some(Self::TYPE_DOUBLE),
                            2 => Some(Self::TYPE_FLOAT),
                            3 => Some(Self::TYPE_INT64),
                            4 => Some(Self::TYPE_UINT64),
                            5 => Some(Self::TYPE_INT32),
                            6 => Some(Self::TYPE_FIXED64),
                            7 => Some(Self::TYPE_FIXED32),
                            8 => Some(Self::TYPE_BOOL),
                            9 => Some(Self::TYPE_STRING),
                            10 => Some(Self::TYPE_GROUP),
                            11 => Some(Self::TYPE_MESSAGE),
                            12 => Some(Self::TYPE_BYTES),
                            13 => Some(Self::TYPE_UINT32),
                            14 => Some(Self::TYPE_ENUM),
                            15 => Some(Self::TYPE_SFIXED32),
                            16 => Some(Self::TYPE_SFIXED64),
                            17 => Some(Self::TYPE_SINT32),
                            18 => Some(Self::TYPE_SINT64),
                            _ => None,
                        }
                    }
                }

                impl ::std::convert::From<i32> for Type {
                    fn from(value: i32) -> Self {
                        Self(value)
                    }
                }

                impl ::std::convert::From<Type> for i32 {
                    fn from(value: Type) -> i32 {
                        value.0
                    }
                }
            }

            pub mod field_options {

                #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq, Copy)]
                #[repr(transparent)]
                pub struct JsType(i32);

                impl JsType {
                    pub const JS_NORMAL: Self = Self(0);
                    pub const JS_STRING: Self = Self(1);
                    pub const JS_NUMBER: Self = Self(2);

                    pub fn inner(&self) -> i32 {
                        self.0
                    }

                    pub fn to_string(&self) -> ::std::string::String {
                        match self {
                            Self(0) => ::std::string::String::from("JS_NORMAL"),
                            Self(1) => ::std::string::String::from("JS_STRING"),
                            Self(2) => ::std::string::String::from("JS_NUMBER"),
                            Self(val) => val.to_string(),
                        }
                    }

                    pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
                        match value {
                            0 => Some(Self::JS_NORMAL),
                            1 => Some(Self::JS_STRING),
                            2 => Some(Self::JS_NUMBER),
                            _ => None,
                        }
                    }
                }

                impl ::std::convert::From<i32> for JsType {
                    fn from(value: i32) -> Self {
                        Self(value)
                    }
                }

                impl ::std::convert::From<JsType> for i32 {
                    fn from(value: JsType) -> i32 {
                        value.0
                    }
                }

                #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq, Copy)]
                #[repr(transparent)]
                pub struct CType(i32);

                impl CType {
                    pub const STRING: Self = Self(0);
                    pub const CORD: Self = Self(1);
                    pub const STRING_PIECE: Self = Self(2);

                    pub fn inner(&self) -> i32 {
                        self.0
                    }

                    pub fn to_string(&self) -> ::std::string::String {
                        match self {
                            Self(0) => ::std::string::String::from("STRING"),
                            Self(1) => ::std::string::String::from("CORD"),
                            Self(2) => ::std::string::String::from("STRING_PIECE"),
                            Self(val) => val.to_string(),
                        }
                    }

                    pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
                        match value {
                            0 => Some(Self::STRING),
                            1 => Some(Self::CORD),
                            2 => Some(Self::STRING_PIECE),
                            _ => None,
                        }
                    }
                }

                impl ::std::convert::From<i32> for CType {
                    fn from(value: i32) -> Self {
                        Self(value)
                    }
                }

                impl ::std::convert::From<CType> for i32 {
                    fn from(value: CType) -> i32 {
                        value.0
                    }
                }
            }

            pub mod file_options {

                #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq, Copy)]
                #[repr(transparent)]
                pub struct OptimizeMode(i32);

                impl OptimizeMode {
                    pub const SPEED: Self = Self(1);
                    pub const CODE_SIZE: Self = Self(2);
                    pub const LITE_RUNTIME: Self = Self(3);

                    pub fn inner(&self) -> i32 {
                        self.0
                    }

                    pub fn to_string(&self) -> ::std::string::String {
                        match self {
                            Self(1) => ::std::string::String::from("SPEED"),
                            Self(2) => ::std::string::String::from("CODE_SIZE"),
                            Self(3) => ::std::string::String::from("LITE_RUNTIME"),
                            Self(val) => val.to_string(),
                        }
                    }

                    pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
                        match value {
                            1 => Some(Self::SPEED),
                            2 => Some(Self::CODE_SIZE),
                            3 => Some(Self::LITE_RUNTIME),
                            _ => None,
                        }
                    }
                }

                impl ::std::convert::From<i32> for OptimizeMode {
                    fn from(value: i32) -> Self {
                        Self(value)
                    }
                }

                impl ::std::convert::From<OptimizeMode> for i32 {
                    fn from(value: OptimizeMode) -> i32 {
                        value.0
                    }
                }
            }

            pub mod generated_code_info {

                #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
                pub struct Annotation {
                    pub path: ::std::vec::Vec<i32>,

                    pub source_file: ::std::option::Option<::pilota::FastStr>,

                    pub begin: ::std::option::Option<i32>,

                    pub end: ::std::option::Option<i32>,
                }
                impl ::pilota::prost::Message for Annotation {
                    #[inline]
                    fn encoded_len(&self) -> usize {
                        0 + ::pilota::prost::encoding::int32::encoded_len_repeated(1, &self.path)
                            + self.source_file.as_ref().map_or(0, |value| {
                                ::pilota::prost::encoding::faststr::encoded_len(2, value)
                            })
                            + self.begin.as_ref().map_or(0, |value| {
                                ::pilota::prost::encoding::int32::encoded_len(3, value)
                            })
                            + self.end.as_ref().map_or(0, |value| {
                                ::pilota::prost::encoding::int32::encoded_len(4, value)
                            })
                    }

                    #[allow(unused_variables)]
                    fn encode_raw<B>(&self, buf: &mut B)
                    where
                        B: ::pilota::prost::bytes::BufMut,
                    {
                        ::pilota::prost::encoding::int32::encode_repeated(1, &self.path, buf);
                        if let Some(_pilota_inner_value) = self.source_file.as_ref() {
                            ::pilota::prost::encoding::faststr::encode(2, _pilota_inner_value, buf);
                        };
                        if let Some(_pilota_inner_value) = self.begin.as_ref() {
                            ::pilota::prost::encoding::int32::encode(3, _pilota_inner_value, buf);
                        };
                        if let Some(_pilota_inner_value) = self.end.as_ref() {
                            ::pilota::prost::encoding::int32::encode(4, _pilota_inner_value, buf);
                        };
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
                        const STRUCT_NAME: &'static str = stringify!(Annotation);

                        match tag {
                            1 => {
                                let mut _inner_pilota_value = &mut self.path;
                                ::pilota::prost::encoding::int32::merge_repeated(
                                    wire_type,
                                    _inner_pilota_value,
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(path));
                                    error
                                })
                            }
                            2 => {
                                let mut _inner_pilota_value = &mut self.source_file;
                                ::pilota::prost::encoding::faststr::merge(
                                    wire_type,
                                    _inner_pilota_value
                                        .get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(source_file));
                                    error
                                })
                            }
                            3 => {
                                let mut _inner_pilota_value = &mut self.begin;
                                ::pilota::prost::encoding::int32::merge(
                                    wire_type,
                                    _inner_pilota_value
                                        .get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(begin));
                                    error
                                })
                            }
                            4 => {
                                let mut _inner_pilota_value = &mut self.end;
                                ::pilota::prost::encoding::int32::merge(
                                    wire_type,
                                    _inner_pilota_value
                                        .get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(end));
                                    error
                                })
                            }
                            _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                        }
                    }
                }
            }

            pub mod method_options {

                #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq, Copy)]
                #[repr(transparent)]
                pub struct IdempotencyLevel(i32);

                impl IdempotencyLevel {
                    pub const IDEMPOTENCY_UNKNOWN: Self = Self(0);
                    pub const NO_SIDE_EFFECTS: Self = Self(1);
                    pub const IDEMPOTENT: Self = Self(2);

                    pub fn inner(&self) -> i32 {
                        self.0
                    }

                    pub fn to_string(&self) -> ::std::string::String {
                        match self {
                            Self(0) => ::std::string::String::from("IDEMPOTENCY_UNKNOWN"),
                            Self(1) => ::std::string::String::from("NO_SIDE_EFFECTS"),
                            Self(2) => ::std::string::String::from("IDEMPOTENT"),
                            Self(val) => val.to_string(),
                        }
                    }

                    pub fn try_from_i32(value: i32) -> ::std::option::Option<Self> {
                        match value {
                            0 => Some(Self::IDEMPOTENCY_UNKNOWN),
                            1 => Some(Self::NO_SIDE_EFFECTS),
                            2 => Some(Self::IDEMPOTENT),
                            _ => None,
                        }
                    }
                }

                impl ::std::convert::From<i32> for IdempotencyLevel {
                    fn from(value: i32) -> Self {
                        Self(value)
                    }
                }

                impl ::std::convert::From<IdempotencyLevel> for i32 {
                    fn from(value: IdempotencyLevel) -> i32 {
                        value.0
                    }
                }
            }

            pub mod source_code_info {

                #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
                pub struct Location {
                    pub path: ::std::vec::Vec<i32>,

                    pub span: ::std::vec::Vec<i32>,

                    pub leading_comments: ::std::option::Option<::pilota::FastStr>,

                    pub trailing_comments: ::std::option::Option<::pilota::FastStr>,

                    pub leading_detached_comments: ::std::vec::Vec<::pilota::FastStr>,
                }
                impl ::pilota::prost::Message for Location {
                    #[inline]
                    fn encoded_len(&self) -> usize {
                        0 + ::pilota::prost::encoding::int32::encoded_len_repeated(1, &self.path)
                            + ::pilota::prost::encoding::int32::encoded_len_repeated(2, &self.span)
                            + self.leading_comments.as_ref().map_or(0, |value| {
                                ::pilota::prost::encoding::faststr::encoded_len(3, value)
                            })
                            + self.trailing_comments.as_ref().map_or(0, |value| {
                                ::pilota::prost::encoding::faststr::encoded_len(4, value)
                            })
                            + ::pilota::prost::encoding::faststr::encoded_len_repeated(
                                6,
                                &self.leading_detached_comments,
                            )
                    }

                    #[allow(unused_variables)]
                    fn encode_raw<B>(&self, buf: &mut B)
                    where
                        B: ::pilota::prost::bytes::BufMut,
                    {
                        ::pilota::prost::encoding::int32::encode_repeated(1, &self.path, buf);
                        ::pilota::prost::encoding::int32::encode_repeated(2, &self.span, buf);
                        if let Some(_pilota_inner_value) = self.leading_comments.as_ref() {
                            ::pilota::prost::encoding::faststr::encode(3, _pilota_inner_value, buf);
                        };
                        if let Some(_pilota_inner_value) = self.trailing_comments.as_ref() {
                            ::pilota::prost::encoding::faststr::encode(4, _pilota_inner_value, buf);
                        };
                        ::pilota::prost::encoding::faststr::encode_repeated(
                            6,
                            &self.leading_detached_comments,
                            buf,
                        );
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
                        const STRUCT_NAME: &'static str = stringify!(Location);

                        match tag {
                            1 => {
                                let mut _inner_pilota_value = &mut self.path;
                                ::pilota::prost::encoding::int32::merge_repeated(
                                    wire_type,
                                    _inner_pilota_value,
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(path));
                                    error
                                })
                            }
                            2 => {
                                let mut _inner_pilota_value = &mut self.span;
                                ::pilota::prost::encoding::int32::merge_repeated(
                                    wire_type,
                                    _inner_pilota_value,
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(span));
                                    error
                                })
                            }
                            3 => {
                                let mut _inner_pilota_value = &mut self.leading_comments;
                                ::pilota::prost::encoding::faststr::merge(
                                    wire_type,
                                    _inner_pilota_value
                                        .get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(leading_comments));
                                    error
                                })
                            }
                            4 => {
                                let mut _inner_pilota_value = &mut self.trailing_comments;
                                ::pilota::prost::encoding::faststr::merge(
                                    wire_type,
                                    _inner_pilota_value
                                        .get_or_insert_with(::core::default::Default::default),
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(trailing_comments));
                                    error
                                })
                            }
                            6 => {
                                let mut _inner_pilota_value = &mut self.leading_detached_comments;
                                ::pilota::prost::encoding::faststr::merge_repeated(
                                    wire_type,
                                    _inner_pilota_value,
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(leading_detached_comments));
                                    error
                                })
                            }
                            _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                        }
                    }
                }
            }

            pub mod uninterpreted_option {

                #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
                pub struct NamePart {
                    pub name_part: ::pilota::FastStr,

                    pub is_extension: bool,
                }
                impl ::pilota::prost::Message for NamePart {
                    #[inline]
                    fn encoded_len(&self) -> usize {
                        0 + ::pilota::prost::encoding::faststr::encoded_len(1, &self.name_part)
                            + ::pilota::prost::encoding::bool::encoded_len(2, &self.is_extension)
                    }

                    #[allow(unused_variables)]
                    fn encode_raw<B>(&self, buf: &mut B)
                    where
                        B: ::pilota::prost::bytes::BufMut,
                    {
                        ::pilota::prost::encoding::faststr::encode(1, &self.name_part, buf);
                        ::pilota::prost::encoding::bool::encode(2, &self.is_extension, buf);
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
                        const STRUCT_NAME: &'static str = stringify!(NamePart);

                        match tag {
                            1 => {
                                let mut _inner_pilota_value = &mut self.name_part;
                                ::pilota::prost::encoding::faststr::merge(
                                    wire_type,
                                    _inner_pilota_value,
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(name_part));
                                    error
                                })
                            }
                            2 => {
                                let mut _inner_pilota_value = &mut self.is_extension;
                                ::pilota::prost::encoding::bool::merge(
                                    wire_type,
                                    _inner_pilota_value,
                                    buf,
                                    ctx,
                                )
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, stringify!(is_extension));
                                    error
                                })
                            }
                            _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                        }
                    }
                }
            }
        }
    }

    pub mod payment_info {

        impl ::std::default::Default for PaymentMethod {
            fn default() -> Self {
                PaymentMethod::CreditCard(::std::default::Default::default())
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Clone, PartialEq)]
        pub enum PaymentMethod {
            CreditCard(super::CreditCardInfo),

            DebitCard(super::DebitCardInfo),

            BankTransfer(super::BankTransferInfo),

            Crypto(super::CryptoCurrencyInfo),
        }
        impl PaymentMethod {
            pub fn encode<B>(&self, buf: &mut B)
            where
                B: ::pilota::prost::bytes::BufMut,
            {
                match self {
                    PaymentMethod::CreditCard(value) => {
                        ::pilota::prost::encoding::message::encode(4, (&*value), buf);
                    }
                    PaymentMethod::DebitCard(value) => {
                        ::pilota::prost::encoding::message::encode(5, (&*value), buf);
                    }
                    PaymentMethod::BankTransfer(value) => {
                        ::pilota::prost::encoding::message::encode(6, (&*value), buf);
                    }
                    PaymentMethod::Crypto(value) => {
                        ::pilota::prost::encoding::message::encode(7, (&*value), buf);
                    }
                }
            }

            #[inline]
            pub fn encoded_len(&self) -> usize {
                match self {
                    PaymentMethod::CreditCard(value) => {
                        ::pilota::prost::encoding::message::encoded_len(4, &*value)
                    }
                    PaymentMethod::DebitCard(value) => {
                        ::pilota::prost::encoding::message::encoded_len(5, &*value)
                    }
                    PaymentMethod::BankTransfer(value) => {
                        ::pilota::prost::encoding::message::encoded_len(6, &*value)
                    }
                    PaymentMethod::Crypto(value) => {
                        ::pilota::prost::encoding::message::encoded_len(7, &*value)
                    }
                }
            }

            #[inline]
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
                    4 => match field {
                        ::core::option::Option::Some(PaymentMethod::CreditCard(value)) => {
                            ::pilota::prost::encoding::message::merge(wire_type, value, buf, ctx)?;
                        }
                        _ => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::prost::encoding::message::merge(wire_type, value, buf, ctx)?;
                            *field = ::core::option::Option::Some(PaymentMethod::CreditCard(
                                owned_value,
                            ));
                        }
                    },
                    5 => match field {
                        ::core::option::Option::Some(PaymentMethod::DebitCard(value)) => {
                            ::pilota::prost::encoding::message::merge(wire_type, value, buf, ctx)?;
                        }
                        _ => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::prost::encoding::message::merge(wire_type, value, buf, ctx)?;
                            *field =
                                ::core::option::Option::Some(PaymentMethod::DebitCard(owned_value));
                        }
                    },
                    6 => match field {
                        ::core::option::Option::Some(PaymentMethod::BankTransfer(value)) => {
                            ::pilota::prost::encoding::message::merge(wire_type, value, buf, ctx)?;
                        }
                        _ => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::prost::encoding::message::merge(wire_type, value, buf, ctx)?;
                            *field = ::core::option::Option::Some(PaymentMethod::BankTransfer(
                                owned_value,
                            ));
                        }
                    },
                    7 => match field {
                        ::core::option::Option::Some(PaymentMethod::Crypto(value)) => {
                            ::pilota::prost::encoding::message::merge(wire_type, value, buf, ctx)?;
                        }
                        _ => {
                            let mut owned_value = ::core::default::Default::default();
                            let value = &mut owned_value;
                            ::pilota::prost::encoding::message::merge(wire_type, value, buf, ctx)?;
                            *field =
                                ::core::option::Option::Some(PaymentMethod::Crypto(owned_value));
                        }
                    },
                    _ => unreachable!(
                        concat!("invalid ", stringify!(PaymentMethod), " tag: {}"),
                        tag
                    ),
                };
                ::core::result::Result::Ok(())
            }
        }
    }
}
