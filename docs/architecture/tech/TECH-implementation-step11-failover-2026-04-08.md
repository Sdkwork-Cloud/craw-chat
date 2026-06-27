> Migrated from `docs/架构/09S-实施计划-step11-failover场景目录收敛-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 09S - 实施计划 - Step 11 failover 场景目录收敛

## 目标

收敛 `Step 11 / CP11-1` 的目录契约，让 `failover` 与 `connection / message / stream / drain-rebalance / restore-recovery / upgrade-rollback` 保持同级冻结。

## 最小实施面

1. 先把 `services/sdkwork-im-cloud-gateway/tests/performance_drill_catalog_test.rs` 收紧到：
   - `scenarioFamilies` 必须包含 `failover`
   - `failover` 必须冻结指标与本地执行资产
2. 运行红灯，确认缺口真实存在
3. 更新 `tools/perf/step-11-scenario-catalog.json`
4. 更新 `docs/部署/性能与灾备演练场景.md`
5. 重新运行同一测试拿到绿灯
6. 回写 `docs/step`、`docs/架构`、`docs/review`

## 约束

- 不引入新的压测框架
- 不扩展 `failover` 的运行语义，只冻结已有本地可信入口
- 不把 `upgrade-rollback` 混成 `failover` 的附属说明

## 放行标准

- `performance_drill_catalog_test` 红绿闭环完成
- 运维文档的“场景族”总表与机器目录口径一致
- `failover` 的指标和资产能被下一轮直接消费

