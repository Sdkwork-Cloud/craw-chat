# 63. system_channel 专用创建与发布权限标准（2026-04-06）

## 1. 目标

在已经冻结 generic create 只支持 `group / direct` 的前提下，为 `system_channel` 打开第一条可商业落地、语义完整、可审计的专用创建路径，并同时冻结其最小发布权限标准。

本标准解决两个问题：

1. `system_channel` 不能再只是数据模型中的保留类型，必须有可用的专用创建路径。
2. `system_channel` 不能沿用普通会话的“所有 active member 都可发言”规则，否则会破坏系统广播语义。

## 2. 适用范围

- `conversation_type = system_channel`
- app-facing runtime
- local deployment profile
- 后续云端多节点实现

不适用于：

- `group`
- `direct`
- `agent_dialog`
- `agent_handoff`

## 3. 创建标准

### 3.1 专用路由

- `POST /im/v3/api/chat/conversations/system_channels`

请求体：

```json
{
  "conversationId": "c_system_channel_xxx",
  "subscriberId": "1"
}
```

### 3.2 身份来源

以下字段必须从认证上下文提取，而不是由客户端提交：

- `tenantId`
- `requesterId`
- `requesterKind`

### 3.3 创建者约束

- `system_channel` 的创建者必须是认证上下文中的真实 `system` 主体。
- 若 `actor_kind != system`，必须拒绝创建。
- 拒绝结果统一映射为 `conversation_permission_denied`。

## 4. 成员拓扑标准

`system_channel` 在第一阶段只允许最小双成员拓扑：

1. publisher member
   - `principalId = auth.actor_id`
   - `principalKind = system`
   - `role = owner`
   - `state = joined`
   - `attributes.channelRole = publisher`

2. subscriber member
   - `principalId = request.subscriberId`
   - `principalKind = user`
   - `role = member`
   - `state = joined`
   - `invitedBy = auth.actor_id`
   - `attributes.channelRole = subscriber`

### 4.1 为什么不开放通用成员治理

当前阶段 `system_channel` 的商业语义是“系统发布，订阅者消费”。如果直接开放：

- `add_member`
- `remove_member`
- `leave`
- `transfer-owner`
- `change-role`

则会立刻引入未冻结的拓扑变种和权限矩阵，破坏标准一致性。因此这些通用治理能力继续保持关闭，后续必须以专用标准逐项开放。

## 5. 发言权限标准

### 5.1 基本规则

- `group / direct / agent_dialog`
  - active member 可发言
- `system_channel`
  - 只有 publisher member 可发言

### 5.2 publisher 识别规则

一个成员只有同时满足以下条件时，才被视为 `system_channel` 的 publisher：

1. `principal_kind == system`
2. `attributes.channelRole == publisher`

任一条件不满足，都必须拒绝发言。

### 5.3 subscriber 发言处理

如果 subscriber 调用：

- `POST /im/v3/api/chat/conversations/{id}/messages`

则必须返回权限拒绝，而不能把该消息写入 durable log。

## 6. 读状态标准

创建 `system_channel` 时，必须同时初始化以下 read cursor：

- publisher read cursor
- subscriber read cursor

这样可以保证：

- publisher 后续可感知投递进度
- subscriber 可以立即参与未读、ack、同步窗口计算

## 7. 审计与事件标准

创建 `system_channel` 成功后，最小事件链为：

1. `conversation.created`
2. `conversation.member_joined` for publisher
3. `conversation.member_joined` for subscriber

所有这些事件的 actor 必须保留真实 system 身份：

- `actor.actor_id = requester_id`
- `actor.actor_kind = system`

不能回退成默认 `user`。

## 8. 数据模型约束

### 8.1 Conversation

- `conversation_type = system_channel`

### 8.2 ConversationMembership.attributes

必须至少支持：

- `channelRole = publisher`
- `channelRole = subscriber`

### 8.3 Message

`system_channel` 暂不引入独立消息表结构，仍沿用统一 `Message` durable truth；
但写入前必须经过会话类型专属发言策略校验。

## 9. Gateway 标准

- app-facing 与 local profile 必须暴露相同专用路由：
  - `POST /im/v3/api/chat/conversations/system_channels`
- 网关层不得允许客户端显式提交：
  - `tenantId`
  - `requesterId`
  - `requesterKind`
- 网关层必须把认证上下文映射到 runtime command。

## 10. 与其他 special conversation 的边界

- `agent_dialog`
  - 已有专用创建标准，但不是系统广播语义
- `agent_handoff`
  - 仍未冻结专用创建契约
- `system_channel`
  - 本次只冻结“系统发布 + 用户订阅 + 订阅者不可发言”的最小商业闭环

## 11. 当前落地结果

已完成：

- runtime 专用命令
- runtime 专用创建逻辑
- runtime system-channel 发言权限校验
- conversation-runtime HTTP 路由
- sdkwork-im-server HTTP 路由
- 单元测试、HTTP 测试、本地 profile E2E 测试

## 12. 后续演进

1. 冻结 `agent_handoff` 专用创建标准。
2. 评估 `system_channel` 是否需要专用批量发布、定时发布、撤回策略。
3. 对 special conversation 建立统一 lifecycle matrix，避免不同类型复用不正确的 generic governance。
