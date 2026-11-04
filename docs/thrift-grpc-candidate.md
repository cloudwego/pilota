# Thrift gRPC 开发分支交接

当前分支为 `feat/ttheader-streaming-20261105`。源码按 Pilota、Volo、下游的顺序同步到现有开发分支，消费者按完整依赖族更新锁，并实际执行普通根 `cargo check`。具体提交、来源和运行结果以 Git 引用及验证记录为准。[实施状态](thrift-streaming-over-grpc-status.md)

## 包与接口

| 包 | 当前源码版本 | 交接建议 |
| --- | --- | --- |
| pilota-build | 0.14.0-alpha.1 | 后续候选使用0.14.0-alpha.2，包含默认backend hooks、qualified path和Arc/T/Send修正；本轮未修改版本或发布 |
| pilota | 0.13.1 | Binary运行时不变，无需为本功能人为升级 |
| pilota-thrift-parser/reflect/fieldmask | 0.13.8 / 0.2.0 / 0.2.1 | 本轮无新协议运行时改动，与同一工作树依赖族一起解析，避免重复来源 |

`MakeBackend::validate`及Backend的完整item重导出、workspace依赖声明hooks有默认实现；既有第三方实现不新增必选方法或字段。`CodegenTy::qualified_path`复用原resolver和formatter；原global_path入口保留。旧Thrift/Protobuf生成入口含义不变。

Volo新入口生成gRPC service，Pilota负责同一份Thrift消息/Args/Result/异常enum。普通unary允许0/N参数，显式unary仍恰一参数；任意service的streaming throws使整个入口失败，oneway仅由新后端拒绝。1→N throws保持方法级enum类型名但新增variant影响穷尽match；0→1会增加内层Result。既有入口的生成约束与本轮新增入口分开描述，不把feature关闭当作自动回退旧签名。

## 来源与复现

`target/thrift-grpc/local-handoff/`包含 `changes.patch`、`manifest.json`、`locks/` 和 `evidence-index.json`。补丁已在临时 Git index 按基线做正向检查、在当前工作树做反向检查。manifest 记录基线 commit、只读查询到的远端 head、实际锁定依赖、非忽略源码和固定 Go 生成器的 hash；`source-manifest-current.json` 是同一份记录。每份运行报告只证明其记录的源码与二进制。

本仓库根普通cargo check可直接运行，offline golden读取已提交数据。源码交接顺序为Pilota→Volo→下游，Cargo.toml使用branch，Cargo.lock固定解析SHA，并按完整依赖族更新。接入远端来源后，消费者分别执行普通根check和feature检查。此前使用显式本地覆盖的日志保留其实际来源。

交付内容为开发分支源码、依赖锁和验证材料。2026-11-11 按最新要求恢复远端 CI，提交和推送不再使用跳过标记。现有 GitHub 工作流由 pull_request 或 main 分支 push 触发；功能分支推送本身不触发该工作流，结果以实际运行记录为准。正式 crate/tag 发布和生产部署另行安排。公共材料使用公共仓库和 IDL 信息。
