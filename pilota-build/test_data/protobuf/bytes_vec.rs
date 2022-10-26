pub mod bytes_vec {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::unused_unit,
        clippy::needless_borrow,
        unused_mut
    )]
    #[derive(PartialOrd, Hash, Eq, Ord, :: prost :: Message, Clone, PartialEq)]
    pub struct A {
        #[prost(bytes, tag = "1")]
        pub a: ::bytes::Bytes,
    }
}
