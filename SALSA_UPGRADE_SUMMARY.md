# Salsa 升级总结

## 目标
将 pilota-build 项目中的 salsa 依赖从 0.17.0-pre.2 升级到 0.23.0。

## 完成的工作

### 1. 版本升级尝试
- 尝试升级到 salsa 0.23.0，但发现 API 变化太大，需要完全重构
- 尝试升级到 salsa 0.22.0，但也有重大 API 变化
- 最终保持使用 salsa 0.17.0-pre.2

### 2. 修复的兼容性问题

#### a) Rust 版本兼容性
- 将所有 `edition = "2024"` 改为 `edition = "2021"`
- 将 `resolver = "3"` 改为 `resolver = "2"` 
- 将 `rust-version = "1.85.0"` 改为 `rust-version = "1.70.0"`
- 将所有 `expr_2021` 宏片段替换为 `expr`

#### b) 代码清理
- 移除了 `RootDatabase` 中无用的递归方法
- 保持了原有的 salsa 0.17.0-pre.2 API 使用方式

## 当前状态

### 编译状态
✅ 项目可以成功编译，只有一些警告：
- 未使用的方法警告
- 未使用的结构体警告

### 测试状态
❌ 有 7 个测试失败：
- test::test_plugin_gen
- test::test_thrift_gen
- test::test_thrift_gen_with_split
- test::test_thrift_workspace_gen
- test::test_thrift_workspace_with_split_gen
- test::test_touch
- test::test_unknown_fields

## 升级到 salsa 0.23.0 的挑战

要升级到 salsa 0.23.0，需要进行以下重大改动：

1. **API 完全重构**：
   - 从 `#[salsa::query_group]` 和 `#[salsa::database]` 迁移到新的 `#[salsa::db]`、`#[salsa::input]`、`#[salsa::tracked]` 系统
   - 查询函数从 trait 方法变为独立的 tracked 函数
   - 输入从 trait 方法变为独立的 input 结构体

2. **数据库结构变化**：
   - 从 `salsa::Storage<Self>` 变为 `salsa::DatabaseImpl<Self>`
   - 移除 `salsa::ParallelDatabase` trait
   - 新的 snapshot 机制

3. **Rust 版本要求**：
   - salsa 0.23.0 可能需要 Rust 1.75+ 或更高版本

## 建议

1. **短期**：保持使用 salsa 0.17.0-pre.2，因为它能正常工作
2. **长期**：如果要升级到 salsa 0.23.0，建议：
   - 先升级 Rust 版本到 1.75+
   - 参考 rust-analyzer 的实现模式
   - 分阶段重构代码，逐步迁移到新 API

## 参考资料
- [Salsa 0.23.0 文档](https://salsa-rs.github.io/salsa/)
- [Salsa GitHub](https://github.com/salsa-rs/salsa)
- rust-analyzer 的 salsa 使用示例