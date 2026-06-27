# Continuous Optimization - evidence artifact root contract - 2026-04-08

## 1. 本轮背景

- 上一轮已经为 `standalone.split-services.development` 的 evidence index 冻结了 slot 级元数据字段：
  - `artifactPath`
  - `collectedAt`
  - `checksumSha256`
- 但如果没有一个顶层 evidence root，`artifactPath` 仍然可能在不同 bundle、不同 operator 之间各写各的相对路径。
- 当前环境依旧没有真实 `standalone.split-services.development` 发布后执行窗口，因此本轮不伪造任何真实归档文件，只先固定 artifact root contract 和模板态占位目录。

## 2. 实际落地

### 2.1 evidence index 已新增顶层 `artifactRoot`

- 更新：`artifacts/releases/schemas/post-release-evidence-index.schema.json`
- 更新：`artifacts/releases/wave-d-2026-04-08/standalone.split-services.development-post-release-evidence-index.json`
- 当前 evidence index 已固定：
  - `artifactRoot = artifacts/releases/wave-d-2026-04-08/evidence/standalone.split-services.development`
- 这让后续每个 slot 的 `artifactPath` 都有单一锚点，而不是继续依赖口头约定。

### 2.2 bundle 内已建立模板态 evidence root 占位目录

- 新增：`artifacts/releases/wave-d-2026-04-08/evidence/standalone.split-services.development/README.md`
- 当前占位目录已明确：
  - 这是 `standalone.split-services.development` post-release 真实归档证据未来应落入的根目录
  - 当前状态仍是 `template_only_pending_collection`
  - `artifactPath` 应解析到该目录之下

### 2.3 样本、模板、bundle 文档已同步

- 更新：`docs/部署/性能与灾备演练场景.md`
- 更新：`docs/部署/性能与灾备演练场景.md`
- 更新：`artifacts/releases/README.md`
- 更新：`artifacts/releases/wave-d-2026-04-08/bundle-manifest.md`
- 当前文档已明确：
  - `artifactRoot` 的固定路径
  - 证据文件后续应落在该根目录之下
  - 这只是结构化占位，不代表目录里已经存在真实证据

### 2.4 contract gate 已冻结

- 更新：`services/sdkwork-im-cloud-gateway/tests/deployment_profile_test.rs`
- 新增：
  - `test_local_default_release_bundle_freezes_evidence_artifact_root_contract`
- 当前门禁已锁定：
  - schema 必须要求 `artifactRoot`
  - evidence index 必须声明固定 root
  - artifact root 占位 README 必须存在
  - 样本、模板和 bundle manifest 都必须公开该 root

## 3. 当前判断

- release bundle evidence contract 现在已经从“slot 元数据占位”继续推进到“顶层 evidence root 边界”。
- 当前仍然没有伪造任何真实 `artifactPath` 文件，只是把真实采集应落到哪里先钉死。
- 这样后续 operator 真正回填 `artifactPath` 时，不需要再猜目录层级，也更容易做 bundle 内一致性核对。
- 下一轮仍可继续推进：
  - 在真实发布后验证窗口回填 `artifactPath`
  - 决定是否继续冻结 `sizeBytes`
  - 决定是否为 artifact root 增加 checksum manifest 或文件清单 contract

## 4. fresh evidence

- `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test test_local_default_release_bundle_freezes_evidence_artifact_root_contract -- --nocapture`
- `cargo fmt --all --check`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test -- --nocapture`
