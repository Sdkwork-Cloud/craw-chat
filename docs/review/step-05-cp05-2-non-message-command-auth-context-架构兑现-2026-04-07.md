# Step 05 CP05-2 Non-Message Command AuthContext 架构兑现 - 2026-04-07

## 1. 对照架构文档

- `docs/架构/09-实施计划.md`
- `docs/架构/130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md`
- `docs/架构/136-关键业务链路与跨Plane时序设计-2026-04-06.md`
- `docs/架构/139-权限能力模型与协议演进设计-2026-04-06.md`
- `docs/架构/147-CCP到Crate与接口模块落地映射设计-2026-04-06.md`

## 2. 本轮兑现的能力

### 2.1 non-message command authority capture 收口

- `conversation-runtime` 为以下 command 提供统一 `from_auth_context(...)` 构造器:
  - `CreateConversationCommand`
  - `CreateAgentDialogCommand`
  - `CreateAgentHandoffCommand`
  - `CreateSystemChannelCommand`
  - `AcceptAgentHandoffCommand`
  - `ResolveAgentHandoffCommand`
  - `CloseAgentHandoffCommand`
  - `AddConversationMemberCommand`
  - `RemoveConversationMemberCommand`
  - `LeaveConversationCommand`
  - `TransferConversationOwnerCommand`
  - `ChangeConversationMemberRoleCommand`
  - `UpdateReadCursorCommand`
- authority field capture 继续从分散入口收口到 command boundary。

### 2.2 入口层重复实现移除

- `conversation-runtime/http`
  - 不再手工内联组装 non-message command 的 `tenant_id / actor_id`
- `sdkwork-im-server`
  - conversation / handoff / membership / projection 写路径不再各自重复组装同类 authority 字段

## 3. 本轮未兑现的能力

- `actor_kind` 仍未和 command authority 完整收口为同一 owner
- 读查询 authority surface 仍未统一
- `CP05-3`
  - direct / group / channel 重新收口未开始闭环
- `CP05-4`
  - projection / notification / multi-client-route sync owner 重新收口未开始闭环

## 4. 是否偏离架构

- 未偏离。
- 本轮是沿着既有 `CP05-2` 方向继续做 command-boundary 收口，而不是引入新的 authority 模型分支。
- 本轮没有扩大 DTO 形状改造范围，也没有跳过 Step 05 既定依赖顺序。

## 5. 证据

### 5.1 代码证据

- `services/conversation-runtime/src/runtime.rs`
- `services/conversation-runtime/src/runtime/http.rs`
- `services/sdkwork-im-cloud-gateway/src/node/conversation.rs`
- `services/sdkwork-im-cloud-gateway/src/node/handoff.rs`
- `services/sdkwork-im-cloud-gateway/src/node/membership.rs`
- `services/sdkwork-im-cloud-gateway/src/node/projection.rs`

### 5.2 测试证据

- `services/conversation-runtime/tests/authority_command_test.rs`
- `services/conversation-runtime/tests/conversation_domain_structure_test.rs`
- `services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`

### 5.3 fresh verification 证据

- `cargo test -p conversation-runtime --test conversation_domain_structure_test test_non_message_commands_offer_auth_context_constructors --offline`
- `cargo test -p conversation-runtime --test authority_command_test test_non_message_commands_from_auth_context_preserve_authority_identity --offline`
- `cargo test -p sdkwork-im-cloud-gateway --test lib_structure_test test_local_minimal_node_non_message_paths_use_auth_context_command_constructors --offline`
- `cargo test -p conversation-runtime --offline`
- `cargo test -p sdkwork-im-cloud-gateway --offline`
- `cargo test -p projection-service --offline`

## 6. 架构结论

- 本轮可以认定:
  - `CP05-2` 获得了第二段真实架构兑现
  - non-message command 的 authority capture 已开始和 message mutation 路径收敛到同一类 command-boundary owner
- 本轮不能认定:
  - `CP05-2` 已架构闭环
  - `Step 05` 已架构闭环
- 当前真实结论:
  - `CP05-2` 进行中
  - `Step 05` 进行中
  - `97` 对 `Step 05` 暂未通过
