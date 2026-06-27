> Migrated from `docs/review/step-09-cp09-3-projection-live-lag-observability-执行卡-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 09 / CP09-3 projection live lag observability 执行卡 - 2026-04-08

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
  - 第四段增量已把 live `inbox / timeline update delay` 带到 `ops health / diagnostics`
  - 但 `docs/架构/140-可观测性与SLO治理设计-2026-04-06.md` 中 `Projection Plane` 的核心缺口仍保留：
    - live `projection lag`
    - `rebuild duration`

## 本轮为什么继续做这个子任务
- 相比 `rebuild duration`，live `projection lag` 更适合继续沿用现有 `projection-service.apply(...)` 主路径落地：
  - 不需要新增 replay / rebuild 状态机
  - 不需要引入新的 telemetry owner
  - 可以直接复用真实 apply 边界上的：
    - 最新已观察到的 ordering seq
    - 当前已投影完成的 seq
- 同时，现有公开读面里：
  - `ops/replay-status` 已经专注于 startup replay drill
  - `ops/lag` 已经是承载 plane lag 的现成入口
- 因此本轮最优决策，是把 live lag 放到 projection owner + `ops/lag`，而不是把更多语义堆进 `projectionPlane.replay` 或 `ops/replay-status`。

## 本轮实际完成

### 1. `projection-service` 新增 live lag owner seam
- `services/projection-service/src/observability.rs`
  - 新增 `ProjectionLagItemView`
  - `ProjectionObservabilityState` 新增 live lag state
  - `TimelineProjectionService` 新增：
    - `projection_live_lag_items()`
    - `record_projection_live_lag_observed(...)`
    - `record_projection_live_lag_committed(...)`
- 这份 owner state 明确暴露：
  - `component`
  - `scopeId`
  - `currentOffset`
  - `committedOffset`
  - `lag`

### 2. 真实 projection apply 主路径现在会更新 live lag
- `services/projection-service/src/lib.rs`
  - `apply(...)` 现在会在 conversation-scoped projection 事件进入真实 apply 前记录：
    - 当前已观察到的 ordering seq
    - 之前已提交的投影 offset
  - apply 成功后会把该 scope 收敛为：
    - `currentOffset == committedOffset`
    - `lag == 0`
- 这意味着：
  - 正常 steady-state 下，可以看到 projection 已追平
  - 若 apply 失败，lag owner 会保留未追平状态，而不是继续伪装成零 lag

### 3. `ops-service` 继续保持 replay / live lag 分工
- `services/ops-service/src/lib.rs`
  - 继续保留 `update_projection_replay_lag(...)`
  - 新增 `update_projection_live_lag(...)`
- 这保证：
  - startup replay 证据仍由 `projection_replay` 组件承载
  - steady-state live lag 由 `projection_live` 组件承载
  - `ops/replay-status` 继续只回答 replay drill，不被 live lag 语义污染

### 4. `sdkwork-im-server` 已把 live lag 映射到真实 ops 面
- `services/sdkwork-im-cloud-gateway/src/node/platform.rs`
  - `refresh_node_operational_view(...)` 现在会把 `projection-service` owner 的 live lag 映射进 `OpsRuntime`
- 这意味着 `Local Minimal` profile 下，业务请求触发的真实 projection apply 已可直接在：
  - `/backend/v3/api/ops/lag`
  - `/backend/v3/api/ops/diagnostics`
 看到 `projection_live` lag item

### 5. 全量回归再次触发 Step 02 结构红线，并已修复
- 第一轮全量 `cargo test -p projection-service --offline` 发现：
  - `services/projection-service/src/lib.rs must stay below 1000 lines for Step 02, found 1009`
- 根因明确：
  - live lag 的 scope/helper 逻辑仍残留在 `lib.rs`
- 修复方式：
  - 新增 `services/projection-service/src/scope.rs`
  - 把：
    - scope key helper
    - live lag tracked scope helper
    抽出到独立模块
  - 最终 `services/projection-service/src/lib.rs` 回落到 `931` 行
- 修复后，`projection-service` 全量结构测试重新通过

## 改动范围
- 代码：
  - `services/projection-service/src/observability.rs`
  - `services/projection-service/src/lib.rs`
  - `services/projection-service/src/scope.rs`
  - `services/ops-service/src/lib.rs`
  - `services/sdkwork-im-cloud-gateway/src/node/platform.rs`
- 测试：
  - `services/projection-service/tests/projection_snapshot_test.rs`
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
  - `cargo test -p projection-service --offline test_projection_service_tracks_live_projection_lag_per_scope -- --exact`
  - `cargo test -p sdkwork-im-cloud-gateway --offline test_default_local_minimal_profile_surfaces_projection_plane_observability_over_ops_health_and_diagnostics -- --exact`
- 红测失败点与预期一致：
  - `TimelineProjectionService` 还没有 `projection_live_lag_items()` owner seam
  - `ops/lag` 还拿不到 `projection_live` item

### Green
- 上述两条定向测试现已全部通过

## 回归验证
- `cargo fmt --all --check`
- `cargo test -p projection-service --offline`
- `cargo test -p ops-service --offline`
- `cargo test -p sdkwork-im-cloud-gateway --offline`

## 结论
- 这是 `Wave C / Step 09 / CP09-3` 的第五个真实代码增量。
- `Projection Plane` 现在除了 startup replay lag，还能通过 steady-state apply 主路径对外说明：
  - 哪个 scope 已追平
  - 当前 offset 与已投影 offset 分别是多少
  - 当前 live lag 是否为零
- 同时，本轮也再次把 `projection-service` 的结构红线修回 Step 02 允许范围内。
- 但 `CP09-3` 仍不应整体判定通过，因为当前仍缺：
  - `rebuild duration`
  - 更完整的 SLO / 告警 / 外部 telemetry sink

## 下一轮继续做什么
1. 继续留在 `CP09-3`，优先补 `rebuild duration`，避免 `Projection Plane` 核心指标长期只剩最后一个硬缺口。
2. 若 `rebuild duration` 找不到清晰 owner seam，再评估是否继续补更完整的 projection failure / alert evidence。
3. 只有在 `CP09-3` 再闭一段后，才进入 `CP09-4` 的 backup / restore / repair / archive 收口。

