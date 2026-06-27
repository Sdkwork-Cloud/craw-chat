# Step 09 / CP09-3 projection update delay observability 质量审计与复盘 - 2026-04-08

## 审计范围
- `services/projection-service/src/observability.rs`
- `services/projection-service/src/lib.rs`
- `services/projection-service/src/update_delay.rs`
- `services/ops-service/src/lib.rs`
- `services/sdkwork-im-cloud-gateway/src/node/platform.rs`
- `services/projection-service/tests/projection_snapshot_test.rs`
- `services/ops-service/tests/http_smoke_test.rs`
- `services/sdkwork-im-cloud-gateway/tests/domain_recovery_persistence_test.rs`

## 审计结论
- 本轮未发现阻塞当前增量交付的剩余缺陷。
- 改动符合 `CP09-3` 当前最小正确方向：
  - live update delay 继续由 `projection-service` 拥有
  - `ops-service` 继续只负责公开 schema
  - `sdkwork-im-server` 继续只负责 owner-to-ops 映射
- 本轮没有为补 `update delay` 再建新的计时服务或旁路指标 owner。

## 正向结果
- `Projection Plane` 现在已经开始具备运行态读面，而不只是在 startup replay 后给证据。
- `inbox / timeline update delay` 直接挂在真实 `projection_service.apply(...)` 主路径上，避免了“代码外再猜一次延迟”的伪指标。
- 默认空闲态的 `health / diagnostics` 现在都能稳定返回 `updateDelay` schema，不会让调用方在字段缺失和零值之间猜测。
- `sdkwork-im-server` 的真实 HTTP 面已经证明：
  - `message.posted` 能驱动 update delay 出现在 `ops/health`
  - snapshot-only restart 后 `ops/diagnostics` 仍保留稳定零值 schema

## 本轮发现并修正的问题
- 全量回归第一轮并没有发现功能错误，而是发现结构回归：
  - `services/projection-service/src/lib.rs` 超过了 Step 02 的 `1000` 行红线
- 根因明确：
  - 新增的 RFC3339 解析和 delay 计算辅助逻辑直接堆进了 `lib.rs`
- 已按最小修复原则处理：
  - 只抽出 `services/projection-service/src/update_delay.rs`
  - 不改变 `updateDelay` 的行为合同
  - 回归后结构测试重新通过

## 剩余风险
- 当前 `updateDelay` 仍然依赖进程本地时钟与事件时间戳的相对差值，是实用型近似指标，不是跨节点全局时钟度量。
- 当前 `updateDelay` 只覆盖：
  - `message.posted`
  - `message.edited`
  - `message.recalled`
  还没有覆盖所有 projection 事件类型。
- 当前 `timelineMs` 与 `inboxMs` 在这一阶段仍是同一 apply 路径上的同值近似，尚未拆分成更细的多阶段延迟。
- 当前仍未提供 live `projection lag` 与 `rebuild duration`。
- 当前 traces / logs 仍停留在进程内 recent view，不是外部 telemetry sink。

## 验证证据
- `cargo fmt --all`
- `cargo test -p projection-service --offline --test projection_snapshot_test test_projection_service_records_projection_update_delay_metrics`
- `cargo test -p ops-service --offline --test http_smoke_test test_cluster_lag_health_runtime_dir_and_diagnostics_over_http`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test domain_recovery_persistence_test test_default_local_minimal_profile_surfaces_projection_plane_observability_over_ops_health_and_diagnostics`
- `cargo test -p projection-service --offline`
- `cargo test -p ops-service --offline`
- `cargo test -p sdkwork-im-cloud-gateway --offline`

## 复盘结论
- 本轮最正确的决策，是选择 `inbox / timeline update delay` 这个最贴近现有 apply 主路径的指标，而不是急于上更重的 live lag 状态机。
- 结构红线测试也证明，`CP09-3` 的推进不能只看功能字段是否出现，还必须同时满足既有模块边界约束。
- 这让 `CP09-3` 又往前走了一段，但还远不能误判为整个 `Projection Plane observability` 已经完成。
