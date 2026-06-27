# 09AC - 实施计划 - SDK 叶子 README 版本决议来源收敛

## 目标

让四个语言叶子 README 与 bundle 级 release catalog 保持同一套版本决议来源占位 contract，避免消费者进入具体语言目录后失去该字段。

## 最小实施面

1. 在 `tools/chat-cli/tests/chat_cli_contract_test.rs` 先写红测
2. 更新四个叶子 README
3. 每个 README 至少补齐：
   - `versionDecisionSourcePath`
   - 当前值 `null`
4. 回归 contract test
5. 回写 `docs/step`、`docs/架构`、`docs/review`

## 约束

- 不伪造真实 freeze 决议来源路径
- 叶子 README 只公开 bundle 真源已有占位，不独立维护决议字段

## 放行标准

- 四个叶子 README 全部显式表达 `versionDecisionSourcePath = null`
- fresh `fmt --check` 与 `chat_cli_contract_test` 保持通过
