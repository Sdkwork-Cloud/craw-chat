> Migrated from `docs/review/step-05-cp05-2-read-query-auth-context-entrypoints-执行补充-2026-04-07.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 05 CP05-2 Read Query AuthContext EntryPoints 执行补充 - 2026-04-07

## 1. 当前定位

- 当前波次: `Wave B`
- 当前 Step: `Step 05`
- 当前子卡: `CP05-2`
- 当前真实状态:
  - `CP05-1` 已闭环
  - `CP05-2` 已完成前三个真实增量，但整体仍未闭环
  - `CP05-3 / CP05-4` 仍未闭环
  - `Step 05` 仍未闭环
  - `Wave B / 93` 仍阻塞

## 2. 本轮为什么继续做这个增量

- 上一轮已经把 non-message mutation 的 command field capture 和 `actor_kind` capture 收口到 `conversation-runtime`。
- 但 runtime 自己持有的读查询 surface 仍然存在明显 authority 漂移:
  - `conversation-runtime/http` 还在直接线程化 `auth.tenant_id / auth.actor_id`
  - `sdkwork-im-server` 的 membership/handoff/access 也还在直接传这些字段
- 因此本轮继续停留在 `CP05-2`，做最小但真实的下一步:
  - 让 runtime 自己拥有 runtime-owned read query 的 auth-context 入口
  - 让 HTTP 与 local node 消费这些入口

## 3. 本轮实际完成

### 3.1 新增 runtime 读查询 auth-context 入口

- `services/conversation-runtime/src/runtime.rs`
  - `require_active_member_from_auth_context(...)`
- `services/conversation-runtime/src/runtime/membership.rs`
  - `list_members_from_auth_context(...)`
  - `read_cursor_view_from_auth_context(...)`
- `services/conversation-runtime/src/runtime/handoff.rs`
  - `get_agent_handoff_state_from_auth_context(...)`

### 3.2 切换 conversation-runtime HTTP 读查询入口

- `services/conversation-runtime/src/runtime/http.rs`
  - `get_agent_handoff_state(...)` 改为调用 `get_agent_handoff_state_from_auth_context(...)`
  - `list_members(...)` 改为调用 `list_members_from_auth_context(...)`
  - `get_read_cursor(...)` 改为调用 `read_cursor_view_from_auth_context(...)`
  - `update_read_cursor(...)` 的回读改为调用 `read_cursor_view_from_auth_context(...)`

### 3.3 切换 sdkwork-im-server 对 runtime-owned read surface 的消费方式

- `services/sdkwork-im-cloud-gateway/src/node/membership.rs`
  - `list_members(...)` 改为调用 `list_members_from_auth_context(...)`
- `services/sdkwork-im-cloud-gateway/src/node/handoff.rs`
  - `get_agent_handoff_state(...)` 以及 accept/resolve/close 前的 previous-state 读取，全部改为调用 `get_agent_handoff_state_from_auth_context(...)`
- `services/sdkwork-im-cloud-gateway/src/node/access.rs`
  - `ensure_conversation_member(...)` 改为调用 `require_active_member_from_auth_context(...)`
  - `resolve_conversation_actor_auth_context(...)` 改为调用 `require_active_member_from_auth_context(...)`

### 3.4 本轮测试补齐

- `services/conversation-runtime/tests/conversation_domain_structure_test.rs`
  - 新增 `test_runtime_exposes_read_query_auth_context_entrypoints`
  - 新增 `test_http_read_query_surface_uses_runtime_auth_context_entrypoints`
- `services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`
  - 新增 `test_local_minimal_node_read_query_paths_use_runtime_auth_context_entrypoints`

## 4. 涉及文件

### 4.1 代码

- `services/conversation-runtime/src/runtime.rs`
- `services/conversation-runtime/src/runtime/membership.rs`
- `services/conversation-runtime/src/runtime/handoff.rs`
- `services/conversation-runtime/src/runtime/http.rs`
- `services/sdkwork-im-cloud-gateway/src/node/membership.rs`
- `services/sdkwork-im-cloud-gateway/src/node/handoff.rs`
- `services/sdkwork-im-cloud-gateway/src/node/access.rs`

### 4.2 测试

- `services/conversation-runtime/tests/conversation_domain_structure_test.rs`
- `services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`

## 5. TDD 与验证证据

### 5.1 Red

- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2d-red-runtime'; cargo test -p conversation-runtime --test conversation_domain_structure_test test_runtime_exposes_read_query_auth_context_entrypoints --offline`
- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2d-red-local-node'; cargo test -p sdkwork-im-cloud-gateway --test lib_structure_test test_local_minimal_node_read_query_paths_use_runtime_auth_context_entrypoints --offline`

