> Migrated from `docs/step/12-CP12-4A-close-error-registry细粒度恢复词汇收敛-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 12-CP12-4A - close / error registry 细粒度恢复词汇收口

## 本轮目的

`Step 12` 已经冻结论close / error registry 基线，从“有恢复方向”继续收敛到“有细粒度公开词汇”，避免 CLI、SDK README 与单一索引页对恢复细节的描述再次分叉

## 发现的问

- `docs/部署/CLI聊天验证与兼容矩阵md` 已经说明 `session.disconnect / realtime.overload / goaway / resume fallback`
- SDK README 与单一索引页还没有统一冻结以下更细粒度恢复词汇
  - `4001`
  - `reconnect_required`
  - `pull-only`
  - `events.pull`
- 结果是：读者知道“要恢复”，但无法从同一套公开文档里读到一致的恢复输入与动作词

## 本轮决策

- 继续使用 `tools/chat-cli/tests/chat_cli_contract_test.rs` 作为文档契约守卫
- 新增细粒度契约要求：
  - `4001`
  - `session.disconnect`
  - `reconnect_required`
  - `pull-only`
  - `events.pull`
- 对齐范围
  - `docs/部署/CLI聊天验证与兼容矩阵md`
  - `sdks/sdkwork-im-sdk/README.md`
  - `sdks/sdkwork-control-plane-sdk/README.md`
  - `docs/部署/兼容矩阵与SDK-CLI-operator验证索引.md`

## 实施结果

- `chat_cli_contract_test.rs` 新增细粒度恢复词汇守
- CLI 文档补齐 close code、close reason stale 请求错误词汇
- app/admin SDK README 补齐 close/error registry 更细粒度公开输入
- 单一索引页补齐同一套恢复词汇，避免“入口页更粗、正文更细”再次分

## 验证

- 红灯
  - `cargo test -p sdkwork-im-cli --offline test_continuous_optimization_docs_freeze_detailed_recovery_registry_baseline -- --exact --nocapture`
  - 失败点：`Step 12 CLI doc must contain detailed recovery baseline text 4001`
- 绿灯
  - `cargo test -p sdkwork-im-cli --offline test_continuous_optimization_docs_freeze_detailed_recovery_registry_baseline -- --exact --nocapture`

## 下一轮建

- 继续检Bash scripted validation 是否还缺少与 PowerShell 对称的公开契约fresh 证据
- 若当前环境不适合Bash E2E，则继续收敛多语言 SDK 发布链的最小机器可读边

