# Step 05 CP05-4 projection realtime fanout target owner 执行补充

## 1. 当前定位

- 波次：`Wave B`
- Step：`Step 05`
- 子项：`CP05-4`
- 本轮目标：把 realtime principal -> client route fanout target 的 owner，从 `sdkwork-im-server` 在 side-effect 路径里直接遍历 projection raw `registered_devices(...)`，收口为 `projection-service` 的统一 seam

## 2. 本轮为什么做这一项

- `CP05-4` 的第一个真实增量已经把 notification public request access owner 收口到 `notification-service::NotificationRuntime`
- 当前仓库里，`services/sdkwork-im-cloud-gateway/src/node/effects.rs` 仍然直接调用 `projection_service.registered_devices(...)`，再自行拼装 principal -> client route fanout target
- 这说明 projection / multi-client-route fanout owner 仍停留在 service edge 侧，无法继续推进 `CP05-4` 所要求的 projection / notification / multi-client-route sync 最终收口

## 3. 本轮实际完成

- `services/projection-service/src/model.rs`
  - 新增 `RealtimeFanoutTarget`
- `services/projection-service/src/lib.rs`
  - 新增 `TimelineProjectionService::realtime_fanout_targets_for_principals(...)`
  - 由 projection owner 统一负责：
    - principal 集合到 registered client route 集合的解析
    - principal -> device realtime target 的稳定排序输出
- `services/sdkwork-im-cloud-gateway/src/node/effects.rs`
  - `publish_realtime_event_to_principals(...)` 改为直接消费 `projection_service.realtime_fanout_targets_for_principals(...)`
  - 删除本地基于 raw `registered_devices(...)` 的 client route target 拼装
- 测试新增/更新
  - `services/projection-service/tests/lib_structure_test.rs`
  - `services/projection-service/tests/timeline_projection_test.rs`
  - `services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`

## 4. 测试与验证证据

### 4.1 TDD Red

- `cargo test -p projection-service --test lib_structure_test test_projection_service_exposes_realtime_fanout_target_owner_seam --offline`
- `cargo test -p projection-service --test timeline_projection_test test_realtime_fanout_targets_for_principals_return_registered_principal_device_pairs --offline`
- `$env:CARGO_TARGET_DIR='target-cp054b-red-local'; cargo test -p sdkwork-im-cloud-gateway --test lib_structure_test test_local_minimal_node_effects_use_projection_owned_realtime_fanout_target_seam --offline`

### 4.2 Green / 结构与行为验证

- `cargo test -p projection-service --test lib_structure_test test_projection_service_exposes_realtime_fanout_target_owner_seam --offline`
- `cargo test -p projection-service --test timeline_projection_test test_realtime_fanout_targets_for_principals_return_registered_principal_device_pairs --offline`
- `$env:CARGO_TARGET_DIR='target-cp054b-green-local'; cargo test -p sdkwork-im-cloud-gateway --test lib_structure_test test_local_minimal_node_effects_use_projection_owned_realtime_fanout_target_seam --offline`

### 4.3 Green / 回归验证

- `cargo test -p projection-service --test timeline_projection_test test_registered_devices_and_latest_sync_seq_are_queryable --offline`
- `$env:CARGO_TARGET_DIR='target-cp054b-green-local'; cargo test -p sdkwork-im-cloud-gateway --test cluster_realtime_routing_e2e_test test_local_minimal_profile_routes_realtime_events_to_remote_owner_node --offline`
- `cargo fmt --all`
- `cargo fmt --all --check`

## 5. 当前判断

- 这是 `CP05-4` 的第二个真实增量
- projection-side principal -> device realtime fanout target owner 已开始收口到 `projection-service`
- 但 `CP05-4` 仍未闭环：
  - notification / projection / multi-client-route sync 的剩余 owner seam 仍未全部清零
  - `Step 05` 仍不能整体判定通过
- 因此：
  - `CP05-4` 仍未通过
  - `Step 05` 仍未闭环
  - `91 / 95 / 97` 仍不能整体判定 `Step 05` 通过
  - `Wave B / 93` 仍阻塞
