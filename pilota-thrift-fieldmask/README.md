# Pilota Thrift FieldMask

一个用 Rust 实现的 Thrift 字段掩码库，支持 Pilota 框架。这是从 Go 语言版本重构而来的高性能字段掩码实现。

**🆕 重要更新：我们将 `FieldMaskType` 和数据字段合并为一个类型安全的枚举 `FieldMaskData`，提供更好的内存效率和编译时安全性。**

## 概述

FieldMask 是一种机制，允许您指定在 Thrift 消息中包含或排除哪些字段。这对于：

- **性能优化**：只序列化/反序列化需要的字段
- **安全控制**：控制哪些字段对客户端可见
- **版本兼容性**：在不同版本间安全地添加/删除字段
- **带宽优化**：减少网络传输的数据量

## 特性

- ✅ **高性能路径解析**：支持复杂的 JSONPath 风格路径语法
- ✅ **完整类型支持**：支持所有 Thrift 类型（STRUCT、LIST、SET、MAP等）
- ✅ **白名单/黑名单模式**：灵活的字段控制策略
- ✅ **通配符支持**：使用 `*` 匹配所有元素
- ✅ **嵌套路径**：支持深度嵌套的字段访问
- ✅ **零拷贝设计**：高效的内存使用
- ✅ **强类型错误处理**：使用 `thiserror` 提供具体的错误类型和详细的错误信息
- ✅ **类型安全设计**：编译时保证类型和数据的匹配
- ✅ **内存效率**：消除冗余字段，只存储实际需要的数据
- ✅ **现代化错误处理**：使用 `thiserror` 自动生成错误实现，减少样板代码

## 安装

将以下内容添加到您的 `Cargo.toml`：

```toml
[dependencies]
pilota-thrift-fieldmask = "0.1.0"
```

## 快速开始

```rust
use pilota_thrift_fieldmask::{FieldMaskBuilder, FieldMaskError, Options};
use pilota_thrift_reflect::descriptor::thrift_reflection::TypeDescriptor;

// 创建类型描述符
let desc = TypeDescriptor {
    name: "STRUCT".into(),
    ..Default::default()
};

// 定义要包含的字段路径
let paths = vec![
    "$.user.name".to_string(),
    "$.user.email".to_string(),
    "$.posts[*].title".to_string(),
];

// 使用 Builder 模式创建字段掩码
let mask = FieldMaskBuilder::new(&desc, &paths)
    .with_options(Options {
        black_list_mode: false,
    })
    .build()?;

// 检查字段是否在掩码中
let (submask, included) = mask.field(1); // 检查字段 ID 1
if included {
    println!("字段 1 包含在掩码中");
}

// 遍历所有子字段
mask.for_each_child(|str_key, int_key, child| {
    println!("字段: {}/{}, 类型: {}", str_key, int_key, child.typ());
    true // 继续遍历
});
```

## 路径语法

FieldMask 使用类似 JSONPath 的语法来指定字段路径：

### 基础语法

- `$` - 根路径标识符
- `.field` - 访问结构体字段（按名称）
- `.123` - 访问结构体字段（按 ID）
- `[index]` - 访问列表/数组元素
- `{key}` - 访问映射键
- `*` - 通配符，匹配所有元素
- `,` - 分隔多个索引或键

### 示例路径

```rust
// 基础字段访问
"$.name"           // 访问根对象的 name 字段
"$.123"            // 访问根对象的字段 ID 123

// 嵌套结构体
"$.user.profile.avatar"  // 深度嵌套访问

// 列表/数组操作
"$.items[0]"       // 访问第一个元素
"$.items[0,1,2]"   // 访问多个指定元素
"$.items[*]"       // 访问所有元素

// 映射操作
"$.metadata{\"key\"}"     // 访问字符串键
"$.counters{123}"         // 访问整数键
"$.tags{\"red\",\"blue\"}" // 访问多个键
"$.config{*}"             // 访问所有键

// 复合路径
"$.users[*].posts[0,1].title"  // 所有用户的前两篇文章标题
"$.data{\"cache\"}.items[*]"   // 缓存中的所有项目
```

## 类型安全设计

### FieldMaskData 枚举

新的设计将类型信息和数据存储合并为一个类型安全的枚举：

