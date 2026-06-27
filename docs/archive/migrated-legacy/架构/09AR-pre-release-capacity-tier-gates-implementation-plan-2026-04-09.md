# 09AR - Pre-Release / Capacity Tier gates implementation plan

## 目标

继续在 Step 11 高阶 gate 机器契约上补最小缺口，使 `step-11-scenario-catalog.json` 本身就能暴露高阶 tier 的 gate 模板和未来证据归档根目录。

## 实施范围

- 更新 `tools/perf/step-11-scenario-catalog.json`
- 更新 `tools/perf/schemas/step-11-scenario-catalog.schema.json`
- 在 operator 文档中公开 catalog 层 `artifactRoot` 回链
- 在 step / review / architecture 回写中说明 catalog 已直接暴露高阶 tier 证据根目录

## 实施规则

- `template_only_pending_execution` 仍然只表示 gate 模板已冻结
- `pending_collection` 仍然只表示证据槽位待采集
- `standalone.split-services.development` 继续作为 `Pre-Release Tier` 的默认 profile
- `capacity-dedicated` 继续作为 `Capacity Tier` 的目标环境名
- catalog 层 `artifactRoot` 只冻结未来归档根目录，不代表目录中已有真实产物

## catalog 回链模板

- `Pre-Release Tier`
  - `gateTemplate = tools/perf/step-11-pre-release-tier-gate.json`
  - `profile = standalone.split-services.development`
  - `artifactRoot = artifacts/perf/step-11/pre-release`
- `Capacity Tier`
  - `gateTemplate = tools/perf/step-11-capacity-tier-gate.json`
  - `profile = capacity-dedicated`
  - `artifactRoot = artifacts/perf/step-11/capacity`
- `step-11-scenario-catalog.schema.json`
  - `tiers[*]` 必须显式定义 `gateTemplate`
  - `tiers[*]` 必须显式定义 `artifactRoot`
- gate 模板内部 contract 仍然必须保留：
  - `collectionSummary`
  - `evidenceSlots`
  - `artifactPath`
  - `suggestedRelativePath`
  - `collectedAt`
  - `sizeBytes`
  - `checksumSha256`

## 输出

- step doc: `docs/step/continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md`
- review doc: `docs/review/continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md`
- design doc: `docs/架构/150AR-pre-release-capacity-tier-gates-design-2026-04-09.md`
