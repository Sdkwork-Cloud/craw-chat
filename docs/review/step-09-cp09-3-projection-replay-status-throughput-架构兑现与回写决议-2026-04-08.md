# Step 09 / CP09-3 projection replay status throughput observability 架构兑现与回写决议 - 2026-04-08

## 对应架构文档
- `docs/架构/09-实施计划.md`
- `docs/架构/132-存储架构与自主演进路线设计-2026-04-06.md`
- `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md`
- `docs/架构/140-可观测性与SLO治理设计-2026-04-06.md`
- `docs/架构/141-数据生命周期与归档成本治理设计-2026-04-06.md`

## 本轮已兑现能力力力力力
- `09`
  - `Wave C / Step 09 / CP09-3` 已完成第三段真实落地：
    - 新增专门的 `ops/replay-status`
    - replay 现在具备 `idle / replayed` 状态表达
    - replay 现在具备派生 `replayThroughputPerSecond`
    - `projection_replay lag` 现在可通过同一条 replay 读面读取
- `132`
  - 统一存储抽象继续支撑“同一恢复合同上的 replay drill 读面”
  - `replay-status` 与 `replay throughput` 继续来自同一条：
    - snapshot metadata
    - snapshot timeline
    - commit journal
    恢复主路径，而不是新造一个旁路状态 owner
- `138`
  - 灾备恢复链路现在不只证明“restore / replay 发生过”，还可以通过公开 `ops/replay-status` 说明：
    - 当前是 `idle` 还是 `replayed`
    - replay 了多少事件
    - replay 花了多久
    - replay 吞吐率大致是多少
    - 对应 `projection_replay` lag 是什么
- `140`
  - `Projection Plane` 已继续兑现本文要求中的：
    - `ops/replay-status`
    - `replay throughput`
  - `Projection Plane` 现在已有：
    - `ops/health`
    - `ops/lag`
    - `ops/diagnostics`
    - `ops/replay-status`
    这四条互补读面，而不是只剩概念性指标名词

## 本轮未兑现能力力力力力
- `140`
  - 持续运行态的 live projection lag 仍未落地
  - `rebuild duration / inbox-timeline update delay` 仍未落地
  - 更完整的 SLO / alert threshold / error taxonomy 仍未落地
- `138`
  - tenant 级恢复、跨 cell / region 灾备演练仍未落地
- `141`
  - archive / lifecycle / retention policy 本轮未触达
- `Step 09`
  - `CP09-3` 仍未整体验收通过
  - `CP09-4` 仍未开始闭环

## 是否偏离架构
- 无偏离。
- 本轮实现继续遵守既有架构边界：
  - `projection-service` 继续拥有 replay metrics
  - `ops-service` 继续汇总公开 replay 读面
  - `sdkwork-im-server` 继续通过真实 replay 路径提供证据
- 本轮也没有把 throughput 做成新的硬编码状态，而是继续从既有 replay metrics 推导。

## 回写决议
- `docs/架构/09-实施计划.md` 追加 `As-Built 91`
- `docs/架构/132-存储架构与自主演进路线设计-2026-04-06.md` 追加 `As-Built 9`
- `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md` 追加 `As-Built 6`
- `docs/架构/140-可观测性与SLO治理设计-2026-04-06.md` 追加 `As-Built 3`
- `docs/架构/141-数据生命周期与归档成本治理设计-2026-04-06.md`
  - 本轮仅复核，不追加回写，等待 archive / lifecycle 代码证据

## 证据
- 代码：
  - `services/ops-service/src/lib.rs`
  - `services/sdkwork-im-cloud-gateway/src/node/build.rs`
  - `services/sdkwork-im-cloud-gateway/src/node/platform.rs`
- 测试：
  - `services/ops-service/tests/ops_runtime_test.rs`
  - `services/ops-service/tests/http_smoke_test.rs`
  - `services/sdkwork-im-cloud-gateway/tests/domain_recovery_persistence_test.rs`
  - `services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`
- 验证：
  - `cargo fmt --all`
  - `cargo test -p ops-service --offline --test ops_runtime_test test_runtime_exposes_projection_replay_status_with_derived_throughput`
  - `cargo test -p ops-service --offline --test http_smoke_test test_cluster_lag_health_runtime_dir_and_diagnostics_over_http`
  - `cargo test -p sdkwork-im-cloud-gateway --offline --test domain_recovery_persistence_test test_default_local_minimal_profile_reports_projection_replay_backlog_and_lag_after_stale_snapshot_restart`
  - `cargo test -p ops-service --offline`
  - `cargo test -p sdkwork-im-cloud-gateway --offline`
  - `cargo test -p projection-service --offline`

## 当前判断
- 这是 `CP09-3` 的真实增量，不是 `Step 09` 的整步通过。
- `CP09-3`：继续推进中，尚不能整体判定通过。
- `Step 09`：未闭环。
- `Wave C / 93`：继续阻塞于 `Step 09`。
