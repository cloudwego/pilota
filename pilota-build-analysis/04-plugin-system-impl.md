# 插件系统实现指南

## 1. 项目结构

```
pilota-build-plugin/
├── src/
│   ├── lib.rs              # 主入口和 trait 定义
│   ├── registry.rs         # 插件注册表
│   ├── context.rs          # 插件上下文
│   ├── builtin/           # 内置插件
│   │   ├── mod.rs
│   │   ├── serde.rs       # Serde 支持
│   │   ├── validator.rs   # 验证器
│   │   └── async_trait.rs # Async trait
│   ├── loader.rs          # 动态加载
│   └── api/              # 插件 API
│       ├── mod.rs
│       ├── ast.rs         # AST 操作
│       ├── codegen.rs     # 代码生成
│       └── metadata.rs    # 元数据
├── examples/              # 示例插件
└── tests/
```

## 2. 核心实现

### 2.1 插件 Trait 定义

```rust
// src/lib.rs
use std::any::Any;

/// 插件元数据
#[derive(Debug, Clone)]
pub struct PluginMetadata {
    pub name: String,
    pub version: Version,
    pub author: String,
    pub description: String,
    pub dependencies: Vec<PluginDependency>,
}

/// 插件生命周期
pub trait Plugin: Any + Send + Sync {
    /// 获取插件元数据
    fn metadata(&self) -> &PluginMetadata;
    
    /// 插件初始化
    fn init(&mut self, ctx: &mut PluginContext) -> Result<(), PluginError> {
        Ok(())
    }
    
    /// 配置插件
    fn configure(&mut self, config: toml::Value) -> Result<(), PluginError> {
        Ok(())
    }
    
    /// 获取插件能力
    fn capabilities(&self) -> PluginCapabilities {
        PluginCapabilities::default()
    }
}

/// 插件能力标记
#[derive(Default)]
pub struct PluginCapabilities {
    pub parse_hook: bool,
    pub resolve_hook: bool,
    pub type_check_hook: bool,
    pub codegen_hook: bool,
    pub optimize_hook: bool,
}

/// 解析阶段钩子
pub trait ParseHook: Plugin {
    fn on_parse_start(&mut self, ctx: &mut ParseContext) -> Result<(), PluginError> {
        Ok(())
    }
    
    fn on_parse_item(&mut self, item: &mut hir::Item, ctx: &mut ParseContext) -> Result<(), PluginError> {
        Ok(())
    }
    
    fn on_parse_complete(&mut self, ast: &mut hir::Crate, ctx: &mut ParseContext) -> Result<(), PluginError> {
        Ok(())
    }
}

/// 符号解析钩子
pub trait ResolveHook: Plugin {
    fn on_resolve_path(&mut self, path: &hir::Path, ctx: &mut ResolveContext) -> Result<Option<DefId>, PluginError> {
        Ok(None)
    }
    
    fn on_resolve_type(&mut self, ty: &mut mir::Type, ctx: &mut ResolveContext) -> Result<(), PluginError> {
        Ok(())
    }
}

/// 类型检查钩子
pub trait TypeCheckHook: Plugin {
    fn on_type_check_item(&mut self, item: &mut mir::Item, ctx: &mut TypeCheckContext) -> Result<(), PluginError> {
        Ok(())
    }
    
    fn on_infer_type(&mut self, expr: &mir::Expr, ctx: &mut TypeCheckContext) -> Result<Option<mir::Type>, PluginError> {
        Ok(None)
    }
}

/// 代码生成钩子
pub trait CodegenHook: Plugin {
    fn on_codegen_item(&mut self, item: &lir::Item, ctx: &mut CodegenContext) -> Result<Option<TokenStream>, PluginError> {
        Ok(None)
    }
    
    fn on_codegen_complete(&mut self, module: &mut lir::Module, ctx: &mut CodegenContext) -> Result<(), PluginError> {
        Ok(())
    }
}
```

### 2.2 插件上下文

