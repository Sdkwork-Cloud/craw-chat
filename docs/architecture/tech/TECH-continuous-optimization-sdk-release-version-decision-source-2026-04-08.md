> Migrated from `docs/review/continuous-optimization-sdk-release-version-decision-source-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization - SDK release version decision source - 2026-04-08

## 1. 本轮背景

- 上一轮已经把 `plannedVersion / versionStatus` 冻结为 machine-readable contract。
- 但 release catalog 还没有一个稳定字段表达“未来版本 freeze 决议来源路径”。
- 这会导致后续真实 freeze evidence 即使出现，也缺少统一的挂载字段。

## 2. 实际落地

### 2.1 contract gate 已新增

- 更新：`tools/chat-cli/tests/chat_cli_contract_test.rs`
- 新增：
  - `test_continuous_optimization_sdk_release_catalog_freezes_version_decision_source_contract`
- 当前门禁会冻结：
  - schema 必须定义 `versionDecisionSourcePath`
  - catalog 中每个 artifact 当前都必须是 `versionDecisionSourcePath = null`
  - root release docs 必须同步该字段

### 2.2 schema 与 catalog 已对齐

- 更新：`artifacts/releases/schemas/sdk-release-catalog.schema.json`
- 更新：`artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`
- 现在每个 SDK artifact 都同时保留：
  - `plannedVersion = null`
  - `versionStatus = version_unassigned_pending_freeze`
  - `versionDecisionSourcePath = null`

### 2.3 root release docs 已同步

- 更新：`sdks/README.md`
- 更新：`artifacts/releases/README.md`
- 更新：`artifacts/releases/wave-d-2026-04-08/bundle-manifest.md`
- 当前 bundle 已明确表达：
  - 版本尚未冻结
  - 决议来源路径也尚未分配

## 3. 当前判断

- SDK release catalog 现在不仅能表达“当前没有版本号”，也能表达“当前还没有版本 freeze 决议来源路径”。
- 当前实现仍然不代表真实 freeze evidence、真实生成物或真实发包已经完成。
- 本轮关键决策仍然是不伪造决议来源，而是先冻结统一字段名与空值占位。

## 4. fresh evidence

- `cargo fmt --all --check`
- `cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test test_continuous_optimization_sdk_release_catalog_freezes_version_decision_source_contract -- --nocapture`
- `cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test -- --nocapture`

