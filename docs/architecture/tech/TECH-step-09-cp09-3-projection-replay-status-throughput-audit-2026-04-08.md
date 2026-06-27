> Migrated from `docs/review/step-09-cp09-3-projection-replay-status-throughput-质量审计与复盘-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 09 / CP09-3 projection replay status throughput observability 质量审计与复盘 - 2026-04-08

## 审计范围
- `services/ops-service/src/lib.rs`
- `services/ops-service/tests/ops_runtime_test.rs`
- `services/ops-service/tests/http_smoke_test.rs`
- `services/sdkwork-im-cloud-gateway/src/node/build.rs`
- `services/sdkwork-im-cloud-gateway/src/node/platform.rs`
- `services/sdkwork-im-cloud-gateway/tests/domain_recovery_persistence_test.rs`
- `services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`

## 审计结论
- 本轮未发现阻塞当前增量交付的剩余缺陷。
- 改动符合 `CP09-3` 当前最小正确方向：
  - `projection-service` 继续拥有 replay metrics
  - `ops-service` 继续只负责公开读面与 runtime 汇总
  - `sdkwork-im-server` 继续只负责把真实 startup replay 证据映射到 ops 面
- 本轮没有为补 `replay-status` 而新增旁路状态机，也没有再造一套与 `health / lag / diagnostics` 脱节的独立数据源。

## 正向结果
- `ops/replay-status` 现在把 replay drill 证据收口为单一读面，调用方不需要再手工拼：
  - `ops/health`
  - `ops/lag`
  - `ops/diagnostics`
- `replayThroughputPerSecond` 现在是从已有 replay metrics 推导得到，而不是额外维护一个新的 counter owner。
- 默认空闲态和真实 stale snapshot replay 态现在都拥有稳定 schema：
  - 空闲态返回 `idle`
  - replay 态返回 `replayed`
- `projection_replay` lag 现在可以和 replay status 放在同一条读面返回，使 replay gap 的解释路径更完整。

## 本轮发现并修正的问题
- 上一轮虽然已经能看到 backlog / lag / duration，但调用方仍要自己跨多个 ops 面做拼接，缺少明确的 replay drill 视图。
- replay throughput 之前完全缺失，无法回答“这次 replay 补齐速度大概是多少”。
- startup replay 在极短路径上可能出现亚毫秒耗时，若直接取毫秒整数会变成 `0ms`，导致吞吐率退化为 `0`，信息失真。

## 剩余风险
- 当前 `replayThroughputPerSecond` 仍然是 startup replay 的近似吞吐率，不是持续运行态 live throughput。
- 当前 `status` 只区分：
  - `idle`
  - `replayed`
  还没有更细的持续进行中或失败中状态。
- 当前 `lag` 仍只返回 `projection_replay` 项，不覆盖更广义的 live projection lag。
- 当前 `durationMs` 仍不等于完整 `rebuild duration`。
- 当前 traces / logs 仍停留在进程内 recent view，不是外部 telemetry sink。
- `CP09-4` 的 backup / restore / repair / archive 仍未闭环。

## 验证证据
- `cargo fmt --all`
- `cargo test -p ops-service --offline --test ops_runtime_test test_runtime_exposes_projection_replay_status_with_derived_throughput`
- `cargo test -p ops-service --offline --test http_smoke_test test_cluster_lag_health_runtime_dir_and_diagnostics_over_http`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test domain_recovery_persistence_test test_default_local_minimal_profile_reports_projection_replay_backlog_and_lag_after_stale_snapshot_restart`
- `cargo test -p ops-service --offline`
- `cargo test -p sdkwork-im-cloud-gateway --offline`
- `cargo test -p projection-service --offline`

## 复盘结论
- 本轮最正确的决策，是把 replay 证据收口成专门的 `ops/replay-status` 读面，而不是继续把更多字段堆进现有 diagnostics。
- 这让 `CP09-3` 从“能观测 replay 片段”推进到“能把 replay drill 结果作为一个完整视图对外解释”。
- 但这仍然只是 `Projection Plane` 在 startup replay 维度上的第三段落地，不应误判为整个 `Step 09` 或整个 observability 体系已经完成。

