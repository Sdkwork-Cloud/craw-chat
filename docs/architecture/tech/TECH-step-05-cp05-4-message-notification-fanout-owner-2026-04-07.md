> Migrated from `docs/review/step-05-cp05-4-message-notification-fanout-owner-执行补充-2026-04-07.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 05 CP05-4 message notification fanout owner 执行补充

## 1. 当前定位

- 波次：`Wave B`
- Step：`Step 05`
- 子项：`CP05-4`
- 本轮目标：把 message notification side-effect 里的 per-recipient fanout orchestration，从 `sdkwork-im-server` effects 收口到 `notification-service::NotificationRuntime`

## 2. 本轮为什么做这一项

- `CP05-4` 的前两个真实增量已经分别处理了：
  - notification public request access owner
  - projection realtime principal -> client route fanout target owner
- 当前仓库里，`services/sdkwork-im-cloud-gateway/src/node/effects.rs` 仍自己做 message notification side-effect fanout：
  - 自己过滤 sender
  - 自己循环 recipient
  - 自己拼 `ntf_{message_id}_{recipient_id}` notification id
  - 自己逐条调用 `notification_runtime.request_notification(...)`
- 这说明 notification side-effect orchestration 仍停留在 service edge，无法继续推进 `CP05-4`

## 3. 本轮实际完成

- `services/notification-service/src/lib.rs`
  - 新增 `RequestNotificationFanout`
  - 新增 `NotificationRuntime::request_notification_fanout(...)`
  - 由 notification owner 统一负责：
    - 跳过当前 actor 自己
    - 按 recipient fanout notification request
    - 统一生成 `ntf_{notification_id_seed}_{recipient_id}` 规则
- `services/sdkwork-im-cloud-gateway/src/node/effects.rs`
  - `fanout_message_notifications(...)` 改为直接调用 `notification_runtime.request_notification_fanout(...)`
  - 删除本地 per-recipient loop 与 notification id 拼装
- 测试新增/更新
  - `services/notification-service/tests/lib_structure_test.rs`
  - `services/notification-service/tests/notification_pipeline_test.rs`
  - `services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`

## 4. 测试与验证证据

### 4.1 TDD Red

- `cargo test -p notification-service --test lib_structure_test test_notification_runtime_exposes_notification_fanout_owner_seam --offline`
- `cargo test -p notification-service --test notification_pipeline_test test_request_notification_fanout_skips_actor_and_creates_notifications_for_other_recipients --offline`
- `$env:CARGO_TARGET_DIR='target-cp054c-red-local'; cargo test -p sdkwork-im-cloud-gateway --test lib_structure_test test_local_minimal_node_effects_use_notification_service_fanout_owner_seam --offline`

### 4.2 Green / 结构与行为验证

- `cargo test -p notification-service --test lib_structure_test test_notification_runtime_exposes_notification_fanout_owner_seam --offline`
- `cargo test -p notification-service --test notification_pipeline_test test_request_notification_fanout_skips_actor_and_creates_notifications_for_other_recipients --offline`
- `$env:CARGO_TARGET_DIR='target-cp054c-green-local'; cargo test -p sdkwork-im-cloud-gateway --test lib_structure_test test_local_minimal_node_effects_use_notification_service_fanout_owner_seam --offline`

### 4.3 Green / 回归验证

- `$env:CARGO_TARGET_DIR='target-cp054c-green-local'; cargo test -p sdkwork-im-cloud-gateway --test http_e2e_test test_local_minimal_profile_fanouts_message_notifications_to_other_active_members_only --offline`
- `cargo test -p notification-service --test notification_pipeline_test test_duplicate_request_notification_is_idempotent_when_payload_matches --offline`
- `cargo fmt --all`
- `cargo fmt --all --check`

## 5. 当前判断

- 这是 `CP05-4` 的第三个真实增量
- notification side-effect fanout orchestration 已开始收口到 `notification-service`
- 但 `CP05-4` 仍未闭环：
  - notification / projection / multi-client-route sync 的剩余 seam 仍未全部清零
  - `Step 05` 仍不能整体判定通过
- 因此：
  - `CP05-4` 仍未通过
  - `Step 05` 仍未闭环
  - `91 / 95 / 97` 仍不能整体判定 `Step 05` 通过
  - `Wave B / 93` 仍阻塞

