# Continuous Optimization - evidence slot metadata contract - 2026-04-08

## 1. 本轮背景

- 上一轮已经把 `standalone.split-services.development` 的 release bundle evidence index 冻结成 machine-readable JSON 与 schema contract。
- 但 slot 级别仍然只有：
  - `id`
  - `kind`
  - `required`
  - `status`
  - `command`
- 这仍不足以支撑后续真实归档，因为 operator 真正采集完证据后，还需要知道：
  - 证据文件放在哪里
  - 何时采集
  - 当前归档物的校验和是什么
- 当前环境依然没有真实 `standalone.split-services.development` 发布后执行窗口，因此本轮继续不伪造采集结果，只冻结 slot metadata contract。

## 2. 实际落地

### 2.1 evidence slot 已新增可空元数据字段

- 更新：`artifacts/releases/schemas/post-release-evidence-index.schema.json`
- 当前 `evidenceSlots[*]` 已新增可空字段：
  - `artifactPath`
  - `collectedAt`
  - `checksumSha256`
- 这三个字段当前都允许 `null`，用于明确：
  - 模板态 bundle 可以先占位
  - 真实采集完成后再回填具体值

### 2.2 当前 evidence index 已显式保留模板态占位

- 更新：`artifacts/releases/wave-d-2026-04-08/standalone.split-services.development-post-release-evidence-index.json`
- 当前每个 slot 都已显式携带：
  - `artifactPath: null`
  - `collectedAt: null`
  - `checksumSha256: null`
- 这样后续真实 operator 执行不会再自己猜“该补哪些元数据”，而是直接沿用同一结构回填。

### 2.3 operator 文档已同步说明如何回填

- 更新：`docs/部署/性能与灾备演练场景.md`
- 更新：`docs/部署/性能与灾备演练场景.md`
- 当前文档已经明确：
  - 建议采集证据时同步补 `artifactPath / collectedAt / checksumSha256`
  - 执行记录模板里也有同名字段可直接记录

### 2.4 contract gate 已冻结

- 更新：`services/sdkwork-im-cloud-gateway/tests/deployment_profile_test.rs`
- 新增：
  - `test_local_default_release_bundle_freezes_evidence_slot_collection_metadata_contract`
- 当前回归门禁已经锁定：
  - schema 必须定义三组 slot metadata 字段
  - 当前模板态 evidence index 必须显式保留三组 `null` 占位
  - 样本文档与执行记录模板必须公开这三组字段

## 3. 当前判断

- release bundle evidence index 现在已经不只是“有哪些证据槽位”，还开始明确“证据采集后要记录哪些归档元数据”。
- 当前状态仍然是 template-only，不代表真实证据已经采集完成。
- 本轮最重要的决策仍然是不伪造任何真实值，而是把未来必填元数据先固定下来，避免后续 bundle 结构继续漂移。
- 下一轮仍可继续推进：
  - 在真实发布后验证窗口回填 `artifactPath / collectedAt / checksumSha256`
  - 继续决定是否需要补 `sizeBytes`、归档根目录或 checksum 清单
  - 若有真实采集物，再把 schema-backed evidence index 与归档目录进行一致性核对

## 4. fresh evidence

- `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test test_local_default_release_bundle_freezes_evidence_slot_collection_metadata_contract -- --nocapture`
- `cargo fmt --all --check`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test -- --nocapture`
