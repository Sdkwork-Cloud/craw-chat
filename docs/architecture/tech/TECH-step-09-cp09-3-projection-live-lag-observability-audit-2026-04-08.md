> Migrated from `docs/review/step-09-cp09-3-projection-live-lag-observability-质量审计与复盘-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 09 / CP09-3 projection live lag observability 质量审计与复盘 - 2026-04-08

## 审计范围
- `services/projection-service/src/observability.rs`
- `services/projection-service/src/lib.rs`
- `services/projection-service/src/scope.rs`
- `services/ops-service/src/lib.rs`
- `services/sdkwork-im-cloud-gateway/src/node/platform.rs`
- `services/projection-service/tests/projection_snapshot_test.rs`
- `services/sdkwork-im-cloud-gateway/tests/domain_recovery_persistence_test.rs`

## 审计结论
- 本轮未发现阻塞当前增量交付的剩余缺陷。
- 改动符合 `CP09-3` 当前最小正确方向：
  - live lag 继续由 `projection-service` 拥有
  - `ops-service` 继续只负责公开 lag 聚合
  - `sdkwork-im-server` 继续只负责 owner-to-ops 映射
- 本轮没有为补 live lag 再建新的 replay 状态机或新的 telemetry 存储 owner。

## 正向结果
- `Projection Plane` 现在已经开始具备 steady-state lag 读面，而不只是在 startup replay 后给证据。
- live lag 直接挂在真实 `projection_service.apply(...)` 主路径上，避免了“在 ops 层再猜一次 offset 差值”的伪指标。
- `ops/lag` 与 `diagnostics.lag` 现在可以明确区分：
  - `projection_replay`
    - startup replay / stale snapshot 补偿语义
  - `projection_live`
    - 运行态 apply 追平语义
- `ops/replay-status` 保持 replay-only 语义，没有因为补 live lag 被稀释成“什么 lag 都塞进去”的混合读面。

## 本轮发现并修正的问题
- 全量回归第一轮发现的是结构回归，不是功能错误：
  - `services/projection-service/src/lib.rs` 再次超过 Step 02 的 `1000` 行红线
- 根因明确：
  - live lag 的 scope helper 逻辑还直接堆在 `lib.rs`
- 已按最小修复原则处理：
  - 抽出 `services/projection-service/src/scope.rs`
  - 让 `lib.rs` 重新只保留装配与业务逻辑
  - 不改变 live lag 的行为合同

## 剩余风险
- 当前 live lag 仍然是“projection 已观察到的最新 offset 与已投影 offset 的差值”，不是异步分布式全局队列深度。
- 当前 steady-state 正常路径下，live lag 多数时间会快速收敛到 `0`；它更擅长回答“有没有追平 / 有没有卡住”，而不是回答“重建一共花了多久”。
- 当前 live lag 只覆盖 conversation-scoped、由 `projection-service` 真正处理的 projection 事件。
- 当前仍未提供 `rebuild duration`。
- 当前 traces / logs 仍停留在进程内 recent view，不是外部 telemetry sink。

## 验证证据
- `cargo fmt --all --check`
- `cargo test -p projection-service --offline`
- `cargo test -p ops-service --offline`
- `cargo test -p sdkwork-im-cloud-gateway --offline`

## 复盘结论
- 本轮最正确的决策，是把 live lag 放回现有 apply owner 与既有 `ops/lag` 入口，而不是继续扩张 `ops/replay-status` 的职责。
- 结构红线再次证明，`CP09-3` 的推进不能只看字段有没有出现，还必须同时满足既有模块边界与文件长度约束。
- 到这一轮为止，`Projection Plane` 的剩余核心硬缺口已经基本收缩到 `rebuild duration`。

