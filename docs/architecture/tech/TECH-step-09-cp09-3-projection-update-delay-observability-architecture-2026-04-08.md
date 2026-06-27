> Migrated from `docs/review/step-09-cp09-3-projection-update-delay-observability-架构兑现与回写决议-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 09 / CP09-3 projection update delay observability 架构兑现与回写决议 - 2026-04-08

## 对应架构文档
- `docs/架构/09-实施计划.md`
- `docs/架构/132-存储架构与自主演进路线设计-2026-04-06.md`
- `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md`
- `docs/架构/140-可观测性与SLO治理设计-2026-04-06.md`
- `docs/架构/141-数据生命周期与归档成本治理设计-2026-04-06.md`

## 本轮已兑现能力力力力力
- `09`
  - `Wave C / Step 09 / CP09-3` 已完成第四段真实落地：
    - `Projection Plane` 新增 `updateDelay`
    - `ops/health` 与 `ops/diagnostics` 现已公开：
      - `timelineMs`
      - `inboxMs`
      - `sourceEventType`
      - `scopeId`
      - `recordedAt`
- `132`
  - 统一投影 owner 现在不仅能从同一存储/恢复合同给出 replay 证据，还能从同一 live apply 主路径给出 `update delay`
  - 这份延迟证据继续依附：
    - projection-owned event apply
    - projection-owned read-side mutation
    而不是再造一个旁路 telemetry pipeline
- `138`
  - 灾备恢复语义之外，运维面现在也能读取 steady-state 的 `timeline / inbox update delay`
  - 这让恢复后的运维判断不只停留在“replay 结束了没有”，还开始具备“恢复后读面更新是否已经稳定”的辅助证据
- `140`
  - `Projection Plane` 核心指标中的 `inbox/timeline update delay` 已开始真实落地
  - 这意味着本文列出的核心指标现在已兑现到：
    - `projection lag`
      - 仅 startup replay 维度
    - `backlog size`
    - `replay duration`
    - `replay throughput`
    - `inbox/timeline update delay`

## 本轮未兑现能力力力力力
- `140`
  - live `projection lag` 仍未落地
  - `rebuild duration` 仍未落地
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
  - `projection-service` 继续拥有 projection observability owner
  - `ops-service` 继续只做公开 schema
  - `sdkwork-im-server` 继续只做映射
- 同时，本轮也修复了因辅助逻辑进入 `projection-service/src/lib.rs` 而触发的 Step 02 结构红线回归，保持既有代码治理要求不被破坏。

## 回写决议
- `docs/架构/09-实施计划.md` 追加 `As-Built 92`
- `docs/架构/132-存储架构与自主演进路线设计-2026-04-06.md` 追加 `As-Built 10`
- `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md` 追加 `As-Built 7`
- `docs/架构/140-可观测性与SLO治理设计-2026-04-06.md` 追加 `As-Built 4`
- `docs/架构/141-数据生命周期与归档成本治理设计-2026-04-06.md`
  - 本轮仅复核，不追加回写，等待 archive / lifecycle 代码证据

## 证据
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
- 验证：
  - `cargo fmt --all`
  - `cargo test -p projection-service --offline --test projection_snapshot_test test_projection_service_records_projection_update_delay_metrics`
  - `cargo test -p ops-service --offline --test http_smoke_test test_cluster_lag_health_runtime_dir_and_diagnostics_over_http`
  - `cargo test -p sdkwork-im-cloud-gateway --offline --test domain_recovery_persistence_test test_default_local_minimal_profile_surfaces_projection_plane_observability_over_ops_health_and_diagnostics`
  - `cargo test -p projection-service --offline`
  - `cargo test -p ops-service --offline`
  - `cargo test -p sdkwork-im-cloud-gateway --offline`

## 当前判断
- 这是 `CP09-3` 的真实增量，不是 `Step 09` 的整步通过。
- `CP09-3`：继续推进中，尚不能整体判定通过。
- `Step 09`：未闭环。
- `Wave C / 93`：继续阻塞于 `Step 09`。

