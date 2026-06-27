> Migrated from `docs/review/continuous-optimization-sdk-container-readmes-release-boundary-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization - SDK container READMEs release boundary - 2026-04-08

## 1. 本轮背景

- 上一轮已经把 `sdk-release-catalog.json` 的发布边界补进四个叶子 README。
- 但 app/admin 两层容器 README 还没有继续公开 bundle 内的 SDK 发布真源。
- 这会导致 SDK release 导航链在中间层出现一个“知道有版本占位，但不知道当前发布目录在哪里”的断点。

## 2. 实际落地

### 2.1 contract gate 已新增

- 更新：`tools/chat-cli/tests/chat_cli_contract_test.rs`
- 新增：
  - `test_continuous_optimization_sdk_container_readmes_freeze_release_catalog_boundary`
- 当前门禁会冻结两份容器 README 都必须包含：
  - `sdk-release-catalog.json`
  - `template_only_pending_generation`
  - `not_published`
  - `artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`

### 2.2 两份容器 README 已对齐发布边界

- 更新：`sdks/sdkwork-im-sdk/README.md`
- 更新：`sdks/sdkwork-control-plane-sdk/README.md`
- 现在 app/admin 容器 README 不再只表达 facade 边界和版本占位，也显式表达当前 bundle 内的唯一 SDK 发布真源

### 2.3 bundle evidence 已同步

- 更新：`artifacts/releases/wave-d-2026-04-08/bundle-manifest.md`
- 当前 SDK release boundary evidence 已覆盖：
  - bundle contract
  - 容器 README
  - 叶子 README

## 3. 当前判断

- SDK release catalog 的发布边界现在已经沿导航链全量公开，不再只有叶子 README 有，中间容器层缺失。
- 当前实现仍然不代表真实 SDK 生成、真实版本冻结或真实发包已经完成。
- 本轮关键决策仍然是不伪造发布状态，而是把“当前发布真源在哪里”写透。

## 4. fresh evidence

- `cargo fmt --all --check`
- `cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test test_continuous_optimization_sdk_container_readmes_freeze_release_catalog_boundary -- --nocapture`
- `cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test -- --nocapture`