```rust
// src/context.rs
pub struct PluginContext {
    /// 编译器会话
    session: Arc<CompilerSession>,
    /// 插件私有数据存储
    storage: DashMap<TypeId, Box<dyn Any + Send + Sync>>,
    /// 诊断处理器
    diagnostics: DiagnosticHandler,
    /// 查询接口
    queries: Arc<dyn CompilerQueries>,
}

impl PluginContext {
    /// 存储插件私有数据
    pub fn store<T: Any + Send + Sync + 'static>(&self, data: T) {
        self.storage.insert(TypeId::of::<T>(), Box::new(data));
    }
    
    /// 获取插件私有数据
    pub fn get<T: Any + Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        self.storage.get(&TypeId::of::<T>())
            .and_then(|data| data.downcast_ref::<T>())
            .map(Arc::new)
    }
    
    /// 发出诊断信息
    pub fn emit_diagnostic(&self, diag: Diagnostic) {
        self.diagnostics.emit(diag);
    }
    
    /// 查询编译器信息
    pub fn query<Q: Query>(&self, key: Q::Key) -> Q::Value {
        self.queries.query::<Q>(key)
    }
}

/// 特定阶段的上下文
pub struct ParseContext {
    base: PluginContext,
    current_file: FileId,
    source_map: Arc<SourceMap>,
}

pub struct CodegenContext {
    base: PluginContext,
    current_module: DefId,
    codegen_options: CodegenOptions,
    /// 允许插件添加导入
    imports: RefCell<Vec<UseDecl>>,
    /// 允许插件添加辅助函数
    helpers: RefCell<Vec<TokenStream>>,
}

impl CodegenContext {
    pub fn add_import(&self, import: UseDecl) {
        self.imports.borrow_mut().push(import);
    }
    
    pub fn add_helper(&self, helper: TokenStream) {
        self.helpers.borrow_mut().push(helper);
    }
    
    pub fn format_type(&self, ty: &lir::Type) -> TokenStream {
        self.base.session.codegen().format_type(ty)
    }
}
```

### 2.3 插件注册表

```rust
// src/registry.rs
pub struct PluginRegistry {
    plugins: Vec<Box<dyn Plugin>>,
    parse_hooks: Vec<Box<dyn ParseHook>>,
    resolve_hooks: Vec<Box<dyn ResolveHook>>,
    type_check_hooks: Vec<Box<dyn TypeCheckHook>>,
    codegen_hooks: Vec<Box<dyn CodegenHook>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            plugins: Vec::new(),
            parse_hooks: Vec::new(),
            resolve_hooks: Vec::new(),
            type_check_hooks: Vec::new(),
            codegen_hooks: Vec::new(),
        };
        
        // 注册内置插件
        registry.register_builtin_plugins();
        registry
    }
    
    pub fn register<P: Plugin + 'static>(&mut self, plugin: P) -> Result<(), PluginError> {
        // 检查依赖
        self.check_dependencies(&plugin)?;
        
        // 检查冲突
        self.check_conflicts(&plugin)?;
        
        // 根据能力注册到不同的钩子列表
        let capabilities = plugin.capabilities();
        let plugin_box = Box::new(plugin);
        
        if capabilities.parse_hook {
            if let Some(hook) = plugin_box.as_any().downcast_ref::<dyn ParseHook>() {
                self.parse_hooks.push(Box::new(hook.clone()));
            }
        }
        
        // ... 注册其他钩子
        
        self.plugins.push(plugin_box);
        Ok(())
    }
    
    /// 执行解析钩子
    pub fn run_parse_hooks(&mut self, item: &mut hir::Item, ctx: &mut ParseContext) -> Result<(), PluginError> {
        for hook in &mut self.parse_hooks {
            hook.on_parse_item(item, ctx)?;
        }
        Ok(())
    }
}
```

### 2.4 内置插件示例

