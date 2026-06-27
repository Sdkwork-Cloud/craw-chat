> Migrated from `docs/review/step-05-cp05-2-auth-context-sender-执行补充-2026-04-07.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 05 CP05-2 AuthContext Sender Snapshot 执行补充 - 2026-04-07

## 1. 当前定位

- 当前波次: `Wave B`
- 当前 Step: `Step 05`
- 当前子卡: `CP05-2`
- 前置状态:
  - `CP05-1` 已闭环
  - `Step 04` 已闭环
  - `CP05-2 / CP05-3 / CP05-4` 仍未闭环

## 2. 本轮为什么先做这一步

- `Step 05` 的真实阻塞已经从 conversation/message aggregate owner 转到 `sender / tenant` authority closure。
- 在消息主链路里，`post / system-channel publish / edit / recall` 仍然存在多处手工拼装 `Sender` 的路径，来源分散在:
  - `conversation-runtime` HTTP 入口
  - `sdkwork-im-server` 本地入口
- 这类重复拼装会带来 authority snapshot 漂移风险，尤其是:
  - `device_id`
  - `session_id`
  - system-channel publisher 与普通 message sender 的快照一致性
- 因此本轮先收口消息变更入口，把 `AuthContext -> Sender` 的映射统一收敛到 `conversation-runtime` command boundary。

## 3. 本轮实际完成

### 3.1 代码落地

- 在 `services/conversation-runtime/src/runtime.rs` 新增共享 helper:
  - `sender_from_auth_context(auth: &AuthContext) -> Sender`
- 为四类消息变更命令新增统一构造入口:
  - `PostMessageCommand::from_auth_context(...)`
  - `PublishSystemChannelMessageCommand::from_auth_context(...)`
  - `EditMessageCommand::from_auth_context(...)`
  - `RecallMessageCommand::from_auth_context(...)`
- 在 `services/conversation-runtime/src/runtime/http.rs` 删除手工 `Sender { ... }` 组装，统一改用上述构造器。
- 在 `services/sdkwork-im-cloud-gateway/src/node/effects.rs` 删除本地 message / system-channel 发送路径的手工 sender 组装，统一改用上述构造器。
- 在 `services/sdkwork-im-cloud-gateway/src/node/message.rs` 删除 edit / recall 路径的手工 sender 组装，统一改用上述构造器。
- 在 `services/sdkwork-im-cloud-gateway/src/node.rs` 删除本地辅助函数:
  - `build_sender(...)`

### 3.2 测试补齐

- `services/conversation-runtime/tests/conversation_domain_structure_test.rs`
  - 新增 `test_message_mutation_commands_offer_auth_context_constructors`
  - 新增 `test_http_message_surface_uses_auth_context_command_constructors`
- `services/conversation-runtime/tests/authority_command_test.rs`
  - 新增 `test_message_mutation_commands_from_auth_context_preserve_authority_snapshot`
- `services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`
  - 新增 `test_local_minimal_node_message_paths_use_auth_context_command_constructors`

## 4. 涉及文件

### 4.1 代码

- `services/conversation-runtime/src/runtime.rs`
- `services/conversation-runtime/src/runtime/http.rs`
- `services/sdkwork-im-cloud-gateway/src/node.rs`
- `services/sdkwork-im-cloud-gateway/src/node/effects.rs`
- `services/sdkwork-im-cloud-gateway/src/node/message.rs`

### 4.2 测试

- `services/conversation-runtime/tests/conversation_domain_structure_test.rs`
- `services/conversation-runtime/tests/authority_command_test.rs`
- `services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`

## 5. 验证证据

### 5.1 Red/Green 证据

- Red
  - `cargo test -p conversation-runtime --test conversation_domain_structure_test test_message_mutation_commands_offer_auth_context_constructors --offline`
  - `cargo test -p sdkwork-im-cloud-gateway --test lib_structure_test test_local_minimal_node_message_paths_use_auth_context_command_constructors --offline`
- Green
  - `cargo test -p conversation-runtime --test conversation_domain_structure_test test_message_mutation_commands_offer_auth_context_constructors --offline`
  - `cargo test -p conversation-runtime --test authority_command_test --offline`
  - `cargo test -p sdkwork-im-cloud-gateway --test lib_structure_test test_local_minimal_node_message_paths_use_auth_context_command_constructors --offline`

### 5.2 完整回归证据

- `rustfmt --edition 2024 services/conversation-runtime/src/runtime.rs services/conversation-runtime/src/runtime/http.rs services/conversation-runtime/tests/conversation_domain_structure_test.rs services/conversation-runtime/tests/authority_command_test.rs services/sdkwork-im-cloud-gateway/src/node.rs services/sdkwork-im-cloud-gateway/src/node/effects.rs services/sdkwork-im-cloud-gateway/src/node/message.rs services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`
- `rustfmt --edition 2024 --check services/conversation-runtime/src/runtime.rs services/conversation-runtime/src/runtime/http.rs services/conversation-runtime/tests/conversation_domain_structure_test.rs services/conversation-runtime/tests/authority_command_test.rs services/sdkwork-im-cloud-gateway/src/node.rs services/sdkwork-im-cloud-gateway/src/node/effects.rs services/sdkwork-im-cloud-gateway/src/node/message.rs services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`
- `cargo test -p conversation-runtime --offline`
- `cargo test -p sdkwork-im-cloud-gateway --offline`
- `cargo test -p projection-service --offline`

### 5.3 验证结论

- 本轮增量的代码、结构测试和受影响服务测试均已形成证据。
- `sdkwork-im-server` 全量测试中仍会打印启动失败/health timeout 示例日志，但该套件退出码为 `0`，不构成本轮阻塞。

## 6. 对应架构能力与兑现判断

### 6.1 对应能力

- `09-实施计划`
  - `Wave B / Step 05 / CP05-2`
- `130-连接优先的AI时代即时通讯架构蓝图`
  - 消息主链路 authority owner 收口
- `136-关键业务链路与跨Plane时序设计`
  - `AuthContext -> command -> sender snapshot` 边界时序
- `139-权限能力模型与协议演进设计`
  - `sender` 作为服务端 authority snapshot 的收口
- `147-CCP到Crate与接口模块落地映射设计`
  - `conversation-runtime` command boundary 对 authority 映射的职责承接

### 6.2 已兑现

- 消息变更路径的 `sender` 快照构造不再分散在 HTTP 和 sdkwork-im-server 各入口。
- system-channel publish 与普通 message mutation 共享同一套 authority snapshot 构造。
- `device_id / session_id / actor identity` 在四类消息变更命令上的映射来源统一。

### 6.3 未兑现

- `tenantId` 在 `Step 05` 全主链路上仍未完成统一 owner 收口。
- conversation create/member/read-cursor 等非 message mutation 路径仍存在 authority 字段采集分散问题。
- `CP05-3`
  - direct / group / channel 重新收口未做完
- `CP05-4`
  - projection / notification / multi-client-route sync owner 重新收口未做完

## 7. 偏离检查

- 本轮没有偏离架构文档定义的方向。
- 本轮采取的是较小真实增量:
  - 保持 command DTO 形状稳定
  - 先把 `AuthContext -> Sender` 统一收口到 command boundary
- 没有把这次增量误判为 `CP05-2` 全部完成。

## 8. 当前结论

- 本轮增量已完成:
  - 代码已落地
  - 测试已补齐
  - 验证证据已形成
  - review 证据已补齐
  - 架构回写已补齐
- 但真实状态仍然是:
  - `CP05-2` 未闭环
  - `Step 05` 未闭环
  - `91 / 95 / 97` 对 `Step 05` 仍未通过
  - `Wave B / 93` 仍被阻塞

## 9. 下一轮继续做什么

- 继续推进 `Wave B / Step 05 / CP05-2`
- 优先检查并收口以下路径的 authority capture:
  - conversation create
  - member governance
  - read-cursor
- 继续坚持:
  - 先红测
  - 再最小实现
  - 再全量验证

