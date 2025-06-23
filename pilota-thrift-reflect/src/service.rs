use std::sync::LazyLock;

use bytes::{Bytes, BytesMut};
use dashmap::DashMap;
use pilota::{
    FastStr,
    thrift::{Message as _, ThriftException},
};

use crate::{ThriftType, error::ReflectorError, thrift_reflection::*};

// global descriptor map for include file search
pub static GLOBAL_DESCRIPTOR: LazyLock<DashMap<FastStr, FileDescriptor>> =
    LazyLock::new(|| DashMap::new());

// global descriptor manager
pub struct Register;

impl Register {
    // register a file descriptor
    pub fn register(name: FastStr, descriptor: FileDescriptor) {
        GLOBAL_DESCRIPTOR.insert(name, descriptor);
    }

    pub fn contains(name: &str) -> bool {
        GLOBAL_DESCRIPTOR.contains_key(name)
    }

    // get a file descriptor
    pub fn get(name: &str) -> Option<dashmap::mapref::one::Ref<'_, FastStr, FileDescriptor>> {
        GLOBAL_DESCRIPTOR.get(name)
    }

    // find descriptors by predicate
    pub fn find<F>(predicate: F) -> Vec<(FastStr, FileDescriptor)>
    where
        F: Fn(&FileDescriptor) -> bool,
    {
        GLOBAL_DESCRIPTOR
            .iter()
            .filter(|entry| predicate(entry.value()))
            .map(|entry| (entry.key().clone(), entry.value().clone()))
            .collect()
    }
}

impl FileDescriptor {
    // find struct by name
    pub fn find_struct_by_name(&self, name: &str) -> Option<&StructDescriptor> {
        self.structs.iter().find(|s| s.name.as_str() == name)
    }

    // find exception by name
    pub fn find_exception_by_name(&self, name: &str) -> Option<&StructDescriptor> {
        self.exceptions.iter().find(|e| e.name.as_str() == name)
    }

    pub fn serialize(&self) -> Bytes {
        let mut data = BytesMut::new();
        let mut protocol = pilota::thrift::binary::TBinaryProtocol::new(&mut data, true);
        let _ = self.encode(&mut protocol);
        data.freeze()
    }

    pub fn deserialize(mut data: Bytes) -> Result<FileDescriptor, ThriftException> {
        let mut protocol = pilota::thrift::binary::TBinaryProtocol::new(&mut data, true);
        FileDescriptor::decode(&mut protocol)
    }
}

impl StructDescriptor {
    pub fn find_field_by_id(&self, id: i32) -> Option<&FieldDescriptor> {
        self.fields.iter().find(|f| f.id == id)
    }

    pub fn find_field_by_name(&self, name: &str) -> Option<&FieldDescriptor> {
        self.fields.iter().find(|f| f.name.as_str() == name)
    }

    pub fn type_descriptor(&self) -> TypeDescriptor {
        TypeDescriptor {
            filepath: self.filepath.clone(),
            name: self.name.clone(),
            key_type: None,
            value_type: None,
            extra: None,
        }
    }
}

impl TypeDescriptor {
    // check if the type is a container type
    pub fn is_container_type(&self) -> bool {
        let ty = self.name.as_str().into();
        matches!(ty, ThriftType::List | ThriftType::Set | ThriftType::Map)
    }

    // check if the type is a primitive type
    pub fn is_primitive_type(&self) -> bool {
        let ty = self.name.as_str().into();
        matches!(
            ty,
            ThriftType::Void
                | ThriftType::Bool
                | ThriftType::Byte
                | ThriftType::I8
                | ThriftType::I16
                | ThriftType::I32
                | ThriftType::I64
                | ThriftType::Double
                | ThriftType::String
                | ThriftType::Binary
                | ThriftType::Uuid
        )
    }

    // check if the type is a user defined type
    pub fn is_user_defined_type(&self) -> bool {
        let ty = self.name.as_str().into();
        matches!(ty, ThriftType::Path(_))
    }

    pub fn get_struct_desc(&self) -> Option<StructDescriptor> {
        let ty = self.name.as_str().into();
        match ty {
            ThriftType::Path(path) => {
                let cur_file_desc = Register::get(self.filepath.as_str()).unwrap();
                let include_path = IncludePath::try_from(path.as_str()).unwrap();

                if !include_path.prefix.is_empty() {
                    let include_file_path = cur_file_desc
                        .includes
                        .get(include_path.prefix.as_str())
                        .expect("include path not found");
                    let included_file_descriptor =
                        Register::get(include_file_path.as_str()).unwrap();
                    included_file_descriptor
                        .find_struct_by_name(include_path.name.as_str())
                        .cloned()
                } else {
                    cur_file_desc
                        .find_struct_by_name(include_path.name.as_str())
                        .cloned()
                }
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct IncludePath {
    pub prefix: FastStr,
    pub name: FastStr,
}

impl TryFrom<&str> for IncludePath {
    type Error = ReflectorError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts = value.split(".").collect::<Vec<&str>>();
        match parts.len() {
            0 => Err(ReflectorError::IncludePathError(format!(
                "Invalid include path: {value}"
            ))),
            1 => Ok(Self {
                prefix: "".into(),
                name: FastStr::new(value),
            }),
            _ => Ok(Self {
                prefix: parts[..parts.len() - 1].join(".").into(),
                name: FastStr::new(parts[parts.len() - 1]),
            }),
        }
    }
}
