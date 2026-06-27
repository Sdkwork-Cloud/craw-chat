# Step 05 CP05-3 direct/group/channel mainline scenario owner 质量审计

## 1. 审计范围

- `crates/im-domain-core/src/conversation.rs`
- `crates/im-domain-core/tests/conversation_domain_builder_test.rs`
- `services/conversation-runtime/src/runtime.rs`
- `services/conversation-runtime/src/runtime/policy.rs`
- `services/conversation-runtime/tests/conversation_domain_structure_test.rs`
- `services/conversation-runtime/tests/conversation_flow_test.rs`
- `services/sdkwork-im-cloud-gateway/tests/access_control_e2e_test.rs`

## 2. 审计结论

- 审计通过项
  - direct / group / channel 主链路场景 owner 已下沉到 domain aggregate
  - runtime policy 已改为消费 `ConversationAggregateState::scenario()`
  - 结构测试锁定 runtime 不得继续依赖 raw `conversation_type` 进行主链路分流
  - direct / group / system channel 三条关键主链路都有 fresh 行为验证
- 当前裁定
  - `CP05-3` 可判定通过
  - `Step 05` 不可判定通过，原因是 `CP05-4` 仍未完成

## 3. 证据

- 结构红绿证据
  - `cargo test -p im-domain-core --test conversation_domain_builder_test test_conversation_aggregate_state_projects_direct_group_and_channel_scenarios --offline`
  - `cargo test -p conversation-runtime --test conversation_domain_structure_test test_runtime_policy_uses_domain_scenario_owner_for_direct_group_channel_paths --offline`
- 主链路行为证据
  - `cargo test -p conversation-runtime --test conversation_flow_test test_direct_conversation_owner_can_add_only_single_non_elevated_peer --offline`
  - `cargo test -p conversation-runtime --test conversation_flow_test test_group_owner_can_transfer_ownership_and_then_leave --offline`
  - `cargo test -p conversation-runtime --test conversation_flow_test test_system_channel_requires_dedicated_publish_command_and_allows_only_publisher --offline`
- local profile e2e 证据
  - 使用 `$env:CARGO_TARGET_DIR='target-cp053'` 规避被常驻 `sdkwork-im-server.exe` 锁住的默认 target
  - 三条 access-control e2e 均已通过

## 4. 风险与后续

- 当前未发现 `CP05-3` 范围内新的架构偏离
- 剩余风险已转移到：
  - `CP05-4` projection / notification / multi-client-route sync 与新模型的最终 owner 收口
  - `Step 05` 整体 `91 / 95 / 97` 闭环
