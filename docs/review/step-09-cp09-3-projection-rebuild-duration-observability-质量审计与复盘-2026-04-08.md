# Step 09 / CP09-3 projection rebuild duration observability 质量审计与复盘 - 2026-04-08

## 审计范围
- `services/projection-service/src/observability.rs`
- `services/ops-service/src/lib.rs`
- `services/sdkwork-im-cloud-gateway/src/node.rs`
- `services/sdkwork-im-cloud-gateway/src/node/build.rs`
- `services/sdkwork-im-cloud-gateway/src/node/platform.rs`
- `services/projection-service/tests/projection_snapshot_test.rs`
- `services/ops-service/tests/http_smoke_test.rs`
- `services/sdkwork-im-cloud-gateway/tests/domain_recovery_persistence_test.rs`

## 审计结论
- 本轮未发现阻塞当前增量交付的剩余缺陷。
- `rebuild duration` 的 owner 语义、公开读面和测试证据是一致的，没有把 replay 指标和 rebuild 指标混写成同一含义。
- 本轮改动符合 `CP09-3` 的最小正确方向：
  - `projection-service` 继续拥有 projection plane observability state
  - `sdkwork-im-server` 继续只测 startup recovery 的真实总时长
  - `ops-service` 继续只负责公开 health / diagnostics 视图

## 正向结果
- `Projection Plane` 现在终于不再只会回答“replay 花了多久”，也能回答“整个 rebuild 花了多久”。
- snapshot-only recovery 现在有了正确的观测语义：
  - replay 可以为 `0`
  - rebuild 仍然可以为正数
- `ops/health` 与 `ops/diagnostics` 现在可以更准确地区分三类问题：
  - 没有发生 recovery
  - 发生了 snapshot restore，但没有 replay
  - 发生了 replay backlog 补偿
- `Projection Plane` 的核心指标集合已从“还差最后一个硬缺口”推进到“已具备 Step 09 / CP09-3 的基本收口证据”。

## 本轮关键判断
- 本轮最重要的决策，不是“新增一个字段”，而是把语义定义正确：
  - `replay.durationMs`
    - 只描述 replay
  - `rebuildDurationMs`
    - 描述整次 projection recovery
- 如果把两者混成一个指标，会直接让 snapshot-only recovery 在运维面失真。
- 当前实现避免了这个问题，而且没有额外引入新的持久化 owner、脚本专用缓存或 ops 侧二次估算逻辑。

## 本轮未发现但仍需明确的残余风险
- 当前 `rebuildDurationMs` 仍是单节点 `Local Minimal` profile 下的进程内 owner 视图，不是多 cell / 多 region 的全局恢复指标。
- 当前 `rebuildDurationMs` 只覆盖 startup projection recovery，不覆盖未来可能出现的后台 archive restore worker 或 tenant 级批量恢复任务。
- 当前 `CP09-4` 仍完全未闭环：
  - backup / restore / repair 已有路径
  - archive / retention / lifecycle 仍缺真实代码与脚本收口
- 更完整的 SLO / alert threshold / external telemetry sink 仍未建立，但这不再阻塞 `CP09-3` 的检查点通过。

## 验证证据
- `cargo fmt --all --check`
- `cargo test -p projection-service --offline`
- `cargo test -p ops-service --offline`
- `cargo test -p sdkwork-im-cloud-gateway --offline`

## 复盘结论
- 到本轮为止，`Projection Plane` 的核心 metrics / tracing / logging 已达到 Step 09 文档要求的“按 plane 基本收口”。
- `CP09-3` 可以判定通过。
- `Step 09` 仍不能判定通过，因为 `CP09-4` 的 archive / retention / lifecycle 治理还没有真实开始。
- 下一轮最正确的动作，不是继续在 `CP09-3` 上堆更完整的理论 SLO，而是转入 `CP09-4`，把 backup / restore / repair / archive 的代码和脚本闭环真正补出来。
