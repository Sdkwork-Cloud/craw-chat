> Migrated from `docs/review/step-12-cp12-1-cli与实时握手契约收口-架构兑现与回写决议-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 12 / CP12-1 CLI 与实时握手契约收口架构兑现与回写决议- 2026-04-08

## 对应架构文档
- `docs/架构/09-实施计划.md`
- `docs/架构/146-CCP协议注册表与多端SDK兼容治理设计-2026-04-06.md`
- `docs/架构/148-CCP控制面注册表与协议发布治理设计2026-04-06.md`
- `docs/架构/149-多Cell多Region协议升级与灾备兼容设计2026-04-06.md`

## 本轮已兑现能力力力力力
- `09`
  - `Wave D / Step 12` 已完成首个真CLI 检查点，当step 不再停留在HTTP 文档contract test
- `146`
  - `tools/chat-cli` 已成为当registry / capability negotiation 的真实消费方
    - `payload.json` 为当前默认协商基
    - `session.resume` 仅在协商成功时启
    - `watch / chat-session / wrapper` 均受同一套主链路约束
- `148`
  - control-plane `effective snapshot` websocket 热路径握手现在重新一致：
    - 未协商`session.resume` 时，runtime 不再继续等待该控制帧
    - 已协商时，CLI 仍能继续执行 `session_resume -> session_resumed`
  - 这让 `hello_ack.capabilities` 恢复盘runtime 的权威输入，而不是“只读但不生效”的旁路信息
- `149`
  - 当前 CLI consumer path 已具备最小可信的升级兼容 / 安全降级证据
    - 默认 snapshot 未协商`session.resume` 时，不再capability drift 卡死首帧
    - future `session.resume` 路径仍保留专门回归测

## 本轮未兑现能力力力力力
- `146`
  - SDK facade README、多语言消费链路仍未进入本轮范围
- `148`
  - tenant / client segment 级治理、release bundle 仍未开
- `149`
  - cell / region rollout orchestration、发布后观测门禁与灾备切换自动化仍未开

## 是否偏离架构
- 发现过一次真实偏离，并已在本轮修正：
  - 偏离现象
    - `hello_ack.capabilities` 默认未协商`session.resume`
    - websocket 热路径却仍强制等`session_resume`
  - 修正结果：
    - runtime 现在按协商条件化要求 `session_resume`
    - CLI 也按协商决定是否发送该控制
- 因此当前实现已回写`146 / 148 / 149` 共同定义的“能力由协商与快照决定”主路径

## 回写决议
- `docs/架构/09-实施计划.md`
  - 追加 `As-Built 106`
- `docs/架构/146-CCP协议注册表与多端SDK兼容治理设计-2026-04-06.md`
  - 追加 `As-Built 4`
- `docs/架构/148-CCP控制面注册表与协议发布治理设计2026-04-06.md`
  - 追加 `As-Built 5`
- `docs/架构/149-多Cell多Region协议升级与灾备兼容设计2026-04-06.md`
  - 追加 `As-Built 7`

## 证据
- 代码
  - `tools/chat-cli/src/lib.rs`
  - `tools/chat-cli/src/realtime.rs`
  - `services/session-gateway/src/websocket.rs`
- 测试
  - `tools/chat-cli/tests/chat_cli_contract_test.rs`
  - `tools/chat-cli/tests/chat_cli_e2e_test.rs`
  - `services/session-gateway/tests/websocket_smoke_test.rs`
- 文档
  - `docs/部署/CLI聊天验证与兼容矩阵md`
- 验证
  - `cargo test -p session-gateway --offline --test websocket_smoke_test -- --nocapture`
  - `cargo test -p sdkwork-im-cli --offline --test chat_cli_contract_test -- --nocapture`
  - `cargo test -p sdkwork-im-cli --offline --test chat_cli_e2e_test -- --nocapture`
  - `cargo fmt --all --check`

## 当前判断
- `CP12-1`：通过
- `Step 12`：继续进行中
- 下一步：进入 `CP12-2`

