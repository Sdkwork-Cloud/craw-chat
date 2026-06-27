> Migrated from `docs/review/continuous-optimization-collection-summary-slot-consistency-contract-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization - collection summary slot consistency contract - 2026-04-08

## 1. 本轮背景

- 上一轮已经冻结了 `collectionSummary`，让 bundle 级 completeness 摘要具备了稳定字段。
- 但如果没有明确把 `collectionSummary` 与 `evidenceSlots[*].required` / `evidenceSlots[*].status` 绑定起来，后续人工更新时仍可能出现：
  - slot 明细已经变化
  - 摘要计数没有同步更新
  - 结构化 evidence index 内部自相矛盾
- 当前环境依然没有真实 `standalone.split-services.development` 发布后执行窗口，因此本轮继续不伪造真实证据，只把“摘要必须由 slot 明细推导”固定成回归门禁与文档合同。

## 2. 实际落地

### 2.1 contract gate 已冻结 collectionSummary 与 slot 明细一致性

- 更新：`services/sdkwork-im-cloud-gateway/tests/deployment_profile_test.rs`
- 新增：
  - `test_local_default_release_bundle_collection_summary_matches_slot_statuses`
- 当前门禁已锁定：
  - `collectionSummary.totalSlots == evidenceSlots.len()`
  - `requiredSlots / optionalSlots` 必须由 `evidenceSlots[*].required` 推导
  - `collectedSlots / pendingSlots / skippedOptionalSlots` 必须由 `evidenceSlots[*].status` 推导

### 2.2 operator / release 文档已明确 source-of-truth 关系

- 更新：`artifacts/releases/README.md`
- 更新：`artifacts/releases/wave-d-2026-04-08/evidence/standalone.split-services.development/README.md`
- 更新：`docs/部署/性能与灾备演练场景.md`
- 更新：`docs/部署/性能与灾备演练场景.md`
- 当前文档已明确：
  - `collectionSummary` 不是独立人工计数源
  - 必须直接由 `evidenceSlots[*].required` 与 `evidenceSlots[*].status` 推导
  - 更新 slot 状态时，摘要计数必须同步回写

## 3. 当前判断

- release bundle evidence contract 现在已经从“有 completeness 摘要”继续推进到“摘要与 slot 明细的一致性门禁”。
- 当前仍然没有伪造任何真实采集状态；本轮只是把模板态 bundle 内部的一致性关系固定下来。
- 这样后续无论是人工 operator 更新，还是自动化归档更新，都不能再让 `collectionSummary` 和 `evidenceSlots` 分别演化。
- 下一轮仍可继续推进：
  - 在真实发布后验证窗口回填真实 `collectedSlots / pendingSlots / skippedOptionalSlots`
  - 继续补 `lastUpdatedAt` 或 collection snapshot 时间戳 contract
  - 若继续保持模板态，则继续冻结 bundle 级更新时间与采集批次边界

## 4. fresh evidence

- `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test test_local_default_release_bundle_collection_summary_matches_slot_statuses -- --nocapture`
- `cargo fmt --all --check`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test -- --nocapture`

