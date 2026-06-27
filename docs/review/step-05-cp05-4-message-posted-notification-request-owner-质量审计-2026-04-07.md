# Step 05 / CP05-4 message-posted notification request owner 质量审计 - 2026-04-07

## 1. 审计结论

- 本轮增量通过。
- `message.posted` notification 的默认字段 owner 已从 `sdkwork-im-server` 收回 `notification-service`。
- 当前实现没有把局部收口误报成 `CP05-4` 或 `Step 05` 完成。

## 2. 审计证据

- 结构证据
  - `test_notification_runtime_exposes_message_posted_notification_owner_seam`
  - `test_local_minimal_node_effects_use_notification_service_message_posted_owner_seam`
- 行为证据
  - `test_request_message_posted_notifications_own_message_notification_defaults`
  - `test_local_minimal_profile_fanouts_message_notifications_to_other_active_members_only`
- 回归证据
  - `$env:CARGO_TARGET_DIR='target-cp054f-reg-notification'; cargo test -p notification-service --offline`
  - `rustfmt --edition 2024 --check services/notification-service/src/lib.rs services/notification-service/tests/lib_structure_test.rs services/notification-service/tests/notification_pipeline_test.rs services/sdkwork-im-cloud-gateway/src/node/effects.rs services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`

## 3. 质量判断

- 通过点
  - 新 seam 让 `message.posted` notification 默认字段只在 `notification-service` 里维护一份。
  - `sdkwork-im-server` 退化为提供会话成员 recipient 集合与 message/conversation 元数据的 consumer。
  - 现有 message notification e2e 行为保持通过。
- 未完成点
  - `CP05-4` 仍有 projection / sync 与 notification 之间的剩余连接点。
  - multi-client-route sync final closure 仍未完成。

## 4. 边界与风险

- 本轮没有关闭 `CP05-4`。
- 本轮没有关闭 `Step 05`。
- `91 / 95 / 97` 仅能判定本增量证据完整，不能判定 `Step 05` 整体通过。
- `Wave B / 93` 继续阻塞。