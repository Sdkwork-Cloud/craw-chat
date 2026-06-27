> Migrated from `docs/review/step-05-cp05-4-automation-result-notification-owner-执行补充-2026-04-07.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 05 CP05-4 automation result notification owner 执行补充

## 1. 当前定位

- 波次：`Wave B`
- Step：`Step 05`
- 子项：`CP05-4`
- 本轮目标：把 automation 完成后的 `automation.result` 通知组装 owner，从 `sdkwork-im-server` platform 收口到 `notification-service::NotificationRuntime`

## 2. 本轮为什么做这一项

- `CP05-4` 的前三个真实增量已经分别处理了：
  - notification public request access owner
  - projection realtime principal -> client route fanout target owner
  - message notification side-effect fanout owner
- 当前仓库里，`services/sdkwork-im-cloud-gateway/src/node/platform.rs` 在 `request_automation_execution(...)` 成功后仍然自己拼装：
  - `ntf_automation_{execution_id}` notification id
  - `evt_{execution_id}_automation_execution_completed` source event id
  - `automation.execution_completed` / `automation.result` 固定元数据
  - `recipient_id = auth.actor_id`
- 这说明 automation result notification 的 owner 仍停留在 service edge，notification owner 还没有在这条链路上闭合。

## 3. 本轮实际完成

- `services/notification-service/src/lib.rs`
  - 新增 `RequestAutomationResultNotification`
  - 新增 `NotificationRuntime::request_automation_result_notification(...)`
  - 由 notification owner 统一负责：
    - automation result notification id 规则
    - source event id / type 规则
    - `automation.result` 固定 category / channel / title
    - 收件人绑定到当前 `auth.actor_id`
- `services/sdkwork-im-cloud-gateway/src/node/platform.rs`
  - `request_automation_execution(...)` 改为直接消费 `notification_runtime.request_automation_result_notification(...)`
  - 删除本地 automation result notification 字段拼装
- 测试新增 / 更新
  - `services/notification-service/tests/lib_structure_test.rs`
  - `services/notification-service/tests/notification_pipeline_test.rs`
  - `services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`

## 4. 测试与验证证据

### 4.1 TDD Red

- `cargo test -p notification-service --test lib_structure_test test_notification_runtime_exposes_automation_result_notification_owner_seam --offline`
- `cargo test -p notification-service --test notification_pipeline_test test_request_automation_result_notification_targets_requesting_actor_idempotently --offline`
- `$env:CARGO_TARGET_DIR='target-cp054d-red-local'; cargo test -p sdkwork-im-cloud-gateway --test lib_structure_test test_local_minimal_node_automation_path_uses_notification_runtime_automation_result_owner --offline`

### 4.2 Green / 结构与行为验证

- `cargo test -p notification-service --test lib_structure_test test_notification_runtime_exposes_automation_result_notification_owner_seam --offline`
- `cargo test -p notification-service --test notification_pipeline_test test_request_automation_result_notification_targets_requesting_actor_idempotently --offline`
- `$env:CARGO_TARGET_DIR='target-cp054d-green-local'; cargo test -p sdkwork-im-cloud-gateway --test lib_structure_test test_local_minimal_node_automation_path_uses_notification_runtime_automation_result_owner --offline`

### 4.3 Green / 回归验证

- `$env:CARGO_TARGET_DIR='target-cp054d-green-local'; cargo test -p sdkwork-im-cloud-gateway --test task10_capabilities_e2e_test test_local_minimal_profile_exposes_notification_automation_audit_and_ops_capabilities --offline`
- `$env:CARGO_TARGET_DIR='target-cp054d-green-local'; cargo test -p sdkwork-im-cloud-gateway --test task10_capabilities_e2e_test test_local_minimal_profile_treats_duplicate_automation_request_as_idempotent --offline`
- `cargo fmt --all`
- `cargo fmt --all --check`

## 5. 当前判断

- 这是 `CP05-4` 的第四个真实增量
- automation result notification orchestration 已开始收口到 `notification-service`
- 但 `CP05-4` 仍未闭环：
  - projection / notification / multi-client-route sync 仍有其他 owner seam 未清零
  - `Step 05` 仍不能整体判定通过
- 因此：
  - `CP05-4` 仍未通过
  - `Step 05` 仍未闭环
  - `91 / 95 / 97` 仍不能整体判定 `Step 05` 通过
  - `Wave B / 93` 仍阻塞

