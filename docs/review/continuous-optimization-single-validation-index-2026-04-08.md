# Continuous Optimization - single validation index - 2026-04-08

## 1. 本轮背景

- 仓库已完成 `Step 13` 与 `Wave D / 93` 总收口，当前阶段处于持续优化模式。
- `docs/review/step-13-next-wave-backlog-2026-04-08.md` 明确提出需要建立：
  - 从 `compatibility matrix`
  - 到 `SDK facade`
  - 再到 CLI / operator 验证入口
  - 的单一索引页

## 2. 实际落地

### 2.1 新增单一索引页

- 新增：`docs/部署/兼容矩阵与SDK-CLI-operator验证索引.md`
- 当前索引页统一收敛：
  - `compatibility matrix`
  - `sdkwork-im-sdk`
  - `sdkwork-control-plane-sdk`
  - `sdkwork-im-cli`
  - `open-chat-test`
  - `chat_cli_contract_test.rs`
  - `chat_cli_e2e_test.rs`
  - `protocol_registry_test.rs`
  - `protocol_governance_test.rs`

### 2.2 入口回链已补齐

- 更新：`docs/部署/README.md`
- 更新：`sdks/README.md`
- 更新：`README.md`
- 当前公共入口不再要求读者自己从部署、SDK、CLI 文档之间跳跃查找。

### 2.3 contract gate 已冻结

- 更新：`tools/chat-cli/tests/chat_cli_contract_test.rs`
- 新增：
  - `test_continuous_optimization_docs_freeze_single_validation_index`
- 当前回归门禁已经锁定：
  - 索引页必须存在
  - 索引页必须覆盖 compatibility / SDK / CLI / operator / tests
  - 三个公开入口 README 必须回链到同一索引页

## 3. 架构兑现

- 本轮没有引入新的协议或运行时语义，而是把已完成的 `Step 12` 与持续优化结果收敛成单一检索入口。
- 这直接降低了：
  - operator 查找控制面、CLI、scripted validation 入口的成本
  - SDK 消费方确认 facade 与验证入口关系的成本
  - 后续 review 文档分散导致的回溯成本

## 4. fresh evidence

- `cargo test -p sdkwork-im-cli --offline test_continuous_optimization_docs_freeze_single_validation_index -- --nocapture`

## 5. 当前判断

- 单一索引页已落地，不再只是 backlog 条目。
- `compatibility matrix -> SDK facade -> CLI / operator` 这条链路当前已有统一入口。
- 下一轮仍可继续推进：
  - release bundle 归档约定
  - runtime ops 最小 smoke 行为核对
  - `standalone.split-services.development` 对称发布后验证样本
