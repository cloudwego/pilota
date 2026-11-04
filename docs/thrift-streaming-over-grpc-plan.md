# Pilota：Thrift unary / streaming over gRPC 数据层子计划

状态：**本仓库有效 checklist 已完成。** 本地数据/生成验证与具名开发分支交付完成，历史失败和延期项见 [实施状态](thrift-streaming-over-grpc-status.md)。源码调查：2026-11-09；开始实施：2026-11-10。

本仓库负责 P0/P1：IDL/IR、消息与异常类型、Thrift Binary payload 和 Go golden。公共协议/API 冻结、gRPC 运行时、代码生成适配与共用 runner 由 Volo G0～G6 负责，配套文件位于 Volo 仓库 `docs/thrift-streaming-over-grpc-plan.md`。本文件包含本仓库完整 checklist、交付与验收，不要求读另一份总计划才能知道 P0/P1 要完成什么。旧 TTStream 的进度和 PASS/FAIL 不被覆盖。

metainfo 的 transit/transient 固定一跳、persistent 跨多跳，由 Volo 与下游 处理；Pilota 不生成 Kitex bugfix 兼容 flag 或相应 payload 字段。

## 1. 模块归属与范围

| 模块 | 本轮交付 | 边界 |
| --- | --- | --- |
| `pilota-build/src/parser/thrift/{mod,streaming}.rs`、Context/tags/IR 查询 | 四模式、Args/Result/异常、原始 service/method/namespace、include/继承信息可供新后端使用 | 能复用则不改，不复制 Thrift parser |
| `pilota/src/thrift` | raw struct、MethodArgs/MethodResult 的 Binary 编解码与边界验证 | 不引入 HTTP/2、framing、连接池、deadline 或服务路由 |
| `pilota-build/tests`、`tests/interop/kitex-codec` | 全 service IDL 矩阵、统一异常 enum、共享类型身份的数据基础、Go golden/manifest | Cargo 测试离线读取提交产物；Go 动态生成/验证统一使用 Volo runner |
| `pilota-thrift-parser` | 原始语法/位置必要查询及回归（仅有缺口才改） | 不为新传输重构所有后端或增加选择 service 的能力 |

实施中新增的协议无关扩展还包括生成前校验、完整 item 重导出、workspace runtime 依赖声明和按定义位置解析的 qualified type path；具体证据见实施状态。它们均有默认实现，不给第三方 backend 增加必选字段/方法。

支持普通无注解 unary、显式 unary、client/server/bidi 和混合/继承 service。oneway 在新 gRPC 后端拒绝的策略属于 Volo，不污染旧 Thrift 生成入口。已有 TTStream 和 Protobuf 的合法生成行为保持。协议嗅探、H2 引擎、服务治理不属于本仓库工作。

源码交付使用现有 `feat/ttheader-streaming-20261105` 开发分支，依次同步 Pilota、Volo 和下游的源码与依赖锁。crate/tag 发布、合并主干、真实部署和服务网格集成延期。远端 CI 自 2026-11-11 起恢复，按仓库现有规则触发，不使用跳过标记。ShmIPC 保留编译验收，运行及修复延期。

## 2. 基线与依赖

| 项目 | 已核对基线 |
| --- | --- |
| 本仓库 HEAD | `50e79266f2dc7e78a71f7b5c9f59eccf3813f7d6` |
| 当前包 | pilota 0.13.1、pilota-build 0.14.0-alpha.1、pilota-thrift-parser 0.13.8 |
| Go oracle | 上游 Kitex v0.16.4、thriftgo v0.4.5 |
| Volo 候选基线 | HEAD `7dd69f9839be31cf6c035e33c92316cf25991045`；实际联调以 lock/manifest 为准 |
| 前置输入 | Volo G0 的 wire/API/名称规则、fixture schema；不依赖 Volo 正式发布 |
| 输出 | P0 结构化查询和编译证据 → G1/G2；P1 golden/数据差异 → G1/G2/G5 |

SHA 仅记录调查来源，不写入 Cargo.toml rev。跨仓 Git 依赖使用具名分支，Cargo.lock/manifest 保存解析 SHA；与本地 checkout 不一致时分别记录。公共文件和证据只包含公共源码、IDL 与工具版本，不含私有仓库或本机绝对路径。

## 3. 数据与 IDL 契约

