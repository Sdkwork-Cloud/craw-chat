# Step 05 CP05-2 Runtime AuthContext EntryPoints 质量审计 - 2026-04-07

## 1. 审计范围

- `services/conversation-runtime/src/runtime/creation.rs`
- `services/conversation-runtime/src/runtime/membership.rs`
- `services/conversation-runtime/src/runtime/handoff.rs`
- `services/conversation-runtime/src/runtime/http.rs`
- `services/sdkwork-im-cloud-gateway/src/node.rs`
- `services/sdkwork-im-cloud-gateway/src/node/conversation.rs`
- `services/sdkwork-im-cloud-gateway/src/node/membership.rs`
- `services/sdkwork-im-cloud-gateway/src/node/handoff.rs`
- `services/sdkwork-im-cloud-gateway/src/node/projection.rs`
- `services/conversation-runtime/tests/conversation_domain_structure_test.rs`
- `services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`

## 2. 本轮主要质量判断

### 2.1 已消除的风险

- 已消除 HTTP 非 message mutation 路径对旧 `*with_*kind` runtime 入口的依赖。
- 已消除 sdkwork-im-server non-message mutation 路径对旧 `*with_*kind` runtime 入口的依赖。
- 已把 `actor_kind` capture 从入口层参数线程化，继续收口到 `conversation-runtime` 自身的 auth-context boundary。

### 2.2 本轮未发现的回归

- 未发现新增 runtime auth-context wrapper 打坏 conversation create/member/handoff/read-cursor 的行为测试。
- 未发现 local profile HTTP/E2E 测试因入口层切换到 runtime wrapper 而出现行为回归。
- 未发现 `sdkwork-im-server/src/node.rs` import 清理后产生新的未解析符号问题。

## 3. 仍然存在的真实风险

### 3.1 CP05-2 仍未闭环

- 本轮解决的是 non-message mutation 入口层 `actor_kind` 线程化问题。
- 但读查询路径上的 `tenant_id / actor_id` 仍未统一收口。
- 因此 authority owner 仍不是完整单体。

### 3.2 Step 级风险仍在

- `CP05-3`
  - direct / group / channel 的重新收口尚未开始闭环
- `CP05-4`
  - projection / notification / multi-client-route sync owner 重新收口尚未开始闭环
- 所以 `Step 05` 仍不能判定通过 `91 / 95 / 97`

## 4. 证据

### 4.1 单点验证

- `cargo test -p conversation-runtime --test conversation_domain_structure_test --offline`
- `cargo test -p conversation-runtime --test authority_command_test --offline`
- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2c-local-node'; cargo test -p sdkwork-im-cloud-gateway --test lib_structure_test --offline`

### 4.2 全量受影响服务验证

- `cargo test -p conversation-runtime --offline`
- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2c-local-node-full'; cargo test -p sdkwork-im-cloud-gateway --offline`
- `cargo test -p projection-service --offline`

## 5. 审计结论

- 本轮可以认定为 `CP05-2` 的第三个真实增量。
- 本轮不能认定为 `CP05-2` 已闭环。
- 本轮不能认定为 `Step 05` 已闭环。
- 当前最合理结论仍然是:
  - `Wave B / Step 05 / CP05-2`
  - 已继续推进
  - 仍未闭环

## 6. 下一步审计关注点

- 读查询 authority query owner 是否继续直接线程化 `tenant_id / actor_id`
- runtime wrapper 之后，是否仍有 downstream side-effect 直接消费 raw `AuthContext`
- 是否需要继续把 `actor_kind` 从 runtime wrapper 升级为更统一的 authority snapshot owner
