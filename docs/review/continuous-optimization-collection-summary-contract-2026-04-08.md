# Continuous Optimization - collection summary contract - 2026-04-08

## 1. 本轮背景

- 上一轮已经冻结了 `artifactFileListPath`，让 bundle 级 artifact 名单入口不再漂移。
- 但如果每次都要靠遍历 `evidenceSlots[*].status` 才能知道当前 bundle 采集完成度，operator 和后续自动化仍缺少一个稳定的机器可读摘要入口。
- 当前环境依然没有真实 `standalone.split-services.development` 发布后执行窗口，因此本轮继续不伪造真实归档结果，只把 bundle 级 `collectionSummary` 合同冻结下来。

## 2. 实际落地

### 2.1 evidence index 顶层已新增 `collectionSummary`

- 更新：`artifacts/releases/schemas/post-release-evidence-index.schema.json`
- 更新：`artifacts/releases/wave-d-2026-04-08/standalone.split-services.development-post-release-evidence-index.json`
- 当前 `collectionSummary` 已固定字段：
  - `totalSlots`
  - `requiredSlots`
  - `optionalSlots`
  - `collectedSlots`
  - `pendingSlots`
  - `skippedOptionalSlots`
- 当前模板态示例已固定为：
  - `totalSlots = 6`
  - `requiredSlots = 5`
  - `optionalSlots = 1`
  - `collectedSlots = 0`
  - `pendingSlots = 6`
  - `skippedOptionalSlots = 0`

### 2.2 operator / release 文档已同步 completeness 摘要口径

- 更新：`artifacts/releases/README.md`
- 更新：`artifacts/releases/wave-d-2026-04-08/evidence/standalone.split-services.development/README.md`
- 更新：`docs/部署/性能与灾备演练场景.md`
- 更新：`docs/部署/性能与灾备演练场景.md`
- 当前文档已明确：
  - `collectionSummary` 是 bundle 级机器可读完成度摘要
  - 模板态 bundle 的默认计数
  - 执行记录模板中建议让人工记录与 evidence index 计数保持一致，避免漂移

### 2.3 contract gate 已冻结

- 更新：`services/sdkwork-im-cloud-gateway/tests/deployment_profile_test.rs`
- 新增：
  - `test_local_default_release_bundle_freezes_collection_summary_contract`
- 当前门禁已锁定：
  - schema required 必须包含 `collectionSummary`
  - `collectionSummary` 必须定义六个整数计数字段
  - 当前 evidence index 必须给出模板态固定计数
  - artifact root README、样本文档、执行记录模板必须公开该字段

## 3. 当前判断

- release bundle evidence contract 现在已经从“bundle 级 artifact 名单入口”继续推进到“bundle 级采集完成度摘要”。
- 当前仍然没有伪造任何真实已采集证据；本轮只是把模板态 bundle 的 completeness 计数固定下来。
- 这样后续无论是人工归档还是自动化归档，都可以先用 `collectionSummary` 判断当前 bundle 到底还缺多少证据，再结合 `artifact-file-list.txt`、`checksum-manifest.txt` 和 slot metadata 做明细核对。
- 下一轮仍可继续推进：
  - 在真实发布后验证窗口回填真实 `collectedSlots / pendingSlots / skippedOptionalSlots`
  - 继续补 `collectionSummary` 与 `evidenceSlots[*].status` 的一致性门禁
  - 若继续保持模板态，则可补 bundle 级 `lastUpdatedAt` / collection snapshot contract

## 4. fresh evidence

- `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test test_local_default_release_bundle_freezes_collection_summary_contract -- --nocapture`
- `cargo fmt --all --check`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test -- --nocapture`
