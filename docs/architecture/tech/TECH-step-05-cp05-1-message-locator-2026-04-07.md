> Migrated from `docs/review/step-05-cp05-1-message-locator-执行补充-2026-04-07.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 05 CP05-1 MessageLocatorIndex 执行补充 - 2026-04-07

## 1. 当前定位

- 当前波次：`Wave B`
- 当前 Step：`05`
- 当前切口：`CP05-1`
- 本轮原因：上一轮只剩 runtime-level `message_index: HashMap<String, String>` 仍在 `conversation-runtime`，导致 `CP05-1` 不能闭环。

## 2. 本轮实际完成

- 新增 domain owner：
  - `crates/im-domain-core/src/message.rs::MessageLocatorIndex`
    - `register(...)`
    - `register_message(...)`
    - `conversation_id(...)`
- runtime 消费改造：
  - `services/conversation-runtime/src/runtime.rs`
    - `RuntimeState` 改为持有 `message_locator: MessageLocatorIndex`
    - 删除 `message_index: HashMap<String, String>`
    - edit / recall 路径改为通过 `MessageLocatorIndex` 找回 conversation owner
  - `services/conversation-runtime/src/runtime/recovery.rs`
    - replay `message.posted` 时恢复 `MessageLocatorIndex`
- 清理：
  - `services/conversation-runtime/src/runtime/support.rs`
    - 删除无消费者 `message_scope_key(...)`

## 3. 改动文件

- 代码
  - `crates/im-domain-core/src/message.rs`
  - `services/conversation-runtime/src/runtime.rs`
  - `services/conversation-runtime/src/runtime/recovery.rs`
  - `services/conversation-runtime/src/runtime/support.rs`
- 测试
  - `crates/im-domain-core/tests/conversation_domain_builder_test.rs`
  - `services/conversation-runtime/tests/conversation_domain_structure_test.rs`

## 4. TDD 证据

- Red
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-1f-red-domain'; cargo test -p im-domain-core --test conversation_domain_builder_test test_message_locator_index_resolves_message_to_conversation --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-1f-red-runtime'; cargo test -p conversation-runtime --test conversation_domain_structure_test test_runtime_state_uses_domain_message_locator_for_cross_conversation_lookup --offline`
- Green
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-1f-green-domain'; cargo test -p im-domain-core --test conversation_domain_builder_test test_message_locator_index_resolves_message_to_conversation --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-1f-green-runtime'; cargo test -p conversation-runtime --test conversation_domain_structure_test test_runtime_state_uses_domain_message_locator_for_cross_conversation_lookup --offline`

## 5. 完整验证

- `rustfmt --edition 2024 crates/im-domain-core/src/message.rs crates/im-domain-core/tests/conversation_domain_builder_test.rs services/conversation-runtime/src/runtime.rs services/conversation-runtime/src/runtime/recovery.rs services/conversation-runtime/src/runtime/support.rs services/conversation-runtime/tests/conversation_domain_structure_test.rs`
- `rustfmt --edition 2024 --check crates/im-domain-core/src/message.rs crates/im-domain-core/tests/conversation_domain_builder_test.rs services/conversation-runtime/src/runtime.rs services/conversation-runtime/src/runtime/recovery.rs services/conversation-runtime/src/runtime/support.rs services/conversation-runtime/tests/conversation_domain_structure_test.rs`
- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-1f-im-domain-core-full'; cargo test -p im-domain-core --offline`
- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-1f-conversation-runtime-full'; cargo test -p conversation-runtime --offline`
- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-1f-projection-service-full'; cargo test -p projection-service --offline`
- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-1f-local-node-full'; cargo test -p sdkwork-im-cloud-gateway --offline`
- 备注：`sdkwork-im-server` 的 `deployment_profile_test` 会打印预期的启动失败/健康检查超时样例日志，但 suite 退出码为 `0`。

## 6. 架构映射

- 对应能力
  - `docs/架构/09-实施计划.md`
  - `docs/架构/130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md`
  - `docs/架构/136-关键业务链路与跨Plane时序设计-2026-04-06.md`
  - `docs/架构/147-CCP到Crate与接口模块落地映射设计-2026-04-06.md`
- 已兑现
  - cross-conversation message lookup owner 已从 runtime-local `HashMap` 迁到 domain crate
- 未兑现
  - `sender / tenant` authority closure
  - direct / group / channel 规则重收口
  - projection / notification / multi-client-route sync owner 重收口
- 偏离判断
  - 无新增架构偏离

## 7. 本轮结论

- `CP05-1` 已闭环
- `Step 05` 仍未闭环
- `91 / 95 / 97` 对 `Step 05` 整体仍未通过
- `Wave B / 93` 仍阻塞
- 下一轮进入 `Wave B / Step 05 / CP05-2`

