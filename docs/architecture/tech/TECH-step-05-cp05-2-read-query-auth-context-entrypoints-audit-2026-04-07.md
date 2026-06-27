> Migrated from `docs/review/step-05-cp05-2-read-query-auth-context-entrypoints-质量审计-2026-04-07.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 05 CP05-2 Read Query AuthContext EntryPoints 质量审计 - 2026-04-07

## 1. 审计范围

- `services/conversation-runtime/src/runtime.rs`
- `services/conversation-runtime/src/runtime/membership.rs`
- `services/conversation-runtime/src/runtime/handoff.rs`
- `services/conversation-runtime/src/runtime/http.rs`
- `services/sdkwork-im-cloud-gateway/src/node/membership.rs`
- `services/sdkwork-im-cloud-gateway/src/node/handoff.rs`
- `services/sdkwork-im-cloud-gateway/src/node/access.rs`
- `services/conversation-runtime/tests/conversation_domain_structure_test.rs`
- `services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`

## 2. 本轮主要审计结论

### 2.1 已消除的风险

- runtime 自己持有的读查询 surface 不再要求每个 service entrypoint 重复传 `tenant_id / actor_id`。
- `conversation-runtime/http` 不再在 handoff/member/read-cursor 读路径上保留第二套 raw auth capture。
- `sdkwork-im-server` 的 membership/handoff/access 不再在这些 runtime-owned read surface 上重复拼装 authority。

### 2.2 本轮未发现的新回归

- 新增 wrapper 没有打破 `conversation-runtime` 结构测试与 authority 测试。
- `sdkwork-im-server` 的结构测试、HTTP/E2E、runtime-dir、websocket、cluster、projection 相关 suite 均继续通过。
- `projection-service` 全量测试继续通过，说明本轮未对投影服务引入回归。

## 3. 仍然存在的真实风险

### 3.1 CP05-2 仍未闭环

- 本轮只收口了 runtime-owned read query surface。
- `projection.rs` 里的 projection-service 查询仍直接线程化 `auth.tenant_id / auth.actor_id`。
- authority owner 仍没有扩展到所有 Step 05 读查询与 side-effect surface。

### 3.2 Step 05 级风险仍在

- `CP05-3` 尚未完成 direct / group / channel 的统一再验收。
- `CP05-4` 尚未完成 projection / notification / multi-client-route sync 的新 owner 收口。
- 因此 `Step 05` 仍不能通过 `91 / 95 / 97`。

## 4. 证据

### 4.1 Red

- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2d-red-runtime'; cargo test -p conversation-runtime --test conversation_domain_structure_test test_runtime_exposes_read_query_auth_context_entrypoints --offline`
- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2d-red-local-node'; cargo test -p sdkwork-im-cloud-gateway --test lib_structure_test test_local_minimal_node_read_query_paths_use_runtime_auth_context_entrypoints --offline`

### 4.2 Green

- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2d-green-runtime'; cargo test -p conversation-runtime --test conversation_domain_structure_test test_runtime_exposes_read_query_auth_context_entrypoints --offline`
- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2d-green-runtime-http'; cargo test -p conversation-runtime --test conversation_domain_structure_test test_http_read_query_surface_uses_runtime_auth_context_entrypoints --offline`
- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2d-green-local-node'; cargo test -p sdkwork-im-cloud-gateway --test lib_structure_test test_local_minimal_node_read_query_paths_use_runtime_auth_context_entrypoints --offline`

### 4.3 Fresh verification

- `rustfmt --edition 2024 --check services/conversation-runtime/src/runtime.rs services/conversation-runtime/src/runtime/membership.rs services/conversation-runtime/src/runtime/handoff.rs services/conversation-runtime/src/runtime/http.rs services/conversation-runtime/tests/conversation_domain_structure_test.rs services/sdkwork-im-cloud-gateway/src/node/membership.rs services/sdkwork-im-cloud-gateway/src/node/handoff.rs services/sdkwork-im-cloud-gateway/src/node/access.rs services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`
- `cargo test -p conversation-runtime --test conversation_domain_structure_test --offline`
- `cargo test -p conversation-runtime --test authority_command_test --offline`
- `cargo test -p conversation-runtime --offline`
- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2d-local-node-structure'; cargo test -p sdkwork-im-cloud-gateway --test lib_structure_test --offline`
- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2d-local-node-full'; cargo test -p sdkwork-im-cloud-gateway --offline`
- `cargo test -p projection-service --offline`

## 5. 审计结论

- 本轮可以判定为 `CP05-2` 的第四个真实增量。
- 本轮不能判定 `CP05-2` 已闭环。
- 本轮不能判定 `Step 05` 已闭环。
- 当前最合理结论仍然是:
  - `Wave B / Step 05 / CP05-2`
  - 继续推进中
  - 仍未闭环

## 6. 下一步审计关注点

- `services/sdkwork-im-cloud-gateway/src/node/projection.rs` 是否继续需要 authority query owner 收口。
- projection-service 读查询是否需要新的 auth-context boundary 或统一 query snapshot owner。
- `effects.rs / session.rs` 等下游面是否仍有 raw `AuthContext` 漂移。

