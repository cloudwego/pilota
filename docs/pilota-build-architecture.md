# Pilota-Build 架构设计文档

## 目录

1. [总体架构概述](#1-总体架构概述)
2. [系统分层架构](#2-系统分层架构)
3. [核心数据流](#3-核心数据流)
4. [类型系统详解](#4-类型系统详解)
5. [各模块详细设计](#5-各模块详细设计)
6. [模块间交互关系](#6-模块间交互关系)
7. [使用示例](#7-使用示例)
8. [总结](#8-总结)

## 1. 总体架构概述

Pilota-Build 是一个强大的代码生成框架，用于从接口定义语言（IDL）文件生成 Rust 代码。它支持 Thrift 和 Protobuf 两种主流的 IDL 格式，通过统一的中间表示和模块化的架构实现高效的代码生成。

### 1.1 设计目标

- **多协议支持**：统一的架构支持 Thrift 和 Protobuf 协议
- **高性能编译**：使用 Salsa 增量编译框架提升编译效率
- **可扩展性**：插件系统允许用户自定义代码生成行为
- **工作空间支持**：原生支持 Cargo 工作空间模式
- **类型安全**：强类型的中间表示确保生成代码的正确性

### 1.2 核心特性

- **增量编译**：只重新编译修改的部分
- **并行处理**：支持并行解析和代码生成
- **智能依赖管理**：自动处理跨文件和跨包依赖
- **灵活的代码生成**：支持文件拆分、自定义命名等

## 2. 系统分层架构

### 2.1 架构层次图

```
┌─────────────────────────────────────────────────────────┐
│                      用户接口层                         │
│                    Builder API                          │
├─────────────────────────────────────────────────────────┤
│                       扩展层                            │
│                   Plugin System                         │
├─────────────────────────────────────────────────────────┤
│                     代码生成层                          │
│  ┌─────────────┐  ┌──────────────┐  ┌────────────────┐│
│  │   Thrift    │  │   Protobuf   │  │   Workspace    ││
│  │   Backend   │  │    Backend   │  │   Generator    ││
│  └─────────────┘  └──────────────┘  └────────────────┘│
├─────────────────────────────────────────────────────────┤
│                       中间层                            │
│  ┌─────────────┐  ┌──────────────┐  ┌────────────────┐│
│  │   Context   │  │   Resolver   │  │  Type System   ││
│  │   Manager   │  │              │  │                ││
│  └─────────────┘  └──────────────┘  └────────────────┘│
├─────────────────────────────────────────────────────────┤
│                  中间表示层 (IR)                        │
│         IR → Resolver → RIR → CodegenTy                │
├─────────────────────────────────────────────────────────┤
│                      解析层                             │
│          ┌─────────────┐      ┌──────────────┐         │
│          │   Thrift    │      │   Protobuf   │         │
│          │   Parser    │      │    Parser    │         │
│          └─────────────┘      └──────────────┘         │
├─────────────────────────────────────────────────────────┤
│                    数据存储层                           │
│                  Salsa Database                         │
└─────────────────────────────────────────────────────────┘
```

### 2.2 层次职责说明

1. **用户接口层**：提供简洁的 Builder API，隐藏内部复杂性
2. **扩展层**：插件系统，支持用户自定义行为
3. **代码生成层**：将中间表示转换为目标 Rust 代码
4. **中间层**：核心业务逻辑，包括符号解析、类型系统等
5. **中间表示层**：语言无关的统一表示
6. **解析层**：将 IDL 文件解析为中间表示
7. **数据存储层**：使用 Salsa 提供增量计算能力

## 3. 核心数据流

### 3.1 整体数据流程

```
┌─────────────┐     ┌─────────┐     ┌─────────┐     ┌──────────────┐
│  IDL Files  │ --> │ Parser  │ --> │   IR    │ --> │   Resolver   │
└─────────────┘     └─────────┘     └─────────┘     └──────────────┘
                                                              │
                                                              ▼
┌─────────────┐     ┌─────────┐     ┌─────────┐     ┌──────────────┐
│ Rust Files  │ <-- │ Codegen │ <-- │ Plugins │ <-- │     RIR      │
└─────────────┘     └─────────┘     └─────────┘     └──────────────┘
```

### 3.2 详细处理步骤

1. **输入处理**
   - 读取 IDL 文件（`.thrift` 或 `.proto`）
   - 处理 include/import 指令
   - 构建文件依赖图

2. **解析阶段**
   - Parser 将 IDL 语法解析为 AST
   - Lower 将 AST 转换为初始 IR
   - 生成未解析的符号引用

3. **符号解析**
   - 构建符号表
   - 解析所有符号引用
   - 生成 RIR（Resolved IR）

4. **类型转换**
   - 将 RIR 类型转换为 `CodegenTy`
   - 处理特殊类型（如递归类型）
   - 应用类型标记和属性

5. **插件处理**
   - 遍历所有启用的插件
   - 允许插件修改 RIR
   - 收集插件生成的额外代码

6. **代码生成**
   - 选择对应的 Backend
   - 生成 Rust 代码
   - 格式化和写入文件

## 4. 类型系统详解

### 4.1 类型层次关系

```
IDL Types
    │
    ▼
IR Types (初始类型表示)
    │
    ├── TyKind ([`src/ir/mod.rs`](../src/ir/mod.rs))
    │   ├── 基础类型: String, Bool, I32, etc.
    │   ├── 容器类型: Vec, Set, Map
    │   └── 路径引用: Path (未解析)
    │
    ▼
RIR Types (解析后类型表示)
    │
    ├── Ty ([`src/middle/ty.rs`](../src/middle/ty.rs))
    │   ├── kind: TyKind
    │   └── tags_id: TagId
    │
    ├── Path ([`src/middle/rir.rs`](../src/middle/rir.rs))
    │   ├── kind: DefKind (Type/Value/Mod)
    │   └── did: DefId (已解析的定义ID)
    │
    ▼
`CodegenTy` (代码生成类型)
    │
    ├── 基础类型映射
    │   ├── FastStr → ::pilota::FastStr
    │   ├── String → ::std::string::String
    │   └── ...
    │
    ├── 容器类型映射
    │   ├── Vec → ::std::vec::Vec<T>
    │   ├── Map → ::pilota::AHashMap<K,V>
    │   └── ...
    │
    └── ADT (代数数据类型)
        ├── Struct
        ├── Enum
        └── NewType
```

### 4.2 类型转换流程

```rust
// IR 中的类型（未解析）
pub enum TyKind {
    Path(Path),  // Path { segments: ["User", "Status"] }
    // ...
}

// RIR 中的类型（已解析）
pub struct Path {
    pub kind: DefKind,  // Type/Value/Mod
    pub did: DefId,     // 定义的唯一标识符
}

// 代码生成类型
pub enum CodegenTy {
    Adt(AdtDef),  // AdtDef { did: DefId, kind: Struct/Enum/NewType }
    // ...
}
```

### 4.3 类型系统特性

1. **延迟解析**：路径引用在解析阶段才转换为具体定义
2. **类型标记**：通过 TagId 支持自定义属性和行为
3. **智能转换**：不同上下文下的类型转换（如常量上下文）
4. **递归类型处理**：自动检测并处理递归类型

## 5. 各模块详细设计

### 5.1 Parser 模块 ([`src/parser/`](../src/parser/))

#### 职责
将 IDL 文件解析为语言无关的中间表示。

#### 核心组件

```rust
// 通用解析器接口
pub trait Parser {
    fn input<P: AsRef<Path>>(&mut self, path: P);
    fn include_dirs(&mut self, dirs: Vec<PathBuf>);
    fn parse(self) -> ParseResult;
}

// 解析结果
pub struct ParseResult {
    pub files: Vec<Arc<File>>,
    pub input_files: Vec<FileId>,
    pub file_ids_map: FxHashMap<Arc<PathBuf>, FileId>,
}
```

#### Thrift Parser
- 使用 `pilota-thrift-parser` 解析 Thrift 语法
- `ThriftLower` 负责 AST → IR 转换
- 支持 namespace、include、typedef 等特性

#### Protobuf Parser
- 使用 `protobuf-parse` 解析 Proto 文件
- 处理 package、import、option 等
- 支持 Proto2 和 Proto3 语法

### 5.2 IR 模块 ([`src/ir/`](../src/ir/))

#### 职责
定义语言无关的初始中间表示。

#### 核心类型

```rust
// 顶层项
pub enum ItemKind {
    Message(Message),    // 结构体/消息
    Enum(Enum),         // 枚举
    Service(Service),   // 服务接口
    NewType(NewType),   // 类型别名
    Const(Const),       // 常量
    Mod(Mod),          // 模块
    Use(Use),          // 导入
}

// 类型定义
pub struct Ty {
    pub tags: Arc<Tags>,
    pub kind: TyKind,
}

// 路径（未解析）
pub struct Path {
    pub segments: Arc<[Ident]>,  // ["package", "Type"]
}
```

### 5.3 Middle 模块 ([`src/middle/`](../src/middle/))

#### 5.3.1 Context ([`context.rs`](../src/middle/context.rs))

**职责**：维护全局编译上下文

```rust
pub struct Context {
    pub source_type: SourceType,              // Thrift/Protobuf
    pub db: salsa::Snapshot<RootDatabase>,    // 数据库快照
    pub adjusts: Arc<DashMap<DefId, Adjust>>, // 类型调整信息
    pub services: Arc<[IdlService]>,          // 服务定义
    pub codegen_items: Arc<[DefId]>,          // 待生成项
    pub mode: Arc<Mode>,                      // 编译模式
    pub plugin_gen: Arc<DashMap<DefLocation, String>>, // 插件生成代码
    // ...
}
```

**关键功能**：
- 符号名称解析
- 路径计算
- 默认值处理
- 插件执行

#### 5.3.2 Resolver ([`resolver.rs`](../src/middle/resolver.rs))

**职责**：符号解析和符号表构建

```rust
pub struct Resolver {
    pub(crate) did_counter: DefId,                    // 定义ID计数器
    pub(crate) file_sym_map: FxHashMap<FileId, SymbolTable>, // 文件符号表
    def_modules: FxHashMap<DefId, ModuleData>,        // 模块数据
    nodes: FxHashMap<DefId, Node>,                    // 节点信息
    tags: FxHashMap<TagId, Arc<Tags>>,                // 标签信息
    // ...
}
```

**解析流程**：
1. 收集所有定义，分配 DefId
2. 构建符号表
3. 解析路径引用
4. 生成 RIR

#### 5.3.3 Type System ([`ty.rs`](../src/middle/ty.rs))

**职责**：类型定义和转换

```rust
// RIR 中的类型
pub struct Ty {
    pub kind: TyKind,
    pub tags_id: TagId,
}

// 代码生成类型
pub enum CodegenTy {
    FastStr,
    String,
    Vec(Arc<CodegenTy>),
    Map(Arc<CodegenTy>, Arc<CodegenTy>),
    Adt(AdtDef),
    // ...
}

// 类型转换器
pub trait TyTransformer {
    fn codegen_item_ty(&self, ty: &TyKind) -> CodegenTy;
    // ...
}
```

### 5.4 Codegen 模块 ([`src/codegen/`](../src/codegen/))

#### 职责
将 RIR 转换为 Rust 代码。

#### 5.4.1 Backend Trait

```rust
pub trait CodegenBackend: Clone {
    const PROTOCOL: &'static str;
    
    fn cx(&self) -> &Context;
    fn codegen_struct_impl(&self, def_id: DefId, stream: &mut String, s: &rir::Message);
    fn codegen_service_impl(&self, def_id: DefId, stream: &mut String, s: &rir::Service);
    fn codegen_enum_impl(&self, def_id: DefId, stream: &mut String, e: &rir::Enum);
    fn codegen_newtype_impl(&self, def_id: DefId, stream: &mut String, t: &rir::NewType);
}
```

#### 5.4.2 Backend 实现

**ThriftBackend** ([`thrift/mod.rs`](../src/codegen/thrift/mod.rs))：
- 生成 Thrift 协议的编解码代码
- 支持 Binary、Compact 等编码格式
- 处理 field ID、required/optional 等

**ProtobufBackend** ([`protobuf/mod.rs`](../src/codegen/protobuf/mod.rs))：
- 生成 Protobuf 协议的编解码代码
- 支持 proto2/proto3 语法
- 处理 field number、repeated 等

**PbBackend** ([`pb/mod.rs`](../src/codegen/pb/mod.rs))：
- 新的 Protobuf 实现
- 使用 pilota 自己的编解码库
- 更好的性能和更小的代码体积

#### 5.4.3 代码生成流程

```rust
impl<B> Codegen<B> {
    pub fn write_item(&self, stream: &mut String, item: CodegenItem, dup: &mut AHashMap<FastStr, Vec<DefId>>) {
        match &*self.item(item.def_id).unwrap() {
            Item::Message(s) => self.write_struct(item.def_id, stream, s),
            Item::Enum(e) => self.write_enum(item.def_id, stream, e),
            Item::Service(s) => self.write_service(item.def_id, stream, s),
            Item::NewType(t) => self.write_new_type(item.def_id, stream, t),
            Item::Const(c) => self.write_const(item.def_id, stream, c),
            Item::Mod(m) => self.write_mod(stream, m.clone()),
        }
    }
}
```

### 5.5 Plugin 模块 ([`src/plugin/`](../src/plugin/))

#### 职责
提供可扩展的插件机制。

#### Plugin Trait

```rust
pub trait Plugin: Sync + Send {
    // 代码生成单元级别的钩子
    fn on_codegen_uint(&mut self, cx: &Context, items: &[DefId]);
    
    // 项级别的钩子
    fn on_item(&mut self, cx: &Context, def_id: DefId, item: Arc<Item>);
    
    // 字段级别的钩子
    fn on_field(&mut self, cx: &Context, def_id: DefId, f: Arc<Field>);
    
    // 枚举变体级别的钩子
    fn on_variant(&mut self, cx: &Context, def_id: DefId, variant: Arc<EnumVariant>);
    
    // 最终输出前的钩子
    fn on_emit(&mut self, cx: &Context);
}
```

#### 内置插件

1. **SerdePlugin**
   - 为结构体添加 `#[derive(Serialize, Deserialize)]`
   - 处理字段重命名等 serde 属性

2. **BoxedPlugin**
   - 检测递归类型
   - 自动添加 `Box<T>` 包装

3. **ImplDefaultPlugin**
   - 为符合条件的类型实现 Default trait

4. **AutoDerivePlugin**
   - 基于谓词函数自动派生 trait
   - 支持传递性推导

### 5.6 Database 模块 ([`src/db.rs`](../src/db.rs))

#### 职责
使用 Salsa 框架实现增量编译。

#### 数据库定义

```rust
#[salsa::database(RirDatabase)]
pub struct RootDatabase {
    storage: salsa::Storage<RootDatabase>,
}

pub trait RirDatabase {
    // 输入数据
    #[salsa::input]
    fn nodes(&self) -> Arc<FxHashMap<DefId, rir::Node>>;
    
    #[salsa::input]
    fn files(&self) -> Arc<FxHashMap<FileId, Arc<rir::File>>>;
    
    // 派生查询
    fn node(&self, def_id: DefId) -> Option<rir::Node>;
    fn item(&self, def_id: DefId) -> Option<Arc<rir::Item>>;
    fn codegen_ty(&self, def_id: DefId) -> CodegenTy;
}
```

#### 增量计算
- 输入变化时自动失效相关缓存
- 只重新计算受影响的部分
- 支持并行查询

### 5.7 Workspace 模块 ([`src/codegen/workspace.rs`](../src/codegen/workspace.rs))

#### 职责
处理多包工作空间的代码生成。

#### 核心功能

1. **依赖分析**
   - 构建包依赖图
   - 确定生成顺序

2. **Cargo.toml 生成**
   ```rust
   fn create_crate(&self, base_dir: impl AsRef<Path>, info: CrateInfo) -> Result<()> {
       // 生成 Cargo.toml
       // 创建目录结构
       // 写入生成的代码
   }
   ```

3. **路径管理**
   - 计算相对路径
   - 处理循环依赖

## 6. 模块间交互关系

### 6.1 核心交互流程

```
┌─────────┐      ┌─────────┐      ┌──────────┐
│ Builder │ ---> │ Parser  │ ---> │ Resolver │
└─────────┘      └─────────┘      └──────────┘
     │                                   │
     │                                   ▼
     │           ┌─────────┐      ┌──────────┐
     └---------> │ Context │ <--- │ Database │
                 └─────────┘      └──────────┘
                      │                  ▲
                      ▼                  │
                 ┌─────────┐      ┌──────────┐
                 │ Plugin  │ ---> │ Codegen  │
                 └─────────┘      └──────────┘
```

### 6.2 关键接口和数据流

1. **Builder → Parser**
   - Builder 配置 Parser（输入文件、include 目录等）
   - Parser 返回 ParseResult

2. **Parser → Resolver**
   - Parser 生成初始 IR
   - Resolver 解析符号引用，生成 RIR

3. **Resolver → Database**
   - Resolver 将 RIR 存储到 Database
   - Database 提供查询接口

4. **Context ↔ All**
   - Context 持有 Database 快照
   - 所有模块通过 Context 访问全局状态

5. **Plugin → RIR**
   - Plugin 通过 Context 访问和修改 RIR
   - 支持多阶段处理

6. **Codegen → Output**
   - Codegen 查询 Database 获取 RIR
   - 生成 Rust 代码并写入文件

### 6.3 错误处理和诊断

1. **解析错误**：Parser 收集并报告语法错误
2. **符号解析错误**：Resolver 报告未定义符号等错误
3. **类型错误**：类型系统检查类型兼容性
4. **生成错误**：Codegen 处理生成过程中的错误

### 6.4 性能优化

1. **增量编译**：Salsa 自动处理增量更新
2. **并行处理**：文件级别的并行解析
3. **缓存策略**：符号表和类型信息缓存
4. **延迟计算**：按需计算类型信息

## 7. 使用示例

### 7.1 基本使用

```rust
use pilota_build::{Builder, Output};

fn main() {
    Builder::thrift()
        .include_dirs(vec!["idl/"])
        .compile(vec!["service.thrift"], Output::File("src/gen.rs"));
}
```

### 7.2 高级配置

```rust
Builder::protobuf()
    .include_dirs(vec!["proto/"])
    .plugin(MyCustomPlugin)
    .split_generated_files(true)
    .keep_unknown_fields(vec!["proto/message.proto"])
    .compile(
        vec!["service.proto"],
        Output::Workspace("generated/")
    );
```

### 7.3 自定义插件

```rust
struct MyPlugin;

impl Plugin for MyPlugin {
    fn on_item(&mut self, cx: &Context, def_id: DefId, item: Arc<Item>) {
        if let Item::Message(msg) = &*item {
            // 为所有消息添加自定义 trait
            cx.with_adjust_mut(def_id, |adj| {
                adj.add_attrs(&[FastStr::from("#[derive(MyTrait)]")]);
            });
        }
    }
}
```

## 8. 总结

Pilota-Build 通过清晰的分层架构和模块化设计，实现了一个功能强大、性能优异、易于扩展的代码生成框架。其核心优势包括：

1. **统一的中间表示**：IR 和 RIR 提供了语言无关的抽象
2. **强大的类型系统**：支持复杂类型的正确处理
3. **灵活的插件机制**：允许深度定制生成行为
4. **优秀的性能**：增量编译和并行处理
5. **完善的工具链集成**：原生支持 Cargo 工作空间