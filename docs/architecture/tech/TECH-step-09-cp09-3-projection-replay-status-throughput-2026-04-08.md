> Migrated from `docs/review/step-09-cp09-3-projection-replay-status-throughput-执行卡-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 09 / CP09-3 projection replay status throughput observability 执行卡 - 2026-04-08

## 当前上下文
- 当前波次：`Wave C`
- 当前 step：`Step 09`
- 当前子任务：`CP09-3`
- 前置状态：
  - `CP09-3` 第一段增量已把 `projection snapshot persist / restore` 的 `metrics / tracing / structured logging` 带到 `ops health / diagnostics`
  - `CP09-3` 第二段增量已把 startup replay 的：
    - `backlogSize`
    - `replayedEventCount`
    - `durationMs`
    - `projection_replay lag`
    带到公开 `ops` 面
  - 但上一轮 review 仍明确保留两个缺口：
    - 缺少专门的 `ops/replay-status` 读面，调用方仍要自己拼 `health / lag / diagnostics`
    - `replay throughput` 仍未落地

## 本轮为什么继续做这个子任务
- `docs/step/09-存储投影与可观测治理.md` 要求 `CP09-3` 形成按 plane 收口的真实观测面，而不是只有零散字段。
- `docs/架构/140-可观测性与SLO治理设计-2026-04-06.md` 在 diagnostics 接口建议中明确列出了 `ops/replay-status`。
- 上一轮 `CP09-3` 复盘也已经指出，下一拍最优动作应是补：
  - 更聚焦的 replay drill 视图
  - replay throughput
- 因此本轮最优决策仍然不是切到 `CP09-4`，而是继续沿用现有 `snapshot + journal replay + ops runtime` 主路径，把 replay 证据收口成独立可读面。

## 本轮实际完成

### 1. `ops-service` 新增专门的 replay status 读面
- `services/ops-service/src/lib.rs`
  - 新增 `ProjectionReplayStatusView`
  - 新增 `OpsRuntime::replay_status_view()`
  - 新增公开路由 `GET /backend/v3/api/ops/replay_status`
- 该读面的返回结构当前明确包括：
  - `generatedAt`
  - `status`
    - `idle`
    - `replayed`
  - `replay`
    - `backlogSize`
    - `replayedEventCount`
    - `durationMs`
  - `replayThroughputPerSecond`
  - `lag`
    - 仅过滤并返回 `projection_replay` 项

### 2. `replay throughput` 由已有 replay 指标推导，不新增旁路 owner
- `services/ops-service/src/lib.rs`
  - `replayThroughputPerSecond` 直接由现有：
    - `replayedEventCount`
    - `durationMs`
    推导得到
- 这意味着本轮没有再引入新的持久化状态或新的指标 owner，而是继续消费：
  - `projection-service` 已经拥有的 replay metrics
  - `ops runtime` 已经拥有的 replay lag

### 3. `sdkwork-im-server` 对齐暴露同一条 replay-status 路由
- `services/sdkwork-im-cloud-gateway/src/node/build.rs`
  - 新增 `GET /backend/v3/api/ops/replay_status`
- `services/sdkwork-im-cloud-gateway/src/node/platform.rs`
  - 新增 `get_ops_replay_status(...)`
  - 直接复用 `ops_runtime.replay_status_view()`
- `services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`
  - 结构守卫已把 `get_ops_replay_status` 纳入公开边界约束

### 4. startup replay 的最小耗时归一化，保证吞吐率可解释
- `services/sdkwork-im-cloud-gateway/src/node/build.rs`
  - 当 startup replay 实际发生时，`durationMs` 现在会最小归一化到 `1ms`
- 这样做的目的不是伪造耗时，而是避免：
  - 回放确实发生
  - 但由于毫秒级取整为 `0`
  - 导致 `replayThroughputPerSecond` 永远退化成 `0`

### 5. 用空闲态与 stale snapshot replay 两类场景锁定合同
- `services/ops-service/tests/ops_runtime_test.rs`
  - 新增 `test_runtime_exposes_projection_replay_status_with_derived_throughput`
- `services/ops-service/tests/http_smoke_test.rs`
  - `ops replay-status` 默认空闲态现在校验：
    - `status == idle`
    - `replayThroughputPerSecond == 0`
    - 默认 `projection_replay` lag 项存在
- `services/sdkwork-im-cloud-gateway/tests/domain_recovery_persistence_test.rs`
  - stale snapshot restart 现在新增校验：
    - `status == replayed`
    - `replay.backlogSize >= 1`
    - `replay.replayedEventCount >= 1`
    - `replay.durationMs >= 1`
    - `replayThroughputPerSecond >= 1`

## 改动范围
- 代码：
  - `services/ops-service/src/lib.rs`
  - `services/sdkwork-im-cloud-gateway/src/node/build.rs`
  - `services/sdkwork-im-cloud-gateway/src/node/platform.rs`
- 测试：
  - `services/ops-service/tests/ops_runtime_test.rs`
  - `services/ops-service/tests/http_smoke_test.rs`
  - `services/sdkwork-im-cloud-gateway/tests/domain_recovery_persistence_test.rs`
  - `services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`
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
  - `cargo test -p ops-service --offline --test ops_runtime_test test_runtime_exposes_projection_replay_status_with_derived_throughput`
  - `cargo test -p ops-service --offline --test http_smoke_test test_cluster_lag_health_runtime_dir_and_diagnostics_over_http`
  - `cargo test -p sdkwork-im-cloud-gateway --offline --test domain_recovery_persistence_test test_default_local_minimal_profile_reports_projection_replay_backlog_and_lag_after_stale_snapshot_restart`
- 红测失败点与预期一致：
  - `ops-service` 还没有 `ProjectionReplayStatusView`
  - `/backend/v3/api/ops/replay_status` 还不存在
  - `replayThroughputPerSecond` 还未暴露
  - stale snapshot replay 场景还不能对外给出 `replayed` 状态与吞吐率

### Green
- 上述三条定向测试现已全部通过

## 回归验证
- `cargo fmt --all`
- `cargo test -p ops-service --offline --test ops_runtime_test test_runtime_exposes_projection_replay_status_with_derived_throughput`
- `cargo test -p ops-service --offline --test http_smoke_test test_cluster_lag_health_runtime_dir_and_diagnostics_over_http`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test domain_recovery_persistence_test test_default_local_minimal_profile_reports_projection_replay_backlog_and_lag_after_stale_snapshot_restart`
- `cargo test -p ops-service --offline`
- `cargo test -p sdkwork-im-cloud-gateway --offline`
- `cargo test -p projection-service --offline`

## 结论
- 这是 `Wave C / Step 09 / CP09-3` 的第三个真实代码增量。
- `Projection Plane` 现在不只会暴露 replay 指标和 lag 片段，还能通过专门的 `ops/replay-status` 读面直接回答：
  - 当前是空闲还是发生过 replay
  - replay 补了多少事件
  - replay 持续多久
  - 估算吞吐率是多少
  - 对应的 `projection_replay` lag 是什么
- 但 `CP09-3` 仍不应整体判定通过，因为当前仍缺：
  - 持续运行态的 live projection lag
  - `rebuild duration`
  - `inbox / timeline update delay`
  - 更完整的 SLO / 告警 / 外部 telemetry sink

## 下一轮继续做什么
1. 继续留在 `CP09-3`，优先评估 live projection lag 与运行态 update delay。
2. 若 live lag 仍不适合最小增量落地，则补更明确的 `rebuild duration` 或 `inbox / timeline update delay` 指标。
3. 只有在 `CP09-3` 的 plane 级观测面再闭一段后，才进入 `CP09-4` 的 backup / restore / repair / archive 收口。

