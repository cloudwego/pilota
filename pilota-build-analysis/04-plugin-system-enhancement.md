# 插件系统增强设计

## 1. 现状分析

### 1.1 当前插件系统的局限性

当前的插件系统过于简单：

```rust
pub trait Plugin: Sync + Send {
    fn on_item(&mut self, cx: &Context, def_id: DefId, item: Arc<Item>);
    fn on_field(&mut self, cx: &Context, def_id: DefId, f: Arc<Field>);
    fn on_variant(&mut self, cx: &Context, def_id: DefId, variant: Arc<EnumVariant>);
    fn on_emit(&mut self, cx: &Context);
}
```

**问题：**
1. 只能在固定的几个点介入
2. 无法影响编译流程
3. 不能修改 IR 结构
4. 缺少插件间的通信机制
5. 没有插件优先级和依赖管理

### 1.2 设计目标

- **全流程介入**：插件可以介入编译的任何阶段
- **强大的 API**：提供丰富的操作接口
- **插件生态**：支持插件间协作和复用
- **安全性**：插件不能破坏核心编译流程
- **性能**：插件系统不应显著影响编译性能

## 2. 新插件架构

### 2.1 插件生命周期

```rust
/// 插件生命周期
pub trait PluginLifecycle {
    /// 插件初始化
    fn init(&mut self, registry: &mut PluginRegistry) -> Result<(), PluginError>;
    
    /// 插件启动（在所有插件初始化后）
    fn start(&mut self, ctx: &CompilerContext) -> Result<(), PluginError>;
    
    /// 插件停止
    fn stop(&mut self) -> Result<(), PluginError>;
    
    /// 插件元信息
    fn metadata(&self) -> PluginMetadata;
}

/// 插件元信息
pub struct PluginMetadata {
    pub name: String,
    pub version: Version,
    pub author: String,
    pub description: String,
    /// 依赖的其他插件
    pub dependencies: Vec<PluginDependency>,
    /// 提供的能力
    pub capabilities: Vec<Capability>,
    /// 需要的权限
    pub permissions: PluginPermissions,
}

/// 插件依赖
pub struct PluginDependency {
    pub name: String,
    pub version_req: VersionReq,
    pub optional: bool,
}
```

### 2.2 编译阶段钩子

```rust
/// 编译阶段插件接口
pub trait CompilerPlugin: PluginLifecycle {
    /// 源文件预处理
    fn preprocess_source(&mut self, _source: &mut SourceFile) -> Result<(), PluginError> {
        Ok(())
    }
    
    /// 解析阶段
    fn on_parse(&mut self, _ast: &mut Ast) -> Result<(), PluginError> {
        Ok(())
    }
    
    /// HIR 构建阶段
    fn on_hir_build(&mut self, _hir: &mut Hir) -> Result<(), PluginError> {
        Ok(())
    }
    
    /// 符号解析阶段
    fn on_resolve(&mut self, _resolver: &mut Resolver) -> Result<(), PluginError> {
        Ok(())
    }
    
    /// 类型检查阶段
    fn on_typecheck(&mut self, _typeck: &mut TypeChecker) -> Result<(), PluginError> {
        Ok(())
    }
    
    /// MIR 构建阶段
    fn on_mir_build(&mut self, _mir: &mut Mir) -> Result<(), PluginError> {
        Ok(())
    }
    
    /// 优化阶段
    fn on_optimize(&mut self, _optimizer: &mut Optimizer) -> Result<(), PluginError> {
        Ok(())
    }
    
    /// LIR 生成阶段
    fn on_lir_build(&mut self, _lir: &mut Lir) -> Result<(), PluginError> {
        Ok(())
    }
    
    /// 代码生成阶段
    fn on_codegen(&mut self, _codegen: &mut CodeGenerator) -> Result<(), PluginError> {
        Ok(())
    }
    
    /// 后处理阶段
    fn postprocess(&mut self, _output: &mut Output) -> Result<(), PluginError> {
        Ok(())
    }
}
```

### 2.3 AST/IR 操作接口

```rust
/// AST 操作接口
pub trait AstVisitor {
    fn visit_item(&mut self, item: &mut Item) -> VisitResult;
    fn visit_expr(&mut self, expr: &mut Expr) -> VisitResult;
    fn visit_type(&mut self, ty: &mut Type) -> VisitResult;
    fn visit_pattern(&mut self, pat: &mut Pattern) -> VisitResult;
}

pub enum VisitResult {
    /// 继续遍历
    Continue,
    /// 跳过子节点
    SkipChildren,
    /// 替换节点
    Replace(Box<dyn AstNode>),
    /// 删除节点
    Remove,
    /// 报告错误
    Error(PluginError),
}

/// IR 转换接口
pub trait IrTransform {
    /// 转换 HIR 节点
    fn transform_hir(&mut self, node: HirNode) -> TransformResult<HirNode>;
    
    /// 转换 MIR 节点
    fn transform_mir(&mut self, node: MirNode) -> TransformResult<MirNode>;
    
    /// 转换 LIR 节点
    fn transform_lir(&mut self, node: LirNode) -> TransformResult<LirNode>;
}

pub enum TransformResult<T> {
    /// 保持不变
    Unchanged(T),
    /// 修改节点
    Modified(T),
    /// 替换为多个节点
    Expanded(Vec<T>),
    /// 删除节点
    Removed,
}
```

