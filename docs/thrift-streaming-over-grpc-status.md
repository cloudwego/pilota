# Thrift over gRPC 实施状态

2026-11-11，分支为 `feat/ttheader-streaming-20261105`。Pilota 数据与生成候选已通过本地验收，开发分支交付按候选说明执行。正式发布和部署延期；2026-11-11 起恢复远端 CI，按原工作流规则触发。

## 数据与生成扩展

- 结构化 IR 已证明五类方法模式、原始 Go namespace/wire 名、include/继承归属、Args/Result 与方法级异常 enum 可复用，无新增 HTTP/2 runtime 依赖。
- 新增默认实现的 MakeBackend 生成前校验、完整 item emission override、workspace runtime dependency hook；既有实现不需增加必选方法。共享类型重导出时不再生成第二份类型/impl。
- CodegenTy 新增按定义位置解析的 qualified_path；原 global_path 复用相同 formatter 并保持默认输出。Volo 真实 workspace 和 T/Send 命名冲突 fixture 验证，不复制一份容器类型 formatter。
- workspace TOML 可为 shorthand/table 依赖增加传输 feature，同时保留用户显式版本/default-features。两种形式均有回归。
- 修复 ArcFolder 对 void/scalar 的 panic，保持 inline；Thrift protocol 泛型改名并限定 Future 的 std::marker::Send，避免 IDL 的 T/Send 遮蔽。74 处预期生成快照已按既有工具更新，未更新 wire golden。
- 新增固定 Kitex v0.16.4/thriftgo v0.4.5 的 19 组真实 gRPC codec golden，含 raw/Args/Result/空 STOP/声明异常与容错差异。工具自身报告 v0.16.3，模块来源仍为 v0.16.4，未篡改版本。Rust→Go、Go→Rust 均已执行；Cargo tests 仅读入库 golden。

## 已执行门禁

| 命令/日志 | 结果 |
| --- | --- |
| 根普通 `cargo check`，`root-check-delivery-final.log` | PASS |
| `cargo test -p pilota-build --tests`，`build-tests-delivery-final.log` | 89 项 PASS：60 lib + 29 integration，含旧生成快照/workspace |
| `cargo +nightly fmt --all -- --check` | PASS |
| `cargo +nightly clippy -p pilota-build --all-targets`，`clippy-delivery-final.log` | 命令完成；现有 111 个 lib-test 告警保留，诊断位置与本次变更行交叉核对无重合 |

更宽的 `clippy -p pilota -p pilota-build --all-targets` 仍有原有 pb/types.rs 四处 approx_constant deny，不计 PASS。不加全局 allow，也不声称整个仓库没有历史 lint。

## 复现与交接

Arc 红例已实际运行：`service S { void Zero() } (pilota.rust_wrapper_arc="true")` 原在 ArcFolder panic，修复后普通 Thrift 生成成功。日志为 `p0-arc-before.log` / `p0-hooks.log`，当前回归：

```sh
cargo test -p pilota-build --test thrift_grpc_ir ordinary_thrift_still_accepts_oneway_and_streaming_union_aliases -- --exact
```

Volo workspace 最初根据 Thrift 语法只添加 volo-thrift，生成 gRPC 代码无法编译；后端依赖 hook 改为声明 volo-grpc/thrift 后通过。随后发现全局类型路径错误包含 workspace crate 前缀，qualified_path 复用 resolver 修复；Volo 保存 workspace-check-before/after/qualified 日志。上述属于生成器支持，不增加协议栈到 Pilota。

Go 对缺 success/多 Result 字段等畸形输入更宽松；Rust 严格 Result 保留差异。合法 trailing bytes 两边接受；Go 部分未声明 requiredness 的容器 nil→空可能改变重编码字节，语义对照不伪装逐字节一致。

源码差异、实际lock、依赖解析和验证索引封存在 `target/thrift-grpc/local-handoff/`。这些快照记录交付前的本地候选。开发分支同步后，消费者的普通根check和新的依赖来源另存日志。Go验证沿用公共module和runner。

## Payload 边界收口

- 原39组普通 Thrift golden 在固定 Kitex gRPC codec 的 `TestGRPCCodecReusesOrdinaryThriftGolden` 上实际重新验证。另19组gRPC raw/Args/Result双向 golden未被改写；空struct/void的Rust→Go字节是`00`。
- `Nested { 1: required i32 value }` 的STOP-only `00` 必须拒绝；显式零值 `0800010000000000` 合法。Rust `cargo test -p pilota-build --test thrift_codec_golden missing_required_field_differs_from_an_explicit_zero_value` 已执行通过。
- 共用 Go module 的 `go test -mod=readonly ./oracle -run TestGRPCMalformedPayloadBoundaries -v` 已实际通过10组：缺required、显式零、负长度、截断、声明巨大容器、未知嵌套深度。原始字节/Go错误在Volo `go-grpc-data-boundary-current.log`。
- 网络上的安全预算由Volo checked adapter执行；Rust负长度/截断回13，超过容器/深度预算回8，固定Kitex将Thrift ProtocolError type ID写为wire 7。双向数据成功语义与恶意输入拒绝分类是两项不同契约；未修改旧Binary runtime或Go SDK。
- 本地候选和版本接入顺序见[候选交接](thrift-grpc-candidate.md)。普通根check已通过；源码已交付同名开发分支，Volo和下游已用实际Git来源完成普通根check，不用本地覆盖结果替代。计划全部有效checklist已完成，正式发布和部署仍未执行。远端 CI 的历史未执行记录保留，恢复后的结果以实际运行记录为准。
