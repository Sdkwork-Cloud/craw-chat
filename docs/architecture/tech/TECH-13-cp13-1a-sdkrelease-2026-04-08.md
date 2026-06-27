> Migrated from `docs/step/13-CP13-1A-sdk发布目录清单收敛-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 13-CP13-1A - SDK 发布目录清单收敛

## 本轮目的

`Wave D` release bundle 中已经存在的 SDK 目录边界，继续收敛为一machine-readable `sdk-release-catalog.json`，避bundle manifest、SDK README 与后续发布审计继续停留在“只有文本说明、没有结构化目录”的状态

## 发现的问

- `sdks/README.md` 已冻app/admin、TypeScript/Flutter facade 边界
- `artifacts/releases/README.md` `artifacts/releases/wave-d-2026-04-08/bundle-manifest.md` 已建release bundle 归档约定
- 但此前还缺少一bundle machine-readable SDK 目录清单来明确：
  - 当前 bundle 对应哪些 SDK 入口
  - 每个 SDK 属于哪个 audience / language
  - 当前仍是模板态，还是已完成真实生/ 发布
- 结果是：公开文档能说明“有 SDK 目录”，但不能让后续脚本、审计或发布检查直接读取统一状态

## 本轮决策

- 继续使用 `tools/chat-cli/tests/chat_cli_contract_test.rs` 作为公开文档bundle 资产的契约守
- 新增 `test_continuous_optimization_docs_freeze_sdk_release_catalog_contract`
- bundle 内新增：
  - `artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`
- 当前最小冻结范围：
  - `app-typescript`
  - `app-flutter`
  - `admin-typescript`
  - `admin-flutter`
- 当前状态明确固定为
  - `state = template_only_pending_generation`
  - `generationStatus = template_only_pending_generation`
  - `releaseStatus = not_published`

## 实施结果

- 新增 `sdk-release-catalog.json`，固`bundleId / artifact / state / sdkArtifacts`
- `sdks/README.md` 已补 machine-readable SDK release catalog 入口
- `artifacts/releases/README.md` 已补 bundle SDK 目录清单说明
- `artifacts/releases/wave-d-2026-04-08/bundle-manifest.md` 已把JSON 纳入 `Wave D` bundle 归档证据

## 验证

- 红灯
  - `cargo test -p sdkwork-im-cli --offline test_continuous_optimization_docs_freeze_sdk_release_catalog_contract -- --exact --nocapture`
  - 失败点：`missing SDK release catalog: artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`
- 绿灯
  - `cargo fmt --all --check`
  - `cargo test -p sdkwork-im-cli --offline test_continuous_optimization_docs_freeze_sdk_release_catalog_contract -- --exact --nocapture`
  - `cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test -- --nocapture`

## 下一轮建

- 若后续出现真SDK 生成链，再在同一 catalog 中推进：
  - 生成时间
  - 版本
  - 产物路径
  - 发布记录
- 在没有真实生成与发布证据前，继续保持 `template_only_pending_generation / not_published`，不伪造 SDK 发布完成状态

