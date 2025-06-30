# 如何让 pilota-build 真正利用 Salsa 的缓存功能

## 问题分析

当前的实现没有利用 Salsa 的缓存功能，主要原因是：

1. **没有使用 `#[salsa::tracked]` 函数** - 所有的查询方法都是直接从 HashMap 读取
2. **DefId 和 FileId 不是 Salsa structs** - 它们只是普通的整数类型，不能作为 tracked 函数的参数
3. **没有利用自动依赖追踪** - Salsa 可以自动追踪查询之间的依赖关系并缓存结果

## 改进方案

### 方案 1：创建 Salsa 包装类型

```rust
// 将 DefId 包装成 interned struct
#[salsa::interned]
struct SalsaDefId {
    id: u32,  // 原始的 DefId 值
}

// 将 FileId 包装成 interned struct  
#[salsa::interned]
struct SalsaFileId {
    id: u32,  // 原始的 FileId 值
}

// 使用 tracked 函数来实现查询
#[salsa::tracked]
fn node(db: &dyn RirDatabase, def_id: SalsaDefId) -> Option<Node> {
    // 从存储中读取节点
    let nodes = db.nodes_storage();
    nodes.get(&def_id.id(db)).cloned()
}

#[salsa::tracked]
fn item(db: &dyn RirDatabase, def_id: SalsaDefId) -> Option<Arc<Item>> {
    let node = node(db, def_id)?;
    match node.kind {
        NodeKind::Item(i) => Some(i),
        _ => None,
    }
}

// 更复杂的查询，会自动缓存结果
#[salsa::tracked]
fn service_methods(db: &dyn RirDatabase, def_id: SalsaDefId) -> Arc<[Arc<Method>]> {
    let item = item(db, def_id)?;
    let service = match &*item {
        Item::Service(s) => s,
        _ => return Arc::new([]),
    };
    
    // 递归调用其他 tracked 函数
    let methods = service
        .extend
        .iter()
        .flat_map(|p| {
            let extend_def_id = SalsaDefId::new(db, p.did);
            service_methods(db, extend_def_id)
                .iter()
                .map(|m| match m.source {
                    MethodSource::Extend(_) => m.clone(),
                    MethodSource::Own => Arc::from(Method {
                        source: MethodSource::Extend(p.did),
                        ..(**m).clone()
                    }),
                })
                .collect::<Vec<_>>()
        })
        .chain(service.methods.iter().cloned());
    
    Arc::from_iter(methods)
}
```

### 方案 2：重新设计数据模型

更彻底的方案是重新设计数据模型，让所有的数据都使用 Salsa structs：

```rust
// 输入：源文件
#[salsa::input]
struct SourceFile {
    path: PathBuf,
    #[returns(ref)]
    content: String,
}

// Tracked struct 来存储解析后的文件
#[salsa::tracked]
struct ParsedFile {
    #[returns(ref)]
    items: Vec<DefId>,
    #[returns(ref)]
    uses: Vec<FileId>,
}

// Tracked 函数来解析文件
#[salsa::tracked]
fn parse_file(db: &dyn Db, file: SourceFile) -> ParsedFile {
    let content = file.content(db);
    // ... 解析逻辑 ...
    ParsedFile::new(db, items, uses)
}

// Tracked struct 来表示 Item
#[salsa::tracked]
struct TrackedItem {
    #[id]
    def_id: DefId,
    #[returns(ref)]
    data: ItemData,
}

// Tracked 函数来获取 item
#[salsa::tracked]
fn get_item(db: &dyn Db, def_id: SalsaDefId) -> Option<TrackedItem> {
    // 这个函数的结果会被缓存
    // 当输入改变时，只有受影响的 item 会重新计算
}
```

### 方案 3：混合方案

保持现有的数据结构，但为计算密集型操作添加缓存层：

```rust
// 保持现有的存储结构
#[salsa::db]
struct RootDatabase {
    storage: salsa::Storage<Self>,
    // 原始数据存储
    raw_nodes: Arc<FxHashMap<DefId, Node>>,
    raw_files: Arc<FxHashMap<FileId, Arc<File>>>,
}

// 为复杂计算创建 tracked 函数
#[salsa::tracked]
fn collect_dependencies(db: &dyn Db, def_id: SalsaDefId) -> Arc<Vec<DefId>> {
    // 收集依赖的逻辑
    // 结果会被缓存，只有当相关节点改变时才重新计算
}

#[salsa::tracked]
fn type_check_item(db: &dyn Db, def_id: SalsaDefId) -> TypeCheckResult {
    // 类型检查逻辑
    // 自动追踪依赖，只在必要时重新计算
}
```

## 优势

使用 Salsa 的缓存功能后：

1. **自动缓存** - 不需要手动管理缓存，Salsa 自动处理
2. **增量计算** - 当输入改变时，只重新计算受影响的部分
3. **依赖追踪** - Salsa 自动追踪查询之间的依赖关系
4. **并发安全** - Salsa 处理并发访问和缓存一致性

## 迁移建议

1. **逐步迁移** - 不需要一次性改变所有代码
2. **从热点开始** - 先为最耗时的计算添加缓存
3. **保持兼容性** - 可以同时保留新旧 API

## 示例：service_methods 的改进

原始版本（无缓存）：
```rust
fn service_methods(&self, def_id: DefId) -> Arc<[Arc<Method>]> {
    // 每次调用都要重新计算
}
```

改进版本（有缓存）：
```rust
#[salsa::tracked]
fn service_methods(db: &dyn Db, def_id: SalsaDefId) -> Arc<[Arc<Method>]> {
    // 结果被缓存，只在相关数据改变时重新计算
}
```

这样，如果一个 service 的方法没有改变，即使其他部分的代码改变了，`service_methods` 也不会重新执行，而是直接返回缓存的结果。