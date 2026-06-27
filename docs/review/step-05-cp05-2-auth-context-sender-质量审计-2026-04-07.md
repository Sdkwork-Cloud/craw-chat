# Step 05 CP05-2 AuthContext Sender Snapshot 质量审计 - 2026-04-07

## 1. 审计范围

- `services/conversation-runtime/src/runtime.rs`
- `services/conversation-runtime/src/runtime/http.rs`
- `services/sdkwork-im-cloud-gateway/src/node.rs`
- `services/sdkwork-im-cloud-gateway/src/node/effects.rs`
- `services/sdkwork-im-cloud-gateway/src/node/message.rs`
- `services/conversation-runtime/tests/conversation_domain_structure_test.rs`
- `services/conversation-runtime/tests/authority_command_test.rs`
- `services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`

## 2. 本轮主要质量判断

### 2.1 已消除的风险

- 已消除 HTTP 入口和 sdkwork-im-server 对 `Sender` 的重复手工组装。
- 已消除 system-channel publish 与普通消息变更命令在 sender snapshot 生成处的分叉实现。
- 已把 `device_id / session_id / actor id / actor kind` 的快照来源收口到同一套构造逻辑。

### 2.2 本轮未发现的回归

- 未发现 message mutation 入口因构造器替换导致的结构性回归。
- 未发现 sdkwork-im-server 因删除 `build_sender(...)` 造成的调用缺口。
- 未发现 `conversation-runtime` 与 `sdkwork-im-server` 之间 authority snapshot 字段名不一致的问题。

## 3. 仍然存在的真实风险

### 3.1 Step 级风险仍在

- `CP05-2` 只完成了 message mutation 子路径，尚未覆盖:
  - conversation create
  - member governance
  - read-cursor
- 因此 `tenantId / sender` authority closure 仍不是 Step 级闭环。

### 3.2 架构级风险仍在

- authority capture 目前仍然不是所有 Step 05 command surface 的统一 owner。
- 若后续 create/member/read-cursor 保持各自手工提取 authority 字段，仍可能出现新的 snapshot 漂移。

### 3.3 波次级风险仍在

- `Step 05` 未闭环，`Step 06` 也未闭环。
- 因此 `Wave B / 93` 仍不能启动总验收。

## 4. 证据

### 4.1 结构与单点验证

- `cargo test -p conversation-runtime --test conversation_domain_structure_test test_message_mutation_commands_offer_auth_context_constructors --offline`
- `cargo test -p conversation-runtime --test authority_command_test --offline`
- `cargo test -p sdkwork-im-cloud-gateway --test lib_structure_test test_local_minimal_node_message_paths_use_auth_context_command_constructors --offline`

### 4.2 全量受影响服务验证

- `rustfmt --edition 2024 --check services/conversation-runtime/src/runtime.rs services/conversation-runtime/src/runtime/http.rs services/conversation-runtime/tests/conversation_domain_structure_test.rs services/conversation-runtime/tests/authority_command_test.rs services/sdkwork-im-cloud-gateway/src/node.rs services/sdkwork-im-cloud-gateway/src/node/effects.rs services/sdkwork-im-cloud-gateway/src/node/message.rs services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`
- `cargo test -p conversation-runtime --offline`
- `cargo test -p sdkwork-im-cloud-gateway --offline`
- `cargo test -p projection-service --offline`

## 5. 审计结论

- 本轮可以认定为 `CP05-2` 的一个真实、可验证增量。
- 本轮不能认定为 `CP05-2` 已闭环。
- 本轮不能认定为 `Step 05` 已通过 `91 / 95 / 97`。
- 当前最合理结论仍然是:
  - `Wave B / Step 05 / CP05-2`
  - 已前进
  - 未闭环

## 6. 下一步审计关注点

- create/member/read-cursor 是否也能收敛到统一 authority owner
- `tenantId` 是否能从 auth-context 在主链路上统一收口
- downstream projection / notification / multi-client-route sync 是否继续消费同一套 authority snapshot
