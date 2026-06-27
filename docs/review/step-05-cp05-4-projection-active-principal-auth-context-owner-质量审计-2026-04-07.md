# Step 05 / CP05-4 projection active-principal auth-context owner 质量审计 - 2026-04-07

## 1. 审计结论

- 本轮增量通过。
- `projection-service::access` 已开始统一拥有 conversation active principal mapping 的 auth-context capture。
- `sdkwork-im-server/effects.rs` 不再在 notification / realtime recipient 解析时直接读取 runtime member roster。

## 2. 证据

- 结构证据
  - `test_projection_service_access_module_exposes_auth_context_entrypoints`
  - `test_local_minimal_node_effects_member_fanout_uses_projection_auth_context_entrypoints`
  - `$env:CARGO_TARGET_DIR='target-cp054j-green-local-structure'; cargo test -p sdkwork-im-cloud-gateway --test lib_structure_test --offline`
- 行为证据
  - `test_active_conversation_principal_ids_from_auth_context_returns_current_active_members`
  - `test_local_minimal_profile_fanouts_message_notifications_to_other_active_members_only`
  - `test_local_minimal_profile_fanouts_realtime_message_events_to_other_conversation_member`
  - `test_local_minimal_profile_fanouts_member_governance_realtime_events_to_registered_owner_device`
  - `test_local_minimal_profile_fanouts_agent_handoff_lifecycle_realtime_events_to_other_device`
- 回归证据
  - `$env:CARGO_TARGET_DIR='target-cp054j-reg-projection-full'; cargo test -p projection-service --offline`
  - `rustfmt --edition 2024 --check services/projection-service/src/access.rs services/projection-service/src/lib.rs services/projection-service/tests/lib_structure_test.rs services/projection-service/tests/timeline_projection_test.rs services/sdkwork-im-cloud-gateway/src/node/effects.rs services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`

## 3. 剩余问题

- 该增量只解决 `CP05-4` 中一个 projection / notification 连接点，不能据此结束 `CP05-4`。
- `projection / sync` 与 notification 的剩余连接点、multi-client-route sync final closure 仍待继续推进。
- `Step 05 / 91 / 95 / 97 / Wave B / 93` 仍未闭环。
