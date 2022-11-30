pub mod string {
    #![allow(warnings, clippy::all)]
    #[derive(PartialOrd, Hash, Eq, Ord, :: prost :: Message, Clone, PartialEq)]
    pub struct A {
        #[prost(string, tag = "1")]
        pub a: ::std::string::String,
    }
}