Red 失败原因符合预期:

- runtime 尚未暴露读查询 auth-context wrapper
- sdkwork-im-server 尚未切到这些 wrapper

### 5.2 Green

- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2d-green-runtime'; cargo test -p conversation-runtime --test conversation_domain_structure_test test_runtime_exposes_read_query_auth_context_entrypoints --offline`
- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2d-green-runtime-http'; cargo test -p conversation-runtime --test conversation_domain_structure_test test_http_read_query_surface_uses_runtime_auth_context_entrypoints --offline`
- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2d-green-local-node'; cargo test -p sdkwork-im-cloud-gateway --test lib_structure_test test_local_minimal_node_read_query_paths_use_runtime_auth_context_entrypoints --offline`

### 5.3 Fresh verification

- `rustfmt --edition 2024 services/conversation-runtime/src/runtime.rs services/conversation-runtime/src/runtime/membership.rs services/conversation-runtime/src/runtime/handoff.rs services/conversation-runtime/src/runtime/http.rs services/conversation-runtime/tests/conversation_domain_structure_test.rs services/sdkwork-im-cloud-gateway/src/node/membership.rs services/sdkwork-im-cloud-gateway/src/node/handoff.rs services/sdkwork-im-cloud-gateway/src/node/access.rs services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`
- `rustfmt --edition 2024 --check services/conversation-runtime/src/runtime.rs services/conversation-runtime/src/runtime/membership.rs services/conversation-runtime/src/runtime/handoff.rs services/conversation-runtime/src/runtime/http.rs services/conversation-runtime/tests/conversation_domain_structure_test.rs services/sdkwork-im-cloud-gateway/src/node/membership.rs services/sdkwork-im-cloud-gateway/src/node/handoff.rs services/sdkwork-im-cloud-gateway/src/node/access.rs services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`
- `cargo test -p conversation-runtime --test conversation_domain_structure_test --offline`
- `cargo test -p conversation-runtime --test authority_command_test --offline`
- `cargo test -p conversation-runtime --offline`
- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2d-local-node-structure'; cargo test -p sdkwork-im-cloud-gateway --test lib_structure_test --offline`
- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2d-local-node-full'; cargo test -p sdkwork-im-cloud-gateway --offline`
- `cargo test -p projection-service --offline`

说明:

- `sdkwork-im-server` 全量测试仍会打印预期的 startup-failure / health-timeout 示例日志，但 suite 退出码为 `0`，不是阻塞项。

## 6. 对 Step 05 的影响

- 本轮是 `CP05-2` 的第四个真实增量。
- 已兑现:
  - runtime-owned read query surface 开始从 service entrypoint 的 raw auth threading 收口到 runtime auth-context boundary
  - `conversation-runtime/http` 与 `sdkwork-im-server` 对这些 runtime-owned 读查询不再重复传 `tenant_id / actor_id`
- 未兑现:
  - projection-service 读查询仍直接消费 `tenant_id / actor_id`
  - downstream side-effects 对 raw `AuthContext` 的剩余依赖尚未清完
  - `CP05-2` 整体仍未闭环
  - `CP05-3 / CP05-4` 仍未闭环

## 7. 当前结论

- 本轮代码、测试、验证、`docs/review` 已补齐。
- 本轮之后的真实状态仍然是:
  - `Wave B / Step 05 / CP05-2`
  - `CP05-2` 继续推进，但未闭环
  - `Step 05` 未闭环
  - `91 / 95 / 97` 对 `Step 05` 仍不能判定通过
  - `Wave B / 93` 仍不能启动

## 8. 下一轮继续做什么

- 继续停留在 `Wave B / Step 05 / CP05-2`
- 优先补下一最小缺口:
  - projection-service 读查询 authority owner 是否需要继续收口
  - sdkwork-im-server `projection.rs` 中仍直接线程化的 `tenant_id / actor_id`
  - downstream side-effects 是否仍直接消费 raw `AuthContext`

