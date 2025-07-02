# 新的 IR 系统设计

## 1. 设计理念

### 1.1 核心原则

1. **渐进式降级**：从高级到低级，每层去除一些抽象
2. **类型安全**：IR 本身是类型安全的 Rust 代码
3. **可扩展性**：易于添加新的节点类型和属性
4. **增量友好**：设计时考虑增量编译需求
5. **并行友好**：避免共享可变状态

### 1.2 三层 IR 体系

```
HIR (High-level IR)
 ├── 保留源语言所有信息
 ├── 未解析的名称引用
 └── 适合语法分析和早期检查

MIR (Mid-level IR) 
 ├── 符号已解析
 ├── 类型已推导
 └── 适合语义分析和优化

LIR (Low-level IR)
 ├── 接近目标语言
 ├── 所有高级特性已展开
 └── 适合代码生成
```

## 2. HIR 设计

### 2.1 基础定义

```rust
use crate::span::Span;
use crate::symbol::{Symbol, FileId};

/// HIR 的唯一标识符
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct HirId {
    pub owner: DefId,
    pub local_id: LocalId,
}

/// 源码位置信息
#[derive(Clone)]
pub struct HirNode<T> {
    pub id: HirId,
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub kind: T,
}

/// 顶级定义
pub enum ItemKind {
    Service(Service),
    Message(Message),
    Enum(Enum),
    Const(Const),
    TypeAlias(TypeAlias),
    Module(Module),
    Use(Use),
}

pub type Item = HirNode<ItemKind>;
```

### 2.2 类型表示（未解析）

```rust
/// HIR 中的类型 - 可能包含未解析的路径
pub enum Type {
    /// 基础类型
    Primitive(PrimitiveType),
    /// 路径引用（未解析）
    Path(Path),
    /// 容器类型
    Vec(Box<Type>),
    Set(Box<Type>),
    Map {
        key: Box<Type>,
        value: Box<Type>,
    },
    /// 可选类型
    Optional(Box<Type>),
    /// 引用（用于递归类型）
    Reference {
        lifetime: Option<Symbol>,
        mutable: bool,
        ty: Box<Type>,
    },
}

/// 未解析的路径
pub struct Path {
    pub segments: Vec<PathSegment>,
    pub resolution: Cell<Option<Resolution>>,
}

pub struct PathSegment {
    pub ident: Symbol,
    pub args: Option<GenericArgs>,
}
```

### 2.3 表达式和模式

```rust
/// HIR 表达式 - 用于常量定义
pub enum ExprKind {
    Literal(Literal),
    Path(Path),
    List(Vec<Expr>),
    Map(Vec<(Expr, Expr)>),
    Struct {
        path: Path,
        fields: Vec<FieldExpr>,
    },
    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
    },
}

pub type Expr = HirNode<ExprKind>;

/// 字面量
pub enum Literal {
    Bool(bool),
    Int(i64, IntTy),
    Float(f64, FloatTy),
    String(Symbol),
    Bytes(Vec<u8>),
}
```

## 3. MIR 设计

### 3.1 核心特点

MIR 是类型检查后的表示：
- 所有符号已解析为 DefId
- 所有类型已完全确定
- 添加了类型信息和语义标注

```rust
/// MIR 定义
pub struct Mir {
    pub items: FxHashMap<DefId, MirItem>,
    pub type_cache: FxHashMap<DefId, Ty>,
    pub impl_blocks: FxHashMap<DefId, Vec<ImplBlock>>,
}

pub struct MirItem {
    pub def_id: DefId,
    pub visibility: Visibility,
    pub attrs: Attributes,
    pub kind: MirItemKind,
}

pub enum MirItemKind {
    Service(MirService),
    Message(MirMessage),
    Enum(MirEnum),
    Const(MirConst),
    TypeAlias(MirTypeAlias),
}
```

### 3.2 类型系统

```rust
/// MIR 中的类型 - 完全解析
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Ty {
    /// 基础类型
    Bool,
    I8, I16, I32, I64,
    U8, U16, U32, U64,
    F32, F64,
    String,
    Bytes,
    
    /// ADT（代数数据类型）
    Adt(DefId, SubstsRef),
    
    /// 容器类型
    Vec(Box<Ty>),
    Set(Box<Ty>),
    Map(Box<Ty>, Box<Ty>),
    
    /// 函数类型
    FnDef(DefId, SubstsRef),
    
    /// 引用类型
    Ref(Region, Box<Ty>, Mutability),
    
    /// 错误类型
    Error,
}

/// 类型参数替换
pub struct Substs {
    pub types: Vec<Ty>,
    pub lifetimes: Vec<Region>,
}
```

### 3.3 语义信息

```rust
pub struct MirMessage {
    pub def_id: DefId,
    pub name: Symbol,
    pub generics: Generics,
    pub fields: Vec<MirField>,
    pub where_clauses: Vec<WherePredicate>,
    /// 派生的 trait
    pub derives: Vec<TraitRef>,
    /// 是否保留未知字段
    pub keep_unknown: bool,
}

pub struct MirField {
    pub def_id: DefId,
    pub name: Symbol,
    pub ty: Ty,
    pub id: FieldId,
    pub attrs: FieldAttrs,
    pub default: Option<ConstValue>,
}

/// 字段属性
pub struct FieldAttrs {
    pub required: bool,
    pub deprecated: Option<String>,
    pub renamed_from: Option<Symbol>,
}
```

## 4. LIR 设计

### 4.1 设计目标

