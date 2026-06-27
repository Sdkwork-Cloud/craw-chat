# 64. agent_handoff 专用创建标准（2026-04-06）

## 1. 目标

在已经冻结 generic create 只支持 `group / direct` 的前提下，为 `agent_handoff` 打开第一条可商业落地、语义完整、可审计的专用创建路径。

本标准解决两个核心问题：

1. `agent_handoff` 不能再只是数据模型中的保留类型，必须具备专用创建入口。
2. `agent_handoff` 不能只被创建出来却无法用于后续交接对话，必须具备最小可通信闭环。

## 2. 适用范围

- `conversation_type = agent_handoff`
- app-facing runtime
- local deployment profile
- 后续云端多节点实现

不适用于：

- `group`
- `direct`
- `agent_dialog`
- `system_channel`

## 3. 创建标准

### 3.1 专用路由

- `POST /im/v3/api/chat/conversations/agent_handoffs`

请求体：

```json
{
  "conversationId": "c_agent_handoff_xxx",
  "targetId": "1",
  "targetKind": "user",
  "handoffSessionId": "hs_demo",
  "handoffReason": "manual_escalation"
}
```

### 3.2 身份来源

以下字段必须从认证上下文提取，而不是由客户端提交：

- `tenantId`
- `sourceId`
- `sourceKind`

当前阶段 source actor 统一来自真实认证 agent 主体。

### 3.3 创建者约束

- `agent_handoff` 的创建者必须是认证上下文中的真实 `agent` 主体。
- 若 `actor_kind != agent`，必须拒绝创建。
- 拒绝结果统一映射为 `conversation_permission_denied`。

## 4. 目标参与者约束

### 4.1 允许的 target kind

当前阶段 `targetKind` 只允许：

- `user`
- `agent`

不允许：

- `system`
- `bot`
- 其他未冻结类型

### 4.2 关键校验

- `handoffSessionId` 不能为空
- `sourceId` 与 `targetId` 不能相同

## 5. 成员拓扑标准

`agent_handoff` 在第一阶段只允许最小双成员拓扑：

1. source agent member
   - `principalId = auth.actor_id`
   - `principalKind = agent`
   - `role = owner`
   - `state = joined`
   - `attributes.handoffRole = source`
   - `attributes.handoffSessionId = request.handoffSessionId`
   - `attributes.targetId = request.targetId`
   - `attributes.targetKind = request.targetKind`

2. target member
   - `principalId = request.targetId`
   - `principalKind = request.targetKind`
   - `role = member`
   - `state = joined`
   - `invitedBy = auth.actor_id`
   - `attributes.handoffRole = target`
   - `attributes.handoffSessionId = request.handoffSessionId`
   - `attributes.sourceAgentId = auth.actor_id`

若请求包含 `handoffReason`，则 source 与 target 成员属性都应保留该元数据，方便后续查询、投影和审计扩展。

## 6. 发言权限标准

### 6.1 基本规则

`agent_handoff` 当前阶段允许：

- source agent 发言
- target principal 发言

也就是说，`agent_handoff` 的最小商业闭环不是单向广播，而是交接后的双向沟通会话。

### 6.2 为什么不是 system_channel 式单向发布

`agent_handoff` 的目标是完成“交接后沟通”，而不是“系统通知广播”。如果 target 无法回复，就会得到一个只能创建、不能实际接管的伪 handoff 会话。

## 7. 读状态标准

创建 `agent_handoff` 时，必须同时初始化：

- source agent read cursor
- target member read cursor

这样可以保证：

- source agent 可继续观测交接后的对话进度
- target 可立即进入未读、ack、同步窗口计算

## 8. 审计与事件标准

创建 `agent_handoff` 成功后，最小事件链为：

1. `conversation.created`
2. `conversation.member_joined` for source agent
3. `conversation.member_joined` for target

这些事件的 actor 必须保留真实 source agent 身份：

- `actor.actor_id = source_id`
- `actor.actor_kind = agent`

不能退化为默认 `user`。

### 8.1 created payload 最小要求

`conversation.created` payload 至少应显式携带：

- `source`
  - `id`
  - `kind`
- `target`
  - `id`
  - `kind`
- `handoff`
  - `sessionId`
  - `reason`

## 9. 数据模型约束

### 9.1 Conversation

- `conversation_type = agent_handoff`

### 9.2 ConversationMembership.attributes

必须至少支持：

- `handoffRole = source`
- `handoffRole = target`
- `handoffSessionId`
- `sourceAgentId`
- `targetId`
- `targetKind`

### 9.3 Message

`agent_handoff` 暂不引入独立消息表结构，仍沿用统一 `Message` durable truth；
但写入前必须经过 conversation type 专属写入策略判定，当前规则为双方 active member 均可写。

## 10. Gateway 标准

- app-facing 与 local profile 必须暴露相同专用路由：
  - `POST /im/v3/api/chat/conversations/agent_handoffs`
- 网关层不得允许客户端显式提交：
  - `tenantId`
  - `sourceId`
  - `sourceKind`
- 网关层必须把认证上下文映射到 runtime command。

## 11. 与其他 special conversation 的边界

- `agent_dialog`
  - 用户与 agent 的主对话，不承载 handoff 语义
- `system_channel`
  - 系统广播模型，不承载双向交接对话
- `agent_handoff`
  - 当前阶段冻结为“agent 发起，目标 user/agent，交接后双方可通信”的最小模型

## 12. 当前落地结果

已完成：

- runtime 专用命令
- runtime 专用创建逻辑
- runtime `agent_handoff` 写入权限开放
- conversation-runtime HTTP 路由
- sdkwork-im-server HTTP 路由
- 单元测试、HTTP 测试、本地 profile E2E 测试

## 13. 后续演进

1. 冻结 `agent_handoff` 的 accept / resolve / close 生命周期标准。
2. 评估是否需要把 handoff 运营态元数据提升为 conversation settings 级 durable truth，而不是只保存在 create payload 与 member attributes。
3. 与 `agent_dialog / system_channel` 一起建立统一 special conversation lifecycle matrix。
