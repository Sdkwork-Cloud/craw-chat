> Migrated from `docs/review/step-05-cp05-1-message-locator-架构兑现-2026-04-07.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 05 CP05-1 MessageLocatorIndex 架构兑现 - 2026-04-07

## 1. 对应架构文档

- `docs/架构/09-实施计划.md`
- `docs/架构/130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md`
- `docs/架构/136-关键业务链路与跨Plane时序设计-2026-04-06.md`
- `docs/架构/147-CCP到Crate与接口模块落地映射设计-2026-04-06.md`

## 2. 已兑现能力力力

- `MessageLocatorIndex` 作为跨会话消息定位 owner 已落到 `im-domain-core::message`
- runtime live path 与 replay path 统一通过 domain owner 完成 message -> conversation 定位
- runtime 不再维护 direct `message_index`
- `CP05-1` 可以据此改判为已闭环

## 3. 未兑现能力力力

- `CP05-2`
  - `sender / tenant` authority closure
- `CP05-3`
  - direct / group / channel 规则 owner 重收口
- `CP05-4`
  - projection / notification / multi-client-route sync owner 重收口

## 4. 偏离判断

- 无新增架构偏离。
- 本轮属于补齐既有 runtime-local 残留 owner，与现有架构蓝图一致。

## 5. 证据

- 代码
  - `crates/im-domain-core/src/message.rs`
  - `services/conversation-runtime/src/runtime.rs`
  - `services/conversation-runtime/src/runtime/recovery.rs`
  - `services/conversation-runtime/src/runtime/support.rs`
- 测试
  - `crates/im-domain-core/tests/conversation_domain_builder_test.rs`
  - `services/conversation-runtime/tests/conversation_domain_structure_test.rs`
- 验证
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-1f-im-domain-core-full'; cargo test -p im-domain-core --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-1f-conversation-runtime-full'; cargo test -p conversation-runtime --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-1f-projection-service-full'; cargo test -p projection-service --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-1f-local-node-full'; cargo test -p sdkwork-im-cloud-gateway --offline`

## 6. 决议

- 本轮确认 `CP05-1` 已闭环。
- `Step 05` 不得提前判完成。
- 下一轮必须切换到 `Wave B / Step 05 / CP05-2`。

