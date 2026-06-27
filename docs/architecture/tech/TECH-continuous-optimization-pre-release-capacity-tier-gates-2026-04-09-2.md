> Migrated from `docs/step/continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 持续优化 - Pre-Release / Capacity Tier 量化门禁模板

## 本轮为何做

上一轮已经把 `Pre-Release Tier` 与 `Capacity Tier` 的 gate 模板补到了 `artifactRoot + collectionSummary + evidenceSlots`。当前最小剩余缺口是 `tools/perf/step-11-scenario-catalog.json` 还没有直接把这两个高阶 tier 的 `artifactRoot` 上浮出来，自动化仍然要先读 catalog、再跳到 gate 模板才能知道未来证据根目录。

## 本轮完成

- 更新 `tools/perf/step-11-scenario-catalog.json`
- 更新 `tools/perf/schemas/step-11-scenario-catalog.schema.json`
- 更新 `docs/部署/性能与灾备演练场景.md`
- 回写 step / review / architecture 文档

## 冻结口径

- `tools/perf/step-11-scenario-catalog.json` 现在直接暴露：
  - `gateTemplate`
  - `artifactRoot`
- `Pre-Release Tier`
  - `gateTemplate = tools/perf/step-11-pre-release-tier-gate.json`
  - `profile = standalone.split-services.development`
  - `artifactRoot = artifacts/perf/step-11/pre-release`
- `Capacity Tier`
  - `gateTemplate = tools/perf/step-11-capacity-tier-gate.json`
  - `profile = capacity-dedicated`
  - `artifactRoot = artifacts/perf/step-11/capacity`
- gate 总状态仍然是 `template_only_pending_execution`
- 证据槽位状态仍然是 `pending_collection`
- gate 模板内部仍然保留：
  - `collectionSummary`
  - `evidenceSlots`
  - `artifactPath`
  - `suggestedRelativePath`
  - `collectedAt`
  - `sizeBytes`
  - `checksumSha256`

## 边界

- 这是 catalog 层的回链补齐，不是新结果。
- 不宣称仓库已经拥有真实 `Pre-Release Tier` 或 `Capacity Tier` 量化结论。
- `artifactRoot` 只表达未来归档根目录，不表达该目录下已经存在真实 artifact。

## 下一步

- 如需继续缩短自动化解析路径，可再把高阶 tier 的最小摘要字段继续上浮到 catalog
- 如环境允许，下一轮更有价值的是开始回填一轮真实 `standalone.split-services.development` 的 `Pre-Release Tier` 样本
## 2026-04-09 Addendum

- This gap is now closed.
- step-11-scenario-catalog.json already exposes tier-level gateTemplate and artifactRoot for Pre-Release Tier and Capacity Tier.
- `step-11-scenario-catalog.json` already exposes `gateTemplate` and `artifactRoot` for `Pre-Release Tier` and `Capacity Tier`.
- Any earlier note in this doc that says the catalog still lacks tier-level artifactRoot is stale and superseded by this addendum.
- The repo now also materializes `artifacts/perf/step-11/pre-release` and `artifacts/perf/step-11/capacity` with artifact-root README guidance.
- The repo now co-locates machine-readable tier evidence indexes under both high-tier artifact roots.
- The remaining gap is no longer catalog discoverability; it is still the absence of real collected `Pre-Release Tier` and `Capacity Tier` evidence.
## 2026-04-09 Addendum 2

- `Pre-Release Tier` now materializes one real collected `failover` artifact at `artifacts/perf/step-11/pre-release/failover/drill.json`.
- `pre-release-tier-evidence-index.json` now advances to `evidence_partially_collected` with `collectedSlots = 1` and `pendingSlots = 6`.
- This backfill is limited to the published CP11-3 local failover evidence.
- `Capacity Tier` is still `template_only_pending_execution`.

