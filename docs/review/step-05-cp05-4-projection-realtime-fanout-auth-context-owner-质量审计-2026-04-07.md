# Step 05 / CP05-4 projection realtime fanout auth-context owner 质量审计 - 2026-04-07

## 1. 审计结论

- 本轮增量通过。
- realtime publish 公共 helper 已不再在 `sdkwork-im-server` edge 侧手工抓取 `tenant_id` 再消费 raw projection seam。
- 当前实现没有把局部收口误报成 `CP05-4` 或 `Step 05` 完成。

## 2. 审计证据

- 结构证据
  - `test_projection_service_access_module_exposes_auth_context_entrypoints`
  - `test_local_minimal_node_effects_use_projection_owned_realtime_fanout_target_seam`
  - `$env:CARGO_TARGET_DIR='target-cp054i-reg-local-structure-full'; cargo test -p sdkwork-im-cloud-gateway --test lib_structure_test --offline`
- 行为证据
  - `test_local_minimal_profile_fanouts_realtime_message_events_to_other_conversation_member`
  - `test_local_minimal_profile_fanouts_member_governance_realtime_events_to_registered_owner_device`
  - `test_local_minimal_profile_fanouts_agent_handoff_lifecycle_realtime_events_to_other_device`
- 回归证据
  - `$env:CARGO_TARGET_DIR='target-cp054i-reg-projection-full'; cargo test -p projection-service --offline`
  - `rustfmt --edition 2024 --check services/projection-service/src/access.rs services/projection-service/tests/lib_structure_test.rs services/sdkwork-im-cloud-gateway/src/node/effects.rs services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`

## 3. 质量判断

- 通过点
  - realtime principal -> client route target owner seam 现在拥有 auth-context capture，consumer 不再自己拼 tenant scope。
  - message / membership / handoff / stream 四类 realtime caller 都复用同一条 access seam。
  - 全量 `sdkwork-im-server` 结构套件已与仓库真实 owner seam 保持一致。
- 未完成点
  - `CP05-4` 仍有 projection / sync 与 notification 之间的剩余连接点。
  - multi-client-route sync final closure 仍未完成。

## 4. 边界与风险

- 本轮没有关闭 `CP05-4`。
- 本轮没有关闭 `Step 05`。
- `91 / 95 / 97` 只能判定本增量证据完整，不能判定 `Step 05` 整体通过。
- `Wave B / 93` 继续阻塞。
