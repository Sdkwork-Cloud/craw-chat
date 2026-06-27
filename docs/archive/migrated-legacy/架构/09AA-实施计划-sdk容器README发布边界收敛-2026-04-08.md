# 09AA - 实施计划 - SDK 容器 README 发布边界收敛

## 目标

让 app/admin 两层 SDK 容器 README 与叶子 README 保持同一套 release catalog 边界，避免根入口和叶子入口已经对齐，但中间层仍然不暴露 bundle 发布真源。

## 最小实施面

1. 在 `tools/chat-cli/tests/chat_cli_contract_test.rs` 先写红测
2. 更新：
   - `sdks/sdkwork-im-sdk/README.md`
   - `sdks/sdkwork-control-plane-sdk/README.md`
3. 每个容器 README 至少补齐：
   - `artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`
   - `generationStatus = template_only_pending_generation`
   - `releaseStatus = not_published`
4. 回归 contract test
5. 回写 `docs/step`、`docs/架构`、`docs/review`

## 约束

- 不伪造真实 SDK 生成结果
- 不伪造真实发布动作
- 容器 README 只公开 bundle 真源，不维护独立发布状态

## 放行标准

- 两个容器 README 全部显式表达 release catalog 边界
- fresh `fmt --check` 与 `chat_cli_contract_test` 保持通过
