# Continuous Optimization - SDK leaf READMEs release boundary - 2026-04-08

## 1. 本轮背景

- `sdk-release-catalog.json` 与对应 schema 已经把 bundle 级 SDK 目录真源冻结下来。
- 但四个叶子 SDK README 仍只描述职责与约束，没有把当前发布边界公开给具体语言入口的读者。
- 这会留下一个真实断层：
  - workspace 总入口知道 catalog
  - release bundle 知道 catalog
  - 叶子目录入口却看不到当前模板态与未发布状态

## 2. 实际落地

### 2.1 新增叶子 README contract gate

- 更新：`tools/chat-cli/tests/chat_cli_contract_test.rs`
- 新增：
  - `test_continuous_optimization_sdk_leaf_readmes_freeze_release_catalog_boundary`
- 当前门禁会冻结四个叶子 README 都必须包含：
  - `sdk-release-catalog.json`
  - `template_only_pending_generation`
  - `not_published`
  - `artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`

### 2.2 四个叶子 README 已对齐 bundle 真源

- 更新：`sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/README.md`
- 更新：`sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/README.md`
- 更新：`sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/README.md`
- 更新：`sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/README.md`
- 当前每个 README 都已显式说明：
  - 当前 bundle 级发布目录真源
  - 当前仍为 `template_only_pending_generation`
  - 当前仍为 `not_published`

### 2.3 bundle 归档证据已补回链

- 更新：`artifacts/releases/wave-d-2026-04-08/bundle-manifest.md`
- 当前 SDK release 连续优化的三份 review 已一起进入 bundle evidence list：
  - release catalog
  - schema contract
  - leaf README release boundary

## 3. 当前判断

- SDK 发布边界现在不再只停留在总入口和 release bundle，而是下沉到了具体语言目录入口。
- 当前实现仍是模板态发布边界，不代表真实 SDK 生成与发包链路已经完成。
- 本轮的关键决策是让叶子 README 只回链 bundle 真源，而不让它们各自维护第二套发布状态。

## 4. fresh evidence

- `cargo fmt --all --check`
- `cargo test -p sdkwork-im-cli --offline test_continuous_optimization_sdk_leaf_readmes_freeze_release_catalog_boundary -- --exact --nocapture`
- `cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test -- --nocapture`
