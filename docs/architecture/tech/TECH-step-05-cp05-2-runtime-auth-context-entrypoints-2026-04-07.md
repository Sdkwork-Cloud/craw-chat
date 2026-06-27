> Migrated from `docs/review/step-05-cp05-2-runtime-auth-context-entrypoints-执行补充-2026-04-07.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 05 CP05-2 Runtime AuthContext EntryPoints 执行补充 - 2026-04-07

## 1. 当前定位

- 当前波次: `Wave B`
- 当前 Step: `Step 05`
- 当前子卡: `CP05-2`
- 前置状态:
  - `CP05-1` 已闭环
  - `CP05-2` 已完成 message mutation authority mapper
  - `CP05-2` 已完成 non-message command field capture 收口
  - `CP05-2 / CP05-3 / CP05-4` 仍未整体闭环

## 2. 本轮为什么继续做这一步

- 上一轮已经把非 message command 的 `tenant_id / actor_id` 采集收口到 `conversation-runtime` command constructor。
- 但 HTTP 与 sdkwork-im-server 入口仍然在大量调用:
  - `*_with_actor_kind(...)`
  - `*_with_creator_kind(...)`
  - `*_with_requester_kind(...)`
  - `*_with_source_kind(...)`
- 这意味着 `actor_kind` 仍然是入口层独立穿透的 authority surface，而不是真正归到 runtime 边界。
- 因此本轮继续做 `CP05-2` 的最小真实收口:
  - 给 `conversation-runtime` 增加 non-message mutation 的 `*_from_auth_context(...)` 入口
  - 让 runtime 同时承接 command field capture 和 `actor_kind` capture
  - 删除 HTTP/sdkwork-im-server 对旧 `*with_*kind` 入口的依赖

## 3. 本轮实际完成

### 3.1 代码落地

- 在 `services/conversation-runtime/src/runtime/creation.rs` 新增:
  - `create_conversation_from_auth_context(...)`
  - `create_agent_dialog_from_auth_context(...)`
  - `create_agent_handoff_from_auth_context(...)`
  - `create_system_channel_from_auth_context(...)`
- 在 `services/conversation-runtime/src/runtime/membership.rs` 新增:
  - `add_member_from_auth_context(...)`
  - `remove_member_from_auth_context(...)`
  - `leave_conversation_from_auth_context(...)`
  - `transfer_conversation_owner_from_auth_context(...)`
  - `change_conversation_member_role_from_auth_context(...)`
  - `update_read_cursor_from_auth_context(...)`
- 在 `services/conversation-runtime/src/runtime/handoff.rs` 新增:
  - `accept_agent_handoff_from_auth_context(...)`
  - `resolve_agent_handoff_from_auth_context(...)`
  - `close_agent_handoff_from_auth_context(...)`
- 在 `services/conversation-runtime/src/runtime/http.rs` 所有 non-message mutation 路径改为调用上述 runtime auth-context entrypoint。
- 在 `services/sdkwork-im-cloud-gateway/src/node/conversation.rs`、`handoff.rs`、`membership.rs`、`projection.rs` 全部改为调用上述 runtime auth-context entrypoint。
- 在 `services/sdkwork-im-cloud-gateway/src/node.rs` 清理了这次边界前移后不再使用的 conversation command imports。

### 3.2 测试补齐

- `services/conversation-runtime/tests/conversation_domain_structure_test.rs`
  - 新增 `test_runtime_exposes_non_message_auth_context_entrypoints`
  - 新增 `test_http_non_message_surface_uses_runtime_auth_context_entrypoints`
  - 同步升级旧的 non-message boundary 断言，禁止 HTTP 继续走旧 `*with_*kind` 入口
- `services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`
  - 新增 `test_local_minimal_node_non_message_paths_use_runtime_auth_context_entrypoints`
  - 同步升级旧的 non-message boundary 断言，禁止 sdkwork-im-server 继续走旧 `*with_*kind` 入口

## 4. 涉及文件

### 4.1 代码

- `services/conversation-runtime/src/runtime/creation.rs`
- `services/conversation-runtime/src/runtime/membership.rs`
- `services/conversation-runtime/src/runtime/handoff.rs`
- `services/conversation-runtime/src/runtime/http.rs`
- `services/sdkwork-im-cloud-gateway/src/node.rs`
- `services/sdkwork-im-cloud-gateway/src/node/conversation.rs`
- `services/sdkwork-im-cloud-gateway/src/node/handoff.rs`
- `services/sdkwork-im-cloud-gateway/src/node/membership.rs`
- `services/sdkwork-im-cloud-gateway/src/node/projection.rs`

### 4.2 测试

- `services/conversation-runtime/tests/conversation_domain_structure_test.rs`
- `services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`
- `services/conversation-runtime/tests/authority_command_test.rs`

## 5. 验证证据

### 5.1 Red/Green 证据

- Red
  - `cargo test -p conversation-runtime --test conversation_domain_structure_test test_runtime_exposes_non_message_auth_context_entrypoints --offline`
  - `cargo test -p conversation-runtime --test conversation_domain_structure_test test_http_non_message_surface_uses_runtime_auth_context_entrypoints --offline`
- Green
  - `cargo test -p conversation-runtime --test conversation_domain_structure_test --offline`
  - `cargo test -p conversation-runtime --test authority_command_test --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2c-local-node'; cargo test -p sdkwork-im-cloud-gateway --test lib_structure_test --offline`

