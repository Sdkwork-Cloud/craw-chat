# Step 09 / CP09-3 projection rebuild duration observability 执行卡 - 2026-04-08

## 当前上下文
- 当前波次：`Wave C`
- 当前 step：`Step 09`
- 当前子任务：`CP09-3`
- 前置状态：
  - `CP09-3` 之前已经连续补齐：
    - snapshot persist / restore metrics、traces、logs
    - replay backlog / replay lag / replay duration
    - `ops/replay-status` 与 `replayThroughputPerSecond`
    - live `inbox / timeline update delay`
    - live `projection lag`
  - 到 live lag 落地后，`docs/架构/140-可观测性与SLO治理设计-2026-04-06.md` 中 `Projection Plane` 剩余唯一硬缺口，就是 `rebuild duration`

## 本轮为什么继续做这个子任务
- 相比继续扩张 SLO / 告警阈值，`rebuild duration` 是 `CP09-3` 最后一个必须落地的核心指标。
- 该指标如果只复用 replay duration，会把语义做错：
  - snapshot-only recovery 明明发生了真实 rebuild
  - 但 replay event count 可能为 `0`
  - 若只看 `replay.durationMs`，会把“已完成 rebuild”误判成“没有恢复动作”
- 因此本轮最优决策是：
  - `projection-service` 继续拥有 `rebuild_duration_ms`
  - `sdkwork-im-server` 只在 startup recovery 主路径上测量真实总时长
  - `ops-service` 只暴露 schema/runtime 视图
- 语义明确为：
  - `replay.durationMs`
    - 只回答 replay 花了多久
  - `rebuildDurationMs`
    - 回答整次 projection recovery 花了多久
    - 包含 snapshot restore + replay recovery

## 本轮实际完成

### 1. `projection-service` 新增 rebuild duration owner seam
- `services/projection-service/src/observability.rs`
  - `ProjectionPlaneObservabilityView` 新增 `rebuild_duration_ms`
  - `ProjectionObservabilityState` 新增同名 owner state
  - `TimelineProjectionService` 新增：
    - `record_projection_rebuild_duration(...)`
- 这意味着 `projection-service` 的 observability owner 现在统一拥有：
  - snapshot persist / restore 证据
  - replay 指标
  - live lag
  - update delay
  - rebuild duration

### 2. startup recovery 现在会记录真实 projection rebuild 总时长
- `services/sdkwork-im-cloud-gateway/src/node/build.rs`
  - `ProjectionReplaySummary` 新增 `rebuild_duration_ms`
  - startup recovery 会在真实 rebuild 发生时记录总耗时
  - 语义为：
    - 若本次 recovery 发生过 snapshot restore 或 replay rebuild，则 `rebuild_duration_ms >= 1`
    - 若根本没有 rebuild 动作，则 `rebuild_duration_ms == 0`
- 这保证：
  - snapshot-only recovery 不会被错误压缩成 `0`
  - replay duration 仍可保持 `0`
  - 两个指标各自回答不同问题

### 3. `ops-service` 的 health / diagnostics 现在公开 `projectionPlane.rebuildDurationMs`
- `services/ops-service/src/lib.rs`
  - `ProjectionPlaneHealthView` 新增 `rebuild_duration_ms`
  - `ProjectionPlaneDiagnosticsView` 新增 `rebuild_duration_ms`
- 默认 idle 视图下：
  - `projectionPlane.rebuildDurationMs == 0`
- recovery 后：
  - `projectionPlane.rebuildDurationMs` 会跟随 projection owner 的真实值对外暴露

### 4. `sdkwork-im-server` 已把 rebuild duration 映射到真实 HTTP 面
- `services/sdkwork-im-cloud-gateway/src/node/platform.rs`
  - `refresh_node_operational_view(...)` 现在会把 `projection-service` owner 的 `rebuild_duration_ms` 写入 `OpsRuntime`
- 这意味着 `Local Minimal` profile 下，启动恢复后的真实 rebuild duration 已可直接在：
  - `/backend/v3/api/ops/health`
  - `/backend/v3/api/ops/diagnostics`
 读取

### 5. snapshot-only recovery 现在具备正确的双指标语义
- `services/sdkwork-im-cloud-gateway/tests/domain_recovery_persistence_test.rs`
  - snapshot-only recovery 场景现在证明：
    - `projectionPlane.replay.durationMs == 0`
    - `projectionPlane.rebuildDurationMs >= 1`
- 这把原先“replay 没发生，所以恢复看起来像没做事”的误解彻底消掉了

## 改动范围
- 代码：
  - `services/projection-service/src/observability.rs`
  - `services/ops-service/src/lib.rs`
  - `services/sdkwork-im-cloud-gateway/src/node.rs`
  - `services/sdkwork-im-cloud-gateway/src/node/build.rs`
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
- 先补测试，再验证缺口：
  - `cargo test -p projection-service --offline test_projection_service_records_projection_rebuild_duration -- --exact`
  - `cargo test -p ops-service --offline test_cluster_lag_health_runtime_dir_and_diagnostics_over_http -- --exact`
  - `cargo test -p sdkwork-im-cloud-gateway --offline test_default_local_minimal_profile_surfaces_projection_plane_observability_over_ops_health_and_diagnostics -- --exact`
- 红测失败点与预期一致：
  - `projection-service` 还没有 `rebuild_duration_ms` owner state
  - `ops-service` 的 `projectionPlane` 视图还没有 `rebuildDurationMs`
  - snapshot-only recovery 还无法给出正向 rebuild duration 证据

### Green
- 上述定向测试现已全部通过

## 回归验证
- `cargo fmt --all --check`
- `cargo test -p projection-service --offline`
- `cargo test -p ops-service --offline`
- `cargo test -p sdkwork-im-cloud-gateway --offline`

## 结论
- 这是 `Wave C / Step 09 / CP09-3` 的第六个真实代码增量。
- `Projection Plane` 现在已完整具备 `docs/架构/140-可观测性与SLO治理设计-2026-04-06.md` 第 `5.5` 节要求的核心指标：
  - `projection lag`
  - `rebuild duration`
  - `backlog size`
  - `inbox/timeline update delay`
  - `replay throughput`
- 因此，`CP09-3` 现在可以按“metrics / tracing / logging 已按 plane 基本收口”判定通过。
- 但 `Step 09` 仍不应整体判定通过，因为：
  - `CP09-4` 的 backup / restore / repair / archive 代码与脚本闭环仍未开始收口
  - 更完整的 SLO / 告警阈值 / 外部 telemetry sink 也不属于本轮 `CP09-3` 的通过门槛

## 下一轮继续做什么
1. 正式切入 `CP09-4`，优先补 archive / retention 的最小真实代码与脚本路径。
2. 继续维持 `Step 09` 阻塞，直到 backup / restore / repair / archive 的代码、脚本、review、架构回写全部齐备。
3. 在 `CP09-4` 闭环前，不允许把 `Step 09` 或 `Wave C / 93` 提前判定为通过。
