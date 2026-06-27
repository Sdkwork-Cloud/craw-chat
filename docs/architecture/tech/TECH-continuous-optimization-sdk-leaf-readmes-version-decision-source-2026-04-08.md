> Migrated from `docs/review/continuous-optimization-sdk-leaf-readmes-version-decision-source-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization - SDK leaf READMEs version decision source - 2026-04-08

## 1. 本轮背景

- 上一轮已经把 `versionDecisionSourcePath = null` 冻结到 release catalog 和 root release docs。
- 但四个叶子 README 还没有继续公开这条占位语义。
- 这会导致具体语言消费者在最接近使用的位置，看不到当前版本决议来源仍未分配。

## 2. 实际落地

### 2.1 contract gate 已新增

- 更新：`tools/chat-cli/tests/chat_cli_contract_test.rs`
- 新增：
  - `test_continuous_optimization_sdk_leaf_readmes_freeze_version_decision_source_boundary`
- 当前门禁会冻结四个叶子 README 都必须包含：
  - `versionDecisionSourcePath`
  - `null`

### 2.2 四个叶子 README 已对齐

- 更新：`sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/README.md`
- 更新：`sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/README.md`
- 更新：`sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-typescript/README.md`
- 更新：`sdks/sdkwork-control-plane-sdk/sdkwork-control-plane-sdk-flutter/README.md`
- 现在四个叶子 README 都显式表达：
  - `plannedVersion = null`
  - `versionStatus = version_unassigned_pending_freeze`
  - `versionDecisionSourcePath = null`

### 2.3 bundle evidence 已同步

- 更新：`artifacts/releases/wave-d-2026-04-08/bundle-manifest.md`
- 当前 SDK release decision-source evidence 已覆盖：
  - bundle contract
  - 叶子 README

## 3. 当前判断

- 版本决议来源占位语义现在已经下沉到具体语言叶子入口，不再只有 bundle 和总入口文档可见。
- 当前实现仍然不代表真实 freeze evidence 或真实版本号已经存在。
- 本轮关键决策仍然是不伪造路径，而是把 `null` 占位沿导航链继续公开。

## 4. fresh evidence

- `cargo fmt --all --check`
- `cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test test_continuous_optimization_sdk_leaf_readmes_freeze_version_decision_source_boundary -- --nocapture`
- `cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test -- --nocapture`

