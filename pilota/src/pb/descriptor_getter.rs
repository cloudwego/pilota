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
