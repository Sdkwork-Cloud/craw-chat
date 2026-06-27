> Migrated from `docs/review/step-07-cp07-3-质量审计与复盘-2026-04-07.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 07 / CP07-3 质量审计与复盘- 2026-04-07

## 审计范围

- `crates/sdkwork-im-runtime-link/Cargo.toml`
- `crates/sdkwork-im-runtime-link/src/lib.rs`

## 审计结论

- 本轮未发现阻`CP07-3` 交付的剩余功能缺陷
- runtime hello 协商已经从本hardcode 切换control-plane `effective snapshot` 驱动，`CP07-2` 产出的治理快照首次真正进入热路径消费
- `CP07-3` 完成不等`Step 07` 完成，`CP07-4` 仍是当前主阻塞

## 正向结果

- runtime 现在同时具备两种受控入口
  - 默认入口：从 `control_plane_v1()` 缓存默认 hello policy
  - 显式入口：通过 `new_with_effective_snapshot(...)` 注入权威快照
- 这使 hello 协商不再散落本地默认值，协议版本、binding、capability 三个决策点已统一control-plane snapshot 约束：
- 包级回归暴露并修正了两个旧测试的权威来源漂移问题，说明这次改动不仅新增了能力，还把旧断言同步收回到了控制面权威字段

## 本轮发现并修复的问题

- 初次绿测编译失败
  - `hello.binding` 在共享引用场景下move
  - 已改`clone()`，属于局部所有权修正，不涉及行为变更
- 包级回归发现两条旧测试与新权baseline 冲突
  - 旧测试默认要`session.resume`
  - 旧测试默认拒`ccp/sse/1`
- 经对`sdkwork-im-ccp-registry` `effective snapshot` baseline，当前真实默认值是
  - `payload.json / realtime.pull / realtime.push`
  - `ccp/http/1 / ccp/ws/1 / ccp/sse/1`
  - `ccp/mqtt/1` 仍被 kill switch 关闭
- 测试已按真实权威结果收口，没有发现实现与控制面快照不一致的情况

## 剩余风险

- 默认 hello policy 仍来源于 `control_plane_v1()` 的静baseline 缓存，不代表 `CP07-4` 已完成动态运维编排或控制面变更传播
- `parse_protocol_version(...)` 依赖 control-plane 输出严格遵守 `family/major.minor` 格式；当前风险可接受，因为该字段仍由内部 registry 冻结生成
- admin auth / audit / ops 未闭环前，Step 07 还不能宣control-plane 治理完整落地

## 验证证据

- `cargo test -p sdkwork-im-runtime-link --offline --target-dir target-step07-cp073-green-runtime-link -- --exact tests::test_link_session_negotiates_hello_from_effective_snapshot`
- `cargo test -p sdkwork-im-runtime-link --offline --target-dir target-step07-cp073-green-runtime-link-bind -- --exact tests::test_link_session_rejects_binding_blocked_by_effective_snapshot`
- `cargo test -p sdkwork-im-runtime-link --offline --target-dir target-step07-cp073-final-runtime-link`
- `cargo test -p sdkwork-im-ccp-registry --offline --target-dir target-step07-cp073-final-registry`
- `cargo test -p control-plane-api --offline --target-dir target-step07-cp073-final-control-plane`
- `cargo test -p im-platform-contracts --test ccp_foundation_smoke_test --offline --target-dir target-step07-cp073-final-platform-smoke`
- `cargo fmt -p sdkwork-im-runtime-link -- --check`

## 复盘结论

- `CP07-3` 的推进顺序是正确的：先冻control-plane snapshot，再runtime 只读消费，符`142 / 148 / 149` 的依赖顺序
- 下一轮不应扩散到 `Step 08`，而应立即推进 `CP07-4`，把管理面权限、审计和运维编排补齐，否`Step 07` 仍停留在“runtime 已消费快照，但治理闭环还不完整”的状态

