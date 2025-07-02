# 类型系统重构设计

## 1. 设计目标

### 1.1 核心需求

1. **统一的类型表示**：避免多处定义类型，减少转换开销
2. **强大的类型推导**：支持泛型、关联类型、类型约束
3. **灵活的类型检查**：支持结构化类型、名义类型混合
4. **高效的类型查询**：类型信息缓存，快速查询
5. **良好的错误报告**：精确的类型错误位置和建议

### 1.2 设计原则

- **类型即文档**：类型系统应该自描述
- **零成本抽象**：类型检查不应影响运行时性能
- **渐进式类型**：支持部分类型推导
- **可扩展性**：易于添加新类型和规则

## 2. 类型表示

### 2.1 核心类型定义

```rust
/// 类型的核心表示
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Type {
    /// 基础类型
    Primitive(PrimitiveType),
    
    /// 用户定义类型（ADT）
    Adt {
        def_id: DefId,
        substs: Substs,
    },
    
    /// 泛型参数
    TypeParam {
        param_id: ParamId,
        name: Symbol,
    },
    
    /// 关联类型
    AssociatedType {
        def_id: DefId,
        trait_ref: TraitRef,
        name: Symbol,
    },
    
    /// 容器类型
    Container(ContainerType),
    
    /// 函数类型
    Function(FnSig),
    
    /// 引用类型
    Reference {
        lifetime: Lifetime,
        mutability: Mutability,
        pointee: Box<Type>,
    },
    
    /// 原始指针
    RawPointer {
        mutability: Mutability,
        pointee: Box<Type>,
    },
    
    /// 元组
    Tuple(Vec<Type>),
    
    /// 切片
    Slice(Box<Type>),
    
    /// 数组
    Array {
        element: Box<Type>,
        length: ConstValue,
    },
    
    /// Never 类型
    Never,
    
    /// 错误类型（用于错误恢复）
    Error,
    
    /// 推导变量（类型推导中使用）
    Infer(InferType),
}

/// 基础类型
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
    Bool,
    I8, I16, I32, I64, I128,
    U8, U16, U32, U64, U128,
    F32, F64,
    Char,
    Str,
    Bytes,
}

/// 容器类型
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ContainerType {
    Vec(Box<Type>),
    HashSet(Box<Type>),
    HashMap {
        key: Box<Type>,
        value: Box<Type>,
    },
    BTreeSet(Box<Type>),
    BTreeMap {
        key: Box<Type>,
        value: Box<Type>,
    },
    Option(Box<Type>),
    Result {
        ok: Box<Type>,
        err: Box<Type>,
    },
}
```

### 2.2 泛型和参数

```rust
/// 泛型参数
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GenericParams {
    pub params: Vec<GenericParam>,
    pub where_clauses: Vec<WherePredicate>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum GenericParam {
    Type {
        id: ParamId,
        name: Symbol,
        bounds: Vec<TypeBound>,
        default: Option<Type>,
    },
    Lifetime {
        id: ParamId,
        name: Symbol,
        bounds: Vec<LifetimeBound>,
    },
    Const {
        id: ParamId,
        name: Symbol,
        ty: Type,
        default: Option<ConstValue>,
    },
}

/// 类型参数替换
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Substs {
    pub types: FxHashMap<ParamId, Type>,
    pub lifetimes: FxHashMap<ParamId, Lifetime>,
    pub consts: FxHashMap<ParamId, ConstValue>,
}

/// 类型约束
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TypeBound {
    Trait(TraitRef),
    Lifetime(Lifetime),
}

/// Where 子句
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum WherePredicate {
    TypeBound {
        ty: Type,
        bounds: Vec<TypeBound>,
    },
    LifetimeBound {
        lifetime: Lifetime,
        bounds: Vec<Lifetime>,
    },
    EqPredicate {
        lhs: Type,
        rhs: Type,
    },
}
```

### 2.3 Trait 系统