### 3.1 Payload 形状

| 模式/结果 | Thrift Binary payload |
| --- | --- |
| unary 请求 | 生成的 MethodArgs，保留每个参数字段 ID；单 struct 参数仍有 Args 包装 |
| unary 成功 | MethodResult 的 success=0 字段；void 为合法空 Result |
| unary 声明异常 | MethodResult 中对应 throws 字段的原始 ID，只设置一个声明分支 |
| 三种 streaming | 原始 Req/Resp 结构体，不加 Args/Result |
| 空结构体/空 Args/void Result | 合法 STOP 字节 `[0x00]`，不是零字节消息 |

不编码 Thrift message begin/end、method/type/seq-id envelope。gRPC 的压缩标志和 4 字节大端长度由 Volo 添加，Pilota 只提供 Binary 大端结构体字节；不新增 Compact/little-endian 支持。字段和容器规则沿用现有 Thrift，map/set golden 允许合法顺序差异。

例如 `Resp Echo(1: Req req) throws (1: ExA a, 2: ExB b)`：

```text
request = EchoArgs   { field 1: Req req; STOP }
success = EchoResult { field 0: Resp success; STOP }
error A = EchoResult { field 1: ExA a; STOP }
error B = EchoResult { field 2: ExB b; STOP }
```

返回空/多个 Result 字段、缺 success/未知异常的 RPC 容错由 Volo G0/G2 冻结；P0/P1 必须确认现有解码是否提前合并或丢弃这些信息。必要时只提供可保留状态的查询/适配基础，不为新后端改变旧 Thrift runtime 的既有语义。最终 Trailer 错误优先于已解码结果由运行时负责，数据层不伪造 Status。

### 3.2 生成和校验

| IDL | 规则 |
| --- | --- |
| 无 streaming 注解 | 普通 unary；零/一/多参数、scalar/struct/container/typedef、void、throws 合法 |
| 显式 `streaming.mode="unary"` | unary Args/Result；保留恰好一个参数的现有规则，其余合法 unary 类型不套用 raw message 限制 |
| client/server/bidirectional | 请求/响应为当前允许的 message 类型，沿用 typedef/union/Arc/Box 解析；非法模式/签名、streaming throws/oneway 拒绝 |
| 任意 service 包含 streaming throws | 整个构建失败，包括未被业务使用的 service；不增加选择 service、忽略 throws 或 opt-out |
| 新 gRPC 后端的全部 oneway | 由 Volo 新入口预检拒绝，不修改普通 Thrift 入口的原有支持 |
| include/继承/重命名 | 保留真实定义路径、模式、声明位置和 wire 名，Rust rename 不重写线上 service/method |

固定 thriftgo v0.4.5 的 ParseStreaming 同样把显式 unary 纳入“恰一参数”检查，Kitex v0.16.4 makeMethod 在选择模板之前调用它。因此“显式 unary 多参数 Go 能编、Rust 不能”不符合本基线。G0/P0 golden 要覆盖无注解/显式 unary × 0/1/2 参数：两边显式 unary 均只接受 1；无注解接受 0/1/2。迁移时多参数方法保留无注解，由客户端显式选择 transport.GRPC。

namespace go、无 namespace、跨文件 include/继承的 PackageName 必须以 thriftgo PkgInfo/实际 Go ServiceInfo 为准。Pilota 只暴露原始信息和归属文件；Volo 负责 wire package/service 覆盖策略，不能把 Protobuf file.package 的规则直接套进 Thrift IR。

### 3.3 统一方法级异常类型

Pilota lower_service 对非空 throws 已生成 `{Service}{Method}Exception` enum，lower_method.exceptions 指向该类型。新 gRPC 后端复用它；单异常也保持枚举，不简化成异常结构体/其别名，不额外生成第二份类型。

```rust
// Foo.Echo throws (1: ExA a, 2: ExB b) 的类型形状
pub enum FooEchoException {
    A(ExA), // field 1
    B(ExB), // field 2
}
```

variant 沿用声明字段名的 Rust 命名和原始 field ID，不能只按异常载荷类型名命名；include/继承通过原定义路径复用。Volo handler/client 内层使用 Result<Ret, FooEchoException>，void 使用 Result<(), FooEchoException>；无 throws 不引入空异常枚举。枚举与 Args/Result 的转换由 Volo 生成，不增加嵌套 wire wrapper。

