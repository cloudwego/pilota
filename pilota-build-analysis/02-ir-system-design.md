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

## 9. 实现指南

### 9.1 项目结构

```
pilota-build/
├── crates/
│   ├── pilota-build-hir/      # HIR 定义和操作
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── ast.rs         # HIR AST 定义
│   │   │   ├── visit.rs       # Visitor trait
│   │   │   ├── lower.rs       # AST → HIR
│   │   │   └── pretty.rs      # Pretty printer
│   │   └── Cargo.toml
│   │
│   ├── pilota-build-mir/      # MIR 定义和操作
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── mir.rs         # MIR 定义
│   │   │   ├── lower.rs       # HIR → MIR
│   │   │   ├── typeck.rs      # 类型检查
│   │   │   └── validate.rs    # MIR 验证
│   │   └── Cargo.toml
│   │
│   └── pilota-build-lir/      # LIR 定义和代码生成
│       ├── src/
│       │   ├── lib.rs
│       │   ├── lir.rs         # LIR 定义
│       │   ├── lower.rs       # MIR → LIR
│       │   ├── codegen.rs     # 代码生成
│       │   └── optimize.rs    # LIR 优化
│       └── Cargo.toml
```

### 9.2 实现步骤

#### Step 1: HIR 基础设施（Week 1-2）

```rust
// pilota-build-hir/src/ast.rs
#[derive(Debug, Clone)]
pub struct HirCrate {
    pub items: Vec<Item>,
    pub span: Span,
}

impl HirCrate {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            span: DUMMY_SPAN,
        }
    }
    
    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }
}

// pilota-build-hir/src/visit.rs
pub trait Visitor: Sized {
    fn visit_item(&mut self, item: &Item) {
        walk_item(self, item);
    }
    
    fn visit_type(&mut self, ty: &Type) {
        walk_type(self, ty);
    }
    
    // ... 其他 visit 方法
}

pub fn walk_item<V: Visitor>(visitor: &mut V, item: &Item) {
    match &item.kind {
        ItemKind::Message(msg) => {
            for field in &msg.fields {
                visitor.visit_field(field);
            }
        }
        // ... 其他情况
    }
}
```

#### Step 2: MIR 转换（Week 3-4）

```rust
// pilota-build-mir/src/lower.rs
pub struct HirToMirCtx<'tcx> {
    tcx: TyCtxt<'tcx>,
    resolver: Resolver,
    current_module: DefId,
}

impl<'tcx> HirToMirCtx<'tcx> {
    pub fn lower_crate(&mut self, hir: &HirCrate) -> MirCrate {
        let mut mir = MirCrate::new();
        
        // Phase 1: 收集所有定义
        for item in &hir.items {
            self.collect_item(item);
        }
        
        // Phase 2: 解析和类型检查
        for item in &hir.items {
            let mir_item = self.lower_item(item);
            mir.add_item(mir_item);
        }
        
        mir
    }
    
    fn collect_item(&mut self, item: &Item) {
        let def_id = self.tcx.create_def_id();
        self.resolver.define(item.name(), def_id);
    }
}
```

#### Step 3: 类型检查集成（Week 5-6）

```rust
// pilota-build-mir/src/typeck.rs
pub struct TypeChecker<'tcx> {
    tcx: TyCtxt<'tcx>,
    infcx: InferCtxt<'tcx>,
    errors: Vec<TypeError>,
}

impl<'tcx> TypeChecker<'tcx> {
    pub fn check_item(&mut self, item: &MirItem) -> Result<(), TypeError> {
        match &item.kind {
            MirItemKind::Message(msg) => self.check_message(msg),
            MirItemKind::Enum(e) => self.check_enum(e),
            // ... 其他类型
        }
    }
    
    fn check_message(&mut self, msg: &MirMessage) -> Result<(), TypeError> {
        // 检查字段类型
        for field in &msg.fields {
            self.check_type(&field.ty)?;
        }
        
        // 检查字段 ID 唯一性
        let mut seen_ids = FxHashSet::default();
        for field in &msg.fields {
            if !seen_ids.insert(field.id) {
                return Err(TypeError::DuplicateFieldId {
                    message: msg.name,
                    field_id: field.id,
                });
            }
        }
        
        Ok(())
    }
}
```

#### Step 4: LIR 代码生成（Week 7-8）