```rust
// src/builtin/serde.rs
pub struct SerdePlugin {
    metadata: PluginMetadata,
    config: SerdeConfig,
}

#[derive(Default)]
struct SerdeConfig {
    derive_serialize: bool,
    derive_deserialize: bool,
    rename_all: Option<RenameRule>,
    skip_serializing_if: Option<String>,
}

impl Plugin for SerdePlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }
    
    fn configure(&mut self, config: toml::Value) -> Result<(), PluginError> {
        self.config = config.try_into()?;
        Ok(())
    }
    
    fn capabilities(&self) -> PluginCapabilities {
        PluginCapabilities {
            codegen_hook: true,
            ..Default::default()
        }
    }
}

impl CodegenHook for SerdePlugin {
    fn on_codegen_item(&mut self, item: &lir::Item, ctx: &mut CodegenContext) -> Result<Option<TokenStream>, PluginError> {
        match item {
            lir::Item::Struct { name, fields, .. } => {
                let mut derives = vec![];
                
                if self.config.derive_serialize {
                    derives.push(quote! { Serialize });
                }
                if self.config.derive_deserialize {
                    derives.push(quote! { Deserialize });
                }
                
                if derives.is_empty() {
                    return Ok(None);
                }
                
                // 添加 serde 导入
                ctx.add_import(UseDecl {
                    path: "serde::{Serialize, Deserialize}".into(),
                });
                
                // 生成 derive 属性
                let derive_attr = quote! {
                    #[derive(#(#derives),*)]
                };
                
                // 生成 serde 属性
                let mut attrs = vec![derive_attr];
                
                if let Some(rename_all) = &self.config.rename_all {
                    attrs.push(quote! {
                        #[serde(rename_all = #rename_all)]
                    });
                }
                
                Ok(Some(quote! {
                    #(#attrs)*
                }))
            }
            _ => Ok(None),
        }
    }
}
```

### 2.5 动态加载

```rust
// src/loader.rs
use libloading::{Library, Symbol};

pub struct DynamicPluginLoader {
    loaded_libraries: Vec<Library>,
}

impl DynamicPluginLoader {
    pub fn load_plugin(&mut self, path: &Path) -> Result<Box<dyn Plugin>, PluginError> {
        unsafe {
            let lib = Library::new(path)
                .map_err(|e| PluginError::LoadFailed(e.to_string()))?;
            
            // 获取插件创建函数
            let create_fn: Symbol<fn() -> Box<dyn Plugin>> = lib.get(b"create_plugin")
                .map_err(|e| PluginError::SymbolNotFound("create_plugin".into()))?;
            
            let plugin = create_fn();
            
            // 验证插件
            self.validate_plugin(&plugin)?;
            
            self.loaded_libraries.push(lib);
            Ok(plugin)
        }
    }
    
    fn validate_plugin(&self, plugin: &dyn Plugin) -> Result<(), PluginError> {
        let metadata = plugin.metadata();
        
        // 检查版本兼容性
        if !self.is_compatible(&metadata.version) {
            return Err(PluginError::IncompatibleVersion);
        }
        
        Ok(())
    }
}

/// 插件入口宏
#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty) => {
        #[no_mangle]
        pub extern "C" fn create_plugin() -> Box<dyn $crate::Plugin> {
            Box::new(<$plugin_type>::new())
        }
    };
}
```

## 3. 高级功能

### 3.1 插件间通信

```rust
pub struct PluginMessageBus {
    subscribers: DashMap<String, Vec<Box<dyn Fn(&dyn Any) + Send + Sync>>>,
}

impl PluginMessageBus {
    pub fn subscribe<T: Any + 'static>(&self, topic: &str, handler: impl Fn(&T) + Send + Sync + 'static) {
        let handler = Box::new(move |msg: &dyn Any| {
            if let Some(typed_msg) = msg.downcast_ref::<T>() {
                handler(typed_msg);
            }
        });
        
        self.subscribers.entry(topic.to_string())
            .or_default()
            .push(handler);
    }
    
    pub fn publish<T: Any>(&self, topic: &str, message: T) {
        if let Some(handlers) = self.subscribers.get(topic) {
            for handler in handlers.iter() {
                handler(&message);
            }
        }
    }
}
```

### 3.2 插件配置系统