1→N throws 保持返回类型名称和已有 variant 的构造方式，新增 variant 仍可能破坏穷尽 match；0→1 会改变公开返回形状。记录这两类变化，不全局添加 non_exhaustive/Unknown variant 或强加 Box/Clone/Sync。两个传输生成入口必须共享 Pilota 消息及异常类型身份，不能只证明文本长得一样。

## 4. 交付接口、Go 设施与证据格式

P0 提交结构化方法查询/复用说明：模式、Args/Result/原始消息、method.exceptions、名称/定义文件/继承关系及必要诊断位置。现有接口足够时以编译 fixture 证明，不为让仓库出现代码改动而重构。普通 Message 不需要 pb::Message 或额外 Default/Clone/Sync；Arc 只承担其本身需要的共享约束。

P1 复用本仓库 tests/interop/kitex-codec 和 Volo tests/interop/kitex-streaming 的共用工具，提交 golden、IDL、Go 生成参数/版本及 manifest。Go 动态工具要从实际 Kitex gRPC codec 路径生成/消费 payload，不能用旧 TTStream 成功报告代替来源证明。Go 生成一次后提交，普通 Cargo 测试离线运行。

manifest 至少记录：schema/case 版本、Kitex/thriftgo/toolchain、Go module/sum、IDL 与产物 hash、生成入口/参数、无注解或显式 unary、payload 类型、字节/字段比较规则、正负例、命令与预期/实际、生成时间与执行状态。Rust→Go 空 STOP 必须单列。完整 gRPC framing/压缩/网络/cardinality 验收由 Volo G1/G5 负责，不在 Pilota 维护第二套 runner/Go module。

## 5. 实施 checklist

以下完整保留拆分前 P0/P1 的 9 条工作项，并加稳定编号。仅在对应实现或明确复用结论及可复现证据齐全时勾选；未运行不写 PASS。

### P0：复用 IDL 语义，提供生成所需信息

依赖：G0 的 wire path/API 契约。主要位置：`pilota-build/src/parser/thrift/`、Context/tags 与 tests。

- [x] **P0-01** 核对四模式、unary 参数/返回/声明异常与 Args/Result、streaming 原始 Req/Resp、继承/include/typedef/union/Arc/Box 的现有信息足够新后端使用；不足部分只新增协议无关查询，不在 Pilota 引入 HTTP/2 或部署适配依赖。
- [x] **P0-02** 暴露生成 wire name 所需的原始 service/method 与 namespace 信息，Rust rename 不改变线上名字；Go 包名计算与覆盖策略由 Volo 后端消费。
- [x] **P0-03** 保持现有全 service 校验；三种 streaming 方法的 throws、oneway、非法模式、非消息签名仍整体失败。普通 unary 的零/多参数、scalar/container/void/throws 保持合法；显式 unary 保留既有单参数规则。新 gRPC 后端拒绝全部 oneway 的检查不污染通用 Pilota 或旧 Thrift 入口。
- [x] **P0-04** 用编译 fixture 确认普通 Thrift Message 不需要 pb::Message、Default/Clone/Sync；Arc 仅施加其自身必要的共享约束。

完成条件：Volo 可从结构化信息生成纯 unary、纯 streaming 与混合 service，无重复解析 IDL 或字符串替换 streaming tag。若无源码修改需求，提交明确的复用验证结果即可，不为“三仓库都有代码改动”人为重构。

### P1：Thrift payload golden 与回归

- [x] **P1-01** 复用 `tests/interop/kitex-codec` 的已提交普通结构体 golden，增加由 **Kitex gRPC codec 路径**生成的 payload 来源校验，不能只拿 TTStream 互通报告当证明。
- [x] **P1-02** 增加 unary Args/Result golden：单 struct 参数与 raw Req 字节不同、零/多参数、原始字段 ID、success=0、每个声明异常分支、void 空 Result、异常与缺字段输入；区分无注解/显式 unary，证明数据层不是复用 streaming raw 消息替代包装。
- [x] **P1-03** 覆盖整数/浮点/binary/空结构体、nested/container、unknown field、optional/required、字段乱序、截断/负长度/深度与大长度；合法 trailing bytes 容错以固定 Go decoder fixture 决定并记录。空结构体为 `[0x00]` STOP，明确提交 Rust 编码后经 Kitex fast codec 解码的 fixture，0 字节负例独立记录，不能只测 Go 发 Rust 收。
- [x] **P1-04** 用 Rust 解 Go bytes、Go 解 Rust bytes；普通 Cargo 测试只读提交的 golden，不启动 Go/联网。Go 动态验证与更新仍由 Volo 共用 runner 负责。
- [x] **P1-05** 无注解 Thrift、已有 TTStream 生成与 Protobuf 样例回归；必要的 Pilota 源码改动执行 default check、nightly fmt/clippy 和相关 tests。

