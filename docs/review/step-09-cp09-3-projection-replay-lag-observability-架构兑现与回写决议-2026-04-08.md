# Step 09 / CP09-3 projection replay lag observability 架构兑现与回写决议 - 2026-04-08

## 对应架构文档
- `docs/架构/09-实施计划.md`
- `docs/架构/132-存储架构与自主演进路线设计-2026-04-06.md`
- `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md`
- `docs/架构/140-可观测性与SLO治理设计-2026-04-06.md`
- `docs/架构/141-数据生命周期与归档成本治理设计-2026-04-06.md`

## 本轮已兑现能力力力力力
- `09`
  - `Wave C / Step 09 / CP09-3` 已完成第二段真实落地：
    - projection replay 现在具备 `backlog / replayed count / duration` 指标
    - startup replay 现在会生成按 scope 的 `projection_replay` lag item
    - `ops/lag`、`ops/health`、`ops/diagnostics` 现在共同暴露这份 replay 证据
- `132`
  - 统一存储抽象开始支撑“checkpoint + journal replay gap”的可证明语义
  - lag/backlog 不再来自额外旁路状态，而是来自同一条：
    - snapshot metadata
    - snapshot timeline
    - commit journal
    恢复主路径
- `138`
  - 灾备恢复链路现在不只证明“restore 发生过”，还可以证明：
    - projection 落后了多少
    - restart 时 replay 了多少
    - replay 花了多久
  - stale snapshot restart 的测试进一步证明了“恢复证据”和“恢复结果”一致
- `140`
  - `Projection Plane` 已继续兑现本文核心指标：
    - `projection lag`
    - `backlog size`
    - `replay duration`
  - `ops/lag` 也开始拥有 plane-specific replay lag evidence，而不是只有通用静态占位

## 本轮未兑现能力力力力力
- `140`
  - 持续运行态的 live projection lag 仍未落地
  - `replay throughput / rebuild duration / inbox-timeline update delay` 仍未落地
  - 更完整的 SLO / alert threshold / error taxonomy 仍未落地
- `138`
  - tenant 级恢复、跨 cell / region 灾备演练与更完整的 restore drill 仍未落地
- `141`
  - archive / lifecycle / retention policy 本轮未触达
- `Step 09`
  - `CP09-3` 仍未整体验收通过
  - `CP09-4` 仍未开始闭环

## 是否偏离架构
- 无偏离。
- 本轮实现继续遵守既有架构边界：
  - `projection-service` 继续拥有 projection observability 状态
  - `ops-service` 继续维护公共 schema 与 runtime 状态
  - `sdkwork-im-server` 继续通过真实 replay 路径计算并映射 evidence
- 本轮也没有把 replay lag 做成新的硬编码脚本字段，而是继续挂在真实恢复合同上。

## 回写决议
- `docs/架构/09-实施计划.md` 追加 `As-Built 90`
- `docs/架构/132-存储架构与自主演进路线设计-2026-04-06.md` 追加 `As-Built 8`
- `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md` 追加 `As-Built 5`
- `docs/架构/140-可观测性与SLO治理设计-2026-04-06.md` 追加 `As-Built 2`
- `docs/架构/141-数据生命周期与归档成本治理设计-2026-04-06.md`
  - 本轮仅复核，不追加回写，等待 archive / lifecycle 代码证据

## 证据
- 代码：
  - `services/projection-service/src/observability.rs`
  - `services/ops-service/src/lib.rs`
  - `services/sdkwork-im-cloud-gateway/src/node/build.rs`
  - `services/sdkwork-im-cloud-gateway/src/node/platform.rs`
- 测试：
  - `services/projection-service/tests/projection_snapshot_test.rs`
  - `services/ops-service/tests/http_smoke_test.rs`
  - `services/sdkwork-im-cloud-gateway/tests/domain_recovery_persistence_test.rs`
- 验证：
  - `cargo test -p projection-service --offline --test projection_snapshot_test test_projection_service_records_projection_replay_metrics`
  - `cargo test -p ops-service --offline --test http_smoke_test test_cluster_lag_health_runtime_dir_and_diagnostics_over_http`
  - `cargo test -p sdkwork-im-cloud-gateway --offline --test domain_recovery_persistence_test test_default_local_minimal_profile_surfaces_projection_plane_observability_over_ops_health_and_diagnostics`
  - `cargo test -p sdkwork-im-cloud-gateway --offline --test domain_recovery_persistence_test test_default_local_minimal_profile_reports_projection_replay_backlog_and_lag_after_stale_snapshot_restart`
  - `cargo test -p projection-service --offline`
  - `cargo test -p ops-service --offline`
  - `cargo test -p sdkwork-im-cloud-gateway --offline`
  - `cargo fmt --all --check`

## 当前判断
- 这是 `CP09-3` 的真实增量，不是 `Step 09` 的整步通过。
- `CP09-3`：继续推进中，尚不能整体判定通过。
- `Step 09`：未闭环。
- `Wave C / 93`：继续阻塞于 `Step 09`。