```rust
#[derive(Debug, Deserialize)]
pub struct PluginConfig {
    pub name: String,
    pub enabled: bool,
    #[serde(default)]
    pub priority: i32,
    pub config: toml::Value,
}

impl PluginRegistry {
    pub fn load_config(&mut self, config_path: &Path) -> Result<(), PluginError> {
        let config_str = fs::read_to_string(config_path)?;
        let config: PluginsConfig = toml::from_str(&config_str)?;
        
        for plugin_config in config.plugins {
            if !plugin_config.enabled {
                continue;
            }
            
            // 查找并配置插件
            if let Some(plugin) = self.find_plugin_mut(&plugin_config.name) {
                plugin.configure(plugin_config.config)?;
            } else {
                // 尝试动态加载
                if let Some(path) = config.plugin_paths.get(&plugin_config.name) {
                    let plugin = self.loader.load_plugin(path)?;
                    plugin.configure(plugin_config.config)?;
                    self.register(plugin)?;
                }
            }
        }
        
        // 按优先级排序
        self.sort_by_priority();
        
        Ok(())
    }
}
```

### 3.3 插件沙箱

```rust
pub struct PluginSandbox {
    memory_limit: usize,
    time_limit: Duration,
    allowed_syscalls: HashSet<String>,
}

impl PluginSandbox {
    pub fn execute<F, R>(&self, plugin_fn: F) -> Result<R, PluginError>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        // 创建新线程执行插件代码
        let (tx, rx) = channel();
        let time_limit = self.time_limit;
        
        let handle = thread::spawn(move || {
            // 设置内存限制
            // ... (平台特定代码)
            
            // 执行插件函数
            let result = panic::catch_unwind(AssertUnwindSafe(plugin_fn));
            tx.send(result).ok();
        });
        
        // 等待执行完成或超时
        match rx.recv_timeout(time_limit) {
            Ok(Ok(result)) => Ok(result),
            Ok(Err(panic)) => Err(PluginError::Panicked),
            Err(_) => {
                // 超时，终止线程
                // ... (平台特定代码)
                Err(PluginError::Timeout)
            }
        }
    }
}
```

## 4. 示例插件

### 4.1 验证器插件

```rust
// examples/validator_plugin.rs
use pilota_build_plugin::*;

pub struct ValidatorPlugin {
    metadata: PluginMetadata,
    rules: Vec<Box<dyn ValidationRule>>,
}

trait ValidationRule: Send + Sync {
    fn validate(&self, item: &mir::Item, ctx: &mut TypeCheckContext) -> Result<(), ValidationError>;
}

struct FieldIdRangeRule {
    min_id: i32,
    max_id: i32,
}

impl ValidationRule for FieldIdRangeRule {
    fn validate(&self, item: &mir::Item, ctx: &mut TypeCheckContext) -> Result<(), ValidationError> {
        if let mir::ItemKind::Message(msg) = &item.kind {
            for field in &msg.fields {
                if field.id < self.min_id || field.id > self.max_id {
                    ctx.emit_diagnostic(
                        Diagnostic::warning(format!(
                            "Field ID {} is outside recommended range [{}, {}]",
                            field.id, self.min_id, self.max_id
                        ))
                        .span(field.span)
                    );
                }
            }
        }
        Ok(())
    }
}

impl Plugin for ValidatorPlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }
    
    fn init(&mut self, ctx: &mut PluginContext) -> Result<(), PluginError> {
        // 注册默认规则
        self.rules.push(Box::new(FieldIdRangeRule {
            min_id: 1,
            max_id: 10000,
        }));
        
        Ok(())
    }
}

impl TypeCheckHook for ValidatorPlugin {
    fn on_type_check_item(&mut self, item: &mut mir::Item, ctx: &mut TypeCheckContext) -> Result<(), PluginError> {
        for rule in &self.rules {
            rule.validate(item, ctx)?;
        }
        Ok(())
    }
}

// 声明插件入口
declare_plugin!(ValidatorPlugin);
```

### 4.2 自定义代码生成插件

