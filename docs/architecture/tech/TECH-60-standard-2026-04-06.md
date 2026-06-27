> Migrated from `docs/架构/60-特殊会话创建与创建者身份标准-2026-04-06.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 60. 特殊会话创建与创建者身份标准（2026-04-06）

## 1. 目标

冻结特殊会话类型在“创建入口”上的最小标准，避免未定义类型和错误的创建者身份污染会话模型、投影、审计与后续治理语义。

## 2. 通用创建入口白名单

当前阶段，generic `POST /im/v3/api/chat/conversations` 只允许：

- `group`
- `direct`

以下值仍然是合法的数据模型类型，但不能通过 generic create 直接创建：

- `agent_dialog`
- `agent_handoff`
- `system_channel`

原因：

- 这三类会话尚未冻结 dedicated create payload
- 尚未冻结专用成员拓扑与生命周期命令
- 继续开放 generic create 会生成语义残缺的半成品聚合

任何不被当前 generic create 支持的值都必须在创建时直接拒绝：

- HTTP 状态码：`400`
- 错误码：`conversation_type_invalid`

## 3. 创建者身份标准

### 3.1 身份来源

会话创建时，创建者身份必须来自认证上下文，而不是请求体：

- `tenant_id` 来自认证上下文
- `creator_id` 来自认证上下文
- `creator_kind` 来自认证上下文中的 `actor_kind`

### 3.2 写入要求

创建成功时，系统必须同时满足：

- 创建者成员记录的 `principalKind` 等于认证上下文的 `actor_kind`
- `conversation.created` 事件中的 `actor.actor_kind` 等于认证上下文的 `actor_kind`

这条规则对当前 generic create 支持的类型立即生效，并且未来 dedicated create 打开后也必须继续生效。

## 4. 当前阶段的特殊类型边界

### 4.1 已冻结边界

当前阶段已经冻结的规则：

- `group / direct` 才能通过 generic create 创建
- special conversation types 只能作为保留类型存在于数据模型与协议设计中
- future dedicated create 打开前，special types 不能通过 generic create 落库
- generic create 的创建者身份必须写入正确的 `actor_kind`

### 4.2 仍然关闭的通用治理

在 dedicated create + lifecycle 标准完成前，以下 generic member governance 继续关闭：

- `add_member`
- `remove_member`
- `leave`
- `transfer-owner`
- `change-role`

适用类型：

- `agent_dialog`
- `agent_handoff`
- `system_channel`

## 5. 实现要求

### 5.1 conversation-runtime

- generic create path 必须只允许 `group / direct`
- reserved special types 必须返回 dedicated-create 语义的 `conversation_type_invalid`
- create path 必须支持注入真实 `creator_kind`
- 低层错误语义必须暴露 `conversation_type_invalid`

### 5.2 app-facing gateway / local profile

- app-facing create API 必须把认证上下文里的 `actor_kind` 传给 runtime create path
- 错误映射必须保持 `400 conversation_type_invalid`

## 6. 与后续阶段的衔接

本标准只冻结“generic create 入口”和“创建者身份”两个边界，不代表特殊类型生命周期已经完整定义。下一阶段需要补齐：

1. `agent_dialog / agent_handoff / system_channel` 的专用成员模型
2. 专用 create 命令、治理命令与事件
3. policy / moderation / automation 对特殊类型的差异化处理

## 7. 2026-04-06 补充：agent_dialog 已开放专用创建

本标准在 generic create 层继续保持收口，但 `agent_dialog` 已经打开第一条 dedicated create 路径：

- 路由：`POST /im/v3/api/chat/conversations/agent_dialogs`
- 请求体只允许：
  - `conversationId`
  - `agentId`
- 请求者身份必须来自认证上下文，且当前要求：
  - `actor_kind = user`

创建完成后，服务端必须一次性写入两个 active member：

1. requester
   - `principalKind = user`
   - `role = owner`
2. target agent
   - `principalKind = agent`
   - `role = member`

仍然保持关闭的 special type：

- `agent_handoff`
- `system_channel`

因此，“special conversation 不能通过 generic create 落库”的冻结标准继续成立；只是 `agent_dialog` 不再是“完全不可创建”，而是“只能通过 dedicated create 创建”。

