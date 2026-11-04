# Kitex codec / IDL fixtures

这些产物由 Volo 共用 `kitex_streaming_interop` 工具的固定 Kitex v0.16.4 Go oracle 实际生成、编译或编码验证。Pilota 不维护独立 Go module 或 runner；普通 Rust 测试完全离线消费此目录。

- `manifest.json`：固定 Go 版本、module/sum、生成参数、源文件与产物 SHA256、候选最低依赖。
- `golden.json`：39 个普通结构体与 ApplicationException raw body；规范字节逐一比较，map/set/未知字段/乱序用字段语义比较。不是 TTHeader 帧或 EXCEPTION envelope。
- `idl-matrix.json`：31 个输入 × StreamX/legacy 两种生成方式的真实结果，含 Go 生成代码编译结果、签名快照和原始诊断；Rust 预期由真实 Builder 子进程逐项验证。
- `payload-matrix.json`：真实 `ttstream.EncodePayload` 对普通 struct 成功、scalar/list/map/set 拒绝。Go 可以生成并编译这些签名，不等于能在 TTStream 上发送；Rust 提前拒绝。
- `rust-golden-go-verify.json`：39 个 Rust 再编码结果经同版本 Go oracle 解码复核通过。

工具 module 是 v0.16.4；该版本生成器自报 v0.16.3，不能用自报值替代 module/sum 和二进制 SHA256。Go 接受 streaming throws；Rust 在任一 service 出现该声明时整体返回构建诊断，沿用生成所有 service 的规则。

离线验证：

```bash
cargo +nightly test -p pilota-build --test thrift_streaming --test thrift_codec_golden
```

生成供共用工具反向验证的 Rust bytes（不会覆盖 golden）：

```bash
PILOTA_CODEC_VERIFY_OUTPUT=/tmp/rust-golden.json cargo +nightly test -p pilota-build --test thrift_codec_golden
```

显式更新只使用 Volo 工具，按计划的 `idl-matrix` / `codec-golden --regenerate` 命令生成，review 差异后更新此目录。不能在 cargo test 中自动更新期望或启动 Go。


The executed maintenance runner source is pinned separately in
`maintenance-tool-v1.json`; the original generation provenance remains in
`codec-source-manifest.json`. `common-codec-verify.json` records 39 Go and 39 Rust
reverse checks through that shared entry point. `common-codec-regenerate.json`
records explicit proposal generation, with no mutation of committed golden bytes.
Broader runner suites and later source revisions are not implied by these reports.
