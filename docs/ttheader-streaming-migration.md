# Pilota TTStream 代码生成迁移

本轮 `pilota-build 0.14.0-alpha.1` 是候选版本，正式目标 `0.14.0`；当前尚未发布。最低配套为 `pilota-thrift-parser 0.13.8`、`pilota 0.13.1`。parser 的新源位置查询是 additive API。不能把本地路径 patch 写入消费者的发布依赖。

没有 streaming 注解的 Thrift 和原有 Protobuf API 保持不变。显式 `streaming.mode="unary"` 仍走普通 unary，但按照 Kitex 注解规则要求单个参数。client/server/bidirectional 方法改变代码生成：不再生成 unary Args/Result 包装，后端直接使用原始 request/response 消息，客户端和 handler 使用下游框架的流式 API。runtime feature 开关不会恢复旧签名或跳过 IDL 校验。

迁移时重新生成全部 service，修改流式调用与 handler，并从任意 service 移除 streaming `throws`；业务错误改用 Biz 或 Application 终态。即使业务没有使用出错 service，整个构建也会失败。普通 unary throws 不变。标量或容器流消息提前拒绝：Kitex 生成器可生成/编译这些签名，但真实 TTStream payload codec 无法发送；typedef 到 message struct、union、Arc wrapper 保持原消息类型查询。

下游构建工具应使用 `Builder::try_compile` 或 `try_compile_with_config`，把返回的 `anyhow::Error` 传递给调用方。旧 `compile` facade 保留 fatal 诊断方式以兼容既有 build.rs；不要使用它来实现返回 `Result` 的服务端构建 API。新增 `Parser::try_parse` 有默认实现，现有自定义 parser 不需补字段。

发布次序为 parser 0.13.8 → build 0.14.0 候选联调 → 正式 build；无 runtime codec 变更，不为本轮人为抬升 runtime minor。下游框架应按各自新 build minor 迁移，单纯依赖升级需与代码生成语义变更分开。

已完成本地 Linux nightly 全 workspace/default、Protobuf feature、doc 和 golden/IDL 验证。原 Clippy 基线仍需按项目门禁处理；Rust 1.85.0 的锁定依赖 Linux workspace/all-targets/pb feature 已通过，锁定依赖中的 psm 版本约束需要保持。严格 Clippy 仍有基线问题；远端跨平台 CI 与发布未执行。
