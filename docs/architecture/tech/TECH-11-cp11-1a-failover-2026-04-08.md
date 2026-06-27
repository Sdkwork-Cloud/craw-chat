> Migrated from `docs/step/11-CP11-1A-failover场景目录收敛-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 11-CP11-1A - failover 场景目录收敛

## 本轮目的

把 `Step 11 / CP11-1` 中已经被文本说明提到的 `failover`，正式冻结为机器可读场景族，避免 `scenario catalog`、运维文档和本地演练入口继续分裂。

## 发现的问题

- `tools/perf/step-11-scenario-catalog.json` 的 `executionOrder` 提到 `failover`，但 `scenarioFamilies` 未显式列出 `failover`
- `docs/部署/性能与灾备演练场景.md` 在 `CP11-3` 段落提到 `failover`，但“场景族”总表与推荐执行顺序未把它作为独立场景冻结
- 结果是：人工阅读知道要做 `failover`，机器目录却不能把它当作正式基线

## 本轮决策

- 将 `failover` 固定为 `Step 11` 的独立场景族
- 统一使用本地已有最小可信入口：
  - `services/sdkwork-im-cloud-gateway/tests/performance_ha_dr_drill_test.rs`
  - `services/sdkwork-im-cloud-gateway/tests/cluster_realtime_routing_e2e_test.rs`
  - `tools/perf/step-11-cp11-3-local-drill-baseline.json`
- 冻结的指标字段为：
  - `takeover_duration_ms`
  - `owner_switch_accuracy`
  - `stale_session_rejection_rate`
  - `resume_takeover_success_rate`

## 实施结果

- 更新 `tools/perf/step-11-scenario-catalog.json`
  - 新增 `failover-drill`
  - 明确 `tierIds`、`metrics`、`repoAssets`
- 更新 `docs/部署/性能与灾备演练场景.md`
  - 在“场景族”总表新增 `failover`
  - 在推荐执行顺序中把 `failover` 放到 `upgrade-rollback` 之前

## 验证

- 红灯：
  - `cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test -- --nocapture`
  - 失败点：`Step 11 catalog must define the failover scenario family`
- 绿灯：
  - `cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test -- --nocapture`

## 下一轮建议

- 继续检查 `Step 11 / CP11-2` 与 `CP11-3` 的机器可读结果是否已经形成统一 evidence index
- 若无更小缺口，再进入 `Step 12` 的 SDK / CLI / operator 收口验证

