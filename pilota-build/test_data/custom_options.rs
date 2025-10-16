pub mod custom_options {
    #![allow(warnings, clippy::all)]
    use ::pilota::{Buf as _, BufMut as _};
    pub mod custom_options {
        use ::pilota::{Buf as _, BufMut as _};

        static FILE_DESCRIPTOR_BYTES_CUSTOM_OPTIONS: ::pilota::Bytes = ::pilota::Bytes::from_static(b"\n\x14custom_options.proto\x12\x0ecustom_options\x1a google/protobuf/descriptor.proto\"[\n\x11FileCustomOptions\x12\x17\n\x03key\x18\xd5\x86\x03 \x01(\tH\0R\x03key\x88\x01\x01\x12\x1b\n\x05value\x18\xd6\x86\x03 \x01(\tH\x01R\x05value\x88\x01\x01B\x06\n\x04_keyB\x08\n\x06_value\"\xfb\x01\n\x11MessageValidation\x12<\n\x13all_fields_required\x18\x01 \x01(\x08:\x05falseH\0R\x11allFieldsRequiredB\0\x88\x01\x01\x12/\n\x11max_nesting_depth\x18\x02 \x01(\x05H\x01R\x0fmaxNestingDepth\x88\x01\x01\x122\n\x12validation_message\x18\x03 \x01(\tH\x02R\x11validationMessage\x88\x01\x01B\x16\n\x14_all_fields_requiredB\x14\n\x12_max_nesting_depthB\x15\n\x13_validation_message\"\x92\x03\n\x0fDatabaseOptions\x12\"\n\ntable_name\x18\x01 \x01(\tH\0R\ttableName\x88\x01\x01\x12%\n\x06engine\x18\x02 \x01(\t:\x06InnoDBH\x01R\x06engineB\0\x88\x01\x01\x122\n\x0eauto_increment\x18\x03 \x01(\x08:\x04trueH\x02R\rautoIncrementB\0\x88\x01\x01\x12F\n\x07indexes\x18\x04 \x03(\x0b2,.custom_options.DatabaseOptions.IndexOptionsR\x07indexes\x1a\x8a\x01\n\x0cIndexOptions\x12\"\n\nindex_name\x18\x01 \x01(\tH\0R\tindexName\x88\x01\x01\x12$\n\x06unique\x18\x02 \x01(\x08:\x05falseH\x01R\x06uniqueB\0\x88\x01\x01\x12\x16\n\x06fields\x18\x03 \x03(\tR\x06fieldsB\r\n\x0b_index_nameB\t\n\x07_uniqueB\r\n\x0b_table_nameB\t\n\x07_engineB\x11\n\x0f_auto_increment\"\xa7\x04\n\x0bApiMetadata\x12\x1f\n\x08endpoint\x18\x01 \x01(\tH\0R\x08endpoint\x88\x01\x01\x12\x1b\n\x06method\x18\x02 \x01(\tH\x01R\x06method\x88\x01\x01\x12,\n\ndeprecated\x18\x03 \x01(\x08:\x05falseH\x02R\ndeprecatedB\0\x88\x01\x01\x12\x1d\n\x07version\x18\x04 \x01(\tH\x03R\x07version\x88\x01\x01\x12\x12\n\x04tags\x18\x05 \x03(\tR\x04tags\x12?\n\x08examples\x18\x06 \x03(\x0b2#.custom_options.ApiMetadata.ExampleR\x08examples28\n\x04test\x18\x9d\x8f\x03 \x01(\t\x12\x1f.google.protobuf.MessageOptionsR\x04test\x88\x01\x01\x1a\xca\x01\n\x07Example\x12\x17\n\x04name\x18\x01 \x01(\tH\0R\x04name\x88\x01\x01\x12\x1d\n\x07request\x18\x02 \x01(\tH\x01R\x07request\x88\x01\x01\x12\x1f\n\x08response\x18\x03 \x01(\tH\x02R\x08response\x88\x01\x012:\n\x05level\x18\x9e\x8f\x03 \x01(\t\x12\x1f.google.protobuf.MessageOptionsR\x05level\x88\x01\x01B\x07\n\x05_nameB\n\n\x08_requestB\x0b\n\t_response:\x08\xea\xf9\x18\x04testB\x0b\n\t_endpointB\t\n\x07_methodB\r\n\x0b_deprecatedB\n\n\x08_version\"\x85\x04\n\x04User\x12A\n\x02id\x18\x01 \x01(\x05R\x02idB1\xea\xc1\x18\x1eUnique identifier for the user\xda\xc1\x18\x07user_id\xe0\xc1\x18\x01\x129\n\x08username\x18\x02 \x01(\tR\x08userNameB\x1d\xd2\xc1\x18\x15required,min=3,max=50\xe0\xc1\x18\x01\x12e\n\x08password\x18\x03 \x01(\tR\x08passwordBI\xea\xc1\x18/User password (never returned in API responses)\xd2\xc1\x18\x0erequired,min=8\xf0\xc1\x18\x01\x12,\n\x05email\x18\x04 \x01(\tR\x05emailB\x16\xd2\xc1\x18\x0erequired,email\xe0\xc1\x18\x01\x12\x1d\n\x08role_ids\x18\x05 \x03(\x05R\x07roleIdsB\x02\x10\x01\x12!\n\ncreated_at\x18\x06 \x01(\x03R\tcreatedAtB\x020\x01\x129\n\x07profile\x18\x07 \x01(\x0b2\x1b.custom_options.UserProfileR\x07profileB\x02(\x01\x12\x1f\n\told_field\x18\x08 \x01(\tR\x08oldFieldB\x02\x18\x01:L\x18\0\xb8\xbb\x18\x90\x1c\xea\xf9\x18\x0fApiMetadatatest\xc2\xbb\x18\x1c\x08\x01\x10\x03\x1a\x16User validation failed\xaa\xbb\x18\x05users\xf2\xf9\x18\x012\xb0\xbb\x18\x01\"[\n\x0bUserProfile\x12\x1b\n\tfull_name\x18\x01 \x01(\tR\x08fullName\x12\x1d\n\navatar_url\x18\x02 \x01(\tR\tavatarUrl\x12\x10\n\x03bio\x18\x03 \x01(\tR\x03bio\"\xa4\x03\n\x0bPaymentInfo\x12\x0e\n\x02id\x18\x01 \x01(\x05R\x02id\x12\x17\n\x07user_id\x18\x02 \x01(\tR\x06userId\x12\x16\n\x06amount\x18\x03 \x01(\x01R\x06amount\x12A\n\x0bcredit_card\x18\x04 \x01(\x0b2\x1e.custom_options.CreditCardInfoH\0R\ncreditCard\x12>\n\ndebit_card\x18\x05 \x01(\x0b2\x1d.custom_options.DebitCardInfoH\0R\tdebitCard\x12G\n\rbank_transfer\x18\x06 \x01(\x0b2 .custom_options.BankTransferInfoH\0R\x0cbankTransfer\x12<\n\x06crypto\x18\x07 \x01(\x0b2\".custom_options.CryptoCurrencyInfoH\0R\x06cryptoBJ\n\x0epayment_method\x128\xea\xe0\x180Payment method details, only one can be selected\xf0\xe0\x18\x01\"\x89\x01\n\x0eCreditCardInfo\x12%\n\x0bcard_number\x18\x01 \x01(\tR\ncardNumberB\x04\xf0\xc1\x18\x01\x12'\n\x0fcardholder_name\x18\x02 \x01(\tR\x0ecardholderName\x12'\n\x0fexpiration_date\x18\x03 \x01(\tR\x0eexpirationDate\"t\n\rDebitCardInfo\x12%\n\x0bcard_number\x18\x01 \x01(\tR\ncardNumberB\x04\xf0\xc1\x18\x01\x12\x1f\n\x0bholder_name\x18\x02 \x01(\tR\nholderName\x12\x1b\n\tbank_name\x18\x03 \x01(\tR\x08bankName\"{\n\x10BankTransferInfo\x12+\n\x0eaccount_number\x18\x01 \x01(\tR\raccountNumberB\x04\xf0\xc1\x18\x01\x12\x1b\n\tbank_name\x18\x02 \x01(\tR\x08bankName\x12\x1d\n\nswift_code\x18\x03 \x01(\tR\tswiftCode\"`\n\x12CryptoCurrencyInfo\x12%\n\x0ewallet_address\x18\x01 \x01(\tR\rwalletAddress\x12#\n\rcurrency_type\x18\x02 \x01(\tR\x0ccurrencyType\" \n\x0eGetUserRequest\x12\x0e\n\x02id\x18\x01 \x01(\tR\x02id\";\n\x0fGetUserResponse\x12(\n\x04user\x18\x01 \x01(\x0b2\x14.custom_options.UserR\x04user\"=\n\x11CreateUserRequest\x12(\n\x04user\x18\x01 \x01(\x0b2\x14.custom_options.UserR\x04user\"X\n\x12CreateUserResponse\x12(\n\x04user\x18\x01 \x01(\x0b2\x14.custom_options.UserR\x04user\x12\x18\n\x07message\x18\x02 \x01(\tR\x07message\"M\n\x11UpdateUserRequest\x12\x0e\n\x02id\x18\x01 \x01(\tR\x02id\x12(\n\x04user\x18\x02 \x01(\x0b2\x14.custom_options.UserR\x04user\"X\n\x12UpdateUserResponse\x12(\n\x04user\x18\x01 \x01(\x0b2\x14.custom_options.UserR\x04user\x12\x18\n\x07message\x18\x02 \x01(\tR\x07message\"#\n\x11DeleteUserRequest\x12\x0e\n\x02id\x18\x01 \x01(\tR\x02id\"H\n\x12DeleteUserResponse\x12\x18\n\x07success\x18\x01 \x01(\x08R\x07success\x12\x18\n\x07message\x18\x02 \x01(\tR\x07message\")\n\x17GetOldUserFormatRequest\x12\x0e\n\x02id\x18\x01 \x01(\tR\x02id\"H\n\x18GetOldUserFormatResponse\x12(\n\x04user\x18\x01 \x01(\x0b2\x14.custom_options.UserR\x04user:\x02\x18\x01*\xbf\x02\n\nUserStatus\x12#\n\x07UNKNOWN\x10\0\x1a\x16\x8a\xce\x18\x07Unknown\x9a\xce\x18\x07#999999\x12%\n\x06ACTIVE\x10\x01\x1a\x19\x8a\xce\x18\x06Active\x90\xce\x18\x01\x9a\xce\x18\x07#00FF00\x12'\n\x07ENABLED\x10\x01\x1a\x1a\x8a\xce\x18\x07Enabled\x90\xce\x18\x01\x9a\xce\x18\x07#00FF00\x12)\n\x08INACTIVE\x10\x02\x1a\x1b\x8a\xce\x18\x08Inactive\x90\xce\x18\0\x9a\xce\x18\x07#FF0000\x12+\n\tSUSPENDED\x10\x03\x1a\x1c\x8a\xce\x18\tSuspended\x90\xce\x18\0\x9a\xce\x18\x07#FFA500\x12)\n\x07DELETED\x10\x04\x1a\x1c\x08\x01\x8a\xce\x18\x07Deleted\x90\xce\x18\0\x9a\xce\x18\x07#000000\x1a9\x10\x01\xea\xc7\x18/Represents the current status of a user account\xf0\xc7\x18\0*\x8f\x01\n\x0bPaymentType\x12\x13\n\x0fPAYMENT_UNKNOWN\x10\0\x12\x12\n\x0ePAYMENT_CREDIT\x10\x01\x12\x11\n\rPAYMENT_DEBIT\x10\x02\x12\x19\n\x15PAYMENT_BANK_TRANSFER\x10\x03\x12\x12\n\x0ePAYMENT_CRYPTO\x10\x04\x12\x15\n\rPAYMENT_CHECK\x10\x05\x1a\x02\x08\x012\xee\x05\n\x0bUserService\x12\x8c\x01\n\x07GetUser\x12\x1e.custom_options.GetUserRequest\x1a\x1f.custom_options.GetUserResponse\"@\x90\x02\x01\xea\xda\x18\nusers.read\xc8\xda\x18\x01\xda\xda\x18\x12/api/v1/users/{id}\xd2\xda\x18\n100/minute\xe2\xda\x18\x03GET\x12\x92\x01\n\nCreateUser\x12!.custom_options.CreateUserRequest\x1a\".custom_options.CreateUserResponse\"=\x90\x02\x02\xea\xda\x18\x0cusers.create\xc8\xda\x18\x01\xda\xda\x18\r/api/v1/users\xd2\xda\x18\t10/minute\xe2\xda\x18\x04POST\x12\x89\x01\n\nUpdateUser\x12!.custom_options.UpdateUserRequest\x1a\".custom_options.UpdateUserResponse\"4\x90\x02\x02\xc8\xda\x18\x01\xda\xda\x18\x12/api/v1/users/{id}\xe2\xda\x18\x03PUT\xea\xda\x18\x0cusers.update\x12\x89\x01\n\nDeleteUser\x12!.custom_options.DeleteUserRequest\x1a\".custom_options.DeleteUserResponse\"4\xc8\xda\x18\x01\xda\xda\x18\x12/api/v1/users/{id}\xe2\xda\x18\x06DELETE\xea\xda\x18\x0cusers.delete\x12\x84\x01\n\x10GetOldUserFormat\x12'.custom_options.GetOldUserFormatRequest\x1a(.custom_options.GetOldUserFormatResponse\"\x1d\x88\x02\x01\xda\xda\x18\x16/api/v1/users/old/{id}\x1a\x1c\x88\x02\0\xaa\xd4\x18\x02v1\xb0\xd4\x18\x01\xba\xd4\x18\x0b1000/minute:D\n\x0cfile_version\x18\xd1\x86\x03 \x01(\x04\x12\x1c.google.protobuf.FileOptionsR\x0bfileVersion\x88\x01\x01:B\n\x0bfile_author\x18\xd2\x86\x03 \x01(\t\x12\x1c.google.protobuf.FileOptionsR\nfileAuthor\x88\x01\x01:J\n\x0ffile_department\x18\xd3\x86\x03 \x01(\t\x12\x1c.google.protobuf.FileOptionsR\x0efileDepartment\x88\x01\x01:M\n\x0cinternal_api\x18\xd4\x86\x03 \x01(\x08\x12\x1c.google.protobuf.FileOptions:\x05falseR\x0binternalApiB\0\x88\x01\x01:]\n\x07file_kv\x18\xd5\x86\x03 \x01(\x0b2!.custom_options.FileCustomOptions\x12\x1c.google.protobuf.FileOptionsR\x06fileKv\x88\x01\x01:?\n\x08db_table\x18\xb5\x87\x03 \x01(\t\x12\x1f.google.protobuf.MessageOptionsR\x07dbTable\x88\x01\x01:J\n\tdb_entity\x18\xb6\x87\x03 \x01(\x08\x12\x1f.google.protobuf.MessageOptions:\x05falseR\x08dbEntityB\0\x88\x01\x01:P\n\x11cache_ttl_seconds\x18\xb7\x87\x03 \x01(\x05\x12\x1f.google.protobuf.MessageOptionsR\x0fcacheTtlSeconds\x88\x01\x01:c\n\x08validate\x18\xb8\x87\x03 \x01(\x0b2!.custom_options.MessageValidation\x12\x1f.google.protobuf.MessageOptionsR\x08validate\x88\x01\x01:I\n\tsensitive\x18\x9e\x88\x03 \x01(\x08\x12\x1d.google.protobuf.FieldOptions:\x05falseR\tsensitiveB\0\x88\x01\x01:B\n\nvalidation\x18\x9a\x88\x03 \x01(\t\x12\x1d.google.protobuf.FieldOptionsR\nvalidation\x88\x01\x01:?\n\tdb_column\x18\x9b\x88\x03 \x01(\t\x12\x1d.google.protobuf.FieldOptionsR\x08dbColumn\x88\x01\x01:F\n\x08db_index\x18\x9c\x88\x03 \x01(\x08\x12\x1d.google.protobuf.FieldOptions:\x05falseR\x07dbIndexB\0\x88\x01\x01:;\n\x07api_doc\x18\x9d\x88\x03 \x01(\t\x12\x1d.google.protobuf.FieldOptionsR\x06apiDoc\x88\x01\x01:L\n\x10enum_description\x18\xfd\x88\x03 \x01(\t\x12\x1c.google.protobuf.EnumOptionsR\x0fenumDescription\x88\x01\x01:K\n\x0bis_internal\x18\xfe\x88\x03 \x01(\x08\x12\x1c.google.protobuf.EnumOptions:\x05falseR\nisInternalB\0\x88\x01\x01:I\n\x0cdisplay_name\x18\xe1\x89\x03 \x01(\t\x12!.google.protobuf.EnumValueOptionsR\x0bdisplayName\x88\x01\x01:N\n\x0caccess_level\x18\xe2\x89\x03 \x01(\x05\x12!.google.protobuf.EnumValueOptions:\x010R\x0baccessLevelB\0\x88\x01\x01:<\n\x05color\x18\xe3\x89\x03 \x01(\t\x12!.google.protobuf.EnumValueOptionsR\x05color\x88\x01\x01:M\n\x0fservice_version\x18\xc5\x8a\x03 \x01(\t\x12\x1f.google.protobuf.ServiceOptionsR\x0eserviceVersion\x88\x01\x01:P\n\x0crequire_auth\x18\xc6\x8a\x03 \x01(\x08\x12\x1f.google.protobuf.ServiceOptions:\x05falseR\x0brequireAuthB\0\x88\x01\x01:C\n\nrate_limit\x18\xc7\x8a\x03 \x01(\t\x12\x1f.google.protobuf.ServiceOptionsR\trateLimit\x88\x01\x01:\\\n\x13method_require_auth\x18\xa9\x8b\x03 \x01(\x08\x12\x1e.google.protobuf.MethodOptions:\x05falseR\x11methodRequireAuthB\0\x88\x01\x01:O\n\x11method_rate_limit\x18\xaa\x8b\x03 \x01(\t\x12\x1e.google.protobuf.MethodOptionsR\x0fmethodRateLimit\x88\x01\x01:?\n\x08endpoint\x18\xab\x8b\x03 \x01(\t\x12\x1e.google.protobuf.MethodOptionsR\x08endpoint\x88\x01\x01:K\n\x0bhttp_method\x18\xac\x8b\x03 \x01(\t\x12\x1e.google.protobuf.MethodOptions:\x03GETR\nhttpMethodB\0\x88\x01\x01:C\n\npermission\x18\xad\x8b\x03 \x01(\t\x12\x1e.google.protobuf.MethodOptionsR\npermission\x88\x01\x01:O\n\x11oneof_description\x18\x8d\x8c\x03 \x01(\t\x12\x1d.google.protobuf.OneofOptionsR\x10oneofDescription\x88\x01\x01:H\n\texclusive\x18\x8e\x8c\x03 \x01(\x08\x12\x1d.google.protobuf.OneofOptions:\x04trueR\texclusiveB\0\x88\x01\x01B\xee\x01\n\x1bcom.example.proto.referenceB\x15OptionsReferenceProtoP\x01H\x01Z\"github.com/example/proto/reference\xf8\x01\x01\xa2\x02\x03EPR\xaa\x02\x17Example.Proto.Reference\xca\x02\x17Example\\Proto\\Reference\xea\x02\x19Example::Proto::Reference\xa0\xb5\x18\0\x88\xb5\x18\xc2\x81\xd4\t\x9a\xb5\x18\x04arch\xaa\xb5\x18\x18\xaa\xb5\x18\x08file_key\xb2\xb5\x18\x08file_val\x92\xb5\x18\x06giggleb\x06proto3");
        static FILE_DESCRIPTOR_PROTO_CUSTOM_OPTIONS: ::std::sync::LazyLock<
            ::pilota::pb::descriptor::FileDescriptorProto,
        > = ::std::sync::LazyLock::new(|| {
            let data: &[u8] = FILE_DESCRIPTOR_BYTES_CUSTOM_OPTIONS.as_ref();
            ::pilota::pb::PbMessage::parse_from_bytes(data)
                .expect("Failed to decode file descriptor")
        });
        pub fn file_descriptor_proto_custom_options()
        -> &'static ::pilota::pb::descriptor::FileDescriptorProto {
            &*FILE_DESCRIPTOR_PROTO_CUSTOM_OPTIONS
        }

        static FILE_DESCRIPTOR_CUSTOM_OPTIONS: ::std::sync::LazyLock<
            ::pilota::pb::reflect::FileDescriptor,
        > = ::std::sync::LazyLock::new(|| {
            let mut deps = ::std::vec::Vec::new();
            deps.push(::pilota::pb::descriptor::file_descriptor().clone());

            ::pilota::pb::reflect::FileDescriptor::new_dynamic(
                file_descriptor_proto_custom_options().clone(),
                &deps,
            )
            .expect("Failed to build dynamic FileDescriptor")
        });

        pub fn file_descriptor_custom_options() -> &'static ::pilota::pb::reflect::FileDescriptor {
            &*FILE_DESCRIPTOR_CUSTOM_OPTIONS
        }
        pub mod exts_custom_options {
            pub const file_version: ::pilota::pb::extension::CustomExtField<
                ::pilota::pb::descriptor::FileOptions,
                ::pilota::pb::extension::UInt64OptionValueExtractor,
            > = ::pilota::pb::extension::CustomExtField::new(50001);
            pub const file_author: ::pilota::pb::extension::CustomExtField<
                ::pilota::pb::descriptor::FileOptions,
                ::pilota::pb::extension::StrOptionValueExtractor,
            > = ::pilota::pb::extension::CustomExtField::new(50002);
            pub const file_department: ::pilota::pb::extension::CustomExtField<
                ::pilota::pb::descriptor::FileOptions,
                ::pilota::pb::extension::StrOptionValueExtractor,
            > = ::pilota::pb::extension::CustomExtField::new(50003);
            pub const internal_api: ::pilota::pb::extension::CustomExtField<
                ::pilota::pb::descriptor::FileOptions,
                ::pilota::pb::extension::BoolOptionValueExtractor,
            > = ::pilota::pb::extension::CustomExtField::new(50004);
            pub const file_kv: ::pilota::pb::extension::CustomExtField<
                ::pilota::pb::descriptor::FileOptions,
                ::pilota::pb::extension::MessageOptionValueExtractor<super::FileCustomOptions>,
            > = ::pilota::pb::extension::CustomExtField::new(50005);
            pub const db_table: ::pilota::pb::extension::CustomExtField<
                ::pilota::pb::descriptor::MessageOptions,
                ::pilota::pb::extension::StrOptionValueExtractor,
            > = ::pilota::pb::extension::CustomExtField::new(50101);
            pub const db_entity: ::pilota::pb::extension::CustomExtField<
                ::pilota::pb::descriptor::MessageOptions,
                ::pilota::pb::extension::BoolOptionValueExtractor,
            > = ::pilota::pb::extension::CustomExtField::new(50102);
            pub const cache_ttl_seconds: ::pilota::pb::extension::CustomExtField<
                ::pilota::pb::descriptor::MessageOptions,
                ::pilota::pb::extension::Int32OptionValueExtractor,
            > = ::pilota::pb::extension::CustomExtField::new(50103);
            pub const validate: ::pilota::pb::extension::CustomExtField<
                ::pilota::pb::descriptor::MessageOptions,
                ::pilota::pb::extension::MessageOptionValueExtractor<super::MessageValidation>,
            > = ::pilota::pb::extension::CustomExtField::new(50104);
            pub const sensitive: ::pilota::pb::extension::CustomExtField<
                ::pilota::pb::descriptor::FieldOptions,
                ::pilota::pb::extension::BoolOptionValueExtractor,
            > = ::pilota::pb::extension::CustomExtField::new(50206);
            pub const validation: ::pilota::pb::extension::CustomExtField<
                ::pilota::pb::descriptor::FieldOptions,
                ::pilota::pb::extension::StrOptionValueExtractor,
            > = ::pilota::pb::extension::CustomExtField::new(50202);
            pub const db_column: ::pilota::pb::extension::CustomExtField<
                ::pilota::pb::descriptor::FieldOptions,
                ::pilota::pb::extension::StrOptionValueExtractor,
            > = ::pilota::pb::extension::CustomExtField::new(50203);
            pub const db_index: ::pilota::pb::extension::CustomExtField<
                ::pilota::pb::descriptor::FieldOptions,
                ::pilota::pb::extension::BoolOptionValueExtractor,
            > = ::pilota::pb::extension::CustomExtField::new(50204);
            pub const api_doc: ::pilota::pb::extension::CustomExtField<
                ::pilota::pb::descriptor::FieldOptions,
                ::pilota::pb::extension::StrOptionValueExtractor,
            > = ::pilota::pb::extension::CustomExtField::new(50205);
            pub const enum_description: ::pilota::pb::extension::CustomExtField<
                ::pilota::pb::descriptor::EnumOptions,
                ::pilota::pb::extension::StrOptionValueExtractor,
            > = ::pilota::pb::extension::CustomExtField::new(50301);
            pub const is_internal: ::pilota::pb::extension::CustomExtField<
                ::pilota::pb::descriptor::EnumOptions,
                ::pilota::pb::extension::BoolOptionValueExtractor,
            > = ::pilota::pb::extension::CustomExtField::new(50302);
            pub const display_name: ::pilota::pb::extension::CustomExtField<
                ::pilota::pb::descriptor::EnumValueOptions,
                ::pilota::pb::extension::StrOptionValueExtractor,
            > = ::pilota::pb::extension::CustomExtField::new(50401);
            pub const access_level: ::pilota::pb::extension::CustomExtField<
                ::pilota::pb::descriptor::EnumValueOptions,
                ::pilota::pb::extension::Int32OptionValueExtractor,
            > = ::pilota::pb::extension::CustomExtField::new(50402);
            pub const color: ::pilota::pb::extension::CustomExtField<
                ::pilota::pb::descriptor::EnumValueOptions,
                ::pilota::pb::extension::StrOptionValueExtractor,
            > = ::pilota::pb::extension::CustomExtField::new(50403);
            pub const service_version: ::pilota::pb::extension::CustomExtField<
                ::pilota::pb::descriptor::ServiceOptions,
                ::pilota::pb::extension::StrOptionValueExtractor,
            > = ::pilota::pb::extension::CustomExtField::new(50501);
            pub const require_auth: ::pilota::pb::extension::CustomExtField<
                ::pilota::pb::descriptor::ServiceOptions,
                ::pilota::pb::extension::BoolOptionValueExtractor,
            > = ::pilota::pb::extension::CustomExtField::new(50502);
            pub const rate_limit: ::pilota::pb::extension::CustomExtField<
                ::pilota::pb::descriptor::ServiceOptions,
                ::pilota::pb::extension::StrOptionValueExtractor,
            > = ::pilota::pb::extension::CustomExtField::new(50503);
            pub const method_require_auth: ::pilota::pb::extension::CustomExtField<
                ::pilota::pb::descriptor::MethodOptions,
                ::pilota::pb::extension::BoolOptionValueExtractor,
            > = ::pilota::pb::extension::CustomExtField::new(50601);
            pub const method_rate_limit: ::pilota::pb::extension::CustomExtField<
                ::pilota::pb::descriptor::MethodOptions,
                ::pilota::pb::extension::StrOptionValueExtractor,
            > = ::pilota::pb::extension::CustomExtField::new(50602);
            pub const endpoint: ::pilota::pb::extension::CustomExtField<
                ::pilota::pb::descriptor::MethodOptions,
                ::pilota::pb::extension::StrOptionValueExtractor,
            > = ::pilota::pb::extension::CustomExtField::new(50603);
            pub const http_method: ::pilota::pb::extension::CustomExtField<
                ::pilota::pb::descriptor::MethodOptions,
                ::pilota::pb::extension::StrOptionValueExtractor,
            > = ::pilota::pb::extension::CustomExtField::new(50604);
            pub const permission: ::pilota::pb::extension::CustomExtField<
                ::pilota::pb::descriptor::MethodOptions,
                ::pilota::pb::extension::StrOptionValueExtractor,
            > = ::pilota::pb::extension::CustomExtField::new(50605);
            pub const oneof_description: ::pilota::pb::extension::CustomExtField<
                ::pilota::pb::descriptor::OneofOptions,
                ::pilota::pb::extension::StrOptionValueExtractor,
            > = ::pilota::pb::extension::CustomExtField::new(50701);
            pub const exclusive: ::pilota::pb::extension::CustomExtField<
                ::pilota::pb::descriptor::OneofOptions,
                ::pilota::pb::extension::BoolOptionValueExtractor,
            > = ::pilota::pb::extension::CustomExtField::new(50702);
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        #[deprecated]
        pub struct GetOldUserFormatResponse {
            pub user: ::std::option::Option<User>,
        }
        impl ::pilota::pb::Message for GetOldUserFormatResponse {
            #[inline]
            fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
                0 + self.user.as_ref().map_or(0, |msg| {
                    ::pilota::pb::encoding::message::encoded_len(ctx, 1, msg)
                })
            }

            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                if let Some(_pilota_inner_value) = self.user.as_ref() {
                    ::pilota::pb::encoding::message::encode(1, _pilota_inner_value, buf);
                }
            }

            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::pilota::pb::encoding::WireType,
                buf: &mut ::pilota::Bytes,
                ctx: &mut ::pilota::pb::encoding::DecodeContext,
                is_root: bool,
            ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
                const STRUCT_NAME: &'static str = stringify!(GetOldUserFormatResponse);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.user;
                        ::pilota::pb::encoding::message::merge(
                            wire_type,
                            _inner_pilota_value
                                .get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(user));
                            error
                        })
                    }
                    _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct UserProfile {
            pub full_name: ::pilota::FastStr,

            pub avatar_url: ::pilota::FastStr,

            pub bio: ::pilota::FastStr,
        }
        impl ::pilota::pb::Message for UserProfile {
            #[inline]
            fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
                0 + ::pilota::pb::encoding::faststr::encoded_len(ctx, 1, &self.full_name)
                    + ::pilota::pb::encoding::faststr::encoded_len(ctx, 2, &self.avatar_url)
                    + ::pilota::pb::encoding::faststr::encoded_len(ctx, 3, &self.bio)
            }

            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                ::pilota::pb::encoding::faststr::encode(1, &self.full_name, buf);
                ::pilota::pb::encoding::faststr::encode(2, &self.avatar_url, buf);
                ::pilota::pb::encoding::faststr::encode(3, &self.bio, buf);
            }

            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::pilota::pb::encoding::WireType,
                buf: &mut ::pilota::Bytes,
                ctx: &mut ::pilota::pb::encoding::DecodeContext,
                is_root: bool,
            ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
                const STRUCT_NAME: &'static str = stringify!(UserProfile);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.full_name;
                        ::pilota::pb::encoding::faststr::merge(
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
                        ::pilota::pb::encoding::faststr::merge(
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
                        ::pilota::pb::encoding::faststr::merge(
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
                    _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct FileCustomOptions {
            pub key: ::std::option::Option<::pilota::FastStr>,

            pub value: ::std::option::Option<::pilota::FastStr>,
        }
        impl ::pilota::pb::Message for FileCustomOptions {
            #[inline]
            fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
                0 + self.key.as_ref().map_or(0, |value| {
                    ::pilota::pb::encoding::faststr::encoded_len(ctx, 50005, value)
                }) + self.value.as_ref().map_or(0, |value| {
                    ::pilota::pb::encoding::faststr::encoded_len(ctx, 50006, value)
                })
            }

            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                if let Some(_pilota_inner_value) = self.key.as_ref() {
                    ::pilota::pb::encoding::faststr::encode(50005, _pilota_inner_value, buf);
                };
                if let Some(_pilota_inner_value) = self.value.as_ref() {
                    ::pilota::pb::encoding::faststr::encode(50006, _pilota_inner_value, buf);
                };
            }

            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::pilota::pb::encoding::WireType,
                buf: &mut ::pilota::Bytes,
                ctx: &mut ::pilota::pb::encoding::DecodeContext,
                is_root: bool,
            ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
                const STRUCT_NAME: &'static str = stringify!(FileCustomOptions);

                match tag {
                    50005 => {
                        let mut _inner_pilota_value = &mut self.key;
                        ::pilota::pb::encoding::faststr::merge(
                            wire_type,
                            _inner_pilota_value
                                .get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(key));
                            error
                        })
                    }
                    50006 => {
                        let mut _inner_pilota_value = &mut self.value;
                        ::pilota::pb::encoding::faststr::merge(
                            wire_type,
                            _inner_pilota_value
                                .get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(value));
                            error
                        })
                    }
                    _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct UpdateUserResponse {
            pub user: ::std::option::Option<User>,

            pub message: ::pilota::FastStr,
        }
        impl ::pilota::pb::Message for UpdateUserResponse {
            #[inline]
            fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
                0 + self.user.as_ref().map_or(0, |msg| {
                    ::pilota::pb::encoding::message::encoded_len(ctx, 1, msg)
                }) + ::pilota::pb::encoding::faststr::encoded_len(ctx, 2, &self.message)
            }

            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                if let Some(_pilota_inner_value) = self.user.as_ref() {
                    ::pilota::pb::encoding::message::encode(1, _pilota_inner_value, buf);
                }
                ::pilota::pb::encoding::faststr::encode(2, &self.message, buf);
            }

            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::pilota::pb::encoding::WireType,
                buf: &mut ::pilota::Bytes,
                ctx: &mut ::pilota::pb::encoding::DecodeContext,
                is_root: bool,
            ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
                const STRUCT_NAME: &'static str = stringify!(UpdateUserResponse);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.user;
                        ::pilota::pb::encoding::message::merge(
                            wire_type,
                            _inner_pilota_value
                                .get_or_insert_with(::core::default::Default::default),
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
                        ::pilota::pb::encoding::faststr::merge(
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
                    _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct CreateUserRequest {
            pub user: ::std::option::Option<User>,
        }
        impl ::pilota::pb::Message for CreateUserRequest {
            #[inline]
            fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
                0 + self.user.as_ref().map_or(0, |msg| {
                    ::pilota::pb::encoding::message::encoded_len(ctx, 1, msg)
                })
            }

            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                if let Some(_pilota_inner_value) = self.user.as_ref() {
                    ::pilota::pb::encoding::message::encode(1, _pilota_inner_value, buf);
                }
            }

            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::pilota::pb::encoding::WireType,
                buf: &mut ::pilota::Bytes,
                ctx: &mut ::pilota::pb::encoding::DecodeContext,
                is_root: bool,
            ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
                const STRUCT_NAME: &'static str = stringify!(CreateUserRequest);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.user;
                        ::pilota::pb::encoding::message::merge(
                            wire_type,
                            _inner_pilota_value
                                .get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(user));
                            error
                        })
                    }
                    _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct GetOldUserFormatRequest {
            pub id: ::pilota::FastStr,
        }
        impl ::pilota::pb::Message for GetOldUserFormatRequest {
            #[inline]
            fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
                0 + ::pilota::pb::encoding::faststr::encoded_len(ctx, 1, &self.id)
            }

            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                ::pilota::pb::encoding::faststr::encode(1, &self.id, buf);
            }

            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::pilota::pb::encoding::WireType,
                buf: &mut ::pilota::Bytes,
                ctx: &mut ::pilota::pb::encoding::DecodeContext,
                is_root: bool,
            ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
                const STRUCT_NAME: &'static str = stringify!(GetOldUserFormatRequest);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.id;
                        ::pilota::pb::encoding::faststr::merge(
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
                    _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
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

            #[deprecated]
            pub old_field: ::pilota::FastStr,
        }
        impl ::pilota::pb::Message for User {
            #[inline]
            fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
                0 + ::pilota::pb::encoding::int32::encoded_len(ctx, 1, &self.id)
                    + ::pilota::pb::encoding::faststr::encoded_len(ctx, 2, &self.username)
                    + ::pilota::pb::encoding::faststr::encoded_len(ctx, 3, &self.password)
                    + ::pilota::pb::encoding::faststr::encoded_len(ctx, 4, &self.email)
                    + ::pilota::pb::encoding::int32::encoded_len_repeated(ctx, 5, &self.role_ids)
                    + ::pilota::pb::encoding::int64::encoded_len(ctx, 6, &self.created_at)
                    + self.profile.as_ref().map_or(0, |msg| {
                        ::pilota::pb::encoding::message::encoded_len(ctx, 7, msg)
                    })
                    + ::pilota::pb::encoding::faststr::encoded_len(ctx, 8, &self.old_field)
            }

            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                ::pilota::pb::encoding::int32::encode(1, &self.id, buf);
                ::pilota::pb::encoding::faststr::encode(2, &self.username, buf);
                ::pilota::pb::encoding::faststr::encode(3, &self.password, buf);
                ::pilota::pb::encoding::faststr::encode(4, &self.email, buf);
                ::pilota::pb::encoding::int32::encode_repeated(5, &self.role_ids, buf);
                ::pilota::pb::encoding::int64::encode(6, &self.created_at, buf);
                if let Some(_pilota_inner_value) = self.profile.as_ref() {
                    ::pilota::pb::encoding::message::encode(7, _pilota_inner_value, buf);
                }
                ::pilota::pb::encoding::faststr::encode(8, &self.old_field, buf);
            }

            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::pilota::pb::encoding::WireType,
                buf: &mut ::pilota::Bytes,
                ctx: &mut ::pilota::pb::encoding::DecodeContext,
                is_root: bool,
            ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
                const STRUCT_NAME: &'static str = stringify!(User);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.id;
                        ::pilota::pb::encoding::int32::merge(
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
                        ::pilota::pb::encoding::faststr::merge(
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
                        ::pilota::pb::encoding::faststr::merge(
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
                        ::pilota::pb::encoding::faststr::merge(
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
                        ::pilota::pb::encoding::int32::merge_repeated(
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
                        ::pilota::pb::encoding::int64::merge(
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
                        ::pilota::pb::encoding::message::merge(
                            wire_type,
                            _inner_pilota_value
                                .get_or_insert_with(::core::default::Default::default),
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
                        ::pilota::pb::encoding::faststr::merge(
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
                    _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct DeleteUserResponse {
            pub success: bool,

            pub message: ::pilota::FastStr,
        }
        impl ::pilota::pb::Message for DeleteUserResponse {
            #[inline]
            fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
                0 + ::pilota::pb::encoding::bool::encoded_len(ctx, 1, &self.success)
                    + ::pilota::pb::encoding::faststr::encoded_len(ctx, 2, &self.message)
            }

            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                ::pilota::pb::encoding::bool::encode(1, &self.success, buf);
                ::pilota::pb::encoding::faststr::encode(2, &self.message, buf);
            }

            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::pilota::pb::encoding::WireType,
                buf: &mut ::pilota::Bytes,
                ctx: &mut ::pilota::pb::encoding::DecodeContext,
                is_root: bool,
            ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
                const STRUCT_NAME: &'static str = stringify!(DeleteUserResponse);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.success;
                        ::pilota::pb::encoding::bool::merge(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(success));
                            error
                        })
                    }
                    2 => {
                        let mut _inner_pilota_value = &mut self.message;
                        ::pilota::pb::encoding::faststr::merge(
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
                    _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct UpdateUserRequest {
            pub id: ::pilota::FastStr,

            pub user: ::std::option::Option<User>,
        }
        impl ::pilota::pb::Message for UpdateUserRequest {
            #[inline]
            fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
                0 + ::pilota::pb::encoding::faststr::encoded_len(ctx, 1, &self.id)
                    + self.user.as_ref().map_or(0, |msg| {
                        ::pilota::pb::encoding::message::encoded_len(ctx, 2, msg)
                    })
            }

            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                ::pilota::pb::encoding::faststr::encode(1, &self.id, buf);
                if let Some(_pilota_inner_value) = self.user.as_ref() {
                    ::pilota::pb::encoding::message::encode(2, _pilota_inner_value, buf);
                }
            }

            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::pilota::pb::encoding::WireType,
                buf: &mut ::pilota::Bytes,
                ctx: &mut ::pilota::pb::encoding::DecodeContext,
                is_root: bool,
            ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
                const STRUCT_NAME: &'static str = stringify!(UpdateUserRequest);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.id;
                        ::pilota::pb::encoding::faststr::merge(
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
                        ::pilota::pb::encoding::message::merge(
                            wire_type,
                            _inner_pilota_value
                                .get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(user));
                            error
                        })
                    }
                    _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct GetUserResponse {
            pub user: ::std::option::Option<User>,
        }
        impl ::pilota::pb::Message for GetUserResponse {
            #[inline]
            fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
                0 + self.user.as_ref().map_or(0, |msg| {
                    ::pilota::pb::encoding::message::encoded_len(ctx, 1, msg)
                })
            }

            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                if let Some(_pilota_inner_value) = self.user.as_ref() {
                    ::pilota::pb::encoding::message::encode(1, _pilota_inner_value, buf);
                }
            }

            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::pilota::pb::encoding::WireType,
                buf: &mut ::pilota::Bytes,
                ctx: &mut ::pilota::pb::encoding::DecodeContext,
                is_root: bool,
            ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
                const STRUCT_NAME: &'static str = stringify!(GetUserResponse);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.user;
                        ::pilota::pb::encoding::message::merge(
                            wire_type,
                            _inner_pilota_value
                                .get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(user));
                            error
                        })
                    }
                    _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }

        pub trait UserService {}
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct MessageValidation {
            pub all_fields_required: ::std::option::Option<bool>,

            pub max_nesting_depth: ::std::option::Option<i32>,

            pub validation_message: ::std::option::Option<::pilota::FastStr>,
        }
        impl ::pilota::pb::Message for MessageValidation {
            #[inline]
            fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
                0 + self.all_fields_required.as_ref().map_or(0, |value| {
                    ::pilota::pb::encoding::bool::encoded_len(ctx, 1, value)
                }) + self.max_nesting_depth.as_ref().map_or(0, |value| {
                    ::pilota::pb::encoding::int32::encoded_len(ctx, 2, value)
                }) + self.validation_message.as_ref().map_or(0, |value| {
                    ::pilota::pb::encoding::faststr::encoded_len(ctx, 3, value)
                })
            }

            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                if let Some(_pilota_inner_value) = self.all_fields_required.as_ref() {
                    ::pilota::pb::encoding::bool::encode(1, _pilota_inner_value, buf);
                };
                if let Some(_pilota_inner_value) = self.max_nesting_depth.as_ref() {
                    ::pilota::pb::encoding::int32::encode(2, _pilota_inner_value, buf);
                };
                if let Some(_pilota_inner_value) = self.validation_message.as_ref() {
                    ::pilota::pb::encoding::faststr::encode(3, _pilota_inner_value, buf);
                };
            }

            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::pilota::pb::encoding::WireType,
                buf: &mut ::pilota::Bytes,
                ctx: &mut ::pilota::pb::encoding::DecodeContext,
                is_root: bool,
            ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
                const STRUCT_NAME: &'static str = stringify!(MessageValidation);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.all_fields_required;
                        ::pilota::pb::encoding::bool::merge(
                            wire_type,
                            _inner_pilota_value
                                .get_or_insert_with(::core::default::Default::default),
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
                        ::pilota::pb::encoding::int32::merge(
                            wire_type,
                            _inner_pilota_value
                                .get_or_insert_with(::core::default::Default::default),
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
                        ::pilota::pb::encoding::faststr::merge(
                            wire_type,
                            _inner_pilota_value
                                .get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(validation_message));
                            error
                        })
                    }
                    _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct DeleteUserRequest {
            pub id: ::pilota::FastStr,
        }
        impl ::pilota::pb::Message for DeleteUserRequest {
            #[inline]
            fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
                0 + ::pilota::pb::encoding::faststr::encoded_len(ctx, 1, &self.id)
            }

            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                ::pilota::pb::encoding::faststr::encode(1, &self.id, buf);
            }

            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::pilota::pb::encoding::WireType,
                buf: &mut ::pilota::Bytes,
                ctx: &mut ::pilota::pb::encoding::DecodeContext,
                is_root: bool,
            ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
                const STRUCT_NAME: &'static str = stringify!(DeleteUserRequest);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.id;
                        ::pilota::pb::encoding::faststr::merge(
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
                    _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct CreateUserResponse {
            pub user: ::std::option::Option<User>,

            pub message: ::pilota::FastStr,
        }
        impl ::pilota::pb::Message for CreateUserResponse {
            #[inline]
            fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
                0 + self.user.as_ref().map_or(0, |msg| {
                    ::pilota::pb::encoding::message::encoded_len(ctx, 1, msg)
                }) + ::pilota::pb::encoding::faststr::encoded_len(ctx, 2, &self.message)
            }

            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                if let Some(_pilota_inner_value) = self.user.as_ref() {
                    ::pilota::pb::encoding::message::encode(1, _pilota_inner_value, buf);
                }
                ::pilota::pb::encoding::faststr::encode(2, &self.message, buf);
            }

            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::pilota::pb::encoding::WireType,
                buf: &mut ::pilota::Bytes,
                ctx: &mut ::pilota::pb::encoding::DecodeContext,
                is_root: bool,
            ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
                const STRUCT_NAME: &'static str = stringify!(CreateUserResponse);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.user;
                        ::pilota::pb::encoding::message::merge(
                            wire_type,
                            _inner_pilota_value
                                .get_or_insert_with(::core::default::Default::default),
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
                        ::pilota::pb::encoding::faststr::merge(
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
                    _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default, Clone, PartialEq)]
        pub struct GetUserRequest {
            pub id: ::pilota::FastStr,
        }
        impl ::pilota::pb::Message for GetUserRequest {
            #[inline]
            fn encoded_len(&self, ctx: &mut ::pilota::pb::EncodeLengthContext) -> usize {
                0 + ::pilota::pb::encoding::faststr::encoded_len(ctx, 1, &self.id)
            }

            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut ::pilota::LinkedBytes) {
                ::pilota::pb::encoding::faststr::encode(1, &self.id, buf);
            }

            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::pilota::pb::encoding::WireType,
                buf: &mut ::pilota::Bytes,
                ctx: &mut ::pilota::pb::encoding::DecodeContext,
                is_root: bool,
            ) -> ::core::result::Result<(), ::pilota::pb::DecodeError> {
                const STRUCT_NAME: &'static str = stringify!(GetUserRequest);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.id;
                        ::pilota::pb::encoding::faststr::merge(
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
                    _ => ::pilota::pb::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }

        pub mod api_metadata {
            use ::pilota::{Buf as _, BufMut as _};
            pub mod exts_api_metadata {
                pub const test: ::pilota::pb::extension::CustomExtField<
                    ::pilota::pb::descriptor::MessageOptions,
                    ::pilota::pb::extension::StrOptionValueExtractor,
                > = ::pilota::pb::extension::CustomExtField::new(51101);
            }

            pub mod example {
                use ::pilota::{Buf as _, BufMut as _};
                pub mod exts_example {
                    pub const level: ::pilota::pb::extension::CustomExtField<
                        ::pilota::pb::descriptor::MessageOptions,
                        ::pilota::pb::extension::StrOptionValueExtractor,
                    > = ::pilota::pb::extension::CustomExtField::new(51102);
                }
            }
        }
    }
}
