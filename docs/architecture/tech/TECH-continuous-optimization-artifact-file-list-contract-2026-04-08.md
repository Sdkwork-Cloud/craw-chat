> Migrated from `docs/review/continuous-optimization-artifact-file-list-contract-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization - artifact file list contract - 2026-04-08

## 1. 本轮背景

- 上一轮已经冻结了 `checksumManifestPath`，让 bundle 级 checksum 汇总入口不再漂移。
- 但如果没有 bundle 级 artifact file list，后续真实归档时仍然缺少一个稳定的“本次 bundle 预期应有哪些文件”的清单入口。
- 当前环境依然没有真实 `standalone.split-services.development` 发布后执行窗口，因此本轮继续不伪造真实证据，只把 artifact file list 的路径和默认内容合同冻结下来。

## 2. 实际落地

### 2.1 evidence index 顶层已新增 `artifactFileListPath`

- 更新：`artifacts/releases/schemas/post-release-evidence-index.schema.json`
- 更新：`artifacts/releases/wave-d-2026-04-08/standalone.split-services.development-post-release-evidence-index.json`
- 当前 contract 已固定：
  - `artifactFileListPath`
  - 路径值：`artifacts/releases/wave-d-2026-04-08/evidence/standalone.split-services.development/artifact-file-list.txt`
- 这意味着未来真实归档时，不再需要临时决定 bundle 级 artifact 清单放到哪里。

### 2.2 evidence root 已新增 artifact file list 占位文件

- 新增：`artifacts/releases/wave-d-2026-04-08/evidence/standalone.split-services.development/artifact-file-list.txt`
- 当前占位文件已明确：
  - `template_only_pending_collection`
  - `path owner: artifactFileListPath`
  - 默认 entries 直接沿用各 slot 的 `suggestedRelativePath`
- 当前仍是模板态说明，不代表这些文件已经在 bundle 中真实采集完成。

### 2.3 operator / release 文档已同步 artifact file list 口径

- 更新：`artifacts/releases/README.md`
- 更新：`artifacts/releases/wave-d-2026-04-08/evidence/standalone.split-services.development/README.md`
- 更新：`docs/部署/性能与灾备演练场景.md`
- 更新：`docs/部署/性能与灾备演练场景.md`
- 当前文档已明确：
  - `artifactFileListPath` 是 bundle 级 artifact 名单入口
  - `artifact-file-list.txt` 推荐直接沿用 `suggestedRelativePath`
  - 当前模板态只冻结路径和默认内容，不伪造真实落盘结果

### 2.4 contract gate 已冻结

- 更新：`services/sdkwork-im-cloud-gateway/tests/deployment_profile_test.rs`
- 新增：
  - `test_local_default_release_bundle_freezes_artifact_file_list_contract`
- 当前门禁已锁定：
  - schema required 必须包含 `artifactFileListPath`
  - evidence index 必须暴露固定 artifact file list 路径
  - artifact file list 文件必须真实存在于 bundle 中
  - artifact root README、样本文档、执行记录模板与 bundle manifest 都必须公开该入口

## 3. 当前判断

- release bundle evidence contract 现在已经从“bundle 级 checksum 汇总入口”继续推进到“bundle 级 artifact 名单入口”。
- 当前仍然没有伪造任何真实归档文件；本轮只是把未来真实发布后证据采集时的默认文件清单固定下来。
- 这样后续无论是人工归档还是自动化归档，都可以先按 `artifact-file-list.txt` 识别“本次 bundle 期望有哪些文件”，再用 `checksum-manifest.txt` 和单 slot metadata 做一致性核对。
- 下一轮仍可继续推进：
  - 在真实发布后验证窗口回填真实 artifact 与 digest
  - 继续补 evidence index 与 artifact file list / checksum manifest 的一致性核对
  - 若继续保持模板态，则优先补 bundle 级 collection status / completeness contract

## 4. fresh evidence

- `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test test_local_default_release_bundle_freezes_artifact_file_list_contract -- --nocapture`
- `cargo fmt --all --check`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test -- --nocapture`