```rust
#[derive(Debug, Clone)]
pub enum FieldMaskData {
    /// 无效或未初始化的类型
    Invalid,
    /// 标量类型（基础类型）
    Scalar,
    /// 结构体类型，包含字段ID到子掩码的映射
    Struct {
        /// 具体字段的子掩码映射
        children: AHashMap<i16, Box<FieldMask>>,
        /// 通配符子掩码（对应 * 语法）
        wildcard: Option<Box<FieldMask>>,
    },
    /// 列表/集合类型，包含索引到子掩码的映射
    List {
        /// 具体索引的子掩码映射
        children: AHashMap<i32, Box<FieldMask>>,
        /// 通配符子掩码（对应 [*] 语法）
        wildcard: Option<Box<FieldMask>>,
    },
    /// 字符串键映射类型
    StrMap {
        /// 具体键的子掩码映射
        children: AHashMap<FastStr, Box<FieldMask>>,
        /// 通配符子掩码（对应 {*} 语法）
        wildcard: Option<Box<FieldMask>>,
    },
    /// 整数键映射类型
    IntMap {
        /// 具体键的子掩码映射
        children: AHashMap<i32, Box<FieldMask>>,
        /// 通配符子掩码（对应 {*} 语法）
        wildcard: Option<Box<FieldMask>>,
    },
}
```

### 类型安全访问

```rust
let struct_mask = FieldMaskBuilder::new(&struct_desc, &["$.field1".to_string()]).build()?;
let list_mask = FieldMaskBuilder::new(&list_desc, &["$[0,1]".to_string()]).build()?;

// 类型安全的访问 - 只有正确的访问方法才会返回有意义的结果
let (_, struct_field_access) = struct_mask.field(1);     // 正确：结构体访问字段
let (_, struct_int_access) = struct_mask.int(0);         // 类型不匹配，返回false

let (_, list_int_access) = list_mask.int(0);             // 正确：列表访问索引  
let (_, list_field_access) = list_mask.field(1);         // 类型不匹配，返回false
```

## 错误处理

FieldMask 提供了具体的错误类型 `FieldMaskError` 和 `PathError`，包含详细的错误信息：

### FieldMaskError 类型

```rust
#[derive(Debug, Clone)]
pub enum FieldMaskError {
    /// 路径解析错误（包装PathError）
    PathError {
        path: String,
        source: PathError,
    },
    /// 类型描述符错误
    DescriptorError {
        type_name: String,
        message: String,
    },
    /// 字段不存在错误
    FieldNotFound {
        field_identifier: String,
        parent_type: String,
    },
    /// 类型不匹配错误
    TypeMismatch {
        expected: String,
        actual: String,
        context: String,
    },
    /// 空集合错误
    EmptyCollection {
        collection_type: String,
    },
    /// 冲突错误
    ConflictError {
        message: String,
    },
    /// 无效的token类型
    InvalidToken {
        token_type: String,
        expected: String,
    },
    /// 通用错误
    GenericError {
        message: String,
    },
}
```

### PathError 类型

`PathError` 专门处理路径解析相关的错误：

```rust
#[derive(Debug, Clone)]
pub enum PathError {
    /// 语法解析错误
    SyntaxError {
        position: usize,
        expected: String,
        found: String,
    },
    /// 无效的字符或token
    InvalidCharacter {
        position: usize,
        character: char,
    },
    /// 未闭合的字符串
    UnterminatedString {
        start_position: usize,
    },
    /// 无效的转义序列
    InvalidEscape {
        position: usize,
        sequence: String,
    },
    /// 数字解析错误
    InvalidNumber {
        position: usize,
        value: String,
    },
    /// 意外的EOF
    UnexpectedEof {
        position: usize,
        expected: String,
    },
    /// 空路径
    EmptyPath,
    /// 通用解析错误
    ParseError {
        position: usize,
        message: String,
    },
}
```

### 错误处理示例

```rust
use pilota_thrift_fieldmask::{FieldMaskBuilder, FieldMaskError, PathError};

match FieldMaskBuilder::new(&desc, &paths).build() {
    Ok(mask) => {
        // 使用掩码
        println!("FieldMask 创建成功，类型: {}", mask.typ());
    }
    Err(FieldMaskError::PathError { path, source }) => {
        println!("路径解析错误 '{}':", path);
        match source {
            PathError::SyntaxError { position, expected, found } => {
                eprintln!("语法错误在位置 {}: 期望 '{}', 但发现 '{}'", 
                         position, expected, found);
            }
            PathError::InvalidCharacter { position, character } => {
                eprintln!("无效字符 '{}' 在位置 {}", character, position);
            }
            PathError::UnterminatedString { start_position } => {
                eprintln!("未闭合的字符串从位置 {} 开始", start_position);
            }
            PathError::UnexpectedEof { position, expected } => {
                eprintln!("意外的文件结束在位置 {}, 期望 '{}'", position, expected);
            }
            _ => {
                eprintln!("其他路径错误: {}", source);
            }
        }
    }
    Err(FieldMaskError::TypeMismatch { expected, actual, context }) => {
        eprintln!("类型不匹配在 {}: 期望 '{}', 实际 '{}'", context, expected, actual);
    }
    Err(e) => {
        eprintln!("创建字段掩码失败: {}", e);
        
        // 演示错误链访问
        if let Some(source) = e.source() {
            eprintln!("根本原因: {}", source);
        }
    }
}
```

