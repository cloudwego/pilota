use std::option::Option;

use protobuf::descriptor;

pub trait ItemDescriptorGetter {
    fn get_message_descriptor_proto(&self, name: &str) -> Option<&descriptor::DescriptorProto>;
    fn get_enum_descriptor_proto(&self, name: &str) -> Option<&descriptor::EnumDescriptorProto>;
    fn get_service_descriptor_proto(
        &self,
        name: &str,
    ) -> Option<&descriptor::ServiceDescriptorProto>;
}

impl ItemDescriptorGetter for descriptor::FileDescriptorProto {
    fn get_message_descriptor_proto(&self, name: &str) -> Option<&descriptor::DescriptorProto> {
        if name == "" {
            return None;
        }

        self.message_type
            .iter()
            .find(|s| s.name.as_deref().unwrap_or("") == name)
    }

    fn get_enum_descriptor_proto(&self, name: &str) -> Option<&descriptor::EnumDescriptorProto> {
        if name == "" {
            return None;
        }

        self.enum_type
            .iter()
            .find(|s| s.name.as_deref().unwrap_or("") == name)
    }

    fn get_service_descriptor_proto(
        &self,
        name: &str,
    ) -> Option<&descriptor::ServiceDescriptorProto> {
        if name == "" {
            return None;
        }

        self.service
            .iter()
            .find(|s| s.name.as_deref().unwrap_or("") == name)
    }
}

pub trait MessageDescriptorGetter {
    fn get_descriptor_proto(&self) -> Option<&descriptor::DescriptorProto>;
}

pub trait OneofDescriptorGetter {
    fn get_oneof_descriptor_proto(&self, name: &str) -> Option<&descriptor::OneofDescriptorProto>;
}

impl OneofDescriptorGetter for descriptor::DescriptorProto {
    fn get_oneof_descriptor_proto(&self, name: &str) -> Option<&descriptor::OneofDescriptorProto> {
        if name == "" {
            return None;
        }

        self.oneof_decl
            .iter()
            .find(|s| s.name.as_deref().unwrap_or("") == name)
    }
}

pub trait EnumDescriptorGetter {
    fn get_descriptor_proto(&self) -> Option<&descriptor::EnumDescriptorProto>;
}

pub trait ServiceDescriptorGetter {
    fn get_descriptor_proto(&self) -> Option<&descriptor::ServiceDescriptorProto>;
}

#[cfg(test)]
mod tests {
    use protobuf::descriptor;

    use super::*;

    fn make_file_with_items() -> descriptor::FileDescriptorProto {
        let mut f = descriptor::FileDescriptorProto::new();

        // message Foo
        let mut m = descriptor::DescriptorProto::new();
        m.name = Some("Foo".to_string());
        f.message_type.push(m);

        // enum Bar
        let mut e = descriptor::EnumDescriptorProto::new();
        e.name = Some("Bar".to_string());
        f.enum_type.push(e);

        // service Baz
        let mut s = descriptor::ServiceDescriptorProto::new();
        s.name = Some("Baz".to_string());
        f.service.push(s);

        f
    }

    #[test]
    fn test_get_message_descriptor_proto_found_and_missing() {
        let f = make_file_with_items();
        // found
        let got = f.get_message_descriptor_proto("Foo");
        assert!(got.is_some());
        assert_eq!(got.unwrap().name.as_deref(), Some("Foo"));
        // missing
        assert!(f.get_message_descriptor_proto("NotExist").is_none());
        // empty name -> None
        assert!(f.get_message_descriptor_proto("").is_none());
    }

    #[test]
    fn test_get_enum_descriptor_proto_found_and_missing() {
        let f = make_file_with_items();
        // found
        let got = f.get_enum_descriptor_proto("Bar");
        assert!(got.is_some());
        assert_eq!(got.unwrap().name.as_deref(), Some("Bar"));
        // missing
        assert!(f.get_enum_descriptor_proto("NotExist").is_none());
        // empty name -> None
        assert!(f.get_enum_descriptor_proto("").is_none());
    }

    #[test]
    fn test_get_service_descriptor_proto_found_and_missing() {
        let f = make_file_with_items();
        // found
        let got = f.get_service_descriptor_proto("Baz");
        assert!(got.is_some());
        assert_eq!(got.unwrap().name.as_deref(), Some("Baz"));
        // missing
        assert!(f.get_service_descriptor_proto("NotExist").is_none());
        // empty name -> None
        assert!(f.get_service_descriptor_proto("").is_none());
    }

    #[test]
    fn test_get_oneof_descriptor_proto_found_and_missing() {
        let mut msg = descriptor::DescriptorProto::new();
        msg.name = Some("Container".to_string());

        // oneof alpha
        let mut o1 = descriptor::OneofDescriptorProto::new();
        o1.name = Some("alpha".to_string());
        msg.oneof_decl.push(o1);
        // oneof beta
        let mut o2 = descriptor::OneofDescriptorProto::new();
        o2.name = Some("beta".to_string());
        msg.oneof_decl.push(o2);

        // found
        let got = msg.get_oneof_descriptor_proto("alpha");
        assert!(got.is_some());
        assert_eq!(got.unwrap().name.as_deref(), Some("alpha"));

        // other found
        let got = msg.get_oneof_descriptor_proto("beta");
        assert!(got.is_some());
        assert_eq!(got.unwrap().name.as_deref(), Some("beta"));

        // missing
        assert!(msg.get_oneof_descriptor_proto("gamma").is_none());
        // empty name -> None
        assert!(msg.get_oneof_descriptor_proto("").is_none());
    }
}
