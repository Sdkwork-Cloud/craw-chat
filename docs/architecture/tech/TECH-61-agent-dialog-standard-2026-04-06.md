> Migrated from `docs/架构/61-agent-dialog专用创建标准-2026-04-06.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 61. agent_dialog 专用创建标准（2026-04-06）

## 1. 目标

在已经冻结 generic create 只支持 `group / direct` 的前提下，为 `agent_dialog` 打开第一条安全、可商业落地的专用创建路径，同时不破坏既有成员治理边界。

## 2. 当前标准

### 2.1 generic create 继续收口

- `POST /im/v3/api/chat/conversations` 只允许：
  - `group`
  - `direct`
- `agent_dialog` 不再通过 generic create 暴露。
- `agent_handoff / system_channel` 继续保持 reserved 状态。

### 2.2 `agent_dialog` 专用创建路由

- 路由：
  - `POST /im/v3/api/chat/conversations/agent_dialogs`
- 请求体：

```json
{
  "conversationId": "c_agent_dialog_xxx",
  "agentId": "ag_demo"
}
```

- 以下字段严禁由客户端显式提交：
  - `tenantId`
  - `requesterId`
  - `requesterKind`

这些字段必须全部来自认证上下文。

## 3. 创建语义

### 3.1 请求者约束

- `agent_dialog` 的请求者必须是认证上下文中的真实用户主体。
- 当前最小落地标准：
  - `actor_kind` 必须等于 `user`
- 非 `user` 主体调用时：
  - HTTP `403`
  - code `conversation_permission_denied`

### 3.2 创建后成员拓扑

成功创建后，服务端必须一次性写入两个 active member：

1. requester member
   - `principalId = auth.actor_id`
   - `principalKind = user`
   - `role = owner`
   - `state = joined`
   - `attributes.dialogRole = requester`

2. agent member
   - `principalId = request.agentId`
   - `principalKind = agent`
   - `role = member`
   - `state = joined`
   - `invitedBy = auth.actor_id`
   - `attributes.agentId = request.agentId`
   - `attributes.dialogRole = assistant`

### 3.3 已读游标

- requester 与 agent 两个成员都必须初始化默认 read cursor。
- 这样后续流式输出、机器人回写、agent 多阶段回复才能复用统一的会话内游标模型。

## 4. 事件与持久化约束

### 4.1 会话创建事件

- 仍写入统一的 `conversation.created`
- `payload.conversationType = agent_dialog`
- `actor.actor_id = requester`
- `actor.actor_kind = user`

### 4.2 成员加入事件

- 仍写入统一的 `conversation.member_joined`
- ordering 建议：
  - `1` requester joined
  - `2` agent joined

### 4.3 Durable Truth 约束

- `Conversation.conversation_type = agent_dialog`
- `ConversationMembership` 仍然复用统一成员表，不新增临时分支表
- 但 `agent_dialog` 的创建路径必须使用专用命令，不能再回退到 generic create

## 5. Gateway 约束

- app-facing 与 local profile 必须统一暴露：
  - `POST /im/v3/api/chat/conversations/agent_dialogs`
- gateway 只负责：
  - 解析认证上下文
  - 把 `tenant / requester / actor_kind` 透传到 runtime
- gateway 不负责：
  - 让客户端伪造 requester 身份
  - 让客户端指定 `principalKind`
  - 让客户端绕过 `actor_kind=user` 的限制

## 6. 与既有成员治理边界的关系

本次只打开 `agent_dialog` 的专用创建，不代表已经开放 special conversation 的通用成员治理。

当前仍然关闭：

- `add_member`
- `remove_member`
- `leave`
- `transfer-owner`
- `change-role`

对 `agent_dialog / agent_handoff / system_channel` 的后续治理命令，必须分别单独冻结标准后再开放。

## 7. 为什么这样设计

### 7.1 先冻结最小闭环，而不是一次性打开全部 special type

- `agent_dialog` 的参与者拓扑最简单：
  - requester user
  - target agent
- 它最适合作为 special conversation dedicated create 的第一条生产级落地链路。

### 7.2 先限制请求者为 user，避免写出伪语义会话

- 如果允许 `system` 或其他主体直接创建 `agent_dialog`，会得到“系统对 agent”的错误语义。
- 那类需求应该在后续通过：
  - system channel
  - internal automation
  - agent handoff
  - explicit impersonation / delegated execution
  来单独建模，而不是污染 `agent_dialog`。

## 8. 当前剩余缺口

1. `system_channel` 仍未定义 dedicated create。
2. `agent_handoff` 仍未定义 source/target/handoff metadata。
3. 成员治理审计事件中的 actor identity 仍需进一步统一硬化。

## 9. 实现映射

- `services/conversation-runtime/src/lib.rs`
  - `CreateAgentDialogCommand`
  - `create_agent_dialog_with_requester_kind(...)`
  - `/im/v3/api/chat/conversations/agent_dialogs`
- `services/sdkwork-im-cloud-gateway/src/lib.rs`
  - `/im/v3/api/chat/conversations/agent_dialogs`
  - auth context -> runtime dedicated create 映射

## 10. 验证基线

- `cargo test -p conversation-runtime --test conversation_flow_test --offline`
- `cargo test -p conversation-runtime --test http_smoke_test --offline`
- `cargo test -p sdkwork-im-cloud-gateway --test access_control_e2e_test --offline`

