# Step 05 CP05-4 notification public access owner 执行补充

## 1. 当前定位

- 波次：`Wave B`
- Step：`Step 05`
- 子项：`CP05-4`
- 本轮目标：把 notification public request 路径上的 cross-recipient 访问控制 owner，从 `notification-service` 与 `sdkwork-im-server` 两边各自维护，收口为 `notification-service::NotificationRuntime` 的统一 public-api seam

## 2. 本轮为什么做这一项

- `CP05-3` 已通过后，`Step 05` 的下一个未闭环子项是 `CP05-4`
- 当前仓库在以下两个 service edge 上重复维护相同通知访问控制：
  - `services/notification-service/src/lib.rs`
  - `services/sdkwork-im-cloud-gateway/src/node/platform.rs`
- 这种 duplicated permission seam 会让 notification owner 漂移继续留在 service entrypoint，无法把 projection / notification / multi-client-route sync 最终 owner 收口推进到 `CP05-4`

## 3. 本轮实际完成

- `services/notification-service/src/lib.rs`
  - 新增 `NotificationRuntime::request_notification_from_public_api(...)`
  - 由 runtime owner 统一负责：
    - bearer public request 的 cross-recipient permission 校验
    - 继续委托现有 `request_notification_with_outcome(...)`
  - HTTP `request_notification(...)` 路径改为消费该 owner seam
- `services/sdkwork-im-cloud-gateway/src/node/platform.rs`
  - app-facing `/im/v3/api/notifications/requests` 路径改为直接消费 `notification_runtime.request_notification_from_public_api(...)`
  - 删除本地重复的 notification request access 判定
- `services/sdkwork-im-cloud-gateway/src/node/access.rs`
  - 删除已不再需要的 `ensure_notification_request_access(...)`
- 测试新增
  - `services/notification-service/tests/lib_structure_test.rs`
  - `services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`

## 4. 测试与验证证据

### 4.1 TDD Red

- `cargo test -p notification-service --test lib_structure_test test_notification_runtime_exposes_public_request_access_owner --offline`
- `$env:CARGO_TARGET_DIR='target-cp054-red-local'; cargo test -p sdkwork-im-cloud-gateway --test lib_structure_test test_local_minimal_node_notification_request_path_uses_notification_runtime_public_access_owner --offline`

### 4.2 Green / 结构验证

- `cargo test -p notification-service --test lib_structure_test test_notification_runtime_exposes_public_request_access_owner --offline`
- `$env:CARGO_TARGET_DIR='target-cp054-green-local'; cargo test -p sdkwork-im-cloud-gateway --test lib_structure_test test_local_minimal_node_notification_request_path_uses_notification_runtime_public_access_owner --offline`

### 4.3 Green / 行为验证

- `cargo test -p notification-service --test public_auth_test test_public_app_rejects_cross_recipient_notification_request_without_permission --offline`
- `cargo test -p notification-service --test public_auth_test test_public_app_accepts_self_notification_request --offline`
- `$env:CARGO_TARGET_DIR='target-cp054-green-local'; cargo test -p sdkwork-im-cloud-gateway --test public_auth_e2e_test test_public_app_rejects_cross_recipient_notification_request_without_permission --offline`
- `$env:CARGO_TARGET_DIR='target-cp054-green-local'; cargo test -p sdkwork-im-cloud-gateway --test public_auth_e2e_test test_public_app_accepts_self_notification_request --offline`
- `cargo fmt --all --check`

## 5. 当前判断

- 这是 `CP05-4` 的第一个真实增量
- notification public access owner 漂移已开始收口到 `notification-service`
- 但 `CP05-4` 仍未闭环：
  - projection / multi-client-route sync 最终 owner 收口仍未完成
  - notification side-effect 与 fanout 仍有后续收口空间
- 因此：
  - `CP05-4` 仍未通过
  - `Step 05` 仍未闭环
  - `91 / 95 / 97` 仍不能整体判定 `Step 05` 通过
  - `Wave B / 93` 仍阻塞
