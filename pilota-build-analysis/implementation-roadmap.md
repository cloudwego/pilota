# Pilota-Build 重构实施路线图

## 概述

本文档提供了 pilota-build 重构项目的详细实施指南，包括具体的执行步骤、技术决策、风险缓解措施和成功标准。

## 1. 准备阶段（开始前2周）

### 1.1 团队组建

**核心团队成员：**
- **技术负责人（1人）**：整体架构设计，技术决策
- **核心开发者（2-3人）**：负责核心模块开发
- **测试工程师（1人）**：测试框架和质量保证
- **文档工程师（1人）**：API 文档和用户指南

**技能要求：**
- 深入理解 Rust 编译器设计
- 熟悉 Thrift/Protobuf 协议
- 有增量编译经验者优先
- 熟悉并行编程

### 1.2 基础设施准备

```bash
# 1. 创建项目仓库结构
mkdir -p pilota-build-v2/{crates,docs,tests,benches,examples}

# 2. 初始化 Cargo 工作空间
cat > pilota-build-v2/Cargo.toml << EOF
[workspace]
members = [
    "crates/pilota-build-core",
    "crates/pilota-build-hir",
    "crates/pilota-build-mir", 
    "crates/pilota-build-lir",
    "crates/pilota-build-diagnostics",
    "crates/pilota-build-plugin",
    "crates/pilota-build-types",
]

[workspace.dependencies]
# 统一版本管理
tokio = "1.35"
salsa = "0.17"
rayon = "1.8"
dashmap = "5.5"
EOF

# 3. 设置 CI/CD
# GitHub Actions 配置
cat > .github/workflows/ci.yml << EOF
name: CI
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --all-features
      - run: cargo clippy -- -D warnings
      - run: cargo fmt -- --check
EOF
```

### 1.3 性能基准建立

```rust
// benches/baseline.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_current_version(c: &mut Criterion) {
    let test_files = vec![
        "small.thrift",   // 100 行
        "medium.thrift",  // 1000 行
        "large.thrift",   // 10000 行
    ];
    
    c.bench_function("current_compile", |b| {
        b.iter(|| {
            for file in &test_files {
                let builder = pilota_build::Builder::new();
                builder.compile_file(black_box(file));
            }
        });
    });
}

criterion_group!(benches, benchmark_current_version);
criterion_main!(benches);
```

### 1.4 迁移策略制定

**双轨运行计划：**
1. 新旧系统通过 feature flag 切换
2. 默认使用旧系统，新系统 opt-in
3. 逐步迁移用户到新系统
4. 最终废弃旧系统

```rust
// 在主 crate 中添加 feature
[features]
default = ["legacy"]
legacy = []
next = ["dep:pilota-build-v2"]
```

## 2. Phase 1 实施细节（第1-3个月）

### 2.1 Week 1-2: 诊断系统开发

**Day 1-3: 错误代码体系设计**

```rust
// 创建错误代码文档
// docs/error-codes.md
# Pilota-Build 错误代码

## E0001-E0999: 语法错误
- E0001: 重复的字段 ID
- E0002: 无效的字段 ID（必须为正数）
- E0003: 缺少必需的字段类型
...

## E1000-E1999: 类型错误  
- E1001: 未定义的类型引用
- E1002: 循环类型依赖
- E1003: 类型不匹配
...
```

**Day 4-7: 核心诊断实现**

```rust
// 实现诊断系统核心
// crates/pilota-build-diagnostics/src/lib.rs
pub struct DiagnosticEngine {
    handler: DiagnosticHandler,
    source_map: Arc<SourceMap>,
    emitter: Box<dyn Emitter>,
}

impl DiagnosticEngine {
    pub fn new() -> Self {
        // 实现...
    }
}
```

**Day 8-10: 集成测试**

```bash
# 创建 UI 测试
mkdir -p tests/ui/{pass,fail}

# 编写测试用例
cat > tests/ui/fail/duplicate_field_id.thrift << EOF
struct Test {
    1: i32 field1,
    1: i32 field2, // ERROR: duplicate field ID
}
EOF

# 运行测试
cargo test --test ui
```

### 2.2 Week 3-4: 测试框架完善

**测试类型：**
1. **单元测试**：每个模块的功能测试
2. **集成测试**：端到端编译测试
3. **UI 测试**：错误消息格式测试
4. **性能测试**：回归测试
5. **模糊测试**：随机输入测试

