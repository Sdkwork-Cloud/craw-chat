> Migrated from `docs/review/continuous-optimization-checksum-manifest-contract-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization - checksum manifest contract - 2026-04-08

## 1. 本轮背景

- 上一轮已经冻结了 `sizeBytes`，让每个 evidence slot 至少具备文件大小占位。
- 但如果没有 bundle 级 checksum manifest 入口，后续真实归档时仍需要在：
  - 每个 slot 的 `checksumSha256`
  - evidence root 下的汇总校验和文件
  之间临时决定谁是权威入口。
- 当前环境依然没有真实 `standalone.split-services.development` 发布后执行窗口，因此本轮继续不伪造真实 digest，只把 bundle 级 checksum manifest 路径与格式合同冻结下来。

## 2. 实际落地

### 2.1 evidence index 顶层已新增 `checksumManifestPath`

- 更新：`artifacts/releases/schemas/post-release-evidence-index.schema.json`
- 更新：`artifacts/releases/wave-d-2026-04-08/standalone.split-services.development-post-release-evidence-index.json`
- 当前 contract 已固定：
  - `checksumManifestPath`
  - 路径值：`artifacts/releases/wave-d-2026-04-08/evidence/standalone.split-services.development/checksum-manifest.txt`
- 这意味着未来真实归档时，不再需要临时决定 bundle 级 checksum 清单放到哪里。

### 2.2 evidence root 已新增 checksum manifest 占位文件

- 新增：`artifacts/releases/wave-d-2026-04-08/evidence/standalone.split-services.development/checksum-manifest.txt`
- 当前占位文件已明确：
  - `template_only_pending_collection`
  - `path owner: checksumManifestPath`
  - 行格式：`sha256:<digest>  <suggestedRelativePath>`
- 当前仍是模板态说明，不代表真实校验和已经采集完成。

### 2.3 operator / release 文档已同步 checksum manifest 口径

- 更新：`artifacts/releases/README.md`
- 更新：`artifacts/releases/wave-d-2026-04-08/evidence/standalone.split-services.development/README.md`
- 更新：`docs/部署/性能与灾备演练场景.md`
- 更新：`docs/部署/性能与灾备演练场景.md`
- 当前文档已明确：
  - `checksumManifestPath` 是 bundle 级 checksum 汇总入口
  - `checksum-manifest.txt` 推荐按 `sha256:<digest>  <suggestedRelativePath>` 逐行记录
  - 当前模板态只冻结路径和格式，不伪造真实 digest

### 2.4 contract gate 已冻结

- 更新：`services/sdkwork-im-cloud-gateway/tests/deployment_profile_test.rs`
- 新增：
  - `test_local_default_release_bundle_freezes_checksum_manifest_contract`
- 当前门禁已锁定：
  - schema required 必须包含 `checksumManifestPath`
  - evidence index 必须暴露固定 checksum manifest 路径
  - checksum manifest 文件必须真实存在于 bundle 中
  - artifact root README、样本文档、执行记录模板与 bundle manifest 都必须公开该入口

## 3. 当前判断

- release bundle evidence contract 现在已经从“slot 级 checksum 字段”继续推进到“bundle 级 checksum 汇总入口”。
- 当前仍然没有伪造任何真实 digest；本轮只是把未来真实归档时的汇总文件位置和格式固定下来。
- 这样后续无论是人工归档还是自动化归档，都可以把单 slot `checksumSha256` 与 bundle 级 `checksum-manifest.txt` 对齐到同一套命名规则。
- 下一轮仍可继续推进：
  - 在真实发布后验证窗口回填 `checksumSha256` 与 `checksum-manifest.txt`
  - 继续冻结 artifact file list contract
  - 若真实证据落盘，再补 evidence index 与 checksum manifest 的一致性核对

## 4. fresh evidence

- `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test test_local_default_release_bundle_freezes_checksum_manifest_contract -- --nocapture`
- `cargo fmt --all --check`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test -- --nocapture`