```rust
// pilota-build-lir/src/codegen.rs
pub struct CodeGenerator {
    output: String,
    indent: usize,
}

impl CodeGenerator {
    pub fn generate_item(&mut self, item: &LirItem) {
        match item {
            LirItem::Struct { name, fields, derives, .. } => {
                self.generate_derives(derives);
                self.writeln(&format!("pub struct {} {{", name));
                self.indent();
                
                for field in fields {
                    self.generate_field(field);
                }
                
                self.dedent();
                self.writeln("}");
            }
            // ... 其他类型
        }
    }
    
    fn generate_field(&mut self, field: &NamedField) {
        for attr in &field.attrs {
            self.writeln(&format!("#[{}]", attr));
        }
        self.writeln(&format!(
            "{} {}: {},",
            field.vis,
            field.name,
            self.format_type(&field.ty)
        ));
    }
}
```

### 9.3 测试策略

#### 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hir_construction() {
        let mut hir = HirCrate::new();
        let item = create_test_item();
        hir.add_item(item);
        
        assert_eq!(hir.items.len(), 1);
    }
    
    #[test]
    fn test_mir_lowering() {
        let hir = create_test_hir();
        let mut ctx = HirToMirCtx::new();
        let mir = ctx.lower_crate(&hir);
        
        assert_eq!(mir.items.len(), hir.items.len());
    }
}
```

#### 集成测试

```rust
// tests/ir_pipeline.rs
#[test]
fn test_full_pipeline() {
    let input = include_str!("fixtures/test.thrift");
    
    // Parse to AST
    let ast = parse_thrift(input).unwrap();
    
    // Lower to HIR
    let hir = ast_to_hir(ast);
    
    // Lower to MIR
    let mir = hir_to_mir(hir);
    
    // Lower to LIR
    let lir = mir_to_lir(mir);
    
    // Generate code
    let output = generate_code(lir);
    
    // Compare with expected
    let expected = include_str!("fixtures/test.rs");
    assert_eq!(output, expected);
}
```

### 9.4 性能考虑

#### 内存池使用

```rust
thread_local! {
    static HIR_NODE_POOL: RefCell<Vec<Box<HirNode<()>>>> = RefCell::new(Vec::new());
}

pub fn alloc_hir_node<T>(kind: T) -> HirNode<T> {
    // 尝试从池中获取
    let recycled = HIR_NODE_POOL.with(|pool| {
        pool.borrow_mut().pop()
    });
    
    // 创建新节点或重用
    match recycled {
        Some(mut node) => {
            // 重用节点，更新内容
            unsafe { std::mem::transmute(node) }
        }
        None => {
            HirNode::new(kind)
        }
    }
}
```

#### 并行化点

1. **文件级并行**：不同文件的 HIR 构建可以并行
2. **项级并行**：独立项的 MIR 降级可以并行
3. **类型检查并行**：无依赖的类型检查可以并行
4. **代码生成并行**：不同模块的代码生成可以并行

### 9.5 调试支持

#### IR Dump 工具

```rust
pub trait IrDump {
    fn dump(&self, writer: &mut dyn Write) -> io::Result<()>;
    fn dump_pretty(&self, writer: &mut dyn Write) -> io::Result<()>;
}

impl IrDump for HirCrate {
    fn dump(&self, writer: &mut dyn Write) -> io::Result<()> {
        writeln!(writer, "HIR Crate {{")?;
        for item in &self.items {
            item.dump(writer)?;
        }
        writeln!(writer, "}}")?;
        Ok(())
    }
}
```

#### 可视化工具

```rust
pub fn generate_dot_graph(mir: &MirCrate) -> String {
    let mut dot = String::from("digraph MIR {\n");
    
    for (def_id, item) in &mir.items {
        dot.push_str(&format!("  {} [label=\"{}\"];\n", def_id, item.name));
        
        // 添加依赖边
        for dep in item.dependencies() {
            dot.push_str(&format!("  {} -> {};\n", def_id, dep));
        }
    }
    
    dot.push_str("}\n");
    dot
}
```

### 9.6 迁移策略

1. **并行运行**：新旧系统并行运行，对比输出
2. **逐步切换**：通过 feature flag 控制使用新旧系统
3. **回退机制**：保留快速回退到旧系统的能力
4. **性能监控**：持续监控新系统的性能表现