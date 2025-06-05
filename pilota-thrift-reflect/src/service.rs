use std::sync::LazyLock;

use crate::{ThriftType, error::ReflectorError, thrift_reflection::*};
use bytes::{Bytes, BytesMut};
use dashmap::DashMap;
use pilota::{
    FastStr,
    thrift::{Message as _, ThriftException},
};

/// 全局文件描述符存储，支持动态读写
pub static GLOBAL_DESCRIPTOR: LazyLock<DashMap<FastStr, FileDescriptor>> =
    LazyLock::new(|| DashMap::new());

/// 全局描述符管理工具
pub struct Register;

impl Register {
    /// 注册一个文件描述符
    pub fn register(name: FastStr, descriptor: FileDescriptor) {
        GLOBAL_DESCRIPTOR.insert(name, descriptor);
    }

    /// 获取文件描述符
    pub fn get(name: &str) -> Option<dashmap::mapref::one::Ref<'_, FastStr, FileDescriptor>> {
        GLOBAL_DESCRIPTOR.get(name)
    }

    /// 移除文件描述符
    pub fn remove(name: &str) -> Option<(FastStr, FileDescriptor)> {
        GLOBAL_DESCRIPTOR.remove(name)
    }

    /// 检查是否存在指定的描述符
    pub fn contains(name: &str) -> bool {
        GLOBAL_DESCRIPTOR.contains_key(name)
    }

    /// 获取所有已注册的描述符名称
    pub fn list_names() -> Vec<FastStr> {
        GLOBAL_DESCRIPTOR
            .iter()
            .map(|entry| entry.key().clone())
            .collect()
    }

    /// 清空所有描述符
    pub fn clear() {
        GLOBAL_DESCRIPTOR.clear();
    }

    /// 批量注册描述符
    pub fn register_batch(descriptors: impl IntoIterator<Item = (FastStr, FileDescriptor)>) {
        for (name, descriptor) in descriptors {
            GLOBAL_DESCRIPTOR.insert(name, descriptor);
        }
    }

    /// 更新现有描述符（如果存在）
    pub fn update(name: &str, descriptor: FileDescriptor) -> bool {
        if let Some(mut entry) = GLOBAL_DESCRIPTOR.get_mut(name) {
            *entry = descriptor;
            true
        } else {
            false
        }
    }

    /// 获取或插入描述符（如果不存在则插入）
    pub fn get_or_insert(name: FastStr, descriptor: FileDescriptor) {
        GLOBAL_DESCRIPTOR.entry(name).or_insert(descriptor);
    }

    /// 按条件查找描述符
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

    /// 获取当前已注册描述符的数量
    pub fn len() -> usize {
        GLOBAL_DESCRIPTOR.len()
    }

    /// 检查是否为空
    pub fn is_empty() -> bool {
        GLOBAL_DESCRIPTOR.is_empty()
    }
}

/// FileDescriptor 的工具方法
impl FileDescriptor {
    /// 按名称查找结构体
    pub fn find_struct_by_name(&self, name: &str) -> Option<&StructDescriptor> {
        self.structs.iter().find(|s| s.name.as_str() == name)
    }

    /// 按名称查找枚举
    pub fn find_enum_by_name(&self, name: &str) -> Option<&EnumDescriptor> {
        self.enums.iter().find(|e| e.name.as_str() == name)
    }

    /// 按名称查找异常
    pub fn find_exception_by_name(&self, name: &str) -> Option<&StructDescriptor> {
        self.exceptions.iter().find(|e| e.name.as_str() == name)
    }

    /// 按名称查找联合体
    pub fn find_union_by_name(&self, name: &str) -> Option<&StructDescriptor> {
        self.unions.iter().find(|u| u.name.as_str() == name)
    }

    /// 按名称查找常量
    pub fn find_const_by_name(&self, name: &str) -> Option<&ConstDescriptor> {
        self.consts.iter().find(|c| c.name.as_str() == name)
    }

    /// 按别名查找类型定义
    pub fn find_typedef_by_alias(&self, alias: &str) -> Option<&TypedefDescriptor> {
        self.typedefs.iter().find(|t| t.alias.as_str() == alias)
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

/// StructDescriptor 的工具方法
impl StructDescriptor {
    pub fn find_field_by_id(&self, id: i32) -> Option<&FieldDescriptor> {
        self.fields.iter().find(|f| f.id == id)
    }

    pub fn find_field_by_name(&self, name: &str) -> Option<&FieldDescriptor> {
        self.fields.iter().find(|f| f.name.as_str() == name)
    }
}

/// EnumDescriptor 的工具方法
impl EnumDescriptor {
    /// 按值查找枚举值
    pub fn find_value_by_int(&self, value: i64) -> Option<&EnumValueDescriptor> {
        self.values.iter().find(|v| v.value == value)
    }

    /// 按名称查找枚举值
    pub fn find_value_by_name(&self, name: &str) -> Option<&EnumValueDescriptor> {
        self.values.iter().find(|v| v.name.as_str() == name)
    }

    /// 获取所有枚举值名称
    pub fn get_value_names(&self) -> Vec<&str> {
        self.values.iter().map(|v| v.name.as_str()).collect()
    }

    /// 获取所有枚举值
    pub fn get_values(&self) -> Vec<i64> {
        self.values.iter().map(|v| v.value).collect()
    }
}

/// FieldDescriptor 的工具方法
impl FieldDescriptor {
    /// 检查是否为必填字段
    pub fn is_required(&self) -> bool {
        self.requiredness.as_str() == "required"
    }

    /// 检查是否为可选字段
    pub fn is_optional(&self) -> bool {
        self.requiredness.as_str() == "optional"
    }

    /// 检查是否有默认值
    pub fn has_default_value(&self) -> bool {
        self.default_value.is_some()
    }

    /// 获取类型名称
    pub fn get_type_name(&self) -> &str {
        self.r#type.name.as_str()
    }

    /// 检查是否为集合类型
    pub fn is_collection_type(&self) -> bool {
        matches!(self.r#type.name.as_str(), "list" | "set" | "map")
    }

    /// 检查是否为基础类型
    pub fn is_primitive_type(&self) -> bool {
        matches!(
            self.r#type.name.as_str(),
            "bool" | "byte" | "i8" | "i16" | "i32" | "i64" | "double" | "string" | "binary"
        )
    }
}

/// TypeDescriptor 的工具方法
impl TypeDescriptor {
    /// 检查是否为容器类型
    pub fn is_container_type(&self) -> bool {
        let ty = self.name.as_str().into();
        matches!(ty, ThriftType::List | ThriftType::Set | ThriftType::Map)
    }

    /// 检查是否为基础类型
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

    /// 检查是否为用户定义类型
    pub fn is_user_defined_type(&self) -> bool {
        let ty = self.name.as_str().into();
        matches!(ty, ThriftType::Path(_))
    }

    /// 获取键类型（对于 map 类型）
    pub fn get_key_type(&self) -> Option<&TypeDescriptor> {
        self.key_type.as_deref()
    }

    /// 获取值类型（对于 list、set、map 类型）
    pub fn get_value_type(&self) -> Option<&TypeDescriptor> {
        self.value_type.as_deref()
    }

    /// 获取元素类型（对于 list 和 set 类型）
    pub fn get_element_type(&self) -> Option<&TypeDescriptor> {
        if matches!(self.name.as_str(), "list" | "set") {
            self.key_type.as_deref()
        } else {
            None
        }
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
