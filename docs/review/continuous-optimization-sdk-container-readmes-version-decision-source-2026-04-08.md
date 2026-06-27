# Continuous Optimization - SDK container READMEs version decision source - 2026-04-08

## 1. 本轮背景

- 上一轮已经把 `versionDecisionSourcePath = null` 下沉到四个叶子 README。
- 但 app/admin 两个容器 README 还没有同步这条占位语义。
- 这会导致 SDK release 导航链在中间层再次出现字段断层。

## 2. 实际落地

### 2.1 contract gate 已新增

- 更新：`tools/chat-cli/tests/chat_cli_contract_test.rs`
- 新增：
  - `test_continuous_optimization_sdk_container_readmes_freeze_version_decision_source_boundary`
- 当前门禁会冻结两份容器 README 都必须包含：
  - `versionDecisionSourcePath`
  - `null`

### 2.2 两份容器 README 已对齐

- 更新：`sdks/sdkwork-im-sdk/README.md`
- 更新：`sdks/sdkwork-control-plane-sdk/README.md`
- 现在两个容器 README 都显式表达：
  - `plannedVersion = null`
  - `versionStatus = version_unassigned_pending_freeze`
  - `versionDecisionSourcePath = null`

### 2.3 bundle evidence 已同步

- 更新：`artifacts/releases/wave-d-2026-04-08/bundle-manifest.md`
- 当前 SDK release decision-source evidence 已覆盖：
  - bundle contract
  - 容器 README
  - 叶子 README

## 3. 当前判断

- SDK release 链关于 version decision source 的导航表达已经贯通，不再出现 root/leaf 有、中间容器层缺失的问题。
- 当前实现仍然不代表真实 freeze evidence 或真实版本号已经存在。
- 本轮关键决策仍然是不伪造路径，而是把 `null` 占位沿整个导航链完全公开。

## 4. fresh evidence

- `cargo fmt --all --check`
- `cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test test_continuous_optimization_sdk_container_readmes_freeze_version_decision_source_boundary -- --nocapture`
- `cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test -- --nocapture`
