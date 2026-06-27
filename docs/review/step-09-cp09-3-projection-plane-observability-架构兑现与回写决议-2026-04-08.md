# Step 09 / CP09-3 projection plane observability 架构兑现与回写决议 - 2026-04-08

## 对应架构文档
- `docs/架构/09-实施计划.md`
- `docs/架构/132-存储架构与自主演进路线设计-2026-04-06.md`
- `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md`
- `docs/架构/140-可观测性与SLO治理设计-2026-04-06.md`
- `docs/架构/141-数据生命周期与归档成本治理设计-2026-04-06.md`

## 本轮已兑现能力力力力力
- `09`
  - `Wave C / Step 09 / CP09-3` 已完成第一段真实落地：
    - `projection-service` 已拥有 projection plane observability owner
    - snapshot persist / restore 已具备真实 `metrics / trace / structured log`
    - `ops-service` 的 `health / diagnostics` 已公开同一份 projection plane 状态
- `132`
  - 统一存储抽象不再只是“可恢复”，还开始具备“可观测的恢复语义”
  - `projection snapshot` 的成功/失败现在能沿同一条 storage-backed 路径回传到运维读面
- `138`
  - 灾备恢复链路开始具备恢复后证据：
    - `ops/health` 能看到 snapshot persist 成功计数
    - `ops/diagnostics` 能看到 snapshot restore 的 trace 与 structured log
  - 这使 “恢复发生过” 不再只能靠本地日志猜测
- `140`
  - `Projection Plane` 已补出第一条真实的 plane-level diagnostics evidence：
    - `projectionPlane.status`
    - snapshot persist / restore counter
    - restore trace
    - restore structured log

## 本轮未兑现能力力力力力
- `140`
  - `projection lag / replay duration / backlog size` 仍未进入同一份 plane 观测合同
  - 更广泛的 SLO / 告警 / error taxonomy 仍未补齐
- `138`
  - 更完整的 tenant 级恢复、domain owner 恢复与跨 region 演练证据仍未建立
- `141`
  - snapshot archive / retention / lifecycle policy 本轮未触达
- `Step 09`
  - `CP09-3` 仍未整体验收通过
  - `CP09-4` 仍未开始闭环

## 是否偏离架构
- 无偏离。
- 本轮实现遵守了当前架构约束：
  - observability owner 继续内聚在 `projection-service`
  - `ops-service` 只负责公共 schema 与 runtime 暴露
  - `sdkwork-im-server` 只做映射，不新增一套旁路状态机
- 本轮也没有为了观测而绕开既有 snapshot/recovery 主路径，证据全部来自真实执行路径。

## 回写决议
- `docs/架构/09-实施计划.md` 追加 `As-Built 89`
- `docs/架构/132-存储架构与自主演进路线设计-2026-04-06.md` 追加 `As-Built 7`
- `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md` 追加 `As-Built 4`
- `docs/架构/140-可观测性与SLO治理设计-2026-04-06.md` 追加 `As-Built 1`
- `docs/架构/141-数据生命周期与归档成本治理设计-2026-04-06.md`
  - 本轮仅复核，不追加回写，等待 archive / lifecycle 证据

## 证据
- 代码：
  - `services/projection-service/src/observability.rs`
  - `services/projection-service/src/snapshot.rs`
  - `services/ops-service/src/lib.rs`
  - `services/sdkwork-im-cloud-gateway/src/node/platform.rs`
- 测试：
  - `services/projection-service/tests/projection_snapshot_test.rs`
  - `services/ops-service/tests/http_smoke_test.rs`
  - `services/sdkwork-im-cloud-gateway/tests/domain_recovery_persistence_test.rs`
- 验证：
  - `cargo test -p projection-service --offline --test projection_snapshot_test test_projection_service_records_snapshot_observability_metrics_traces_and_logs`
  - `cargo test -p ops-service --offline --test http_smoke_test test_cluster_lag_health_runtime_dir_and_diagnostics_over_http`
  - `cargo test -p sdkwork-im-cloud-gateway --offline --test domain_recovery_persistence_test test_default_local_minimal_profile_surfaces_projection_plane_observability_over_ops_health_and_diagnostics`
  - `cargo test -p projection-service --offline`
  - `cargo test -p ops-service --offline`
  - `cargo test -p sdkwork-im-cloud-gateway --offline`
  - `cargo fmt --all --check`

## 当前判断
- 这是 `CP09-3` 的真实增量，不是 `Step 09` 的整步通过。
- `CP09-3`：继续推进中，尚不能整体判定通过。
- `Step 09`：未闭环。
- `Wave C / 93`：继续阻塞于 `Step 09`。