```rust
/// Trait 引用
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TraitRef {
    pub def_id: DefId,
    pub substs: Substs,
}

/// Trait 定义
pub struct TraitDef {
    pub def_id: DefId,
    pub name: Symbol,
    pub generics: GenericParams,
    pub supertraits: Vec<TraitRef>,
    pub items: FxHashMap<Symbol, TraitItem>,
}

/// Trait 项
pub enum TraitItem {
    Type {
        name: Symbol,
        bounds: Vec<TypeBound>,
        default: Option<Type>,
    },
    Method {
        name: Symbol,
        sig: FnSig,
        default: Option<DefId>,
    },
    Const {
        name: Symbol,
        ty: Type,
        default: Option<ConstValue>,
    },
}

/// 实现
pub struct ImplDef {
    pub def_id: DefId,
    pub generics: GenericParams,
    pub trait_ref: Option<TraitRef>,
    pub self_ty: Type,
    pub items: FxHashMap<Symbol, ImplItem>,
}
```

## 3. 类型推导

### 3.1 推导引擎

```rust
pub struct InferenceEngine<'tcx> {
    tcx: TyCtxt<'tcx>,
    /// 类型变量表
    type_vars: UnificationTable<TypeVarId>,
    /// 类型变量的值
    type_var_values: FxHashMap<TypeVarId, Type>,
    /// 约束集合
    constraints: Vec<Constraint>,
    /// 推导上下文
    infcx: InferCtxt<'tcx>,
}

/// 类型推导变量
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum InferType {
    /// 类型变量
    TypeVar(TypeVarId),
    /// 整数类型变量
    IntVar(IntVarId),
    /// 浮点类型变量
    FloatVar(FloatVarId),
}

/// 约束类型
pub enum Constraint {
    /// 类型相等
    Eq(Type, Type),
    /// 子类型关系
    Subtype(Type, Type),
    /// Trait 约束
    TraitBound(Type, TraitRef),
    /// 生命周期约束
    Outlives(Lifetime, Lifetime),
}
```

### 3.2 推导算法

```rust
impl<'tcx> InferenceEngine<'tcx> {
    /// 统一两个类型
    pub fn unify(&mut self, a: Type, b: Type) -> Result<Type, TypeError> {
        match (a, b) {
            // 相同类型
            (a, b) if a == b => Ok(a),
            
            // 推导变量
            (Type::Infer(InferType::TypeVar(v)), ty) |
            (ty, Type::Infer(InferType::TypeVar(v))) => {
                self.unify_var(v, ty)
            }
            
            // ADT 类型
            (Type::Adt { def_id: d1, substs: s1 },
             Type::Adt { def_id: d2, substs: s2 }) if d1 == d2 => {
                let substs = self.unify_substs(s1, s2)?;
                Ok(Type::Adt { def_id: d1, substs })
            }
            
            // 容器类型
            (Type::Container(c1), Type::Container(c2)) => {
                self.unify_container(c1, c2)
            }
            
            // 其他情况
            _ => Err(TypeError::Mismatch(a, b)),
        }
    }
    
    /// 解决约束
    pub fn solve_constraints(&mut self) -> Result<(), TypeError> {
        // 固定点迭代
        loop {
            let mut changed = false;
            
            for constraint in &self.constraints.clone() {
                match constraint {
                    Constraint::Eq(a, b) => {
                        self.unify(a.clone(), b.clone())?;
                        changed = true;
                    }
                    Constraint::TraitBound(ty, trait_ref) => {
                        self.check_trait_bound(ty, trait_ref)?;
                    }
                    // ... 其他约束
                }
            }
            
            if !changed {
                break;
            }
        }
        
        Ok(())
    }
}
```

## 4. 类型检查

### 4.1 类型检查器

```rust
pub struct TypeChecker<'tcx> {
    tcx: TyCtxt<'tcx>,
    engine: InferenceEngine<'tcx>,
    /// 当前作用域的类型环境
    env: TypeEnv,
    /// 错误收集器
    errors: DiagnosticBuilder<'tcx>,
}

/// 类型环境
pub struct TypeEnv {
    /// 变量类型
    vars: FxHashMap<DefId, Type>,
    /// 泛型参数
    generics: GenericParams,
    /// 当前 trait 实现
    current_impl: Option<ImplDef>,
}
```

