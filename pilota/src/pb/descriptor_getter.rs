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
        if name.is_empty() {
            return None;
        }

        self.message_type.iter().find(|s| s.name() == name)
    }

    fn get_enum_descriptor_proto(&self, name: &str) -> Option<&descriptor::EnumDescriptorProto> {
        if name.is_empty() {
            return None;
        }

        self.enum_type.iter().find(|s| s.name() == name)
    }

    fn get_service_descriptor_proto(
        &self,
        name: &str,
    ) -> Option<&descriptor::ServiceDescriptorProto> {
        if name.is_empty() {
            return None;
        }

        self.service.iter().find(|s| s.name() == name)
    }
}

pub trait FieldDescriptorGetter {
    fn get_field_descriptor_proto(&self, name: &str) -> Option<&descriptor::FieldDescriptorProto>;
    fn get_oneof_descriptor_proto(&self, name: &str) -> Option<&descriptor::OneofDescriptorProto>;
    fn get_enum_descriptor_proto(&self, name: &str) -> Option<&descriptor::EnumDescriptorProto>;
    fn get_message_descriptor_proto(&self, name: &str) -> Option<&descriptor::DescriptorProto>;
}

impl FieldDescriptorGetter for descriptor::DescriptorProto {
    fn get_field_descriptor_proto(&self, name: &str) -> Option<&descriptor::FieldDescriptorProto> {
        if name.is_empty() {
            return None;
        }

        self.field.iter().find(|s| s.name() == name)
    }

    fn get_oneof_descriptor_proto(&self, name: &str) -> Option<&descriptor::OneofDescriptorProto> {
        if name.is_empty() {
            return None;
        }

        self.oneof_decl.iter().find(|s| s.name() == name)
    }

    fn get_enum_descriptor_proto(&self, name: &str) -> Option<&descriptor::EnumDescriptorProto> {
        if name.is_empty() {
            return None;
        }

        self.enum_type.iter().find(|s| s.name() == name)
    }

    fn get_message_descriptor_proto(&self, name: &str) -> Option<&descriptor::DescriptorProto> {
        if name.is_empty() {
            return None;
        }

        self.nested_type.iter().find(|s| s.name() == name)
    }
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

    #[test]
    fn test_get_field_descriptor_proto_found_and_missing() {
        let mut msg = descriptor::DescriptorProto::new();
        msg.name = Some("Container".to_string());

        // field id
        let mut f1 = descriptor::FieldDescriptorProto::new();
        f1.name = Some("id".to_string());
        msg.field.push(f1);
        // field name
        let mut f2 = descriptor::FieldDescriptorProto::new();
        f2.name = Some("name".to_string());
        msg.field.push(f2);

        // found
        let got = msg.get_field_descriptor_proto("id");
        assert!(got.is_some());
        assert_eq!(got.unwrap().name.as_deref(), Some("id"));

        // other found
        let got = msg.get_field_descriptor_proto("name");
        assert!(got.is_some());
        assert_eq!(got.unwrap().name.as_deref(), Some("name"));

        // missing
        assert!(msg.get_field_descriptor_proto("age").is_none());
        // empty name -> None
        assert!(msg.get_field_descriptor_proto("").is_none());
    }

    #[test]
    fn test_get_enum_descriptor_proto_in_descriptor_proto_found_and_missing() {
        let mut msg = descriptor::DescriptorProto::new();
        msg.name = Some("Container".to_string());

        // enum Color
        let mut e = descriptor::EnumDescriptorProto::new();
        e.name = Some("Color".to_string());
        msg.enum_type.push(e);

        // found
        let got = msg.get_enum_descriptor_proto("Color");
        assert!(got.is_some());
        assert_eq!(got.unwrap().name.as_deref(), Some("Color"));

        // missing
        assert!(msg.get_enum_descriptor_proto("Size").is_none());
        // empty name -> None
        assert!(msg.get_enum_descriptor_proto("").is_none());
    }

    #[test]
    fn test_get_message_descriptor_proto_in_descriptor_proto_found_and_missing() {
        let mut msg = descriptor::DescriptorProto::new();
        msg.name = Some("Container".to_string());

        // nested message Item
        let mut m = descriptor::DescriptorProto::new();
        m.name = Some("Item".to_string());
        msg.nested_type.push(m);

        // found
        let got = msg.get_message_descriptor_proto("Item");
        assert!(got.is_some());
        assert_eq!(got.unwrap().name.as_deref(), Some("Item"));

        // missing
        assert!(msg.get_message_descriptor_proto("Detail").is_none());
        // empty name -> None
        assert!(msg.get_message_descriptor_proto("").is_none());
    }
}
