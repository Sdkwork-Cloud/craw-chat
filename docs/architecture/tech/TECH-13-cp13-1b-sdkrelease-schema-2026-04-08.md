> Migrated from `docs/step/13-CP13-1B-sdk发布目录schema收敛-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 13-CP13-1B - SDK 发布目录 schema 收敛

## 本轮目的

把上一轮已经落地的 `sdk-release-catalog.json`，继续收敛到`$schema` 的结构化 contract，避bundle machine-readable SDK 目录再次退化成“只有样JSON，没有正schema”的状态

## 发现的问

- `artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json` 已存
- `artifacts/releases/README.md` `bundle-manifest.md` 也已回链JSON
- 但此前还缺少
  - `artifacts/releases/schemas/sdk-release-catalog.schema.json`
  - `sdk-release-catalog.json` 自身`$schema`
  - release 文档schema 路径的公开说明
- 结果是：catalog 已可读，但字段结构、状态枚举与 bundle 间一致性仍缺正contract

## 本轮决策

- 继续使用 `tools/chat-cli/tests/chat_cli_contract_test.rs` 做公开 contract gate
- 新增 `test_continuous_optimization_docs_freeze_sdk_release_catalog_schema_contract`
- 当前最schema contract 固定
  - `artifact = sdk-release-catalog`
  - `state` 枚举
  - `sdkArtifacts[*].releaseStatus` 枚举
  - `sdk-release-catalog.json` 必须声明
    - `$schema = ../schemas/sdk-release-catalog.schema.json`

## 实施结果

- 新增 `artifacts/releases/schemas/sdk-release-catalog.schema.json`
- `sdk-release-catalog.json` 已补 `$schema`
- `artifacts/releases/README.md` 已补 schema 入口说明
- `artifacts/releases/wave-d-2026-04-08/bundle-manifest.md` 已把 schema 纳入 bundle 归档证据

## 验证

- 红灯
  - `cargo test -p sdkwork-im-cli --offline test_continuous_optimization_docs_freeze_sdk_release_catalog_schema_contract -- --exact --nocapture`
  - 失败点：`missing SDK release catalog schema: artifacts/releases/schemas/sdk-release-catalog.schema.json`
- 绿灯
  - `cargo fmt --all --check`
  - `cargo test -p sdkwork-im-cli --offline test_continuous_optimization_docs_freeze_sdk_release_catalog_schema_contract -- --exact --nocapture`
  - `cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test -- --nocapture`

## 下一轮建

- 若继续深SDK 发布链，可在 schema 上继续补齐
  - 真实版本
  - 生成时间
  - artifact 路径
  - 发布仓库坐标
- 在没有真实生成与发布证据前，仍保持模板schema，不catalog 扩写成假发布记录