### 4.2 类型检查规则

```rust
impl<'tcx> TypeChecker<'tcx> {
    /// 检查表达式类型
    pub fn check_expr(&mut self, expr: &Expr) -> Type {
        match &expr.kind {
            ExprKind::Literal(lit) => self.check_literal(lit),
            ExprKind::Path(path) => self.check_path(path),
            ExprKind::Call { func, args } => self.check_call(func, args),
            ExprKind::Field { base, field } => self.check_field(base, field),
            ExprKind::Binary { op, lhs, rhs } => self.check_binary(op, lhs, rhs),
            // ... 其他表达式
        }
    }
    
    /// 检查语句
    pub fn check_stmt(&mut self, stmt: &Stmt) {
        match &stmt.kind {
            StmtKind::Let { pat, ty, init } => {
                let init_ty = init.as_ref()
                    .map(|e| self.check_expr(e))
                    .unwrap_or(Type::Infer(InferType::TypeVar(self.engine.fresh_var())));
                
                if let Some(ty) = ty {
                    let ty = self.lower_type(ty);
                    self.engine.unify(init_ty, ty)?;
                }
                
                self.check_pattern(pat, init_ty);
            }
            // ... 其他语句
        }
    }
    
    /// 检查函数签名
    pub fn check_fn_sig(&mut self, sig: &FnSig) -> Result<(), TypeError> {
        // 检查参数类型
        for param in &sig.params {
            self.check_type(&param.ty)?;
        }
        
        // 检查返回类型
        self.check_type(&sig.return_ty)?;
        
        // 检查 where 子句
        for pred in &sig.generics.where_clauses {
            self.check_where_predicate(pred)?;
        }
        
        Ok(())
    }
}
```

## 5. 类型查询系统

### 5.1 类型上下文

```rust
/// 全局类型上下文
pub struct TyCtxt<'tcx> {
    /// 类型池（内部化）
    types: TypeInterner<'tcx>,
    /// 类型定义
    type_defs: FxHashMap<DefId, TypeDef<'tcx>>,
    /// Trait 定义
    trait_defs: FxHashMap<DefId, TraitDef>,
    /// 实现
    impls: FxHashMap<DefId, ImplDef>,
    /// 类型缓存
    cache: TypeCache<'tcx>,
}

/// 类型缓存
pub struct TypeCache<'tcx> {
    /// 规范化类型缓存
    normalized: FxHashMap<Type, Type>,
    /// Trait 实现缓存
    trait_impls: FxHashMap<(Type, DefId), Option<ImplDef>>,
    /// 方法解析缓存
    method_resolution: FxHashMap<(Type, Symbol), Option<DefId>>,
}
```

### 5.2 查询接口

```rust
impl<'tcx> TyCtxt<'tcx> {
    /// 获取类型定义
    pub fn type_def(&self, def_id: DefId) -> &TypeDef<'tcx> {
        &self.type_defs[&def_id]
    }
    
    /// 规范化类型（展开类型别名等）
    pub fn normalize(&self, ty: Type) -> Type {
        if let Some(normalized) = self.cache.normalized.get(&ty) {
            return normalized.clone();
        }
        
        let normalized = match ty {
            Type::Adt { def_id, substs } => {
                // 展开类型别名
                if let TypeDefKind::Alias(target) = &self.type_def(def_id).kind {
                    self.normalize(target.subst(&substs))
                } else {
                    ty
                }
            }
            Type::Container(c) => {
                Type::Container(self.normalize_container(c))
            }
            _ => ty,
        };
        
        self.cache.normalized.insert(ty, normalized.clone());
        normalized
    }
    
    /// 查找 trait 实现
    pub fn find_impl(
        &self,
        ty: &Type,
        trait_id: DefId,
    ) -> Option<&ImplDef> {
        let key = (ty.clone(), trait_id);
        
        if let Some(cached) = self.cache.trait_impls.get(&key) {
            return cached.as_ref();
        }
        
        // 搜索匹配的实现
        let impl_def = self.impls.values()
            .find(|impl_def| {
                impl_def.trait_ref.as_ref()
                    .map(|tr| tr.def_id == trait_id)
                    .unwrap_or(false)
                    && self.types_match(&impl_def.self_ty, ty)
            });
        
        self.cache.trait_impls.insert(key, impl_def.cloned());
        impl_def
    }
}
```

