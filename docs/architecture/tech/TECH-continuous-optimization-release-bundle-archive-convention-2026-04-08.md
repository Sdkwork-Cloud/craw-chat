> Migrated from `docs/review/continuous-optimization-release-bundle-archive-convention-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization - release bundle archive convention - 2026-04-08

## 1. 本轮背景

- `Step 13` 的 review 证据已经齐全，但 `step-13-next-wave-backlog-2026-04-08.md` 仍明确要求把它们提升为可归档、可审计、可回滚的 release bundle。
- 当前仓库此前只有 review 文档，没有正式的 release bundle 目录约定。

## 2. 实际落地

### 2.1 release bundle 根目录已建立

- 新增：`artifacts/releases/README.md`
- 当前固定了：
  - `artifacts/releases/<bundle-id>/`
  - `<bundle-id>` 推荐格式
  - bundle 至少应包含的 manifest 与证据项

### 2.2 Wave D 示例 bundle 已落地

- 新增：`artifacts/releases/wave-d-2026-04-08/bundle-manifest.md`
- 当前 manifest 已收录：
  - `Step 13` / `Wave D / 93`
  - `go / no-go = Go`
  - fresh verification 命令
  - 升级 / 回滚入口
  - 当前可审计 / 可回滚边界

### 2.3 公开入口已补回链

- 更新：`README.md`
- 更新：`docs/部署/README.md`
- 现在 release bundle 不再只藏在 review backlog 里，而是进入公开入口。

### 2.4 contract gate 已冻结

- 更新：`tools/chat-cli/tests/chat_cli_contract_test.rs`
- 新增：
  - `test_continuous_optimization_docs_freeze_release_bundle_archive_convention`

## 3. 当前判断

- release bundle 归档约定已从 backlog 条目变成真实目录资产。
- 当前实现的是“最小可信归档约定”，不是完整自动发布流水线。
- 仍可继续深化：
  - checksum / 版本清单
  - 机器生成 release note
  - 真实 bundle 产物收集脚本

## 4. fresh evidence

- `cargo test -p sdkwork-im-cli --offline test_continuous_optimization_docs_freeze_release_bundle_archive_convention -- --nocapture`

