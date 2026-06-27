> Migrated from `docs/架构/09Y-实施计划-sdk叶子README版本占位收敛-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 09Y - 实施计划 - SDK 叶子 README 版本占位收敛

## 目标

让具体语言目录入口与 bundle 级 `plannedVersion / versionStatus` contract 保持一致，避免版本冻结语义只存在于总入口和 release bundle，而不下沉到叶子 README。

## 最小实施面

1. 先在 `tools/chat-cli/tests/chat_cli_contract_test.rs` 写红测
2. 更新四个叶子 README
3. 每个 README 至少补齐：
   - `plannedVersion = null`
   - `versionStatus = version_unassigned_pending_freeze`
4. 回归 contract test
5. 回写 `docs/step`、`docs/架构`、`docs/review`

## 约束

- 不伪造真实版本号
- 不伪造 freeze 决议来源
- 叶子 README 仍只回链 bundle 真源，不单独维护版本真源

## 放行标准

- 四个叶子 README 全部显式公开版本占位状态
- fresh `fmt --check` 与 `chat_cli_contract_test` 保持通过

