> Migrated from `docs/review/continuous-optimization-sdk-container-readmes-version-placeholder-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization - SDK container READMEs version placeholder - 2026-04-08

## 1. 本轮背景

- 上一轮已经把 `plannedVersion / versionStatus` 补进了 bundle、workspace 根入口和四个叶子 README。
- 但 app/admin 两层容器 README 仍然没有把这组版本占位状态公开出来。
- 这会导致 SDK release 导航链出现一个中间层断点。

## 2. 实际落地

### 2.1 contract gate 已新增

- 更新：`tools/chat-cli/tests/chat_cli_contract_test.rs`
- 新增：
  - `test_continuous_optimization_sdk_container_readmes_freeze_version_placeholder_boundary`
- 当前门禁会冻结两份容器 README 都必须包含：
  - `plannedVersion`
  - `null`
  - `versionStatus`
  - `version_unassigned_pending_freeze`

### 2.2 两份容器 README 已对齐版本占位边界

- 更新：`sdks/sdkwork-im-sdk/README.md`
- 更新：`sdks/sdkwork-control-plane-sdk/README.md`
- 现在 app/admin 容器 README 不再只表达 facade 边界，也显式表达当前没有冻结版本号

### 2.3 bundle evidence 已同步

- 更新：`artifacts/releases/wave-d-2026-04-08/bundle-manifest.md`
- 当前 SDK release 版本占位 evidence 已覆盖：
  - bundle contract
  - 容器 README
  - 叶子 README

## 3. 当前判断

- SDK release 版本占位语义现在已经沿导航链全量公开，不再只有根入口或叶子入口有，容器层空白。
- 当前实现仍然不代表真实版本 freeze、真实生成或真实发包已经完成。
- 本轮关键决策仍然是不伪造版本，而是把“没有版本号”这件事显式写透。

## 4. fresh evidence

- `cargo fmt --all --check`
- `cargo test -p sdkwork-im-cli --offline test_continuous_optimization_sdk_container_readmes_freeze_version_placeholder_boundary -- --exact --nocapture`
- `cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test -- --nocapture`

