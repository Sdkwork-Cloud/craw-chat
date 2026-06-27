> Migrated from `docs/step/13-CP13-1E-sdk叶子README版本占位收敛-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 13-CP13-1E - SDK 叶子 README 版本占位收敛

## 本轮目的

`plannedVersion / versionStatus` 版本占位 contract，从 bundle workspace 总入口继续下沉到四个叶子 SDK README，避免具体语言入口看不到“版本尚未冻结”的真实边界：

## 发现的问

- bundle `sdk-release-catalog.json` 已明确：
  - `plannedVersion = null`
  - `versionStatus = version_unassigned_pending_freeze`
- `sdks/README.md`、`artifacts/releases/README.md`、`bundle-manifest.md` 也已对齐这组边界
- 但四个叶SDK README 还未显式说明版本占位状态
- 结果是：总入口知道版本未冻结，具体语言入口仍可能被误读为“只是没写版本

## 本轮决策

- 继续使用 `tools/chat-cli/tests/chat_cli_contract_test.rs` contract gate
- 新增 `test_continuous_optimization_sdk_leaf_readmes_freeze_version_placeholder_boundary`
- 最小要求：
  - 四个叶子 README 都必须公开
    - `plannedVersion = null`
    - `versionStatus = version_unassigned_pending_freeze`

## 实施结果

- 更新
  - `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/README.md`
  - `sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/README.md`
  - `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/README.md`
  - `sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/README.md`
- 四个叶子 README 现已同时具备
  - bundle 级发布目录真源边
  - 版本未冻结边

## 验证

- 红灯
  - `cargo test -p sdkwork-im-cli --offline test_continuous_optimization_sdk_leaf_readmes_freeze_version_placeholder_boundary -- --exact --nocapture`
  - 失败点：`app TypeScript README must contain version placeholder boundary text plannedVersion`
- 绿灯
  - `cargo fmt --all --check`
  - `cargo test -p sdkwork-im-cli --offline test_continuous_optimization_sdk_leaf_readmes_freeze_version_placeholder_boundary -- --exact --nocapture`
  - `cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test -- --nocapture`

## 下一轮建

- 若继续推SDK 发布链，下一条真gap 不再是“版本占位是否公开”，而是
  - 真实版本 freeze 决议来源
  - 真实生成输入
  - 真实发包归档

