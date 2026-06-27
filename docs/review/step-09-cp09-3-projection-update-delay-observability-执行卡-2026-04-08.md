# Step 09 / CP09-3 projection update delay observability 执行卡 - 2026-04-08

## 当前上下文
- 当前波次：`Wave C`
- 当前 step：`Step 09`
- 当前子任务：`CP09-3`
- 前置状态：
  - `CP09-3` 第一段增量已把 `projection snapshot persist / restore` 的 `metrics / tracing / structured logging` 带到 `ops health / diagnostics`
  - 第二段增量已把 startup replay 的：
    - `backlogSize`
    - `replayedEventCount`
    - `durationMs`
    - `projection_replay lag`
    带到公开 `ops` 面
  - 第三段增量已把 replay 证据收口成：
    - `ops/replay-status`
    - `replayThroughputPerSecond`
  - 但 `docs/架构/140-可观测性与SLO治理设计-2026-04-06.md` 中 `Projection Plane` 的核心指标仍明确保留：
    - `rebuild duration`
    - `inbox/timeline update delay`
    - live `projection lag`

## 本轮为什么继续做这个子任务
- 在剩余缺口里，`inbox/timeline update delay` 最适合继续沿用现有 `projection-service.apply(...)` 主路径落地：
  - 不需要新建状态 owner
  - 不需要新增 replay / rebuild 状态机
  - 可以直接复用事件上的 `committedAt / editedAt / recalledAt`
- 相比之下：
  - live `projection lag` 需要额外的持续状态更新链路
  - `rebuild duration` 则需要更明确的 rebuild owner 边界
- 因此本轮最优决策，是先把 `Projection Plane` 的运行态 `update delay` 补成真实指标，而不是提前跳去做更重的运行态 lag 状态机。

## 本轮实际完成

### 1. `projection-service` 新增 live update delay owner
- `services/projection-service/src/observability.rs`
  - 新增 `ProjectionUpdateDelayView`
  - `ProjectionPlaneObservabilityView` 新增 `updateDelay`
  - `TimelineProjectionService` 新增 `record_projection_update_delay(...)`
- 当前 `updateDelay` 明确暴露：
  - `timelineMs`
  - `inboxMs`
  - `sourceEventType`
  - `scopeId`
  - `recordedAt`

### 2. `message.posted / edited / recalled` 现在会记录真实 projection update delay
- `services/projection-service/src/lib.rs`
  - `apply_message_posted(...)`
  - `apply_message_edited(...)`
  - `apply_message_recalled(...)`
    现在都会在真实 read-side apply 完成后记录 `updateDelay`
- delay 的来源不是额外埋点，而是直接基于事件主路径时间戳：
  - `message.posted`：`committedAt` 或 `occurredAt`
  - `message.edited`：`editedAt`
  - `message.recalled`：`recalledAt`

### 3. `ops-service` 的 `health / diagnostics` schema 已补齐 `updateDelay`
- `services/ops-service/src/lib.rs`
  - `ProjectionPlaneHealthView` 新增 `updateDelay`
  - `ProjectionPlaneDiagnosticsView` 新增 `updateDelay`
- 默认空闲态现在也会稳定返回零值 schema，而不是缺字段。

### 4. `sdkwork-im-server` 已把 live update delay 映射到真实 HTTP 面
- `services/sdkwork-im-cloud-gateway/src/node/platform.rs`
  - `map_projection_plane_observability(...)` 现在会把 `projection-service` 的 `updateDelay` 映射到 `ops-service`
- 这意味着 `Local Minimal` profile 下，业务请求触发的真实 projection apply 可以直接在：
  - `/backend/v3/api/ops/health`
  - `/backend/v3/api/ops/diagnostics`
  看到 `timeline / inbox update delay`

### 5. 全量回归发现并修复了 Step 02 结构红线回归
- 全量跑 `cargo test -p projection-service --offline` 时，`lib_structure_test` 首次失败：
  - `services/projection-service/src/lib.rs must stay below 1000 lines for Step 02, found 1092`
