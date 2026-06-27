> Migrated from `docs/架构/09AB-实施计划-sdk版本决议来源占位收敛-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 09AB - 实施计划 - SDK 版本决议来源占位收敛

## 目标

让 SDK release catalog 不仅表达“版本还没冻结”，也表达“未来 freeze 决议来源路径目前尚未分配”，从而把版本治理证据入口冻结成稳定 contract。

## 最小实施面

1. 在 `tools/chat-cli/tests/chat_cli_contract_test.rs` 先写红测
2. 更新：
   - `artifacts/releases/schemas/sdk-release-catalog.schema.json`
   - `artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`
   - `sdks/README.md`
   - `artifacts/releases/README.md`
   - `artifacts/releases/wave-d-2026-04-08/bundle-manifest.md`
3. 至少补齐：
   - `versionDecisionSourcePath`
   - 当前值 `null`
4. 回归 contract test
5. 回写 `docs/step`、`docs/架构`、`docs/review`

## 约束

- 不伪造真实版本号
- 不伪造真实 freeze 决议来源路径
- 先冻结字段和空值占位，再等待真实 evidence 落地

## 放行标准

- schema 与 catalog 都已显式定义 `versionDecisionSourcePath`
- root release docs 全部同步 `versionDecisionSourcePath = null`
- fresh `fmt --check` 与 `chat_cli_contract_test` 保持通过

