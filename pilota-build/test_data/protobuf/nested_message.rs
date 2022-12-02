pub mod nested_message {
    #![allow(warnings, clippy::all)]
    pub mod tt1 {
        pub mod t2 {
            #[derive(:: prost :: Message, Clone, PartialEq)]
            pub struct Tt3 {
                #[prost(int32, tag = "1", optional)]
                pub a: ::std::option::Option<i32>,
                #[prost(map = "int32 , message", tag = "2")]
                pub m: ::std::collections::HashMap<i32, T2>,
            }
            #[derive(:: prost :: Message, Clone, PartialEq)]
            pub struct T2 {
                #[prost(message, tag = "1", optional)]
                pub t3: ::std::option::Option<Tt3>,
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, :: prost :: Enumeration, Debug, Clone, PartialEq)]
        #[repr(i32)]
        #[derive(Copy)]
        pub enum Label {
            LabelOptional = 1i32,
            LabelRequired = 2i32,
            LabelRepeated = 3i32,
        }
        #[derive(:: prost :: Message, Clone, PartialEq)]
        pub struct Tt1 {
            #[prost(message, tag = "1", optional)]
            pub t2: ::std::option::Option<t2::T2>,
            #[prost(enumeration = "Label", tag = "2")]
            pub t3: i32,
            #[prost(message, tag = "4", optional)]
            pub t4: ::std::option::Option<t2::Tt3>,
        }
    }
}