LIR 是最接近目标代码的表示：
- 所有高级特性已展开
- 直接对应到 Rust 代码结构
- 优化已完成

```rust
pub struct Lir {
    pub crates: FxHashMap<CrateId, LirCrate>,
}

pub struct LirCrate {
    pub name: Symbol,
    pub root_module: LirModule,
    pub dependencies: Vec<CrateDep>,
}

pub struct LirModule {
    pub items: Vec<LirItem>,
    pub imports: Vec<UseDecl>,
}
```

### 4.2 代码生成友好的设计

```rust
pub enum LirItem {
    /// 结构体定义
    Struct {
        name: Symbol,
        generics: GenericParams,
        fields: StructFields,
        derives: Vec<Symbol>,
    },
    
    /// 枚举定义
    Enum {
        name: Symbol,
        generics: GenericParams,
        variants: Vec<Variant>,
        derives: Vec<Symbol>,
    },
    
    /// Trait 定义
    Trait {
        name: Symbol,
        generics: GenericParams,
        items: Vec<TraitItem>,
    },
    
    /// 实现块
    Impl {
        generics: GenericParams,
        trait_ref: Option<TraitRef>,
        self_ty: Type,
        items: Vec<ImplItem>,
    },
    
    /// 常量定义
    Const {
        name: Symbol,
        ty: Type,
        value: ConstExpr,
    },
}

/// 结构体字段
pub enum StructFields {
    Named(Vec<NamedField>),
    Tuple(Vec<Type>),
    Unit,
}

pub struct NamedField {
    pub vis: Visibility,
    pub name: Symbol,
    pub ty: Type,
    pub attrs: Vec<Attr>,
}
```

## 5. IR 转换

### 5.1 HIR → MIR

```rust
pub trait HirToMir {
    fn lower_item(&mut self, item: &hir::Item) -> MirItem;
    fn lower_type(&mut self, ty: &hir::Type) -> Ty;
    fn resolve_path(&mut self, path: &hir::Path) -> DefId;
}

impl HirToMir for MirBuilder<'_> {
    fn lower_item(&mut self, item: &hir::Item) -> MirItem {
        match &item.kind {
            hir::ItemKind::Message(msg) => {
                let fields = msg.fields
                    .iter()
                    .map(|f| self.lower_field(f))
                    .collect();
                
                MirItem {
                    def_id: self.def_id(item.id),
                    kind: MirItemKind::Message(MirMessage {
                        name: msg.name,
                        fields,
                        // ... 其他字段
                    }),
                }
            }
            // ... 其他类型
        }
    }
}
```

### 5.2 MIR → LIR

```rust
pub trait MirToLir {
    fn lower_item(&mut self, item: &MirItem) -> Vec<LirItem>;
    fn monomorphize(&mut self, def_id: DefId, substs: &Substs) -> DefId;
}

impl MirToLir for LirBuilder<'_> {
    fn lower_item(&mut self, item: &MirItem) -> Vec<LirItem> {
        let mut items = vec![];
        
        match &item.kind {
            MirItemKind::Message(msg) => {
                // 生成结构体
                items.push(self.build_struct(msg));
                
                // 生成 trait 实现
                if msg.derives.contains(&SERIALIZE_TRAIT) {
                    items.push(self.build_serialize_impl(msg));
                }
                
                // ... 其他实现
            }
            // ... 其他类型
        }
        
        items
    }
}
```

## 6. 增量编译支持

### 6.1 查询定义

```rust
#[salsa::query_group(HirDatabaseStorage)]
pub trait HirDatabase: SourceDatabase {
    /// 获取文件的 HIR
    #[salsa::invoke(parse_file)]
    fn file_hir(&self, file: FileId) -> Arc<Vec<hir::Item>>;
    
    /// 获取项的 HIR
    #[salsa::invoke(get_item_hir)]
    fn item_hir(&self, def_id: DefId) -> Arc<hir::Item>;
}

#[salsa::query_group(MirDatabaseStorage)]
pub trait MirDatabase: HirDatabase {
    /// 类型检查并生成 MIR
    #[salsa::invoke(typeck_item)]
    fn item_mir(&self, def_id: DefId) -> Arc<MirItem>;
    
    /// 获取类型信息
    #[salsa::invoke(item_ty)]
    fn ty(&self, def_id: DefId) -> Ty;
}
```

### 6.2 依赖追踪

```rust
pub struct QueryContext<'db> {
    db: &'db dyn MirDatabase,
    dependencies: RefCell<FxHashSet<QueryDep>>,
}

pub enum QueryDep {
    Item(DefId),
    Type(DefId),
    File(FileId),
}

impl QueryContext<'_> {
    pub fn track_dependency(&self, dep: QueryDep) {
        self.dependencies.borrow_mut().insert(dep);
    }
}
```

## 7. 性能优化

### 7.1 并行处理

```rust
pub fn lower_items_parallel(items: &[hir::Item]) -> Vec<MirItem> {
    items.par_iter()
        .map(|item| {
            let mut builder = MirBuilder::new();
            builder.lower_item(item)
        })
        .collect()
}
```

### 7.2 内存优化

- 使用 `Arc` 共享大型数据结构
- 使用 `SmallVec` 优化小集合
- 延迟计算某些属性
- 使用内存池管理临时对象

## 8. 总结

新的 IR 系统通过三层设计实现了：
1. **更好的关注点分离**：每层专注于特定任务
2. **更强的类型安全**：IR 本身是类型安全的
3. **更好的增量编译**：细粒度的查询和依赖追踪
4. **更好的扩展性**：易于添加新特性和优化