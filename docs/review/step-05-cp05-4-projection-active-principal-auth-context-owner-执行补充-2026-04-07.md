# Step 05 / CP05-4 projection active-principal auth-context owner 执行补充 - 2026-04-07

## 1. 当前上下文

- 波次：`Wave B`
- Step：`Step 05`
- 当前子项：`CP05-4`
- 本轮目标：把 conversation active principal mapping 从 `sdkwork-im-server` side-effect 路径继续收口到 `projection-service::access`，去掉 message notification / realtime fanout 仍在 edge 侧读取 member roster 的剩余连接点。

## 2. 本轮实际落地

- `services/projection-service/src/access.rs`
  - 新增 `active_conversation_principal_ids_from_auth_context(...)`
- `services/projection-service/src/lib.rs`
  - `active_conversation_principal_ids(...)` 调整为 crate 内 owner seam，供 access 模块复用
- `services/sdkwork-im-cloud-gateway/src/node/effects.rs`
  - `conversation_member_principal_ids_from_auth_context(...)` 改为委托 `projection-service` 的 auth-context seam
  - message notification、conversation realtime、membership realtime、handoff realtime、conversation-scoped stream realtime 的 principal 解析不再直接命中 runtime member roster
- `services/projection-service/tests/lib_structure_test.rs`
  - access module 结构断言增加 `active_conversation_principal_ids_from_auth_context(...)`
- `services/projection-service/tests/timeline_projection_test.rs`
  - 新增行为测试，验证 auth-context caller 只能读取当前 active principal 集合
- `services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`
  - effects 结构断言改为锁定 projection-owned active-principal seam，而不是旧的 runtime roster seam

## 3. 本轮验证

- Red
  - `$env:CARGO_TARGET_DIR='target-cp054j-red-projection-structure'; cargo test -p projection-service --test lib_structure_test test_projection_service_access_module_exposes_auth_context_entrypoints --offline`
  - `$env:CARGO_TARGET_DIR='target-cp054j-red-projection-behavior'; cargo test -p projection-service --test timeline_projection_test test_active_conversation_principal_ids_from_auth_context_returns_current_active_members --offline`
  - `$env:CARGO_TARGET_DIR='target-cp054j-red-local-structure'; cargo test -p sdkwork-im-cloud-gateway --test lib_structure_test test_local_minimal_node_effects_member_fanout_uses_projection_auth_context_entrypoints --offline`
- Green / Regression
  - `$env:CARGO_TARGET_DIR='target-cp054j-green-projection-structure'; cargo test -p projection-service --test lib_structure_test test_projection_service_access_module_exposes_auth_context_entrypoints --offline`
  - `$env:CARGO_TARGET_DIR='target-cp054j-green-projection-behavior'; cargo test -p projection-service --test timeline_projection_test test_active_conversation_principal_ids_from_auth_context_returns_current_active_members --offline`
  - `$env:CARGO_TARGET_DIR='target-cp054j-green-local-structure'; cargo test -p sdkwork-im-cloud-gateway --test lib_structure_test --offline`
  - `$env:CARGO_TARGET_DIR='target-cp054j-reg-message-notification'; cargo test -p sdkwork-im-cloud-gateway --test http_e2e_test test_local_minimal_profile_fanouts_message_notifications_to_other_active_members_only --offline`
  - `$env:CARGO_TARGET_DIR='target-cp054j-reg-message-realtime'; cargo test -p sdkwork-im-cloud-gateway --test http_e2e_test test_local_minimal_profile_fanouts_realtime_message_events_to_other_conversation_member --offline`
  - `$env:CARGO_TARGET_DIR='target-cp054j-reg-membership-realtime'; cargo test -p sdkwork-im-cloud-gateway --test http_e2e_test test_local_minimal_profile_fanouts_member_governance_realtime_events_to_registered_owner_device --offline`
  - `$env:CARGO_TARGET_DIR='target-cp054j-reg-handoff-realtime'; cargo test -p sdkwork-im-cloud-gateway --test http_e2e_test test_local_minimal_profile_fanouts_agent_handoff_lifecycle_realtime_events_to_other_device --offline`
  - `$env:CARGO_TARGET_DIR='target-cp054j-reg-projection-full'; cargo test -p projection-service --offline`
  - `rustfmt --edition 2024 --check services/projection-service/src/access.rs services/projection-service/src/lib.rs services/projection-service/tests/lib_structure_test.rs services/projection-service/tests/timeline_projection_test.rs services/sdkwork-im-cloud-gateway/src/node/effects.rs services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`

## 4. 本轮结论

- `projection-service::access` 已开始统一拥有 conversation active principal 的 auth-context capture。
- `sdkwork-im-server` 的 effects side-effect 路径不再直接读取 runtime member roster 解析 recipient principals。
- 本轮是 `CP05-4` 的有效增量，但仍不构成 `CP05-4` 总闭环。
- 当前仍未完成：
  - `CP05-4`
  - `Step 05`
  - `91 / 95 / 97` 针对 `Step 05` 的整体通过
  - `Wave B / 93`
