# Continuous Optimization - SDK release catalog schema - 2026-04-08

## 1. 本轮背景

- 上一轮已经把 `Wave D` bundle 内的 SDK 目录冻结成 `sdk-release-catalog.json`。
- 但该 JSON 还只是 machine-readable 样例，没有正式 schema contract。
- 这会留下一个真实风险：
  - 当前 bundle 可读
  - 下一 bundle 也许还能继续写
  - 但字段结构、状态枚举与公共入口不一定保持一致

## 2. 实际落地

### 2.1 新增 SDK release catalog schema

- 新增：`artifacts/releases/schemas/sdk-release-catalog.schema.json`
- 当前 schema 已固定：
  - `$id = artifacts/releases/schemas/sdk-release-catalog.schema.json`
  - `title = sdkwork-im sdk release catalog`
  - `artifact = sdk-release-catalog`
  - `state` 最小枚举
  - `sdkArtifacts[*].releaseStatus` 最小枚举

### 2.2 catalog 已自带 `$schema`

- 更新：`artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`
- 当前已显式声明：
  - `$schema = ../schemas/sdk-release-catalog.schema.json`

### 2.3 release 文档已统一回链 schema

- 更新：`artifacts/releases/README.md`
- 更新：`artifacts/releases/wave-d-2026-04-08/bundle-manifest.md`
- 当前 release README、bundle manifest、catalog JSON 已形成一致入口：
  - README 说明 contract
  - manifest 归档 schema
  - catalog 通过 `$schema` 指向真源

### 2.4 contract gate 已冻结

- 更新：`tools/chat-cli/tests/chat_cli_contract_test.rs`
- 新增：
  - `test_continuous_optimization_docs_freeze_sdk_release_catalog_schema_contract`
- 当前门禁会冻结：
  - schema 文件存在且可解析
  - catalog 自带 `$schema`
  - state / releaseStatus 枚举存在
  - release 文档回链 schema

## 3. 当前判断

- `sdk-release-catalog` 现在已经从“可机读样例”继续推进到“带 schema 的正式目录 contract”。
- 当前实现仍是模板态 SDK 发布目录，不代表真实 SDK 生成与发包链路已经完成。
- 本轮的关键决策是先冻结字段 contract，再等待未来真实生成链接入，而不是让 bundle 长期依赖无 schema JSON 样例。

## 4. fresh evidence

- `cargo fmt --all --check`
- `cargo test -p sdkwork-im-cli --offline test_continuous_optimization_docs_freeze_sdk_release_catalog_schema_contract -- --exact --nocapture`
- `cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test -- --nocapture`
