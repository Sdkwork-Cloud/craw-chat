# Step 05 CP05-2 Non-Message Command AuthContext 质量审计 - 2026-04-07

## 1. 审计范围

- `services/conversation-runtime/src/runtime.rs`
- `services/conversation-runtime/src/runtime/http.rs`
- `services/sdkwork-im-cloud-gateway/src/node/conversation.rs`
- `services/sdkwork-im-cloud-gateway/src/node/handoff.rs`
- `services/sdkwork-im-cloud-gateway/src/node/membership.rs`
- `services/sdkwork-im-cloud-gateway/src/node/projection.rs`
- `services/conversation-runtime/tests/authority_command_test.rs`
- `services/conversation-runtime/tests/conversation_domain_structure_test.rs`
- `services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`

## 2. 本轮主要质量判断

### 2.1 已消除的风险

- 已消除非 message command 在 HTTP 和 sdkwork-im-server 入口重复采集 `tenant_id / actor_id` 的问题。
- 已消除 create/member/handoff/read-cursor 命令在多个入口分别内联 authority field 的结构漂移风险。
- 已把非 message command authority capture 与 message mutation authority capture 放到同一个 runtime command-boundary 模式下，减少 Step 05 内部边界分叉。

### 2.2 本轮未发现的回归

- 未发现 `conversation-runtime` HTTP 写路径因替换为 `from_auth_context(...)` 构造器而产生结构性回归。
- 未发现 sdkwork-im-server conversation/handoff/membership/projection 路径因替换命令构造方式而出现调用缺口。
- 未发现新增构造器覆盖后，非 message command 的 `tenant_id / actor_id` identity 映射出现字段丢失。

## 3. 仍然存在的真实风险

### 3.1 CP05-2 仍未闭环

- `actor_kind` 仍以独立参数形式继续穿透到 runtime:
  - `create_conversation_with_creator_kind(...)`
  - `create_agent_dialog_with_requester_kind(...)`
  - `create_agent_handoff_with_source_kind(...)`
  - `add_member_with_actor_kind(...)`
  - `update_read_cursor_with_actor_kind(...)`
  - 以及 handoff / membership 相关 runtime surface
- 这意味着 authority owner 还不是完全统一对象。

### 3.2 读侧 authority surface 仍分散

- `get_agent_handoff_state`
- `list_members`
- `get_read_cursor`
- `read_cursor_view`

这些读查询路径仍直接线程化传递 `auth.tenant_id / auth.actor_id`，尚未形成统一的 authority query owner。

### 3.3 波次级风险仍在

- `Step 05` 未闭环，`Step 06` 也未闭环。
- 因此 `Wave B / 93` 仍不能启动总验收。

## 4. 证据

### 4.1 单点验证

- `cargo test -p conversation-runtime --test conversation_domain_structure_test test_non_message_commands_offer_auth_context_constructors --offline`
- `cargo test -p conversation-runtime --test authority_command_test test_non_message_commands_from_auth_context_preserve_authority_identity --offline`
- `cargo test -p sdkwork-im-cloud-gateway --test lib_structure_test test_local_minimal_node_non_message_paths_use_auth_context_command_constructors --offline`

### 4.2 全量受影响服务验证

- `rustfmt --edition 2024 --check services/conversation-runtime/src/runtime.rs services/conversation-runtime/src/runtime/http.rs services/conversation-runtime/tests/authority_command_test.rs services/conversation-runtime/tests/conversation_domain_structure_test.rs services/sdkwork-im-cloud-gateway/src/node/conversation.rs services/sdkwork-im-cloud-gateway/src/node/handoff.rs services/sdkwork-im-cloud-gateway/src/node/membership.rs services/sdkwork-im-cloud-gateway/src/node/projection.rs services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`
- `cargo test -p conversation-runtime --offline`
- `cargo test -p sdkwork-im-cloud-gateway --offline`
- `cargo test -p projection-service --offline`

## 5. 审计结论

- 本轮可以认定为 `CP05-2` 的第二个真实增量。
- 本轮不能认定为 `CP05-2` 已闭环。
- 本轮不能认定为 `Step 05` 已通过 `91 / 95 / 97`。
- 当前最合理结论仍然是:
  - `Wave B / Step 05 / CP05-2`
  - 已继续前进
  - 仍未闭环

## 6. 下一步审计关注点

- 能否继续把 `actor_kind` 从入口独立参数收口到同一 authority owner
- 能否把读查询 surface 的 `auth.tenant_id / auth.actor_id` 收敛到统一 query boundary
- downstream projection / notification / multi-client-route sync 是否继续消费统一 authority snapshot
