pub mod optional {
    #![allow(warnings, clippy::all)]
    #[derive(PartialOrd, Hash, Eq, Ord, :: prost :: Message, Clone, PartialEq)]
    pub struct SearchRequest {
        #[prost(int32, tag = "2", optional)]
        pub page_number: ::std::option::Option<i32>,
    }
}
