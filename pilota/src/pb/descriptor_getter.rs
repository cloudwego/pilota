use std::option::Option;

use protobuf::descriptor;

pub trait ItemDescriptorGetter {
    fn get_message_descriptor_proto(&self, name: &str) -> Option<&descriptor::DescriptorProto>;
}

impl ItemDescriptorGetter for descriptor::FileDescriptorProto {
    fn get_message_descriptor_proto(&self, name: &str) -> Option<&descriptor::DescriptorProto> {
        self.message_type
            .iter()
            .find(|s| s.name.as_deref().unwrap_or("") == name)
    }
}

pub trait MessageDescriptorGetter {
    fn get_descriptor_proto(&self) -> Option<&descriptor::DescriptorProto>;
}
