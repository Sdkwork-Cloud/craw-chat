> Migrated from `docs/架构/66-agent-handoff-读模型投影与会话摘要基线标准-2026-04-06.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 66. agent_handoff 读模型投影与会话摘要基线标准（2026-04-06）

## 1. 背景

`agent_handoff` 的运行时生命周期已经具备以下能力：

- `open -> accepted`
- `accepted -> resolved`
- `open|accepted|resolved -> closed`

但如果读模型层仍然只有消息摘要，没有生命周期摘要，就会出现两个问题：

1. 刚创建但尚未发送消息的 handoff 会话无法通过 summary 查询。
2. inbox / summary 看不到 `accepted / resolved / closed`，只能回退运行时专用接口。

因此本标准要求：`agent_handoff` 生命周期必须进入统一读模型，而不是只存在于运行时 API。

## 2. 标准目标

### 2.1 Summary 基线必须在 `conversation.created` 建立

对任意会话类型，`conversation.created` 一旦被投影，读模型侧就必须存在 summary 基线记录。

对尚未产生消息的会话：

- `messageCount = 0`
- `lastMessageSeq = 0`
- `lastMessageId = null`
- `lastSenderId = null`
- `lastSenderKind = null`
- `lastSender = null`
- `lastSummary = null`
- `lastMessageAt = null`

也就是说，summary API 不能再把“没有消息”错误地解释为“没有会话”。

### 2.2 `agent_handoff` 在 summary / inbox 中必须携带专用嵌套对象

统一字段名：

- `agentHandoff`

统一结构：

```json
{
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
  "acceptedAt": "2026-04-06T10:01:00Z",
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

非 `agent_handoff` 会话：

- `agentHandoff = null`

## 3. 事件投影规则

### 3.1 `conversation.created`

当 `conversationType = agent_handoff` 时，投影层必须从 create payload 读取：

- `source`
- `target`
- `handoff.sessionId`
- `handoff.reason`
- `handoff.status`

并立即初始化 summary 的 `agentHandoff` 字段。

### 3.2 `conversation.agent_handoff_status_changed`

该事件是 handoff 生命周期的唯一读模型增量更新源。

投影层必须使用 payload 中的 `state` 完整覆盖当前 `agentHandoff` 视图，而不是自行推断增量。

原因：

- 可避免依赖消息正文推理 handoff 状态
- 可避免事件重放时出现局部字段漂移
- 可直接复用运行时 durable truth

## 4. Inbox 排序标准

`inbox.lastActivityAt` 不是“最后一条消息时间”的同义词，而是“最后一次会话级活动时间”。

对 `agent_handoff`，活动时间必须取以下时间的最大值：

- `lastMessageAt`
- `acceptedAt`
- `resolvedAt`
- `closedAt`
- 若都不存在，则回退 `conversation.created_at`

这样可以保证：

- handoff 被接受后会话能前移
- handoff 被关闭后仍然能被正确排序
- 无消息会话不会因为缺少 `lastMessageAt` 而失真

## 5. API 行为标准

### 5.1 `GET /im/v3/api/chat/conversations/{conversationId}`

只要会话存在且调用方是有效成员，summary 就必须返回 `200`。

不能因为：

- 没有消息
- 只有生命周期状态
- 只有成员与创建事件

而返回 `404 conversation_summary_not_found`。

### 5.2 `GET /im/v3/api/chat/inbox`

当会话类型为 `agent_handoff` 时，返回项必须携带：

- `conversationType = agent_handoff`
- `agentHandoff`

## 6. 不在本轮范围

本标准当前不要求：

- 独立 admin read model
- client-route event window 的 handoff 生命周期同步项
- `agent_dialog` close/archive
- `system_channel` scheduled/bulk publish 生命周期

这些能力在后续标准继续冻结。

## 7. 最小落地清单

本标准最小实现至少包含：

1. summary 基线在 `conversation.created` 建立
2. summary 支持 nullable message-tail 字段
3. summary / inbox 支持 `agentHandoff`
4. `conversation.agent_handoff_status_changed` 可更新投影
5. 本地最小节点 HTTP 用例覆盖：
   - create 后 summary 可读
   - create 后 inbox 可见 `open`
   - accept 后 summary / inbox 变为 `accepted`