### 5.2 完整回归证据

- `rustfmt --edition 2024 services/conversation-runtime/src/runtime/creation.rs services/conversation-runtime/src/runtime/membership.rs services/conversation-runtime/src/runtime/handoff.rs services/conversation-runtime/src/runtime/http.rs services/conversation-runtime/tests/conversation_domain_structure_test.rs services/sdkwork-im-cloud-gateway/src/node.rs services/sdkwork-im-cloud-gateway/src/node/conversation.rs services/sdkwork-im-cloud-gateway/src/node/membership.rs services/sdkwork-im-cloud-gateway/src/node/handoff.rs services/sdkwork-im-cloud-gateway/src/node/projection.rs services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`
- `rustfmt --edition 2024 --check services/conversation-runtime/src/runtime/creation.rs services/conversation-runtime/src/runtime/membership.rs services/conversation-runtime/src/runtime/handoff.rs services/conversation-runtime/src/runtime/http.rs services/conversation-runtime/tests/conversation_domain_structure_test.rs services/sdkwork-im-cloud-gateway/src/node.rs services/sdkwork-im-cloud-gateway/src/node/conversation.rs services/sdkwork-im-cloud-gateway/src/node/membership.rs services/sdkwork-im-cloud-gateway/src/node/handoff.rs services/sdkwork-im-cloud-gateway/src/node/projection.rs services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`
- `cargo test -p conversation-runtime --offline`
- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2c-local-node-full'; cargo test -p sdkwork-im-cloud-gateway --offline`
- `cargo test -p projection-service --offline`

### 5.3 验证结论

- 本轮代码、结构测试、conversation-runtime 全量测试、sdkwork-im-server 全量测试、projection-service 全量测试均已形成 fresh evidence。
- `sdkwork-im-server` 全量测试中仍会出现预期的启动失败/health timeout 示例日志，但该 suite 退出码为 `0`，不构成本轮阻塞。

## 6. 对应架构能力与兑现判断

### 6.1 对应能力

- `09-实施计划`
  - `Wave B / Step 05 / CP05-2`
- `130-连接优先的AI时代即时通讯架构蓝图`
  - Step 05 non-message mutation authority surface 继续从入口 glue 收口到 runtime
- `136-关键业务链路与跨Plane时序设计`
  - non-message mutation 的 `actor_kind` timing 不再由入口层显式线程化
- `139-权限能力模型与协议演进设计`
  - non-message mutation authority owner 继续向单一 runtime auth-context boundary 收敛
- `147-CCP到Crate与接口模块落地映射设计`
  - runtime crate 继续承接 non-message authority boundary

### 6.2 已兑现

- HTTP 和 sdkwork-im-server 的 non-message mutation 路径不再直接调用旧 `*with_*kind` runtime 入口。
- `conversation-runtime` 自身开始同时承接:
  - command field capture
  - `actor_kind` capture
- `CP05-2` 的 non-message write surface 在 runtime boundary 上进一步统一。

### 6.3 未兑现

- `actor_kind` 仍未和 command DTO 形状本体合并为单一 authority object，只是入口 capture 已收口到 runtime。
- 读查询路径仍大量直接线程化使用 `tenant_id / actor_id`。
- downstream projection / notification / multi-client-route sync 仍未围绕同一 authority owner 重新验收。

## 7. 偏离检查

- 本轮没有偏离架构方向。
- 本轮没有引入新的 authority 模型分支。
- 本轮仍坚持增量式收口，没有把 `CP05-2` 误报为已整体完成。

## 8. 当前结论

- 本轮增量已完成:
  - 代码已落地
  - 测试已补齐
  - 验证已完成
  - `docs/review` 已补齐
  - `docs/架构` 回写需要同步补齐
- 但真实状态仍然是:
  - `CP05-2` 未闭环
  - `Step 05` 未闭环
  - `91 / 95 / 97` 对 `Step 05` 仍未通过
  - `Wave B / 93` 仍被阻塞

## 9. 下一轮继续做什么

- 继续停留在 `Wave B / Step 05 / CP05-2`
- 优先检查:
  - 读查询 authority query owner
  - downstream side-effect 对 raw `AuthContext` 的剩余依赖
  - 是否需要把 `actor_kind` 进一步从 runtime wrapper 收口到更稳定的 authority snapshot owner

