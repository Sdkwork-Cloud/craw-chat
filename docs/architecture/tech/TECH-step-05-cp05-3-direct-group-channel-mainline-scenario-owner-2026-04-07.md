> Migrated from `docs/review/step-05-cp05-3-direct-group-channel-mainline-scenario-owner-执行补充-2026-04-07.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 05 CP05-3 direct/group/channel mainline scenario owner 执行补充

## 1. 当前定位

- 波次：`Wave B`
- Step：`Step 05`
- 子项：`CP05-3`
- 本轮目标：把 direct / group / channel 主链路的场景分流 owner 从 runtime 内部原始 `conversation_type` 字符串分支，收口到 domain aggregate 自身拥有的 scenario 语义

## 2. 本轮为什么做这一项

- `CP05-2` 闭环后，`Step 05` 剩余的下一个顺序子项就是 `CP05-3`，不能跳到 `CP05-4` 或 `Step 06`
- `docs/step/05-消息与会话主链路重构.md` 明确要求 direct / group / channel 主链路必须跑通，而且要收口到统一主链路 owner
- 当前仓库在 `services/conversation-runtime/src/runtime/policy.rs` 里仍保留了多处基于原始 `conversation_type` 的主链路能力分流，这会让 direct / group / channel 的主链路收口停留在 runtime 字符串分支层，达不到 `91 / 95 / 97` 所要求的领域 owner 闭环

## 3. 本轮实际完成

- `crates/im-domain-core/src/conversation.rs`
  - 新增 `ConversationScenario`
    - `Group`
    - `Direct`
    - `AgentDialog`
    - `AgentHandoff`
    - `SystemChannel`
    - `Unknown`
  - 新增 `ConversationScenario::from_conversation_type(...)`
  - 新增 `ConversationAggregateState::scenario()`
- `services/conversation-runtime/src/runtime/policy.rs`
  - direct / group / channel 主链路能力分支改为消费 `conversation.aggregate.scenario()`
  - 主要覆盖：
    - 创建类型校验
    - handoff 场景约束
    - member add / remove / leave
    - owner transfer / role change
    - message post
    - system channel publish
    - conversation-bound write
  - edit / recall 规则辅助函数也改为消费 `ConversationScenario`
- `services/conversation-runtime/src/runtime.rs`
  - edit / recall 路径改为从 aggregate 提取 `scenario()` 并透传到 policy
- 测试
  - `crates/im-domain-core/tests/conversation_domain_builder_test.rs`
    - 新增 `test_conversation_aggregate_state_projects_direct_group_and_channel_scenarios`
  - `services/conversation-runtime/tests/conversation_domain_structure_test.rs`
    - 新增 `test_runtime_policy_uses_domain_scenario_owner_for_direct_group_channel_paths`

## 4. 测试与验证证据

### 4.1 TDD Red

- `cargo test -p im-domain-core --test conversation_domain_builder_test test_conversation_aggregate_state_projects_direct_group_and_channel_scenarios --offline`
- `cargo test -p conversation-runtime --test conversation_domain_structure_test test_runtime_policy_uses_domain_scenario_owner_for_direct_group_channel_paths --offline`

### 4.2 Green / 主链路验证

- `cargo fmt --all`
- `cargo fmt --all --check`
- `cargo test -p im-domain-core --test conversation_domain_builder_test --offline`
- `cargo test -p conversation-runtime --test conversation_domain_structure_test --offline`
- `cargo test -p conversation-runtime --test conversation_flow_test test_direct_conversation_owner_can_add_only_single_non_elevated_peer --offline`
- `cargo test -p conversation-runtime --test conversation_flow_test test_group_owner_can_transfer_ownership_and_then_leave --offline`
- `cargo test -p conversation-runtime --test conversation_flow_test test_system_channel_requires_dedicated_publish_command_and_allows_only_publisher --offline`

### 4.3 sdkwork-im-server e2e

- 注意：默认 `target\\debug\\sdkwork-im-server.exe` 被常驻进程锁定，直接复用默认 target 会触发 Windows `os error 5`
- 本轮 e2e 使用隔离 target：
  - `$env:CARGO_TARGET_DIR='target-cp053'; cargo test -p sdkwork-im-cloud-gateway --test access_control_e2e_test test_direct_conversation_member_management_is_restricted --offline`
  - `$env:CARGO_TARGET_DIR='target-cp053'; cargo test -p sdkwork-im-cloud-gateway --test access_control_e2e_test test_group_owner_transfer_allows_safe_handoff_and_leave --offline`
  - `$env:CARGO_TARGET_DIR='target-cp053'; cargo test -p sdkwork-im-cloud-gateway --test access_control_e2e_test test_system_channel_dedicated_publish_allows_only_publisher_in_local_profile --offline`

## 5. 当前判断

- 基于当前代码与测试证据，`CP05-3` 已进入闭环状态
- 本轮没有偏离 `Step 05` 的 direct / group / channel 主链路收口目标
- 但 `Step 05` 整体仍未闭环：
  - `CP05-4` 未完成
  - `91 / 95 / 97` 不能整体判定 `Step 05` 通过
  - `Wave B / 93` 仍然阻塞

