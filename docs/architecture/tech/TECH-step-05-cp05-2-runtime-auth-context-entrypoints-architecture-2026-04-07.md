> Migrated from `docs/review/step-05-cp05-2-runtime-auth-context-entrypoints-架构兑现-2026-04-07.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 05 CP05-2 Runtime AuthContext EntryPoints 架构兑现 - 2026-04-07

## 1. 对照架构文档

- `docs/架构/09-实施计划.md`
- `docs/架构/130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md`
- `docs/架构/136-关键业务链路与跨Plane时序设计-2026-04-06.md`
- `docs/架构/139-权限能力模型与协议演进设计-2026-04-06.md`
- `docs/架构/147-CCP到Crate与接口模块落地映射设计-2026-04-06.md`

## 2. 本轮兑现的能力

### 2.1 runtime owns auth-context-backed non-message mutation entrypoints

- `conversation-runtime` 新增 non-message mutation auth-context entrypoint:
  - create 系列
  - handoff status mutation 系列
  - membership governance 系列
  - read-cursor mutation
- runtime 自己完成:
  - command constructor 选择
  - `actor_kind` capture

### 2.2 service entrypoints stop threading old *with_*kind runtime APIs

- `conversation-runtime/http`
  - 不再显式走旧 `*with_*kind` 入口
- `sdkwork-im-server`
  - 不再显式走旧 `*with_*kind` 入口

## 3. 本轮未兑现的能力

- authority owner 仍未覆盖读查询边界
- `actor_kind` 仍未和 command snapshot 彻底合并成单一 authority object
- `CP05-3 / CP05-4` 仍未开始闭环

## 4. 是否偏离架构

- 未偏离。
- 本轮仍沿着 `CP05-2` 的同一条 runtime-boundary 收口路线推进。
- 本轮没有跳过 Step 05 顺序，也没有把兼容层重新做成新的 authority owner。

## 5. 证据

### 5.1 代码证据

- `services/conversation-runtime/src/runtime/creation.rs`
- `services/conversation-runtime/src/runtime/membership.rs`
- `services/conversation-runtime/src/runtime/handoff.rs`
- `services/conversation-runtime/src/runtime/http.rs`
- `services/sdkwork-im-cloud-gateway/src/node/conversation.rs`
- `services/sdkwork-im-cloud-gateway/src/node/membership.rs`
- `services/sdkwork-im-cloud-gateway/src/node/handoff.rs`
- `services/sdkwork-im-cloud-gateway/src/node/projection.rs`

### 5.2 测试证据

- `services/conversation-runtime/tests/conversation_domain_structure_test.rs`
- `services/conversation-runtime/tests/authority_command_test.rs`
- `services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`

### 5.3 fresh verification 证据

- `cargo test -p conversation-runtime --offline`
- `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step05-cp05-2c-local-node-full'; cargo test -p sdkwork-im-cloud-gateway --offline`
- `cargo test -p projection-service --offline`

## 6. 架构结论

- 本轮可以认定:
  - `CP05-2` 获得了第三段真实架构兑现
  - non-message mutation 入口层的 `actor_kind` capture 已继续向 runtime auth-context boundary 收口
- 本轮不能认定:
  - `CP05-2` 已架构闭环
  - `Step 05` 已架构闭环
- 当前真实结论:
  - `CP05-2` 进行中
  - `Step 05` 进行中
  - `97` 对 `Step 05` 暂未通过

