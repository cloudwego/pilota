pub mod bytes {
    #![allow(warnings, clippy::all)]
    #[derive(PartialOrd, Hash, Eq, Ord, :: prost :: Message, Clone, PartialEq)]
    pub struct A {
        #[prost(bytes, tag = "1", optional)]
        pub a: ::std::option::Option<::pilota::Bytes>,
    }
}
