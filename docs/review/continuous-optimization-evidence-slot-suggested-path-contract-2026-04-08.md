# Continuous Optimization - evidence slot suggested path contract - 2026-04-08

## 1. 本轮背景

- 上一轮已经冻结了 `artifactRoot`，明确了真实归档证据应落入 bundle 内哪个根目录。
- 但如果每个 slot 没有稳定的默认相对路径，后续 `artifactPath` 仍可能在不同执行人、不同 bundle 之间继续漂移。
- 当前环境依然没有真实 `standalone.split-services.development` 发布后执行窗口，因此本轮继续不伪造真实文件，而是把默认命名规则先冻结成 contract。

## 2. 实际落地

### 2.1 每个 evidence slot 已新增 `suggestedRelativePath`

- 更新：`artifacts/releases/schemas/post-release-evidence-index.schema.json`
- 更新：`artifacts/releases/wave-d-2026-04-08/standalone.split-services.development-post-release-evidence-index.json`
- 当前每个 slot 都已固定默认命名规则，例如：
  - `retired-lifecycle-deploy/retired-lifecycle-deploy.ps1.log`
  - `retired-lifecycle-status/retired-lifecycle-status.ps1.txt`
  - `smoke/local_stack_smoke.ps1.txt`
  - `open-chat-test/open-chat-test.ps1.md`
  - `inspect-runtime/inspect-runtime-local.ps1.txt`
  - `screenshots/runtime-window.png`

### 2.2 artifact root 占位文档已说明默认命名规则

- 更新：`artifacts/releases/wave-d-2026-04-08/evidence/standalone.split-services.development/README.md`
- 当前已明确：
  - `artifactPath` 如无特殊原因，应优先取 `artifactRoot + "/" + suggestedRelativePath`
  - 当前命名规则仍处于模板态，用于冻结未来真实归档路径，而不是声称这些文件已经存在

### 2.3 operator 文档已同步 naming contract

- 更新：`docs/部署/性能与灾备演练场景.md`
- 更新：`docs/部署/性能与灾备演练场景.md`
- 当前样本与模板都已明确：
  - `suggestedRelativePath` 是 evidence slot 的默认归档命名
  - 若 `artifactPath` 偏离默认命名，需要记录原因

### 2.4 contract gate 已冻结

- 更新：`services/sdkwork-im-cloud-gateway/tests/deployment_profile_test.rs`
- 新增：
  - `test_local_default_release_bundle_freezes_evidence_slot_suggested_relative_path_contract`
- 当前门禁已锁定：
  - schema 必须定义 `suggestedRelativePath`
  - 每个 slot 都必须给出非空、使用 `/` 的相对路径
  - artifact root README、样本文档、执行记录模板都必须公开该字段

## 3. 当前判断

- release bundle evidence contract 现在已经从“有 artifact root”继续推进到“每个 slot 有默认命名规则”。
- 当前仍然没有伪造任何真实 `artifactPath` 文件；本轮只是让未来真实归档时不再各自发明文件名。
- 这样后续如果拿到真实 `standalone.split-services.development` 发布后窗口，operator 可以直接按 `suggestedRelativePath` 回填，而不需要重新决定目录与命名策略。
- 下一轮仍可继续推进：
  - 在真实窗口把 `artifactPath` 回填到这些默认相对路径
  - 决定是否补 `sizeBytes`
  - 决定是否为 artifact root 增加 checksum manifest / file list contract

## 4. fresh evidence

- `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test test_local_default_release_bundle_freezes_evidence_slot_suggested_relative_path_contract -- --nocapture`
- `cargo fmt --all --check`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test deployment_profile_test -- --nocapture`