完成条件：新 codec 的结构体来源可追溯、字节/语义一致、失败边界有回归；不新增第三套 runner/Go module。

### 数据与 API 交付的专项 checklist

- [x] **P-CONTRACT-01** G0/P0 编译矩阵包含无注解/显式 unary × 0/1/2 参数、全 service 失败、include/继承、非法 mode/重复 annotation；保存固定 Go 的实际诊断。
- [x] **P-CONTRACT-02** method.exceptions 在单/多异常和 void 场景保留同一枚举形状；声明名称/ID、重名/关键字、原始类型路径可供后端读取。
- [x] **P-CONTRACT-03** 为 Volo G2 的双生成入口提供共享数据路径/复用配置和最小 IDL，证明一份消息/异常定义；不在本仓库实现 HTTP/2 或 gRPC service 模板。
- [x] **P-CONTRACT-04** 核对 malformed Result 的多字段/未知字段信息是否被现有 decoder 丢弃，给 G0/G2 明确证据和最小适配方案，不暗改旧 decoder 语义。

## 6. Go 数据互操作与编译验收

沿用共同 I 编号，下表是本仓库需交付的数据/IR证据；Volo 负责同编号的网络层部分。每行都在 manifest 落到实际 case。

| ID | 本仓库必须完成的验证 | 与 Volo 的交接 |
| --- | --- | --- |
| I02/I23 | raw 与 Args/Result 字节区分；零/多参数、非 1 字段 ID、success=0、scalar/container/typedef/void | G1 添加 prefix；G2 选择包装；G5 做两向网络 |
| I03 | Go bytes→Rust、Rust bytes→Go；空 STOP/零字节、未知字段、嵌套/大容器/截断/负长度/深度 | 保留正常值和错误差异；明确 Rust 发 Go fast codec 收空结构体 |
| I04 | namespace/无 namespace/include/继承/rename 及原始名称/文件归属 | 提供 IR 和 Go ServiceInfo golden，G0/G2 冻结路径 |
| I22 | 五类 IDL 模式、全 service 校验、throws/oneway、显式 unary 参数限制 | 只在新 gRPC 后端拒绝所有 oneway；普通 Thrift 兼容性由 P0 证明 |
| I24 | 单/多声明异常、每个 field ID、void、缺 success/未知/多字段 Result；异常 enum 形状 | G2 生成 Result/enum 转换与 handler；G5 保证 Trailer 优先级 |
| I27 | 普通 Thrift Message 的无 pb/Default/Clone/Sync 强加、Arc/Box 的真实约束 | Volo 负责 RecvStream/自定义 Decoder 的兼容 fixture |
| I32 | 同一 include 消息/异常可被两个传输后端直接复用；继承路径正确 | G2 在真实双模块生成 crate 证明类型身份，不仅比较生成文本 |

- [x] **P-INTEROP-01** Volo 共用 runner 提供本表 case 的 Go payload 更新/校验入口，P1 记录实际命令；Go 工具不可用或未跑时明确标 NOT_EXECUTED。
- [x] **P-INTEROP-02** golden 来源为实际 Kitex gRPC codec；Rust 解 Go 与 Go 解 Rust 都有证据，不能用 Rust 自回环或一种方向代替。
- [x] **P-INTEROP-03** 已提交 golden 的离线 Rust 测试可运行，不在 cargo check/test 中联网或启动 Go；更新需明确 review，不自动覆盖失败输入。
- [x] **P-INTEROP-04** 失败附最小 IDL/字节、编译或运行命令、触发条件、预期/实际、原因、是否执行；Go 容错或不同版本差异保留原结果。

最小注解反例（等价 0/1/2 参数矩阵已在固定 Go/Rust 生成器上实际执行）：

