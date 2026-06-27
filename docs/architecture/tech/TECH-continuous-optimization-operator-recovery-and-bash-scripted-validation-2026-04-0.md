> Migrated from `docs/review/continuous-optimization-operator-recovery-and-bash-scripted-validation-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization - operator recovery and bash scripted validation - 2026-04-08

## 1. 本轮背景

- `00-13`、`Step 13` 与 `Wave D / 93` 已全部闭环。
- 当前仓库已进入持续优化模式，本轮优先处理 `docs/review/step-13-next-wave-backlog-2026-04-08.md` 中两项高价值收口：
  - `open-chat-test` 的 Bash scripted validation 补齐与固化
  - close / error registry 的客户端恢复策略公开基线

## 2. 实际落地

### 2.1 Bash scripted validation 摘要已追平最小 contract

- 更新：`bin/open-chat-test.sh`
- 当前 Bash scripted mode 新增：
  - `watchFrameTypes`
  - 空帧保护，避免 watch 成功退出但无任何 frame 时被误判为成功
  - 文本模式下同步打印 `watchFrameTypes`
- 保持不变：
  - 仍由 `open-chat-test` 统一创建 conversation、添加成员、发送验证消息
  - 仍沿 `watch + timeline` 主链路做最小 consumer-side 验证

### 2.2 Step 12 operator / SDK 文档已补恢复基线

- 更新：`docs/部署/CLI聊天验证与兼容矩阵.md`
- 更新：`sdks/sdkwork-im-sdk/README.md`
- 更新：`sdks/sdkwork-control-plane-sdk/README.md`
- 当前公开恢复语义已明确冻结：
  - `session.disconnect`
  - `realtime.overload`
  - `goaway`
  - `resume fallback`
- 当前 operator / SDK 不再只知道“能连上”，还明确知道：
  - 哪些场景应 fresh resume fallback
  - 哪些场景应先退化到 pull-only
  - 哪些 close / goaway 语义应被视为正式恢复决策输入

### 2.3 contract gate 已补齐

- 更新：`tools/chat-cli/tests/chat_cli_contract_test.rs`
- 更新：`tools/chat-cli/tests/chat_cli_e2e_test.rs`
- 新增/强化的回归门禁：
  - `test_step12_open_chat_test_scripts_freeze_scripted_validation_contract`
  - `test_step12_cli_and_sdk_docs_freeze_recovery_baseline`
  - `test_open_chat_test_bash_scripted_validation_emits_json_summary`

## 3. 当前环境限制

- 当前 Windows agent 上 Git Bash 启动即报：
  - `couldn't create signal pipe, Win32 error 5`
- `wsl.exe --status` 也返回访问拒绝，说明本机不存在可供测试直接消费的可用 Bash 宿主。
- 因此：
  - Bash scripted validation 的 contract 已补齐
  - Bash E2E 用例已进入仓库，但在当前 agent 上会显式条件跳过，而不是伪造通过

## 4. fresh evidence

- `cargo test -p sdkwork-im-cli --offline test_step12_open_chat_test_scripts_freeze_scripted_validation_contract -- --nocapture`
- `cargo test -p sdkwork-im-cli --offline test_step12_cli_and_sdk_docs_freeze_recovery_baseline -- --nocapture`
- `cargo test -p sdkwork-im-cli --offline test_open_chat_test_bash_scripted_validation_emits_json_summary -- --nocapture`

## 5. 当前判断

- 持续优化已正式启动，不再停留在 Step 13 总结态。
- `open-chat-test` 的 Bash 路径已补到与 PowerShell 更接近的最小摘要 contract。
- close / error registry 的 operator / SDK 恢复基线已从 review 待办推进到公开文档。
- 剩余 gap：
  - 需要在具备可用 Bash runtime 的节点上补 fresh Bash E2E 证据
  - 仍可继续推进单一索引页、release bundle 归档、runtime ops smoke 等 backlog 条目

