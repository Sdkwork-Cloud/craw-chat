# Continuous Optimization - Pre-Release / Capacity Tier gates

## 结论

本轮继续收敛 Step 11 高阶 gate 的机器可读 contract，把 `step-11-scenario-catalog.json` 从“只指向 gateTemplate”推进到“直接暴露高阶 tier 的 artifactRoot”，并同步冻结 schema 和回写文档。

## 红绿证据

- 红灯：
  - `cargo test -p sdkwork-im-cloud-gateway --offline --test performance_drill_catalog_test test_continuous_optimization_surfaces_tier_gate_artifact_roots_in_catalog_schema_and_backwrites -- --exact --nocapture`
  - 失败信息：缺少 `gateTemplate`
- 绿灯：
  - 同一条测试在补齐 catalog tier `artifactRoot`、schema 字段和文档回写后保持通过

## 本轮改动

- 更新 `tools/perf/step-11-scenario-catalog.json`
- 更新 `tools/perf/schemas/step-11-scenario-catalog.schema.json`
- 更新 `docs/部署/性能与灾备演练场景.md`
- 回写 step / review / architecture 文档
- 为 catalog tier 直接暴露：
  - `tools/perf/step-11-pre-release-tier-gate.json`
  - `tools/perf/step-11-capacity-tier-gate.json`
  - `artifacts/perf/step-11/pre-release`
  - `artifacts/perf/step-11/capacity`

## 当前收益

- 自动化现在只读 `step-11-scenario-catalog.json` 就能知道高阶 tier 的 gate 模板和证据根目录
- `step-11-scenario-catalog.schema.json` 不再把 `tiers` 视为完全自由对象
- 仓库继续明确区分：
  - gate 状态：`template_only_pending_execution`
  - slot 状态：`pending_collection`
- `standalone.split-services.development` 继续作为 `Pre-Release Tier` 的默认 profile
- `capacity-dedicated` 继续作为 `Capacity Tier` 的目标环境名
- gate 模板内部既有 contract 仍然保持：
  - `collectionSummary`
  - `evidenceSlots`
  - `artifactPath`
  - `suggestedRelativePath`
  - `collectedAt`
  - `sizeBytes`
  - `checksumSha256`

## 残余风险

- 当前仍然没有真实 `Pre-Release Tier` 结果
- 当前仍然没有真实 `Capacity Tier` 报告
- `artifactRoot` 只是目录 contract，不代表目录内已有真实 artifact

## 下一步

- 优先考虑回填一轮真实 `standalone.split-services.development` 的 `Pre-Release Tier` 样本
- 若继续做模板 contract，可再评估是否把高阶 tier 的摘要字段继续上浮到 catalog
