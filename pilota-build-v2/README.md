# Pilota Build v2 - 重构实现

这是 pilota-build 的重构版本，基于之前的设计文档实现。

## 项目结构

```
crates/
├── pilota-build-common/        # 通用基础类型和工具
├── pilota-build-diagnostics/   # 诊断和错误报告系统
├── pilota-build-hir/          # 高级中间表示 (HIR)
├── pilota-build-mir/          # 中级中间表示 (MIR)
├── pilota-build-lir/          # 低级中间表示 (LIR)
├── pilota-build-types/        # 类型系统
├── pilota-build-plugin/       # 插件系统
├── pilota-build-core/         # 核心编译器
└── pilota-build-test-utils/   # 测试工具
```

## 当前进展

### Phase 1: 基础设施建设 ✓

1. **诊断系统** ✓
   - 实现了完整的错误报告系统
   - 支持结构化的错误代码
   - 友好的终端输出格式
   - 代码建议和修复提示

2. **通用模块** ✓
   - Span 和源码位置跟踪
   - Symbol 字符串内部化
   - DefId 定义标识符系统
   - SourceMap 源码管理

3. **HIR 系统** ✓
   - 定义了 HIR AST 结构
   - 实现了 Visitor 模式
   - 准备好了 lowering 框架

4. **核心框架** ✓
   - Builder API 用户接口
   - 编译器会话管理
   - 编译器上下文
   - 简化的数据库实现

### 待完成

- Phase 2: 核心模块重构
  - [ ] 完整的三层 IR 实现
  - [ ] 解析器集成
  - [ ] 符号解析
  - [ ] 类型检查

- Phase 3: 性能优化
  - [ ] 并行编译
  - [ ] 增量编译（Salsa 集成）
  - [ ] 内存优化

- Phase 4: 稳定化
  - [ ] 完整的测试套件
  - [ ] 文档完善
  - [ ] 性能基准测试

## 使用示例

```rust
use pilota_build_core::{Builder, CompilerOptions};

fn main() {
    let builder = Builder::new()
        .with_options(CompilerOptions {
            debug: true,
            verbose: true,
            threads: 4,
            incremental: true,
            output_dir: Some("./generated".into()),
        })
        .add_file("schema.thrift");

    match builder.compile() {
        Ok(()) => println!("Success!"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## 构建

```bash
cargo build --all
cargo test --all
cargo run --example basic
```

## 设计亮点

1. **分层架构**：清晰的模块化设计，每个模块职责单一
2. **强大的诊断系统**：提供编译器级别的错误报告质量
3. **灵活的 IR 系统**：三层 IR 设计支持不同层次的优化
4. **可扩展性**：预留了插件系统接口，方便后续扩展
5. **性能考虑**：架构设计充分考虑了并行和增量编译

## 下一步计划

1. 集成 Thrift/Protobuf 解析器
2. 实现完整的符号解析
3. 完成类型系统实现
4. 添加代码生成后端
5. 集成 Salsa 实现增量编译