## 6. 错误处理

### 6.1 类型错误

```rust
#[derive(Debug, Clone)]
pub enum TypeError {
    /// 类型不匹配
    Mismatch(Type, Type),
    
    /// 未找到字段
    FieldNotFound {
        ty: Type,
        field: Symbol,
        similar: Vec<Symbol>,
    },
    
    /// 未满足 trait 约束
    TraitNotSatisfied {
        ty: Type,
        trait_ref: TraitRef,
        reason: String,
    },
    
    /// 类型参数数量不匹配
    ParamCountMismatch {
        expected: usize,
        found: usize,
    },
    
    /// 生命周期错误
    LifetimeError(LifetimeError),
    
    /// 循环类型依赖
    CyclicType(Vec<DefId>),
}

/// 诊断信息构建器
pub struct DiagnosticBuilder<'tcx> {
    tcx: TyCtxt<'tcx>,
    errors: Vec<Diagnostic>,
}

pub struct Diagnostic {
    level: DiagnosticLevel,
    message: String,
    span: Span,
    notes: Vec<Note>,
    suggestions: Vec<Suggestion>,
}

impl<'tcx> DiagnosticBuilder<'tcx> {
    /// 报告类型错误
    pub fn type_error(&mut self, span: Span, err: TypeError) {
        let diag = match err {
            TypeError::Mismatch(expected, found) => {
                Diagnostic {
                    level: DiagnosticLevel::Error,
                    message: format!(
                        "type mismatch: expected `{}`, found `{}`",
                        self.tcx.display_type(&expected),
                        self.tcx.display_type(&found)
                    ),
                    span,
                    notes: self.explain_type_diff(&expected, &found),
                    suggestions: self.suggest_type_fixes(&expected, &found),
                }
            }
            // ... 其他错误类型
        };
        
        self.errors.push(diag);
    }
}
```

## 7. 类型系统扩展

### 7.1 插件接口

```rust
/// 类型系统插件
pub trait TypePlugin {
    /// 自定义类型检查规则
    fn check_custom_type(&self, ty: &Type, ctx: &TypeChecker) -> Result<(), TypeError>;
    
    /// 自定义类型推导规则
    fn infer_custom_type(&self, expr: &Expr, ctx: &mut InferenceEngine) -> Option<Type>;
    
    /// 自定义类型转换
    fn coerce_custom_type(&self, from: &Type, to: &Type) -> Option<CoercionKind>;
}

/// 注册插件
impl<'tcx> TyCtxt<'tcx> {
    pub fn register_plugin(&mut self, plugin: Box<dyn TypePlugin>) {
        self.plugins.push(plugin);
    }
}
```

### 7.2 类型宏

```rust
/// 类型构造宏
#[macro_export]
macro_rules! ty {
    // 基础类型
    (bool) => { Type::Primitive(PrimitiveType::Bool) };
    (i32) => { Type::Primitive(PrimitiveType::I32) };
    
    // 容器类型
    (Vec<$t:tt>) => {
        Type::Container(ContainerType::Vec(Box::new(ty!($t))))
    };
    
    // 泛型类型
    ($name:ident<$($arg:tt),*>) => {
        Type::Adt {
            def_id: DefId::from_str(stringify!($name)),
            substs: substs!($($arg),*),
        }
    };
}
```

## 8. 总结

新的类型系统设计实现了：

1. **统一的类型表示**：所有类型信息集中管理
2. **强大的类型推导**：支持复杂的类型推导和约束求解
3. **灵活的扩展机制**：通过插件和宏扩展类型系统
4. **高效的查询缓存**：避免重复计算
5. **友好的错误提示**：提供详细的错误信息和修复建议

这个设计为 pilota-build 提供了一个现代化、可扩展的类型系统基础。