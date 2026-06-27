> Migrated from `docs/架构/150S-step11-failover-scenario-catalog设计-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 150S - Step 11 failover 场景目录设计

## 设计目标

`failover` 必须是 `Step 11` 的一等场景，而不是埋在说明文字里的隐式补充项。

## 契约定义

### 1. 目录层

`tools/perf/step-11-scenario-catalog.json` 的 `scenarioFamilies` 必须显式存在：

- `family = failover`
- `id = failover-drill`
- `tierIds = ["ci-smoke", "pre-release"]`

### 2. 指标层

冻结以下指标名：

- `takeover_duration_ms`
- `owner_switch_accuracy`
- `stale_session_rejection_rate`
- `resume_takeover_success_rate`

这些字段用于表达：

- owner 是否切换到目标节点
- takeover 延迟是否可观测
- 旧 session 是否被显式拒绝
- 新 session 的恢复路径是否完成

### 3. 资产层

最小可信资产固定为：

- `services/sdkwork-im-cloud-gateway/tests/performance_ha_dr_drill_test.rs`
- `services/sdkwork-im-cloud-gateway/tests/cluster_realtime_routing_e2e_test.rs`
- `tools/perf/step-11-cp11-3-local-drill-baseline.json`

### 4. 文档层

`docs/部署/性能与灾备演练场景.md` 必须在：

- 场景族总表
- 推荐执行顺序

两处显式出现 `failover`，避免“目录缺失但正文提到”的分裂状态。

## 非目标

- 不新增新的 `failover` 运行时能力
- 不引入 multi-cell / multi-region 演练
- 不改写 `Step 11` 其余场景族的指标口径