```rust
// 测试工具实现
// crates/pilota-build-test/src/lib.rs
pub struct TestBuilder {
    input: String,
    expected_output: Option<String>,
    expected_errors: Vec<ExpectedError>,
}

impl TestBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn input(mut self, input: impl Into<String>) -> Self {
        self.input = input.into();
        self
    }
    
    pub fn expect_error(mut self, code: ErrorCode, message: &str) -> Self {
        self.expected_errors.push(ExpectedError {
            code,
            message: message.to_string(),
        });
        self
    }
    
    pub fn run(self) {
        // 执行测试...
    }
}
```

### 2.3 Week 5-8: 类型系统开发

**类型系统架构：**

```rust
// crates/pilota-build-types/src/lib.rs
/// 统一的类型表示
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    // 基础类型
    Bool,
    I8, I16, I32, I64,
    U8, U16, U32, U64,
    F32, F64,
    String,
    Bytes,
    
    // 复合类型
    Vec(Box<Type>),
    Set(Box<Type>),
    Map(Box<Type>, Box<Type>),
    
    // 用户定义类型
    Named(DefId),
    
    // 类型变量（用于推导）
    Infer(InferTy),
    
    // 错误类型
    Error,
}

/// 类型上下文
pub struct TypeContext {
    types: Arena<Type>,
    cache: FxHashMap<Type, TypeId>,
}

impl TypeContext {
    pub fn intern(&mut self, ty: Type) -> TypeId {
        if let Some(&id) = self.cache.get(&ty) {
            return id;
        }
        
        let id = self.types.alloc(ty.clone());
        self.cache.insert(ty, id);
        id
    }
}
```

### 2.4 Week 9-12: 插件系统基础

**插件加载流程：**

```mermaid
graph LR
    A[发现插件] --> B[验证元数据]
    B --> C[检查依赖]
    C --> D[初始化插件]
    D --> E[注册钩子]
    E --> F[就绪]
```

**实现步骤：**

1. **定义插件接口**
2. **实现插件加载器**
3. **创建示例插件**
4. **编写插件文档**

## 3. Phase 2 实施细节（第4-6个月）

### 3.1 IR 系统实现计划

**IR 转换流程：**

```
Source → AST → HIR → MIR → LIR → Code
         ↓      ↓      ↓      ↓
       Parse  Lower  Check  Codegen
```

**关键里程碑：**
- Month 4 Week 2: HIR 完成
- Month 4 Week 4: MIR 完成  
- Month 5 Week 2: 符号解析完成
- Month 5 Week 4: 类型检查集成
- Month 6 Week 2: LIR 完成
- Month 6 Week 4: 新旧系统对比测试通过

### 3.2 并行开发策略

```rust
// 使用 trait 抽象，允许新旧系统共存
pub trait CompilerBackend {
    fn compile(&self, input: Input) -> Result<Output, Error>;
}

pub struct LegacyBackend {
    // 现有实现
}

pub struct NextBackend {
    // 新实现
}

pub struct HybridCompiler {
    legacy: LegacyBackend,
    next: NextBackend,
    use_next: bool,
}

impl HybridCompiler {
    pub fn compile(&self, input: Input) -> Result<Output, Error> {
        if self.use_next {
            self.next.compile(input)
        } else {
            self.legacy.compile(input)
        }
    }
}
```

## 4. Phase 3 实施细节（第7-9个月）

### 4.1 性能优化清单

**优化优先级：**

1. **P0 - 必须优化**
   - 文件解析并行化
   - 类型检查缓存
   - 增量编译

2. **P1 - 重要优化**
   - 内存池使用
   - 查询结果缓存
   - I/O 并行化

3. **P2 - 锦上添花**
   - SIMD 加速
   - 自定义分配器
   - 预编译缓存

### 4.2 性能测试矩阵

| 测试场景 | 文件数 | 总行数 | 目标时间 | 当前时间 |
|---------|--------|--------|----------|----------|
| 小项目   | 10     | 1K     | < 100ms  | 300ms    |
| 中项目   | 100    | 10K    | < 500ms  | 2s       |
| 大项目   | 1000   | 100K   | < 3s     | 15s      |

### 4.3 优化实施步骤

