> Migrated from `docs/review/continuous-optimization-sdk-release-version-placeholder-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization - SDK release version placeholder - 2026-04-08

## 1. 本轮背景

- `sdk-release-catalog` 已经补齐了：
  - bundle 级目录真源
  - schema contract
  - 叶子 README 发布边界
- 但这条链还缺一层关键约束：
  - 当前尚未有真实 SDK 版本号
  - 这个事实此前并没有被 machine-readable 明确表达

## 2. 实际落地

### 2.1 contract gate 已新增

- 更新：`tools/chat-cli/tests/chat_cli_contract_test.rs`
- 新增：
  - `test_continuous_optimization_sdk_release_catalog_freezes_version_placeholder_contract`
- 当前门禁会冻结：
  - schema 中必须存在 `plannedVersion / versionStatus`
  - 四个 SDK 入口都必须显式保留：
    - `plannedVersion = null`
    - `versionStatus = version_unassigned_pending_freeze`

### 2.2 schema 与 catalog 已显式表达“版本未冻结”

- 更新：`artifacts/releases/schemas/sdk-release-catalog.schema.json`
- 更新：`artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`
- 当前每个 SDK 入口都不再只是“未生成 / 未发布”，而是进一步明确：
  - 当前还没有版本号
  - 当前仍待后续版本冻结决议

### 2.3 公开入口已同步版本占位边界

- 更新：`sdks/README.md`
- 更新：`artifacts/releases/README.md`
- 更新：`artifacts/releases/wave-d-2026-04-08/bundle-manifest.md`
- 当前 workspace 入口与 release bundle 入口已统一说明：
  - `plannedVersion`
  - `version_unassigned_pending_freeze`

## 3. 当前判断

- SDK release 链现在已经从“有目录、无版本语义”推进到“有显式版本占位 contract”。
- 当前实现仍然没有真实版本号，也没有真实生成与发包链路。
- 本轮的关键决策是不伪造 semver，而是先把“尚未冻结版本”冻结成正式 contract。

## 4. fresh evidence

- `cargo fmt --all --check`
- `cargo test -p sdkwork-im-cli --offline test_continuous_optimization_sdk_release_catalog_freezes_version_placeholder_contract -- --exact --nocapture`
- `cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test -- --nocapture`

