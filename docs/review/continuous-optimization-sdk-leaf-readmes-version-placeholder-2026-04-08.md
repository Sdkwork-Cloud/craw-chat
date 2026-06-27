# Continuous Optimization - SDK leaf READMEs version placeholder - 2026-04-08

## 1. 本轮背景

- 上一轮已经把 SDK 版本未冻结状态冻结成 bundle 级 contract：
  - `plannedVersion = null`
  - `versionStatus = version_unassigned_pending_freeze`
- 但四个叶子 SDK README 仍然没有把这组边界带到具体语言入口。

## 2. 实际落地

### 2.1 contract gate 已新增

- 更新：`tools/chat-cli/tests/chat_cli_contract_test.rs`
- 新增：
  - `test_continuous_optimization_sdk_leaf_readmes_freeze_version_placeholder_boundary`
- 当前门禁会冻结四个叶子 README 都必须包含：
  - `plannedVersion`
  - `null`
  - `versionStatus`
  - `version_unassigned_pending_freeze`

### 2.2 四个叶子 README 已对齐版本占位状态

- 更新：`sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/README.md`
- 更新：`sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/README.md`
- 更新：`sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/README.md`
- 更新：`sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/README.md`
- 当前每个叶子 README 都已显式公开：
  - 当前没有计划版本号
  - 当前仍待版本冻结

### 2.3 bundle 证据链已补齐

- `artifacts/releases/wave-d-2026-04-08/bundle-manifest.md` 已挂载该 review 证据
- 当前 SDK release 连续优化的版本链条现已包含：
  - bundle version placeholder contract
  - leaf README version placeholder boundary

## 3. 当前判断

- SDK 发布链现在不仅在 bundle/root 文档层表达了“版本未冻结”，也在具体语言入口层表达了同一事实。
- 当前实现仍不代表真实版本 freeze、真实生成或真实发包已经完成。
- 本轮的关键决策是让叶子 README 只公开 placeholder，而不伪造 semver。

## 4. fresh evidence

- `cargo fmt --all --check`
- `cargo test -p sdkwork-im-cli --offline test_continuous_optimization_sdk_leaf_readmes_freeze_version_placeholder_boundary -- --exact --nocapture`
- `cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test -- --nocapture`
