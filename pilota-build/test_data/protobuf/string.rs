pub mod string {
    #![allow(warnings, clippy::all)]
    #[derive(PartialOrd, Hash, Eq, Ord, :: prost :: Message, Clone, PartialEq)]
    pub struct A {
        #[prost(string, tag = "1", optional)]
        pub a: ::std::option::Option<::std::string::String>,
    }
}
