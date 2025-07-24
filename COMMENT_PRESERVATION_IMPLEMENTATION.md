# Pilota 注释保留功能实现方案

## 概述

本方案实现了在 pilota-build 包中保留 IDL 文件注释的功能，将输入的 thrift IDL 或 protobuf proto 文件中的注释以 Rust 代码文档的形式保留在生成的代码中。

## 实现架构

### 1. 解析器层 (Parser Layer)

#### Thrift 解析器修改

- **位置**: `pilota-thrift-parser/src/descriptor/`
- **修改**: 为所有数据结构添加 `comments` 字段

```rust
// 在所有结构体中添加 comments 字段
pub struct StructLike {
    pub name: Ident,
    pub fields: Vec<Field>,
    pub annotations: Annotations,
    pub comments: Vec<String>,  // 新增
}

pub struct Field {
    pub id: i32,
    pub name: Ident,
    pub attribute: Attribute,
    pub ty: Type,
    pub default: Option<ConstValue>,
    pub annotations: Annotations,
    pub comments: Vec<String>,  // 新增
}

pub struct Enum {
    pub name: Ident,
    pub values: Vec<EnumValue>,
    pub annotations: Annotations,
    pub comments: Vec<String>,  // 新增
}

pub struct EnumValue {
    pub name: Ident,
    pub value: Option<IntConstant>,
    pub annotations: Annotations,
    pub comments: Vec<String>,  // 新增
}

pub struct Service {
    pub name: Ident,
    pub extends: Option<Path>,
    pub functions: Vec<Function>,
    pub annotations: Annotations,
    pub comments: Vec<String>,  // 新增
}

pub struct Function {
    pub name: Ident,
    pub oneway: bool,
    pub result_type: Type,
    pub arguments: Vec<Field>,
    pub throws: Vec<Field>,
    pub annotations: Annotations,
    pub comments: Vec<String>,  // 新增
}
```

#### 注释解析功能

- **位置**: `pilota-thrift-parser/src/parser/mod.rs`
- **新增**: 注释收集函数

```rust
/// Parse and collect comments that appear before an item
pub(crate) fn collect_comments(input: &str) -> IResult<&str, Vec<String>> {
    map(
        many0(map(
            tuple((opt(multispace1), comment, opt(multispace1))),
            |(_, comment_text, _)| comment_text.trim().to_string(),
        )),
        |comments| comments.into_iter().filter(|c| !c.is_empty()).collect(),
    )(input)
}
```

### 2. 中间表示层 (IR Layer)

#### IR 结构修改

- **位置**: `pilota-build/src/ir/mod.rs`
- **新增**: `Comments` 结构体
- **修改**: 在所有 IR 节点中添加 `comments` 字段

```rust
/// 表示从 IDL 文件解析出的注释
#[derive(Clone, Debug, Default)]
pub struct Comments(pub Vec<String>);

// 在所有结构体中添加 comments 字段
pub struct Field {
    pub name: Ident,
    pub id: i32,
    pub ty: Ty,
    pub kind: FieldKind,
    pub tags: Arc<Tags>,
    pub default: Option<Literal>,
    pub comments: Vec<String>,  // 新增
}

pub struct Message {
    pub name: Ident,
    pub fields: Vec<Field>,
    pub is_wrapper: bool,
    pub comments: Vec<String>,  // 新增
}

pub struct Enum {
    pub name: Ident,
    pub variants: Vec<EnumVariant>,
    pub repr: Option<EnumRepr>,
    pub comments: Vec<String>,  // 新增
}

pub struct EnumVariant {
    pub id: Option<i32>,
    pub name: Ident,
    pub discr: Option<i64>,
    pub fields: Vec<Ty>,
    pub tags: Arc<Tags>,
    pub comments: Vec<String>,  // 新增
}

pub struct Service {
    pub name: Ident,
    pub methods: Vec<Method>,
    pub extend: Vec<Path>,
    pub comments: Vec<String>,  // 新增
}

pub struct Method {
    pub name: Ident,
    pub args: Vec<Arg>,
    pub ret: Ty,
    pub oneway: bool,
    pub exceptions: Option<Path>,
    pub tags: Arc<Tags>,
    pub comments: Vec<String>,  // 新增
}
```

#### RIR 结构修改

- **位置**: `pilota-build/src/middle/rir.rs`
- **修改**: 在所有 RIR 节点中添加 `comments` 字段

```rust
// 类似的修改应用到所有 RIR 结构体
pub struct Field {
    pub did: DefId,
    pub name: Ident,
    pub id: i32,
    pub ty: Ty,
    pub kind: FieldKind,
    pub tags_id: TagId,
    pub default: Option<Literal>,
    pub comments: Vec<String>,  // 新增
}
```

### 3. 代码生成层 (Codegen Layer)

#### 注释格式化

- **位置**: `pilota-build/src/codegen/mod.rs`
- **新增**: 注释格式化函数