- 根因不是功能逻辑错误，而是本轮把时间解析与 delay 计算辅助逻辑直接塞进了 `lib.rs`
- 修复方式：
  - 新增 `services/projection-service/src/update_delay.rs`
  - 仅把 `update delay` 解析/计算辅助逻辑抽出
  - 不改变功能合同
- 修复后，`projection-service` 全量测试重新通过

## 改动范围
- 代码：
  - `services/projection-service/src/observability.rs`
  - `services/projection-service/src/lib.rs`
  - `services/projection-service/src/update_delay.rs`
  - `services/ops-service/src/lib.rs`
  - `services/sdkwork-im-cloud-gateway/src/node/platform.rs`
- 测试：
  - `services/projection-service/tests/projection_snapshot_test.rs`
  - `services/ops-service/tests/http_smoke_test.rs`
  - `services/sdkwork-im-cloud-gateway/tests/domain_recovery_persistence_test.rs`
- 文档：
  - 本执行卡
  - 本轮质量审计与复盘
  - 本轮架构兑现与回写决议
  - `docs/架构/09-实施计划.md`
  - `docs/架构/132-存储架构与自主演进路线设计-2026-04-06.md`
  - `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md`
  - `docs/架构/140-可观测性与SLO治理设计-2026-04-06.md`

## TDD 证据

### Red
- 先写测试，再验证缺口：
  - `cargo test -p projection-service --offline --test projection_snapshot_test test_projection_service_records_projection_update_delay_metrics`
  - `cargo test -p ops-service --offline --test http_smoke_test test_cluster_lag_health_runtime_dir_and_diagnostics_over_http`
  - `cargo test -p sdkwork-im-cloud-gateway --offline --test domain_recovery_persistence_test test_default_local_minimal_profile_surfaces_projection_plane_observability_over_ops_health_and_diagnostics`
- 红测失败点与预期一致：
  - `projection-service` 的 observability JSON 中还没有 `updateDelay`
  - `ops-service` 默认 `projectionPlane.updateDelay.*` 还是 `null`
  - `sdkwork-im-server` 的 `ops health` 还拿不到 live projection update delay

### Green
- 上述三条定向测试现已全部通过

## 回归验证
- `cargo fmt --all`
- `cargo test -p projection-service --offline --test projection_snapshot_test test_projection_service_records_projection_update_delay_metrics`
- `cargo test -p ops-service --offline --test http_smoke_test test_cluster_lag_health_runtime_dir_and_diagnostics_over_http`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test domain_recovery_persistence_test test_default_local_minimal_profile_surfaces_projection_plane_observability_over_ops_health_and_diagnostics`
- `cargo test -p projection-service --offline`
- `cargo test -p ops-service --offline`
- `cargo test -p sdkwork-im-cloud-gateway --offline`

## 结论
- 这是 `Wave C / Step 09 / CP09-3` 的第四个真实代码增量。
- `Projection Plane` 现在除了 replay 证据，还能通过 live apply 主路径对外说明：
  - timeline 更新延迟
  - inbox 更新延迟
  - 这份延迟来自哪个事件类型
  - 作用于哪个 scope
- 同时，本轮也把 `projection-service` 的结构红线回归修回到 Step 02 允许范围内。
- 但 `CP09-3` 仍不应整体判定通过，因为当前仍缺：
  - live `projection lag`
  - `rebuild duration`
  - 更完整的 SLO / 告警 / 外部 telemetry sink

## 下一轮继续做什么
1. 继续留在 `CP09-3`，优先评估 live `projection lag` 是否能在现有主路径上最小落地。
2. 若 live lag 仍过重，则补 `rebuild duration`，避免 `Projection Plane` 核心指标长期缺项。
3. 只有在 `CP09-3` 再闭一段后，才进入 `CP09-4` 的 backup / restore / repair / archive 收口。