### 路径验证

在创建 FieldMask 之前，可以预先验证路径语法：

```rust
use pilota_thrift_fieldmask::PathError;
use crate::path::PathIterator;

// 验证路径语法
match PathIterator::validate("$.invalid[path") {
    Ok(()) => println!("路径语法正确"),
    Err(PathError::UnexpectedEof { position, expected }) => {
        eprintln!("路径不完整：在位置 {} 期望 '{}'", position, expected);
    }
    Err(e) => {
        eprintln!("路径语法错误: {}", e);
    }
}
```

## 选项配置

### 使用 Builder 模式

```rust
use pilota_thrift_fieldmask::{FieldMaskBuilder, Options};

// 黑名单模式
let mask = FieldMaskBuilder::new(&desc, &paths)
    .with_options(Options {
        black_list_mode: true,
    })
    .build()?;

// 白名单模式（默认）
let mask = FieldMaskBuilder::new(&desc, &paths)
    .build()?; // 使用默认选项
```

## API 参考

### 创建 FieldMask

```rust
// 使用 Builder 模式（推荐）
let mask = FieldMaskBuilder::new(&type_descriptor, &paths)
    .with_options(options)
    .build()?;
```

### 查询方法

```rust
// 检查字段是否存在（适用于结构体）
let (submask, included) = mask.field(field_id);

// 检查数组索引（适用于列表和整数键映射）
let (submask, included) = mask.int(index);

// 检查字符串键（适用于字符串键映射）
let (submask, included) = mask.str("key");

// 检查是否匹配所有元素
let matches_all = mask.all();

// 检查掩码类型
let mask_type = mask.typ(); // 返回 &str

// 检查是否为黑名单模式
let is_blacklist = mask.is_black();

// 检查掩码是否已设置
let exists = mask.exist();
```

### 遍历方法

```rust
mask.for_each_child(|str_key, int_key, child| {
    // str_key: 字符串键（对于 StrMap）
    // int_key: 整数键（对于 Struct、List、IntMap）
    // child: 子掩码
    
    println!("键: {}/{}, 子类型: {}", str_key, int_key, child.typ());
    
    true // 返回 true 继续遍历，false 停止
});
```

## 性能特性

- **零分配路径解析**：使用高效的解析器避免不必要的内存分配
- **AHashMap优化**：使用高性能的 `AHashMap` 实现 O(1) 字段查找
- **FastStr字符串**：使用 `FastStr` 减少字符串分配
- **延迟计算**：只在需要时计算子掩码
- **内存复用**：避免重复的类型描述符
- **类型安全**：编译时保证类型匹配，零运行时开销
- **内存高效**：枚举设计消除冗余字段，只存储实际需要的数据

## 常见错误及解决方案

| 错误类型 | 原因 | 解决方案 |
|---------|------|----------|
| `PathError::SyntaxError` | 路径语法错误 | 检查路径语法，确保符合 JSONPath 规范 |
| `PathError::InvalidCharacter` | 无效字符 | 移除或转义无效字符 |
| `PathError::UnterminatedString` | 未闭合字符串 | 确保所有字符串都有闭合引号 |
| `FieldMaskError::TypeMismatch` | 类型不匹配 | 验证类型描述符与路径的兼容性 |
| `FieldMaskError::FieldNotFound` | 字段不存在 | 确认字段名或ID在结构体中存在 |
| `FieldMaskError::EmptyCollection` | 空的索引或键集合 | 在 `[]` 或 `{}` 中提供有效的索引或键 |
| `FieldMaskError::ConflictError` | 通配符与具体字段冲突 | 避免在同一级别同时使用 `*` 和具体字段 |

## 与 Go 版本的差异

1. **内存安全**：Rust 版本提供编译时内存安全保证
2. **强类型错误**：使用具体的 `FieldMaskError` 和 `PathError` 类型
3. **Builder 模式**：提供更现代化的 API 设计
4. **类型安全**：强类型系统防止运行时错误
5. **性能优化**：零成本抽象和编译器优化
6. **类型安全设计**：合并类型标识和数据字段为一个枚举
7. **内存效率**：消除冗余字段，只存储实际需要的数据

## 许可证

Copyright 2023 ByteDance Inc.

Licensed under the Apache License, Version 2.0

## 贡献

欢迎贡献代码！请确保：

1. 添加适当的测试
2. 更新文档
3. 遵循 Rust 编码规范
4. 通过所有 CI 检查

## 更多示例

查看 `examples/` 目录获取更多使用示例和最佳实践。 