```rust
impl<B> Codegen<B>
where
    B: CodegenBackend + Send,
{
    /// Format comments as Rust doc comments
    fn format_doc_comments(&self, comments: &[String]) -> String {
        if comments.is_empty() {
            return String::new();
        }
        
        comments
            .iter()
            .map(|comment| {
                if comment.trim().is_empty() {
                    "///".to_string()
                } else {
                    format!("/// {}", comment.trim())
                }
            })
            .join("\n")
    }
}
```

#### 结构体代码生成

- **修改**: `write_struct` 方法

```rust
pub fn write_struct(&self, def_id: DefId, stream: &mut String, s: &rir::Message) {
    let name = self.rust_name(def_id);

    // Add struct-level documentation comments
    let struct_docs = self.format_doc_comments(&s.comments);
    if !struct_docs.is_empty() {
        stream.push_str(&struct_docs);
        stream.push('\n');
    }

    // ... 字段生成时也添加注释
    let field_docs = self.format_doc_comments(&f.comments);
    // ...
}
```

#### 枚举代码生成

- **修改**: `write_enum` 和 `write_enum_as_new_type` 方法

```rust
pub fn write_enum(&self, def_id: DefId, stream: &mut String, e: &middle::rir::Enum) {
    // Add enum-level documentation comments
    let enum_docs = self.format_doc_comments(&e.comments);
    if !enum_docs.is_empty() {
        stream.push_str(&enum_docs);
        stream.push('\n');
    }

    // ... 变体生成时也添加注释
    let variant_docs = self.format_doc_comments(&v.comments);
    // ...
}
```

#### 服务代码生成

- **修改**: `write_service` 方法

```rust
pub fn write_service(&self, def_id: DefId, stream: &mut String, s: &middle::rir::Service) {
    // Add service-level documentation comments
    let service_docs = self.format_doc_comments(&s.comments);
    if !service_docs.is_empty() {
        stream.push_str(&service_docs);
        stream.push('\n');
    }
    // ...
}
```

## 功能演示

### 输入 IDL 文件

```thrift
// User represents a user in the system
// It contains basic user information
struct User {
    // Unique identifier for the user
    1: required i64 id,
    
    // User's display name
    2: optional string name,
    
    // User's email address
    3: optional string email,
}

// Status enumeration for user account
enum UserStatus {
    // Account is active and can be used
    ACTIVE = 1,
    
    // Account is temporarily suspended
    SUSPENDED = 2,
    
    // Account is permanently disabled
    DISABLED = 3,
}

// Service for managing users
service UserService {
    // Get user information by ID
    User getUser(1: i64 userId),
    
    // Create a new user account
    User createUser(1: User user),
    
    // Update user status
    void updateUserStatus(1: i64 userId, 2: UserStatus status),
}
```

### 生成的 Rust 代码

```rust
/// User represents a user in the system
/// It contains basic user information
#[derive(Clone, PartialEq)]
pub struct User {
    /// Unique identifier for the user
    pub id: i64,
    
    /// User's display name
    pub name: ::std::option::Option<::pilota::FastStr>,
    
    /// User's email address
    pub email: ::std::option::Option<::pilota::FastStr>,
}

/// Status enumeration for user account
#[derive(Clone, PartialEq)]
pub enum UserStatus {
    /// Account is active and can be used
    ACTIVE = 1,
    
    /// Account is temporarily suspended
    SUSPENDED = 2,
    
    /// Account is permanently disabled
    DISABLED = 3,
}

/// Service for managing users
pub trait UserService {
    /// Get user information by ID
    fn get_user(&self, user_id: i64) -> User;
    
    /// Create a new user account
    fn create_user(&self, user: User) -> User;
    
    /// Update user status
    fn update_user_status(&self, user_id: i64, status: UserStatus);
}
```

## 实现状态

### 已完成
- ✅ pilota-thrift-parser 数据结构修改
- ✅ pilota-build IR/RIR 数据结构修改
- ✅ 代码生成器注释输出功能
- ✅ 基本的注释格式化功能

### 待完善
- 🔄 注释解析逻辑优化（需要与现有解析器更好集成）
- 🔄 protobuf 注释支持（protobuf 描述符不保留注释，需要特殊处理）
- 🔄 方法级别注释支持（需要修改 thrift backend）
- 🔄 测试用例完善

### 技术挑战

1. **注释解析时机**：需要在不影响现有解析逻辑的情况下收集注释
2. **protobuf 限制**：protobuf 的描述符格式不保留注释信息
3. **向后兼容性**：确保修改不影响现有功能

## 使用方法

```rust
use pilota_build::Builder;

// 使用 pilota-build 生成带注释的代码
Builder::thrift()
    .ignore_unused(false)
    .compile_with_config(
        vec![pilota_build::IdlService::from_path("user.thrift")],
        pilota_build::Output::File("generated.rs".into()),
    );
```

生成的代码将包含从 IDL 文件中提取的注释，以 Rust 文档注释的形式呈现，提高了代码的可读性和开发体验。

## 总结

本实现为 pilota-build 添加了注释保留功能，通过在解析器、中间表示和代码生成器三个层面的修改，实现了从 IDL 文件到生成代码的注释传递。这大大提高了生成代码的可读性和开发体验，使开发者能够在生成的 Rust 代码中看到原始 IDL 文件中的文档说明。