```thrift
struct Req { 1: string value }
struct Resp { 1: string value }
service Edge {
    Resp Echo(1: Req a, 2: Req b) (streaming.mode="unary")
}
```

Go 的 `TestGRPCIDLUnaryParameterMatrix` 实际以 `-streamx` 生成无注解/显式 unary 的 0/1/2 参数组，Rust 的 `thrift_grpc_ir::explicit_unary_parameter_matrix_preserves_the_existing_idl_rules` 对照同一规则：显式 0/2 拒绝，无注解及显式 1 通过。固定工具、执行结果与原始命令见实施状态。长度/深度的网络安全预算在 Volo 的 checked Binary adapter 验证，不对不可信输入直接调用无预算的旧 Pilota decoder。

## 7. 编译、回归与性能 checklist

```bash
cargo check
cargo +nightly fmt --all -- --check
cargo test -p pilota-build --test thrift_streaming --test thrift_codec_golden
cargo +nightly clippy -p pilota -p pilota-build --all-targets -- -D warnings
# 仅修改 parser 时，追加其适用 feature/target 的检查
cargo check -p pilota-thrift-parser --all-targets
```

- [x] **P-QUALITY-01** 根普通 cargo check、实际变更包/feature/target 编译通过，不要求手动 path patch、临时环境变量或在线生成。
- [x] **P-QUALITY-02** 现有 Thrift/TTStream/Protobuf 生成与 offline golden、无注解/显式 unary、单/多异常及 shared types 回归通过。
- [x] **P-QUALITY-03** nightly fmt/clippy 无新增问题，基线已有问题与无法运行原因单列；沿用现有 lint 配置，不新增全局 allow。
- [x] **P-QUALITY-04** 若修改 Binary runtime，测相关 payload 的吞吐/分配/retained memory 与原基线；不把只新增 metadata 查询的变更扩展成全网压测。
- [x] **P-QUALITY-05** 新增约束/IR/trait/public 字段的下游源码兼容已验证，尤其不迫使第三方填新必选字段；错误诊断定位到文件/service/method。

Volo G6 负责 end-to-end 吞吐、H2/池与 600 秒资源验收；Pilota 只交付自己改动引入的数据层成本证据。产物写仓库 target/ 或显式证据目录并明确清理归属，不向 /tmp 堆积。仅文档变更不运行或宣称上述验证通过。

## 8. 候选、版本与交付顺序

```text
Volo G0 协议/API/schema
  → P0 查询/校验/异常类型复用
  → P1 Kitex payload golden 与离线回归
  → Volo G1/G2/G5 集成
  → P0/P1 根据公共候选反馈完成复核
```

不依赖正式发布。若 Pilota 信息与 codec 已足够，则提交复用证明/golden/文档即可，不人为改 runtime。需要新增 build 查询时在当前 0.14.0 候选线上准备 alpha.2；runtime/parser 按实际变更及兼容性决定版本。0.x 公共破坏应升 minor 并附迁移表，不在普通 patch 中偷偷改变旧生成语义。

Git 依赖只用 branch，不用 rev；候选锁记录解析 SHA。公共提交、IDL、生成物和报告审计仅包含开源内容。准备版本/材料不等于已经发布，实际发布不在本轮动作内；远端 CI 恢复正常触发，历史未执行记录保留。

## 9. 本仓库最终完成 checklist

- [x] **P-DONE-01** P0/P1 和专项/互通/质量项全部有可复现证据；没有源码改动的模块有明确复用结论。
- [x] **P-DONE-02** unary Args/Result、raw streaming、统一异常 enum 与显式 unary 参数限制完整交付，Go 双向数据验证和离线 golden 通过。
- [x] **P-DONE-03** Volo G1/G2 能只依赖结构化查询和同一份 Pilota 消息/异常类型完成接入，名称/归属与未知字段边界没有未归属缺口。
- [x] **P-DONE-04** 普通 cargo check、nightly fmt/clippy 和相关回归无新增问题；公共源码兼容或必要版本迁移有证据。
- [x] **P-DONE-05** manifest、最简失败复现、候选分支/锁、版本方案与未执行清单完整；没有执行发布或部署；远端 CI 已恢复正常触发，历史未执行记录不计 PASS。材料位置见[候选交接](thrift-grpc-candidate.md)。
