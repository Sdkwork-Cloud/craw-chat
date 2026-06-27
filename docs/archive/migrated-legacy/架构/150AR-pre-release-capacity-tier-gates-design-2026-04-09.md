# 150AR - Pre-Release / Capacity Tier gates design

## 设计结论

高阶 gate 的下一步最小机器契约不是继续复制更多 slot 细节，而是把 `artifactRoot` 直接上浮到 `step-11-scenario-catalog.json` 的 tier 层，让 catalog 成为高阶 tier 入口，而不是只做跳板。

## 资产落点

- `tools/perf/step-11-scenario-catalog.json`
- `tools/perf/schemas/step-11-scenario-catalog.schema.json`
- `tools/perf/step-11-pre-release-tier-gate.json`
- `tools/perf/step-11-capacity-tier-gate.json`
- `artifacts/perf/step-11/pre-release`
- `artifacts/perf/step-11/capacity`

## Catalog 层结构

### Pre-Release Tier

- profile: `standalone.split-services.development`
- state: `template_only_pending_execution`
- gateTemplate: `tools/perf/step-11-pre-release-tier-gate.json`
- artifactRoot: `artifacts/perf/step-11/pre-release`
- evidenceSlots status: `pending_collection`

### Capacity Tier

- profile: `capacity-dedicated`
- state: `template_only_pending_execution`
- gateTemplate: `tools/perf/step-11-capacity-tier-gate.json`
- artifactRoot: `artifacts/perf/step-11/capacity`
- evidenceSlots status: `pending_collection`

## 设计约束

- `step-11-scenario-catalog.json` 必须是高阶 tier 的第一入口
- `step-11-scenario-catalog.schema.json` 不能再把 `tiers` 视为完全自由对象
- catalog 的 `artifactRoot` 只表达未来归档目录，不暗示真实结果已经存在
- gate 模板中的 `collectionSummary / evidenceSlots` 仍然保留在二跳模板里，不在本轮重复上浮
- gate 模板内部 slot metadata 仍然必须冻结：
  - `artifactPath`
  - `suggestedRelativePath`
  - `collectedAt`
  - `sizeBytes`
  - `checksumSha256`

## 非目标

- 不在本轮生成真实 `Pre-Release Tier` 指标
- 不在本轮生成真实 `Capacity Tier` 报告
- 不在本轮把完整 slot 明细复制进 catalog