```rust
pub struct CustomDerivePlugin {
    metadata: PluginMetadata,
    derives: HashMap<String, Box<dyn CustomDerive>>,
}

trait CustomDerive: Send + Sync {
    fn generate(&self, item: &lir::Struct, ctx: &CodegenContext) -> TokenStream;
}

struct BuilderDerive;

impl CustomDerive for BuilderDerive {
    fn generate(&self, item: &lir::Struct, ctx: &CodegenContext) -> TokenStream {
        let struct_name = &item.name;
        let builder_name = format_ident!("{}Builder", struct_name);
        
        let fields = item.fields.iter().map(|f| {
            let name = &f.name;
            let ty = ctx.format_type(&f.ty);
            quote! {
                #name: Option<#ty>
            }
        });
        
        let setters = item.fields.iter().map(|f| {
            let name = &f.name;
            let ty = ctx.format_type(&f.ty);
            quote! {
                pub fn #name(mut self, value: #ty) -> Self {
                    self.#name = Some(value);
                    self
                }
            }
        });
        
        quote! {
            pub struct #builder_name {
                #(#fields,)*
            }
            
            impl #builder_name {
                #(#setters)*
                
                pub fn build(self) -> Result<#struct_name, &'static str> {
                    // ... 构建逻辑
                }
            }
            
            impl #struct_name {
                pub fn builder() -> #builder_name {
                    #builder_name::default()
                }
            }
        }
    }
}
```

## 5. 测试

### 5.1 插件测试框架

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_plugin_lifecycle() {
        let mut plugin = TestPlugin::new();
        let mut ctx = PluginContext::new_test();
        
        // 测试初始化
        assert!(plugin.init(&mut ctx).is_ok());
        
        // 测试配置
        let config = toml::from_str(r#"
            enabled = true
            option = "value"
        "#).unwrap();
        assert!(plugin.configure(config).is_ok());
    }
    
    #[test]
    fn test_plugin_hook() {
        let mut registry = PluginRegistry::new();
        registry.register(TestPlugin::new()).unwrap();
        
        let mut item = create_test_item();
        let mut ctx = ParseContext::new_test();
        
        registry.run_parse_hooks(&mut item, &mut ctx).unwrap();
        
        // 验证插件修改
        assert_eq!(item.attrs.len(), 1);
    }
}
```

### 5.2 集成测试

```rust
#[test]
fn test_plugin_integration() {
    let mut builder = Builder::new();
    
    // 加载插件
    builder.plugin(SerdePlugin::new());
    builder.plugin(ValidatorPlugin::new());
    
    // 编译
    let result = builder.compile_string(r#"
        struct User {
            1: string name,
            2: i32 age,
        }
    "#);
    
    assert!(result.is_ok());
    
    // 验证生成的代码包含 serde derives
    let output = result.unwrap();
    assert!(output.contains("#[derive(Serialize, Deserialize)]"));
}
```

## 6. 性能考虑

### 6.1 插件缓存

```rust
pub struct PluginCache {
    cache: DashMap<CacheKey, CacheEntry>,
}

impl PluginCache {
    pub fn get_or_compute<F, R>(&self, key: CacheKey, compute: F) -> R
    where
        F: FnOnce() -> R,
        R: Clone + Send + Sync + 'static,
    {
        if let Some(entry) = self.cache.get(&key) {
            if !entry.is_expired() {
                return entry.value.downcast_ref::<R>().unwrap().clone();
            }
        }
        
        let value = compute();
        self.cache.insert(key, CacheEntry::new(Box::new(value.clone())));
        value
    }
}
```

### 6.2 并行执行

```rust
impl PluginRegistry {
    pub fn run_parallel_hooks<F>(&mut self, items: Vec<hir::Item>, hook_fn: F) -> Result<Vec<hir::Item>, PluginError>
    where
        F: Fn(&mut dyn Plugin, &mut hir::Item) -> Result<(), PluginError> + Sync,
    {
        items.into_par_iter()
            .map(|mut item| {
                for plugin in &mut self.plugins {
                    hook_fn(plugin.as_mut(), &mut item)?;
                }
                Ok(item)
            })
            .collect()
    }
}
```