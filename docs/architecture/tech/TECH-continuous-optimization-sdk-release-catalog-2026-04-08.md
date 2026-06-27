> Migrated from `docs/review/continuous-optimization-sdk-release-catalog-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization - SDK release catalog - 2026-04-08

## 1. 本轮背景

- `Step 13` 与 `Wave D / 93` 已完成发布就绪闭环，仓库已具备 release bundle 归档约定。
- `Step 12 / CP12-2` 也已经冻结了 SDK workspace 的 facade 边界与 README。
- 但这两部分之间仍缺一条小而真实的机器可读边界：
  - bundle 知道“有 SDK”
  - README 知道“有哪些语言和 audience”
  - 但缺少一份可被脚本与审计直接读取的 SDK release catalog

## 2. 实际落地

### 2.1 新增 bundle 级 SDK 目录清单

- 新增：`artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`
- 当前 JSON 已固定：
  - `bundleId = wave-d-2026-04-08`
  - `artifact = sdk-release-catalog`
  - `state = template_only_pending_generation`
  - 最小 `sdkArtifacts` 四项：
    - `app-typescript`
    - `app-flutter`
    - `admin-typescript`
    - `admin-flutter`

### 2.2 不伪造生成链与发布态

- 每个 SDK 入口都明确保持：
  - `generationStatus = template_only_pending_generation`
  - `releaseStatus = not_published`
- 这表示本轮只冻结目录 contract，不假装已经存在真实 SDK 生成或发包结果。

### 2.3 SDK / release 文档已统一回链

- 更新：`sdks/README.md`
- 更新：`artifacts/releases/README.md`
- 更新：`artifacts/releases/wave-d-2026-04-08/bundle-manifest.md`
- 当前关系已经明确：
  - SDK workspace 说明消费边界
  - release bundle 说明归档边界
  - `sdk-release-catalog.json` 负责提供结构化目录真源

### 2.4 contract gate 已冻结

- 更新：`tools/chat-cli/tests/chat_cli_contract_test.rs`
- 新增：
  - `test_continuous_optimization_docs_freeze_sdk_release_catalog_contract`
- 当前门禁会冻结：
  - JSON 产物存在且可解析
  - `bundleId / artifact / state` 合同存在
  - 四个 SDK 入口合同存在
  - SDK / release 文档全部回链该 JSON

## 3. 当前判断

- `Wave D` release bundle 现在不再只有 README 与 manifest 的文本说明，也具备了 SDK 目录的 machine-readable contract。
- 当前实现的是“最小可信目录清单”，不是完整 SDK 发布流水线。
- 本轮的关键决策是先冻结真源目录，再等待未来真实生成链接入，而不是反过来用文档暗示发布已经完成。

## 4. fresh evidence

- `cargo fmt --all --check`
- `cargo test -p sdkwork-im-cli --offline test_continuous_optimization_docs_freeze_sdk_release_catalog_contract -- --exact --nocapture`
- `cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test -- --nocapture`