```rust
// 1. 建立性能监控
pub struct PerfMonitor {
    spans: Vec<Span>,
}

impl PerfMonitor {
    pub fn start(&mut self, name: &str) -> SpanGuard {
        let span = Span::new(name);
        self.spans.push(span.clone());
        SpanGuard(span)
    }
    
    pub fn report(&self) {
        for span in &self.spans {
            println!("{}: {:?}", span.name, span.duration);
        }
    }
}

// 2. 识别瓶颈
// 使用 cargo flamegraph 生成火焰图

// 3. 逐步优化
// 每次优化后运行基准测试，确保有改进
```

## 5. Phase 4 实施细节（第10-12个月）

### 5.1 发布检查清单

- [ ] 所有测试通过
- [ ] 性能目标达成
- [ ] 文档完整
- [ ] 迁移指南编写
- [ ] 兼容性验证
- [ ] 安全审计完成

### 5.2 发布流程

```bash
# 1. 创建发布分支
git checkout -b release/1.0

# 2. 更新版本号
cargo set-version 1.0.0

# 3. 生成变更日志
cargo changelog

# 4. 创建标签
git tag -a v1.0.0 -m "Release version 1.0.0"

# 5. 发布到 crates.io
cargo publish --dry-run
cargo publish
```

### 5.3 用户迁移支持

**迁移工具：**

```rust
// 自动迁移脚本
pub fn migrate_project(project_path: &Path) -> Result<(), Error> {
    // 1. 检测项目类型
    let project_type = detect_project_type(project_path)?;
    
    // 2. 备份现有配置
    backup_configuration(project_path)?;
    
    // 3. 更新依赖
    update_dependencies(project_path)?;
    
    // 4. 运行测试验证
    run_tests(project_path)?;
    
    Ok(())
}
```

## 6. 风险管理

### 6.1 技术风险及缓解

| 风险 | 影响 | 概率 | 缓解措施 |
|------|------|------|----------|
| IR 设计缺陷 | 高 | 中 | 早期原型验证，专家评审 |
| 性能退化 | 高 | 低 | 持续性能测试，A/B 测试 |
| 兼容性问题 | 中 | 中 | 完整的兼容性测试套件 |
| 资源不足 | 中 | 低 | 明确优先级，核心功能优先 |

### 6.2 应急预案

**场景1：重大设计缺陷**
- 触发条件：原型测试发现根本性问题
- 应对措施：暂停开发，重新设计，最多延期2周

**场景2：性能目标未达成**
- 触发条件：Phase 3 结束时性能提升 < 2x
- 应对措施：延长优化阶段1个月，调整发布计划

**场景3：用户反馈负面**
- 触发条件：Beta 测试满意度 < 3/5
- 应对措施：收集具体反馈，快速迭代改进

## 7. 成功标准

### 7.1 技术指标

- ✅ 编译速度提升 ≥ 3x
- ✅ 内存使用降低 ≥ 30%
- ✅ 增量编译延迟 < 100ms
- ✅ 测试覆盖率 > 90%
- ✅ 零 P0 缺陷

### 7.2 用户指标

- ✅ 迁移成功率 > 95%
- ✅ 用户满意度 > 4.5/5
- ✅ 社区贡献者 > 20人
- ✅ 文档完整度 100%

### 7.3 项目指标

- ✅ 按时交付率 > 80%
- ✅ 缺陷密度 < 1/KLOC
- ✅ 代码审查覆盖率 100%

## 8. 长期维护计划

### 8.1 版本策略

- **1.x**：稳定版本，只修复 bug
- **2.x**：新功能开发
- **LTS**：每年一个长期支持版本

### 8.2 社区建设

1. **开源治理**
   - 建立 RFC 流程
   - 定期社区会议
   - 贡献者指南

2. **生态系统**
   - 插件市场
   - 集成工具
   - 示例项目

3. **文档和教育**
   - 视频教程
   - 最佳实践指南
   - 性能调优指南

## 9. 总结

本实施路线图提供了 pilota-build 重构项目的完整执行计划。通过分阶段实施、持续验证和风险管理，我们有信心在12个月内交付一个高质量、高性能的新版本编译器。

关键成功因素：
1. 坚持渐进式重构，保持系统可用
2. 重视测试和文档，确保质量
3. 持续收集反馈，快速迭代
4. 建立强大的社区，共同发展