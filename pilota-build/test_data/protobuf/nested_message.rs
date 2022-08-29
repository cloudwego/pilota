pub mod nested_message {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::unused_unit,
        clippy::needless_borrow,
        unused_mut
    )]
    pub mod nested_message {
        pub mod tt1 {
            pub mod t2 {
                #[derive(PartialOrd, Hash, Eq, Ord, :: prost :: Message, Clone, PartialEq)]
                pub struct Tt3 {
                    #[prost(int32, tag = "1")]
                    pub a: i32,
                }
                #[derive(PartialOrd, Hash, Eq, Ord, :: prost :: Message, Clone, PartialEq)]
                pub struct T2 {
                    #[prost(message, tag = "1", optional)]
                    pub t3: ::std::option::Option<Tt3>,
                }
            }
            #[derive(PartialOrd, Hash, Eq, Ord, :: prost :: Message, Clone, PartialEq)]
            pub struct Tt1 {
                #[prost(message, tag = "1", optional)]
                pub t2: ::std::option::Option<T2::T2>,
            }
        }
    }
}
