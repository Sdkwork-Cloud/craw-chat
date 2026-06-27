> Migrated from `docs/review/step-05-cp05-4-projection-realtime-fanout-auth-context-owner-执行补充-2026-04-07.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 05 / CP05-4 projection realtime fanout auth-context owner 执行补充 - 2026-04-07

## 1. 当前定位

- 波次：`Wave B`
- Step：`Step 05`
- 当前子项：`CP05-4`
- 本轮不允许把 `CP05-4`、`Step 05`、`91 / 95 / 97`、`Wave B / 93` 写成完成。

## 2. 本轮为什么做这个增量

前面已经把 realtime principal -> client route fanout target 的 owner 收口到 `projection-service`，但 `services/sdkwork-im-cloud-gateway/src/node/effects.rs` 里公共 helper `publish_realtime_event_to_principals(...)` 仍然在 edge 侧自己抓取 `auth.tenant_id`，再直接调用 `realtime_fanout_targets_for_principals(tenant_id, principal_ids)`。

这说明 `CP05-4` 里 projection realtime fanout seam 还差最后一层 auth-context capture owner：message / membership / handoff / stream 四类 realtime side-effect 虽然已经不再本地重建 client route target，但仍保留了一层 duplicated projection access glue。为避免把这层 edge glue 误当成已经完全 owner 化，本轮继续停留在 `Wave B / Step 05 / CP05-4`，只收这条 seam。

## 3. 本轮实际完成

- `projection-service` access 新增 auth-context realtime fanout seam：
  - `services/projection-service/src/access.rs`
  - `realtime_fanout_targets_from_auth_context(...)`
- `sdkwork-im-server` realtime side-effect 公共 helper 改为统一消费这条 seam：
  - `services/sdkwork-im-cloud-gateway/src/node/effects.rs`
  - `publish_realtime_event_to_principals(...)`
- 下列 caller 现在都经由 projection-owned auth-context seam 完成 tenant capture：
  - `publish_realtime_conversation_message_event(...)`
  - `publish_realtime_membership_event(...)`
  - `publish_realtime_agent_handoff_status_changed_event(...)`
  - `publish_realtime_stream_frame_event(...)`
  - `publish_realtime_stream_lifecycle_event(...)`
- 同步修正 `services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs` 中既有 fanout owner 断言，使全量结构套件与仓库已落地的 `request_message_posted_notifications(...)` seam 保持一致，不再把旧的 generic fanout 断言误判为唯一合法实现。

## 4. 改动文件

- `services/projection-service/src/access.rs`
- `services/projection-service/tests/lib_structure_test.rs`
- `services/sdkwork-im-cloud-gateway/src/node/effects.rs`
- `services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`

## 5. 验证

### 5.1 Red

- `$env:CARGO_TARGET_DIR='target-cp054i-red-projection-structure'; cargo test -p projection-service --test lib_structure_test test_projection_service_access_module_exposes_auth_context_entrypoints --offline`
- `$env:CARGO_TARGET_DIR='target-cp054i-red-local-structure'; cargo test -p sdkwork-im-cloud-gateway --test lib_structure_test test_local_minimal_node_effects_use_projection_owned_realtime_fanout_target_seam --offline`

### 5.2 Green

- `$env:CARGO_TARGET_DIR='target-cp054i-green-projection-structure'; cargo test -p projection-service --test lib_structure_test test_projection_service_access_module_exposes_auth_context_entrypoints --offline`
- `$env:CARGO_TARGET_DIR='target-cp054i-green-local-structure'; cargo test -p sdkwork-im-cloud-gateway --test lib_structure_test test_local_minimal_node_effects_use_projection_owned_realtime_fanout_target_seam --offline`
- `$env:CARGO_TARGET_DIR='target-cp054i-reg-realtime-message'; cargo test -p sdkwork-im-cloud-gateway --test http_e2e_test test_local_minimal_profile_fanouts_realtime_message_events_to_other_conversation_member --offline`
- `$env:CARGO_TARGET_DIR='target-cp054i-reg-realtime-membership'; cargo test -p sdkwork-im-cloud-gateway --test http_e2e_test test_local_minimal_profile_fanouts_member_governance_realtime_events_to_registered_owner_device --offline`
- `$env:CARGO_TARGET_DIR='target-cp054i-reg-realtime-handoff'; cargo test -p sdkwork-im-cloud-gateway --test http_e2e_test test_local_minimal_profile_fanouts_agent_handoff_lifecycle_realtime_events_to_other_device --offline`

### 5.3 Regression

- `rustfmt --edition 2024 services/projection-service/src/access.rs services/projection-service/tests/lib_structure_test.rs services/sdkwork-im-cloud-gateway/src/node/effects.rs services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`
- `rustfmt --edition 2024 --check services/projection-service/src/access.rs services/projection-service/tests/lib_structure_test.rs services/sdkwork-im-cloud-gateway/src/node/effects.rs services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`
- `$env:CARGO_TARGET_DIR='target-cp054i-reg-projection-full'; cargo test -p projection-service --offline`
- `$env:CARGO_TARGET_DIR='target-cp054i-reg-local-structure-full'; cargo test -p sdkwork-im-cloud-gateway --test lib_structure_test --offline`
- `rg -n "realtime_fanout_targets_for_principals\\(|realtime_fanout_targets_from_auth_context\\(|publish_realtime_event_to_principals\\(" services/sdkwork-im-cloud-gateway/src/node/effects.rs services/projection-service/src/access.rs -S`

## 6. 当前结论

- 本轮是 `CP05-4` 的一个有效 projection realtime seam 增量。
- `projection-service::access` 现在开始统一拥有 realtime fanout target 的 auth-context capture。
- `sdkwork-im-server/effects.rs` 不再在公共 realtime publish helper 里手工透传 `tenant_id` 去消费 raw projection fanout target seam。
- 但 `CP05-4` 仍未闭环，`Step 05` 仍未闭环，`91 / 95 / 97` 仍不能整体判定通过，`Wave B / 93` 仍阻塞。

