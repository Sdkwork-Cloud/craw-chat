# 09AD - 实施计划 - SDK 容器 README 版本决议来源收敛

## 目标

让 app/admin 两层 SDK 容器 README 与 bundle、root、leaf 三层入口保持同一套 `versionDecisionSourcePath = null` 占位 contract，消除导航链中间层断层。

## 最小实施面

1. 在 `tools/chat-cli/tests/chat_cli_contract_test.rs` 先写红测
2. 更新：
   - `sdks/sdkwork-im-sdk/README.md`
   - `sdks/sdkwork-control-plane-sdk/README.md`
3. 每个容器 README 至少补齐：
   - `versionDecisionSourcePath`
   - 当前值 `null`
4. 回归 contract test
5. 回写 `docs/step`、`docs/架构`、`docs/review`

## 约束

- 不伪造真实 freeze 决议来源路径
- 容器 README 只继续公开 bundle 已存在的占位字段，不单独维护真实值

## 放行标准

- 两个容器 README 全部显式表达 `versionDecisionSourcePath = null`
- fresh `fmt --check` 与 `chat_cli_contract_test` 保持通过