### 2.4 类型系统扩展

```rust
/// 类型系统插件
pub trait TypeSystemPlugin: CompilerPlugin {
    /// 注册自定义类型
    fn register_types(&mut self, registry: &mut TypeRegistry) -> Result<(), PluginError> {
        Ok(())
    }
    
    /// 自定义类型检查规则
    fn custom_type_rules(&self) -> Vec<Box<dyn TypeRule>> {
        vec![]
    }
    
    /// 自定义类型转换
    fn custom_coercions(&self) -> Vec<Box<dyn Coercion>> {
        vec![]
    }
    
    /// 自定义 trait 实现
    fn custom_trait_impls(&self) -> Vec<Box<dyn TraitImpl>> {
        vec![]
    }
}

/// 自定义类型规则
pub trait TypeRule {
    /// 规则名称
    fn name(&self) -> &str;
    
    /// 检查类型是否满足规则
    fn check(&self, ty: &Type, ctx: &TypeContext) -> Result<(), TypeError>;
}

/// 自定义类型转换
pub trait Coercion {
    /// 是否可以转换
    fn can_coerce(&self, from: &Type, to: &Type) -> bool;
    
    /// 执行转换
    fn coerce(&self, from: &Type, to: &Type, ctx: &mut TypeContext) -> Result<Type, TypeError>;
}
```

## 3. 插件通信机制

### 3.1 事件系统

```rust
/// 事件总线
pub struct EventBus {
    subscribers: FxHashMap<TypeId, Vec<Box<dyn EventHandler>>>,
}

/// 事件 trait
pub trait Event: Any + Send + Sync {
    fn event_type(&self) -> &'static str;
}

/// 事件处理器
pub trait EventHandler: Send + Sync {
    fn handle(&mut self, event: &dyn Event) -> Result<(), PluginError>;
}

impl EventBus {
    /// 发布事件
    pub fn publish<E: Event>(&self, event: E) -> Result<(), PluginError> {
        let type_id = TypeId::of::<E>();
        if let Some(handlers) = self.subscribers.get(&type_id) {
            for handler in handlers {
                handler.handle(&event)?;
            }
        }
        Ok(())
    }
    
    /// 订阅事件
    pub fn subscribe<E: Event>(&mut self, handler: Box<dyn EventHandler>) {
        let type_id = TypeId::of::<E>();
        self.subscribers.entry(type_id)
            .or_insert_with(Vec::new)
            .push(handler);
    }
}
```

### 3.2 共享状态

```rust
/// 插件共享状态
pub struct PluginState {
    /// 类型化的状态存储
    state: AnyMap,
    /// 读写锁保护
    locks: FxHashMap<TypeId, RwLock<()>>,
}

impl PluginState {
    /// 获取状态（只读）
    pub fn get<T: Any + Send + Sync>(&self) -> Option<&T> {
        let _lock = self.locks.get(&TypeId::of::<T>())?.read().ok()?;
        self.state.get::<T>()
    }
    
    /// 获取状态（可变）
    pub fn get_mut<T: Any + Send + Sync>(&mut self) -> Option<&mut T> {
        let _lock = self.locks.get(&TypeId::of::<T>())?.write().ok()?;
        self.state.get_mut::<T>()
    }
    
    /// 插入状态
    pub fn insert<T: Any + Send + Sync>(&mut self, value: T) {
        let type_id = TypeId::of::<T>();
        self.locks.insert(type_id, RwLock::new(()));
        self.state.insert(value);
    }
}
```

## 4. 插件开发框架

### 4.1 插件脚手架

```rust
/// 插件 trait 宏
#[proc_macro_attribute]
pub fn plugin(args: TokenStream, input: TokenStream) -> TokenStream {
    // 自动生成插件元信息和生命周期方法
    // ...
}

/// 使用示例
#[plugin(
    name = "my_plugin",
    version = "0.1.0",
    author = "author",
    capabilities = ["type_check", "codegen"]
)]
struct MyPlugin {
    config: PluginConfig,
}

impl CompilerPlugin for MyPlugin {
    fn on_typecheck(&mut self, typeck: &mut TypeChecker) -> Result<(), PluginError> {
        // 自定义类型检查逻辑
        Ok(())
    }
}
```

