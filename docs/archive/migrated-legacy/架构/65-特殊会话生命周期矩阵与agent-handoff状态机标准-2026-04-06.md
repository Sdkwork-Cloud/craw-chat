# 65. 特殊会话生命周期矩阵与 agent_handoff 状态机标准（2026-04-06）

## 1. 目标

在已经完成 `agent_dialog / system_channel / agent_handoff` 专用创建链路之后，继续冻结“创建之后还能做什么”的边界，避免：

- 继续把特殊会话退化成普通 `group / direct`
- 让业务状态只能依赖消息正文推断
- 在后续商业化场景中出现审计真相和运行时真相不一致

本标准先冻结矩阵，再只对 `agent_handoff` 落一条最小但可商业应用的 durable lifecycle。

## 2. 生命周期矩阵

### 2.1 `agent_dialog`

- 允许：
  - 专用创建
  - 活跃成员消息读写
- 不允许：
  - generic `add/remove/leave/transfer-owner/change-role`
  - 本轮内的 dedicated close/archive
- 结论：
  - `agent_dialog` 当前是“已创建即可持续对话”的最小商业闭环，但还不是完整生命周期闭环

### 2.2 `system_channel`

- 允许：
  - 专用创建
  - system publisher 发消息
  - subscriber 读消息
- 不允许：
  - subscriber 发消息
  - generic 成员治理
  - 本轮内的 scheduled publish / bulk publish / delegation
- 结论：
  - `system_channel` 当前是“创建 + 单向发布”的闭环，不扩展为通用通知编排系统

### 2.3 `agent_handoff`

- 允许：
  - 专用创建
  - source / target 双方在 handoff 期间消息沟通
  - 专用状态读
  - 专用状态迁移
- 不允许：
  - generic 成员治理
  - 用普通消息代替 handoff 运行时状态真相

## 3. `agent_handoff` 状态机

### 3.1 状态定义

- `open`
  - 刚创建，尚未被 target 接单
- `accepted`
  - target 已显式接受交接
- `resolved`
  - target 已完成本次交接处理
- `closed`
  - handoff 生命周期终态

### 3.2 状态迁移

- `open -> accepted`
- `accepted -> resolved`
- `open -> closed`
- `accepted -> closed`
- `resolved -> closed`

不允许：

- `open -> resolved`
- `resolved -> accepted`
- `closed -> *`

### 3.3 角色权限

- `accept`
  - 仅 `target`
- `resolve`
  - 仅 `target`
- `close`
  - `source` 或 `target`

### 3.4 幂等语义

- 对已经达到目标状态的同一类重复请求，返回当前状态视图，不重复追加事件
- 对非法状态迁移，返回 `409 conversation_conflict`

## 4. Durable Truth

`agent_handoff` 的运行时真相必须由服务端持久的状态对象表示，而不是依赖消息正文推断。

最小状态视图：

```json
{
  "tenantId": "100001",
  "conversationId": "c_agent_handoff_xxx",
  "status": "accepted",
  "source": {
    "id": "ag_source",
    "kind": "agent"
  },
  "target": {
    "id": "1",
    "kind": "user"
  },
  "handoffSessionId": "hs_demo",
  "handoffReason": "manual_escalation",
  "acceptedAt": "2026-04-06T10:00:10.000Z",
  "acceptedBy": {
    "id": "1",
    "kind": "user"
  },
  "resolvedAt": null,
  "resolvedBy": null,
  "closedAt": null,
  "closedBy": null
}
```

## 5. 事件标准

### 5.1 创建事件

`conversation.created` 在 `agent_handoff` 上必须继续携带：

- `source`
- `target`
- `handoff.sessionId`
- `handoff.reason`
- `handoff.status = open`

### 5.2 状态变更事件

新增事件：

- `conversation.agent_handoff_status_changed`

最小载荷：

- `tenantId`
- `conversationId`
- `previousStatus`
- `currentStatus`
- `changedBy`
- `changedAt`
- `state`

该事件是 handoff 生命周期审计锚点，不能用普通 `message.posted` 代替。

## 6. 写路径约束

### 6.1 关闭后的会话写入

当 `agent_handoff.status = closed` 时，服务端必须拒绝：

- `POST /im/v3/api/chat/conversations/{conversationId}/messages`
- `POST /im/v3/api/chat/messages/{messageId}/edit`
- `POST /im/v3/api/chat/messages/{messageId}/recall`

返回：

- `409 conversation_conflict`

### 6.2 关闭后的读能力

关闭并不等于历史不可见。关闭后的 handoff 仍允许：

- 状态读取
- 历史消息读取
- 已有 read cursor 查询/推进

## 7. Gateway API

新增专用接口：

- `GET /im/v3/api/chat/conversations/{conversationId}/agent-handoff`
- `POST /im/v3/api/chat/conversations/{conversationId}/agent-handoff/accept`
- `POST /im/v3/api/chat/conversations/{conversationId}/agent-handoff/resolve`
- `POST /im/v3/api/chat/conversations/{conversationId}/agent-handoff/close`

规则：

- `tenantId` 来自认证上下文
- actor id / actor kind 来自认证上下文
- body 中不允许重复提交 actor 身份

## 8. 本轮落地边界

本标准本轮只落地：

- `agent_handoff` 的 runtime durable state
- dedicated lifecycle commands
- app-facing runtime HTTP
- local profile HTTP
- 关闭后消息写入拒绝

本轮不落地：

- inbox/summary 对 handoff 状态的投影展示
- `agent_dialog` dedicated close/archive
- `system_channel` scheduled/bulk publish
- stream/RTC 对 special lifecycle 的联动门禁

## 9. 后续迭代

1. 将 `agent_handoff` 状态投影到 inbox、summary、admin 读模型。
2. 补齐 `agent_dialog` 的终态标准，而不是长期停留在“仅创建可用”。
3. 为 `system_channel` 增加专用发布能力，而不是让通知编排长期依赖普通消息写入。
