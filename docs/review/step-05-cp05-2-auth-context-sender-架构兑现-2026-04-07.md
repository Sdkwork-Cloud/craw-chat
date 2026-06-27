# Step 05 CP05-2 AuthContext Sender Snapshot 架构兑现 - 2026-04-07

## 1. 对照架构文档

- `docs/架构/09-实施计划.md`
- `docs/架构/130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md`
- `docs/架构/136-关键业务链路与跨Plane时序设计-2026-04-06.md`
- `docs/架构/139-权限能力模型与协议演进设计-2026-04-06.md`
- `docs/架构/147-CCP到Crate与接口模块落地映射设计-2026-04-06.md`

## 2. 本轮兑现的能力

### 2.1 命令边界 authority 收口

- `conversation-runtime` 为以下命令提供统一的 `from_auth_context(...)` 构造器:
  - `PostMessageCommand`
  - `PublishSystemChannelMessageCommand`
  - `EditMessageCommand`
  - `RecallMessageCommand`
- authority snapshot 的职责从分散入口收口到 command boundary。

### 2.2 入口侧重复实现移除

- `conversation-runtime/http`
  - 不再自行手工组装 `Sender`
- `sdkwork-im-server`
  - 不再维护本地 `build_sender(...)`
  - 不再在 message/effects 路径重复组装 `Sender`

### 2.3 authority 字段一致性提升

- `tenant_id`
- `sender.id`
- `sender.kind`
- `sender.device_id`
- `sender.session_id`

上述字段在消息变更主路径上改为由同一 authority 映射逻辑生成。

## 3. 本轮未兑现的能力

- `Step 05` 全主链路 authority owner 尚未统一:
  - create
  - member
  - read-cursor
- `tenantId` 仍未在所有 Step 05 command surface 上完成统一收口。
- `CP05-3`
  - direct / group / channel 重新收口尚未开始闭环
- `CP05-4`
  - projection / notification / multi-client-route sync owner 重新收口尚未开始闭环

## 4. 是否偏离架构

- 未偏离。
- 本轮采取的是符合现状的渐进式收口，而不是一次性改写所有 command DTO。
- 该做法与 Step 05 当前阶段目标一致:
  - 先去掉最明显的 authority snapshot 漂移源
  - 再继续向 create/member/read-cursor 扩展

## 5. 证据

### 5.1 代码证据

- `services/conversation-runtime/src/runtime.rs`
- `services/conversation-runtime/src/runtime/http.rs`
- `services/sdkwork-im-cloud-gateway/src/node.rs`
- `services/sdkwork-im-cloud-gateway/src/node/effects.rs`
- `services/sdkwork-im-cloud-gateway/src/node/message.rs`

### 5.2 测试证据

- `services/conversation-runtime/tests/conversation_domain_structure_test.rs`
- `services/conversation-runtime/tests/authority_command_test.rs`
- `services/sdkwork-im-cloud-gateway/tests/lib_structure_test.rs`

### 5.3 验证证据

- `cargo test -p conversation-runtime --test authority_command_test --offline`
- `cargo test -p conversation-runtime --offline`
- `cargo test -p sdkwork-im-cloud-gateway --offline`
- `cargo test -p projection-service --offline`

## 6. 架构结论

- 本轮可以认定:
  - `CP05-2` 已获得一段真实架构兑现
  - 消息变更命令边界上的 authority 收口开始落地
- 本轮不能认定:
  - `CP05-2` 已架构闭环
  - `Step 05` 已架构闭环
- 当前真实结论:
  - `CP05-2` 进行中
  - `Step 05` 进行中
  - `97` 对 `Step 05` 暂未通过
