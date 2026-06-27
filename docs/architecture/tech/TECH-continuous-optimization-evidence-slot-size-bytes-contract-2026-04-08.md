> Migrated from `docs/review/continuous-optimization-evidence-slot-size-bytes-contract-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization - evidence slot size bytes contract - 2026-04-08

## 1. 本轮背景

- 上一轮已经冻结了 `suggestedRelativePath`，让每个 evidence slot 都有稳定的默认归档命名。
- 但如果缺少文件大小字段，后续真实归档时仍无法快速核对：
  - 证据文件是否已经真正落盘
  - 归档物是否被截断
  - 不同执行窗口之间是否拿到了同一份文件
- 当前环境依然没有真实 `standalone.split-services.development` 发布后执行窗口，因此本轮继续不伪造真实证据文件，而是先把 `sizeBytes` 固定为模板态 contract。

## 2. 实际落地

### 2.1 evidence slot schema 已新增 `sizeBytes`

- 更新：`artifacts/releases/schemas/post-release-evidence-index.schema.json`
- 当前 `evidenceSlots[*]` 已新增：
  - `sizeBytes`
- 合同要求已固定为：
  - 类型允许 `integer | null`
  - 最小值为 `0`
- 这意味着：
  - 模板态 bundle 可先保留 `null`
  - 一旦真实证据落盘，就必须回填明确的字节数

### 2.2 当前 evidence index 已显式保留模板态 `sizeBytes`

- 更新：`artifacts/releases/wave-d-2026-04-08/standalone.split-services.development-post-release-evidence-index.json`
- 当前每个 slot 都已显式携带：
  - `sizeBytes: null`
- 这延续了前几轮的边界决策：
  - 不伪造真实采集结果
  - 只把未来真实归档必须补齐的元数据结构先冻结下来

### 2.3 operator / release 文档已同步 size contract

- 更新：`docs/部署/性能与灾备演练场景.md`
- 更新：`docs/部署/性能与灾备演练场景.md`
- 更新：`artifacts/releases/README.md`
- 更新：`artifacts/releases/wave-d-2026-04-08/evidence/standalone.split-services.development/README.md`
- 当前文档已明确：
  - `sizeBytes` 用于记录归档文件的实际字节数
  - 当前模板态 evidence index 统一保留 `null`
  - 真实采集完成后再回填具体大小

### 2.4 contract gate 已冻结

- 更新：`services/sdkwork-im-cloud-gateway/tests/deployment_profile_test.rs`
- 新增：
  - `test_local_default_release_bundle_freezes_evidence_slot_size_bytes_contract`
- 当前回归门禁已锁定：
  - schema 必须定义 `sizeBytes`
  - 每个 slot 必须显式暴露 `sizeBytes`
  - 当前模板态 slot 的 `sizeBytes` 必须为 `null`
  - artifact root README、样本文档、执行记录模板必须公开该字段

## 3. 当前判断

- release bundle evidence contract 现在已经从“命名规则固定”继续推进到“文件大小元数据固定”。
- 当前仍然没有伪造任何真实 `sizeBytes` 数值；本轮只是把未来真实证据落盘后的回填口径冻结下来。
- 这样后续无论是 operator 手工归档，还是后面接自动化归档，都能沿用同一字段记录实际字节数，而不是各自定义本地字段。
- 下一轮仍可继续推进：
  - 在真实发布后验证窗口回填 `artifactPath / collectedAt / sizeBytes / checksumSha256`
  - 决定是否继续冻结 checksum manifest / artifact file list contract
  - 若出现真实归档产物，再补证据索引与 artifact root 目录的一致性核对

## 4. fresh evidence

- `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test test_local_default_release_bundle_freezes_evidence_slot_size_bytes_contract -- --nocapture`
- `cargo fmt --all --check`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test -- --nocapture`

