# Continuous Optimization - Step 11 failover scenario catalog

## 结论

本轮修复了 `Step 11` 的一个真实契约缺口：`failover` 已被文本与测试入口支持，但未被 `scenario catalog` 正式声明为场景族。

## 证据

- 红灯：
  - `cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test -- --nocapture`
  - 失败信息：`Step 11 catalog must define the failover scenario family`
- 绿灯：
  - `cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test -- --nocapture`

## 本轮改动

- 收紧 `services/sdkwork-im-cloud-gateway/tests/performance_drill_catalog_test.rs`
- 更新 `tools/perf/step-11-scenario-catalog.json`
- 更新 `docs/部署/性能与灾备演练场景.md`

## 当前收益

- `failover` 不再只是描述性文字，而是可被机器读取和回归测试冻结的正式场景
- `CP11-3` 的 `failover` 本地 drill 与 `CP11-1` 场景清单达成同一口径
- 下一轮在写 evidence index 或预发布演练清单时，不会遗漏 `failover`

## 残余风险

- 当前仅冻结 `CI Smoke / standalone.split-services.development` 入口，尚未扩展到 `Pre-Release Tier` 的独立 evidence 索引
- `failover` 指标仍以单轮本地演练为主，尚未形成统一 artifact schema

## 下一步

- 优先检查 `Step 11` 是否还缺少统一 evidence index / artifact schema
- 若 `Step 11` 无更小真实缺口，则推进 `Step 12`