### 4.2 插件测试框架

```rust
/// 插件测试工具
pub struct PluginTestHarness {
    compiler: MockCompiler,
    plugin: Box<dyn CompilerPlugin>,
}

impl PluginTestHarness {
    /// 测试特定编译阶段
    pub fn test_phase<F>(&mut self, phase: CompilePhase, setup: F) 
    where 
        F: FnOnce(&mut MockCompiler)
    {
        setup(&mut self.compiler);
        
        match phase {
            CompilePhase::Parse => {
                let mut ast = self.compiler.parse();
                self.plugin.on_parse(&mut ast).unwrap();
            }
            // ... 其他阶段
        }
    }
    
    /// 断言输出
    pub fn assert_output(&self, expected: &str) {
        let output = self.compiler.get_output();
        assert_eq!(output, expected);
    }
}

/// 测试宏
#[macro_export]
macro_rules! plugin_test {
    ($name:ident, $plugin:expr, $test:expr) => {
        #[test]
        fn $name() {
            let mut harness = PluginTestHarness::new($plugin);
            $test(&mut harness);
        }
    };
}
```

## 5. 插件管理

### 5.1 插件注册表

```rust
/// 插件注册表
pub struct PluginRegistry {
    /// 已注册的插件
    plugins: FxHashMap<String, PluginEntry>,
    /// 插件加载顺序（拓扑排序）
    load_order: Vec<String>,
    /// 插件能力索引
    capabilities: FxHashMap<Capability, Vec<String>>,
}

pub struct PluginEntry {
    metadata: PluginMetadata,
    factory: Box<dyn PluginFactory>,
    instance: Option<Box<dyn CompilerPlugin>>,
}

/// 插件工厂
pub trait PluginFactory: Send + Sync {
    fn create(&self) -> Result<Box<dyn CompilerPlugin>, PluginError>;
}

impl PluginRegistry {
    /// 注册插件
    pub fn register<F>(&mut self, factory: F) -> Result<(), PluginError>
    where
        F: PluginFactory + 'static
    {
        let plugin = factory.create()?;
        let metadata = plugin.metadata();
        
        // 检查依赖
        self.check_dependencies(&metadata)?;
        
        // 添加到注册表
        let entry = PluginEntry {
            metadata: metadata.clone(),
            factory: Box::new(factory),
            instance: Some(plugin),
        };
        
        self.plugins.insert(metadata.name.clone(), entry);
        self.update_load_order()?;
        
        Ok(())
    }
    
    /// 获取具有特定能力的插件
    pub fn get_by_capability(&self, cap: &Capability) -> Vec<&dyn CompilerPlugin> {
        self.capabilities.get(cap)
            .map(|names| {
                names.iter()
                    .filter_map(|name| {
                        self.plugins.get(name)
                            .and_then(|e| e.instance.as_ref())
                            .map(|p| p.as_ref())
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}
```

### 5.2 插件配置

```rust
/// 插件配置
#[derive(Serialize, Deserialize)]
pub struct PluginConfig {
    /// 启用的插件
    pub enabled: Vec<String>,
    /// 禁用的插件
    pub disabled: Vec<String>,
    /// 插件特定配置
    pub plugin_configs: FxHashMap<String, Value>,
    /// 全局插件设置
    pub global_settings: GlobalPluginSettings,
}

#[derive(Serialize, Deserialize)]
pub struct GlobalPluginSettings {
    /// 插件超时时间
    pub timeout: Duration,
    /// 是否启用插件沙箱
    pub sandbox: bool,
    /// 插件日志级别
    pub log_level: LogLevel,
    /// 性能分析
    pub profiling: bool,
}

/// 插件配置加载器
pub struct PluginConfigLoader {
    search_paths: Vec<PathBuf>,
}

impl PluginConfigLoader {
    /// 加载配置文件
    pub fn load(&self) -> Result<PluginConfig, ConfigError> {
        // 搜索配置文件：pilota-plugin.toml, .pilota/plugins.toml 等
        for path in &self.search_paths {
            if let Ok(config) = self.load_from_path(path) {
                return Ok(config);
            }
        }
        
        // 使用默认配置
        Ok(PluginConfig::default())
    }
}
```

## 6. 内置插件示例

### 6.1 验证插件

