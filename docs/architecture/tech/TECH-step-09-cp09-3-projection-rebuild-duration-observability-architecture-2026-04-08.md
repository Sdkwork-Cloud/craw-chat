> Migrated from `docs/review/step-09-cp09-3-projection-rebuild-duration-observability-架构兑现与回写决议-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 09 / CP09-3 projection rebuild duration observability 架构兑现与回写决议 - 2026-04-08

## 对应架构文档
- `docs/架构/09-实施计划.md`
- `docs/架构/132-存储架构与自主演进路线设计-2026-04-06.md`
- `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md`
- `docs/架构/140-可观测性与SLO治理设计-2026-04-06.md`
- `docs/架构/141-数据生命周期与归档成本治理设计-2026-04-06.md`

## 本轮已兑现能力力力力力
- `09`
  - `Wave C / Step 09 / CP09-3` 已完成第六段真实落地：
    - `projectionPlane.rebuildDurationMs` 已通过真实代码、测试和公开 ops 读面落地
    - `CP09-3` 现在可整体判定通过
- `132`
  - 统一投影 owner 现在不仅拥有 replay / live lag / update delay，也拥有 startup recovery 的总 rebuild duration
  - 这份时长继续依附：
    - projection-owned observability state
    - 同一条 snapshot + replay recovery 合同
    而不是再造一条 telemetry-only 旁路
- `138`
  - 灾备恢复语义现在开始具备“整次 projection recovery 花了多久”的公开证据
  - snapshot-only recovery 也能在不伪造 replay duration 的前提下，对外说明 rebuild 已真实发生
- `140`
  - `Projection Plane` 的核心指标现已全部落地到真实代码：
    - `projection lag`
    - `rebuild duration`
    - `backlog size`
    - `inbox/timeline update delay`
    - `replay throughput`
  - 因此本文第 `5.5 Projection Plane` 的核心指标集合，现在已具备 code-backed evidence

## 本轮未兑现能力力力力力
- `140`
  - 更完整的 SLO / alert threshold / error taxonomy / external telemetry sink 仍未落地
  - 但这些已不再阻塞 `CP09-3` 的“基本收口”判定
- `138`
  - tenant 级恢复、跨 cell / region 灾备演练仍未落地
- `141`
  - archive / lifecycle / retention policy 仍未触达
  - `CP09-4` 仍未开始闭环
- `Step 09`
  - `CP09-3` 已通过
  - `CP09-4` 未完成，因此 `Step 09` 仍未闭环

## 是否偏离架构
- 无偏离。
- 本轮实现继续遵守既有架构边界：
  - `projection-service` 继续拥有 projection observability owner
  - `sdkwork-im-server` 只在 startup recovery 主路径上测量并回填 rebuild duration
  - `ops-service` 继续只做公开 health / diagnostics 读面
- 同时，本轮明确了一个原架构文档未完全显式写清、但与设计一致的 as-built 语义：
  - `rebuildDurationMs` 是整次 projection recovery 总时长
  - `replay.durationMs` 只回答 replay 时长

## 回写决议
- `docs/架构/09-实施计划.md` 追加 `As-Built 94`
- `docs/架构/132-存储架构与自主演进路线设计-2026-04-06.md` 追加 `As-Built 12`
- `docs/架构/138-高可用与灾备恢复设计-2026-04-06.md` 追加 `As-Built 9`
- `docs/架构/140-可观测性与SLO治理设计-2026-04-06.md` 追加 `As-Built 6`
- `docs/架构/141-数据生命周期与归档成本治理设计-2026-04-06.md`
  - 本轮仅复核，不追加回写，等待 `CP09-4` 的 archive / retention / lifecycle 代码证据

## 证据
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
- 验证：
  - `cargo fmt --all --check`
  - `cargo test -p projection-service --offline`
  - `cargo test -p ops-service --offline`
  - `cargo test -p sdkwork-im-cloud-gateway --offline`

## 当前判断
- `CP09-3`：通过。
- `Step 09`：未闭环。
- `Wave C / 93`：继续阻塞于 `Step 09 / CP09-4`。

