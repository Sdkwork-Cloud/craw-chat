> Migrated from `docs/review/step-05-cp05-2-non-message-command-auth-context-执行补充-2026-04-07.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 05 CP05-2 Non-Message Command AuthContext 执行补充 - 2026-04-07

## 1. 当前定位

- 当前波次: `Wave B`
- 当前 Step: `Step 05`
- 当前子卡: `CP05-2`
- 前置状态:
  - `CP05-1` 已闭环
  - `Step 04` 已闭环
  - `CP05-2 / CP05-3 / CP05-4` 仍未闭环

## 2. 本轮为什么继续做这一步

- 第一轮 `CP05-2` 只收口了 message mutation 路径上的 `AuthContext -> Sender` authority snapshot。
- `create / handoff / membership / read-cursor` 这些非 message command 仍然在多个入口重复采集:
  - `tenant_id`
  - `actor_id`
- 这会让 Step 05 的 authority capture 继续分散在:
  - `conversation-runtime` HTTP surface
  - `sdkwork-im-server` local entrypoints
- 因此本轮继续沿着同一个 `CP05-2` 方向推进，但只做真实的最小增量:
  - 把非 message command 的 `AuthContext -> command authority fields` 收口到 `conversation-runtime` command boundary。

## 3. 本轮实际完成

### 3.1 代码落地

- 在 `services/conversation-runtime/src/runtime.rs` 新增统一构造入口:
  - `CreateConversationCommand::from_auth_context(...)`
  - `CreateAgentDialogCommand::from_auth_context(...)`
  - `CreateAgentHandoffCommand::from_auth_context(...)`
  - `CreateSystemChannelCommand::from_auth_context(...)`
  - `AcceptAgentHandoffCommand::from_auth_context(...)`
  - `ResolveAgentHandoffCommand::from_auth_context(...)`
  - `CloseAgentHandoffCommand::from_auth_context(...)`
  - `AddConversationMemberCommand::from_auth_context(...)`
  - `RemoveConversationMemberCommand::from_auth_context(...)`
  - `LeaveConversationCommand::from_auth_context(...)`
  - `TransferConversationOwnerCommand::from_auth_context(...)`
  - `ChangeConversationMemberRoleCommand::from_auth_context(...)`
  - `UpdateReadCursorCommand::from_auth_context(...)`
- 在 `services/conversation-runtime/src/runtime/http.rs` 删除非 message command 的入口级 authority 内联采集，改为统一调用上述构造器。
- 在 `services/sdkwork-im-cloud-gateway/src/node/conversation.rs` 改为统一使用 create 系列 `from_auth_context(...)` 构造器。
- 在 `services/sdkwork-im-cloud-gateway/src/node/handoff.rs` 改为统一使用 handoff 系列 `from_auth_context(...)` 构造器。
- 在 `services/sdkwork-im-cloud-gateway/src/node/membership.rs` 改为统一使用 membership governance 系列 `from_auth_context(...)` 构造器。
- 在 `services/sdkwork-im-cloud-gateway/src/node/projection.rs` 改为统一使用 `UpdateReadCursorCommand::from_auth_context(...)`。

### 3.2 测试补齐

- `services/conversation-runtime/tests/authority_command_test.rs`
  - 新增 `test_non_message_commands_from_auth_context_preserve_authority_identity`
- `services/conversation-runtime/tests/conversation_domain_structure_test.rs`
  - 新增 `test_non_message_commands_offer_auth_context_constructors`
  - 新增 `test_http_non_message_surface_uses_auth_context_command_constructors`
- `services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`
  - 新增 `test_local_minimal_node_non_message_paths_use_auth_context_command_constructors`

## 4. 涉及文件

### 4.1 代码

- `services/conversation-runtime/src/runtime.rs`
- `services/conversation-runtime/src/runtime/http.rs`
- `services/sdkwork-im-cloud-gateway/src/node/conversation.rs`
- `services/sdkwork-im-cloud-gateway/src/node/handoff.rs`
- `services/sdkwork-im-cloud-gateway/src/node/membership.rs`
- `services/sdkwork-im-cloud-gateway/src/node/projection.rs`

### 4.2 测试

- `services/conversation-runtime/tests/authority_command_test.rs`
- `services/conversation-runtime/tests/conversation_domain_structure_test.rs`
- `services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`

## 5. 验证证据

### 5.1 Red/Green 证据

- Red
  - `cargo test -p conversation-runtime --test conversation_domain_structure_test test_non_message_commands_offer_auth_context_constructors --offline`
  - `cargo test -p conversation-runtime --test authority_command_test test_non_message_commands_from_auth_context_preserve_authority_identity --offline`
  - `cargo test -p sdkwork-im-cloud-gateway --test lib_structure_test test_local_minimal_node_non_message_paths_use_auth_context_command_constructors --offline`
- Green
  - `cargo test -p conversation-runtime --test conversation_domain_structure_test test_non_message_commands_offer_auth_context_constructors --offline`
  - `cargo test -p conversation-runtime --test authority_command_test test_non_message_commands_from_auth_context_preserve_authority_identity --offline`
  - `cargo test -p sdkwork-im-cloud-gateway --test lib_structure_test test_local_minimal_node_non_message_paths_use_auth_context_command_constructors --offline`

### 5.2 完整回归证据

- `rustfmt --edition 2024 services/conversation-runtime/src/runtime.rs services/conversation-runtime/src/runtime/http.rs services/conversation-runtime/tests/authority_command_test.rs services/conversation-runtime/tests/conversation_domain_structure_test.rs services/sdkwork-im-cloud-gateway/src/node/conversation.rs services/sdkwork-im-cloud-gateway/src/node/handoff.rs services/sdkwork-im-cloud-gateway/src/node/membership.rs services/sdkwork-im-cloud-gateway/src/node/projection.rs services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`
- `rustfmt --edition 2024 --check services/conversation-runtime/src/runtime.rs services/conversation-runtime/src/runtime/http.rs services/conversation-runtime/tests/authority_command_test.rs services/conversation-runtime/tests/conversation_domain_structure_test.rs services/sdkwork-im-cloud-gateway/src/node/conversation.rs services/sdkwork-im-cloud-gateway/src/node/handoff.rs services/sdkwork-im-cloud-gateway/src/node/membership.rs services/sdkwork-im-cloud-gateway/src/node/projection.rs services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`
- `cargo test -p conversation-runtime --offline`
- `cargo test -p sdkwork-im-cloud-gateway --offline`
- `cargo test -p projection-service --offline`

### 5.3 验证结论

- 本轮增量的代码、结构测试和受影响服务测试都已有 fresh evidence。
- `sdkwork-im-server` 全量测试中仍会输出预期的启动失败/health timeout 示例日志，但命令退出码为 `0`，不构成本轮阻塞。

## 6. 对应架构能力与兑现判断

### 6.1 对应能力

- `09-实施计划`
  - `Wave B / Step 05 / CP05-2`
- `130-连接优先的AI时代即时通讯架构蓝图`
  - Step 05 authority capture 继续从入口级 glue 收口到统一边界
- `136-关键业务链路与跨Plane时序设计`
  - `AuthContext -> non-message command` 的 authority timing 统一
- `139-权限能力模型与协议演进设计`
  - 非 message command 的 `tenant_id / actor_id` 不再由入口层各自采集
- `147-CCP到Crate与接口模块落地映射设计`
  - `conversation-runtime` command boundary 继续承接 authority 映射职责

### 6.2 已兑现

- 非 message command 的 authority field capture 已开始统一收口到 runtime command boundary。
- HTTP 与 sdkwork-im-server 不再分别内联组装 create/member/handoff/read-cursor command 的 `tenant_id / actor_id`。
- `conversation-runtime` 对 Step 05 非消息写路径形成了一致的 `AuthContext -> command` 映射面。

### 6.3 未兑现

- `actor_kind` 仍然在多个入口作为独立参数继续穿透到 runtime，而不是和 command authority 完全一体化收口。
- 读查询路径仍然大量直接线程化使用 `auth.tenant_id / auth.actor_id`。
- `CP05-3`
  - direct / group / channel 重新收口尚未开始闭环
- `CP05-4`
  - projection / notification / multi-client-route sync owner 重新收口尚未开始闭环

## 7. 偏离检查

- 本轮没有偏离架构文档方向。
- 本轮继续采用小步真实增量，而不是扩大 command DTO 形状改造范围。
- 本轮没有把阶段推进误报为 `CP05-2` 已闭环。

## 8. 当前结论

- 本轮增量已完成:
  - 代码已落地
  - 测试已补齐
  - 验证证据已形成
  - `docs/review` 已补齐
  - `docs/架构` 回写需要同步补齐
- 但真实状态仍然是:
  - `CP05-2` 未闭环
  - `Step 05` 未闭环
  - `91 / 95 / 97` 对 `Step 05` 仍未通过
  - `Wave B / 93` 仍被阻塞

## 9. 下一轮继续做什么

- 继续停留在 `Wave B / Step 05 / CP05-2`
- 优先检查并收口下一批 authority gap:
  - `actor_kind` 独立穿透
  - 读查询路径上的 `auth.tenant_id / auth.actor_id`
  - downstream side effects 对原始 `AuthContext` 的依赖面