```rust
/// 协议验证插件
pub struct ValidationPlugin {
    rules: Vec<Box<dyn ValidationRule>>,
}

impl CompilerPlugin for ValidationPlugin {
    fn on_mir_build(&mut self, mir: &mut Mir) -> Result<(), PluginError> {
        // 遍历所有类型定义
        for (def_id, item) in &mir.items {
            for rule in &self.rules {
                rule.validate(item, mir)?;
            }
        }
        Ok(())
    }
}

/// 验证规则
pub trait ValidationRule {
    fn name(&self) -> &str;
    fn validate(&self, item: &MirItem, mir: &Mir) -> Result<(), ValidationError>;
}

/// 字段 ID 唯一性验证
pub struct UniqueFieldIdRule;

impl ValidationRule for UniqueFieldIdRule {
    fn name(&self) -> &str {
        "unique_field_id"
    }
    
    fn validate(&self, item: &MirItem, _mir: &Mir) -> Result<(), ValidationError> {
        if let MirItemKind::Message(msg) = &item.kind {
            let mut seen_ids = FxHashSet::default();
            for field in &msg.fields {
                if !seen_ids.insert(field.id) {
                    return Err(ValidationError::DuplicateFieldId {
                        message: msg.name.clone(),
                        field_id: field.id,
                    });
                }
            }
        }
        Ok(())
    }
}
```

### 6.2 优化插件

```rust
/// 代码优化插件
pub struct OptimizationPlugin {
    passes: Vec<Box<dyn OptimizationPass>>,
}

impl CompilerPlugin for OptimizationPlugin {
    fn on_optimize(&mut self, optimizer: &mut Optimizer) -> Result<(), PluginError> {
        for pass in &mut self.passes {
            pass.run(optimizer)?;
        }
        Ok(())
    }
}

/// 优化 Pass
pub trait OptimizationPass {
    fn name(&self) -> &str;
    fn run(&mut self, optimizer: &mut Optimizer) -> Result<(), PluginError>;
}

/// 死代码消除
pub struct DeadCodeElimination;

impl OptimizationPass for DeadCodeElimination {
    fn name(&self) -> &str {
        "dead_code_elimination"
    }
    
    fn run(&mut self, optimizer: &mut Optimizer) -> Result<(), PluginError> {
        let used_items = optimizer.collect_used_items();
        optimizer.retain_items(|def_id| used_items.contains(&def_id));
        Ok(())
    }
}
```

## 7. 安全性考虑

### 7.1 权限系统

```rust
/// 插件权限
#[derive(Clone, Debug)]
pub struct PluginPermissions {
    /// 允许修改 AST
    pub modify_ast: bool,
    /// 允许修改类型系统
    pub modify_types: bool,
    /// 允许访问文件系统
    pub file_access: FileAccessPermission,
    /// 允许网络访问
    pub network_access: bool,
    /// 允许执行外部命令
    pub execute_commands: bool,
    /// CPU 时间限制
    pub cpu_time_limit: Option<Duration>,
    /// 内存限制
    pub memory_limit: Option<usize>,
}

#[derive(Clone, Debug)]
pub enum FileAccessPermission {
    None,
    ReadOnly(Vec<PathBuf>),
    ReadWrite(Vec<PathBuf>),
    Full,
}

/// 权限检查器
pub struct PermissionChecker {
    permissions: FxHashMap<String, PluginPermissions>,
}

impl PermissionChecker {
    pub fn check(&self, plugin_name: &str, action: PermissionAction) -> Result<(), PermissionError> {
        let perms = self.permissions.get(plugin_name)
            .ok_or(PermissionError::PluginNotFound)?;
        
        match action {
            PermissionAction::ModifyAst if !perms.modify_ast => {
                Err(PermissionError::Denied("modify AST"))
            }
            PermissionAction::FileRead(path) => {
                self.check_file_access(perms, &path, false)
            }
            // ... 其他权限检查
            _ => Ok(()),
        }
    }
}
```

### 7.2 沙箱执行

```rust
/// 插件沙箱
pub struct PluginSandbox {
    /// 资源限制器
    limiter: ResourceLimiter,
    /// 系统调用过滤器
    syscall_filter: SyscallFilter,
}

impl PluginSandbox {
    /// 在沙箱中执行插件代码
    pub fn execute<F, R>(&self, plugin_name: &str, f: F) -> Result<R, SandboxError>
    where
        F: FnOnce() -> R
    {
        // 设置资源限制
        self.limiter.apply(plugin_name)?;
        
        // 应用系统调用过滤
        self.syscall_filter.apply()?;
        
        // 执行插件代码
        let result = std::panic::catch_unwind(f)
            .map_err(|_| SandboxError::PluginPanic)?;
        
        // 清理资源
        self.limiter.cleanup(plugin_name)?;
        
        Ok(result)
    }
}
```

## 8. 总结

增强后的插件系统提供了：

1. **全流程控制**：插件可以介入编译的每个阶段
2. **强大的 API**：丰富的 AST/IR 操作接口
3. **插件生态**：完善的依赖管理和通信机制
4. **开发体验**：便捷的开发和测试框架
5. **安全可控**：细粒度的权限控制和沙箱执行

这个设计使 pilota-build 成为一个真正可扩展的编译器框架。