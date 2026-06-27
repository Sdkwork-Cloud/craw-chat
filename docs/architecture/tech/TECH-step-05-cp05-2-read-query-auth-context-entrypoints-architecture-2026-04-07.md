> Migrated from `docs/review/step-05-cp05-2-read-query-auth-context-entrypoints-架构兑现-2026-04-07.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 05 CP05-2 Read Query AuthContext EntryPoints 架构兑现 - 2026-04-07

## 1. 对应架构文档

- `docs/架构/09-实施计划.md`
- `docs/架构/130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md`
- `docs/架构/136-关键业务链路与跨Plane时序设计-2026-04-06.md`
- `docs/架构/139-权限能力模型与协议演进设计-2026-04-06.md`
- `docs/架构/147-CCP到Crate与接口模块落地映射设计-2026-04-06.md`

## 2. 本轮兑现的能力

### 2.1 runtime 开始拥有 runtime-owned 读查询 authority 入口

- `conversation-runtime` 新增:
  - `require_active_member_from_auth_context(...)`
  - `list_members_from_auth_context(...)`
  - `read_cursor_view_from_auth_context(...)`
  - `get_agent_handoff_state_from_auth_context(...)`

这意味着:

- runtime 不只拥有 non-message write 的 auth-context boundary
- runtime 也开始拥有自己负责的 read query surface 的 authority boundary

### 2.2 service entrypoint 不再重复线程化同一份 read authority

- `conversation-runtime/http`
  - handoff state / members / read-cursor 路径已切到 runtime auth-context wrapper
- `sdkwork-im-server`
  - membership / handoff / access 路径已切到 runtime auth-context wrapper

这意味着:

- service entrypoint 不再在 runtime-owned read surface 上重复拼接 `tenant_id / actor_id`
- `CP05-2` 的 authority owner 继续从 service glue 向 runtime boundary 收口

## 3. 本轮未兑现的能力

- projection-service 读查询 authority 仍未收口。
- `sdkwork-im-server/src/node/projection.rs` 仍直接线程化 `tenant_id / actor_id`。
- `actor_kind` 仍未升级成更稳定的统一 authority snapshot owner。
- downstream side-effects 对 raw `AuthContext` 的剩余依赖仍未清空。
- 因此:
  - `CP05-2` 仍未闭环
  - `Step 05` 仍未闭环

## 4. 是否偏离架构

- 本轮未偏离架构方向。
- 本轮没有新增第二套 authority 模型。
- 本轮仍严格属于 `Wave B / Step 05 / CP05-2` 的增量收口，不是跨 step 跳转。

## 5. 证据

### 5.1 代码证据

- `services/conversation-runtime/src/runtime.rs`
- `services/conversation-runtime/src/runtime/membership.rs`
- `services/conversation-runtime/src/runtime/handoff.rs`
- `services/conversation-runtime/src/runtime/http.rs`
- `services/sdkwork-im-cloud-gateway/src/node/membership.rs`
- `services/sdkwork-im-cloud-gateway/src/node/handoff.rs`
- `services/sdkwork-im-cloud-gateway/src/node/access.rs`

### 5.2 测试证据

- `services/conversation-runtime/tests/conversation_domain_structure_test.rs`
- `services/conversation-runtime/tests/authority_command_test.rs`
- `services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`

### 5.3 Fresh verification

- `cargo test -p conversation-runtime --offline`
- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2d-local-node-full'; cargo test -p sdkwork-im-cloud-gateway --offline`
- `cargo test -p projection-service --offline`

## 6. 架构结论

- 本轮可认定:
  - `CP05-2` 获得第四段真实架构兑现
  - runtime-owned read query authority boundary 开始成形
- 本轮不可认定:
  - `CP05-2` 已架构闭环
  - `Step 05` 已架构闭环
- 当前真实结论:
  - `97` 对 `Step 05` 暂未通过
  - 需要继续回写 `docs/架构` 的增量 As-Built，但不能把 `Step 05` 标记为完成

