> Migrated from `docs/架构/09X-实施计划-sdk版本冻结占位收敛-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 09X - 实施计划 - SDK 版本冻结占位收敛

## 目标

让 bundle 级 SDK release catalog 不仅表达“未生成 / 未发布”，还表达“版本尚未冻结”，为后续真实版本冻结与发包链留下结构化 contract。

## 最小实施面

1. 先在 `tools/chat-cli/tests/chat_cli_contract_test.rs` 写红测
2. 为 `sdk-release-catalog.schema.json` 新增：
   - `plannedVersion`
   - `versionStatus`
3. 为 `sdk-release-catalog.json` 的四个 SDK 入口新增：
   - `plannedVersion = null`
   - `versionStatus = version_unassigned_pending_freeze`
4. 更新：
   - `sdks/README.md`
   - `artifacts/releases/README.md`
   - `artifacts/releases/wave-d-2026-04-08/bundle-manifest.md`
5. 回归 contract test
6. 回写 `docs/step`、`docs/架构`、`docs/review`

## 约束

- 不伪造真实版本号
- 不伪造真实 freeze 时间
- 不伪造真实生成或发包结果
- 当前只冻结“版本尚未分配”的显式 contract

## 放行标准

- schema 明确存在 `plannedVersion / versionStatus`
- catalog 四个 SDK 入口全部显式表达未冻结版本状态
- SDK / release 公开入口全部回链这组版本占位 contract
- fresh `fmt --check` 与 `chat_cli_contract_test` 保持通过

