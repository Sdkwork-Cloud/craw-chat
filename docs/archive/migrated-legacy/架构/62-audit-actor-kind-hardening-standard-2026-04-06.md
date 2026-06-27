# 62. 审计 Actor Kind 加固标准 2026-04-06

## 1. 目标

统一 IM 服务端在会话创建、成员治理、读游标、消息变更等事件中的审计身份标准，保证 `actor_id` 与 `actor_kind` 全链路一致，避免出现同一真实操作者在不同事件里被降级为默认 `user` 的问题。

## 2. 标准结论

所有审计事件必须显式记录真实操作者身份：

- `actor.actor_id`
- `actor.actor_kind`
- `actor.actor_session_id`，当前没有真实会话信息时可为空

禁止做法：

- 事件构造器内部硬编码 `actor_kind = "user"`
- 仅传 `actor_id`，再由下游猜测操作者类型
- 将创建路径和变更路径采用不同的身份标准

## 3. 适用范围

本标准覆盖以下运行时事件：

- `conversation.created`
- `conversation.member_joined`
- `conversation.member_removed`
- `conversation.member_left`
- `conversation.read_cursor_updated`
- `conversation.owner_transferred`
- `conversation.member_role_changed`
- 后续新增的所有治理类、控制类、审计类事件

## 4. 来源规则

### 4.1 来自认证上下文的场景

如果事件由网关请求直接触发，并且运行时已经拿到认证上下文，则必须把认证上下文中的真实 actor 信息传入运行时。

- `tenant_id` 来自认证上下文
- `actor_id` 来自认证上下文
- `actor_kind` 来自认证上下文

### 4.2 来自活跃成员解析的场景

如果运行时在会话内部通过成员关系解析操作者，则 `actor_kind` 必须来自该活跃成员的：

- `ConversationMember.principal_kind`

不能再重新假设为 `user`。

## 5. 当前实现标准

`conversation-runtime` 当前采用如下规则：

- `create_conversation_with_creator_kind(...)`
  - `conversation.created` 使用 `creator_kind`
  - 创建者的 `conversation.member_joined` 也使用同一个 `creator_kind`
- `create_agent_dialog_with_requester_kind(...)`
  - `conversation.created` 使用 `requester_kind`
  - requester 与 agent 初始成员事件的 actor 都使用 `requester_kind`
- `add_member(...)`
  - actor kind 来自邀请者活跃成员的 `principal_kind`
- `remove_member(...)`
  - actor kind 来自移除者活跃成员的 `principal_kind`
- `leave_conversation(...)`
  - actor kind 来自离开者活跃成员的 `principal_kind`
- `update_read_cursor(...)`
  - actor kind 来自读游标更新者活跃成员的 `principal_kind`
- `transfer_conversation_owner(...)`
  - actor kind 来自发起转移的 owner 成员 `principal_kind`
- `change_conversation_member_role(...)`
  - actor kind 来自发起修改的成员 `principal_kind`

## 6. 构造器约束

所有事件构造器必须显式接收 `actor_kind` 参数。

本轮已经加固的构造器：

- `build_member_envelope(...)`
- `build_read_cursor_envelope(...)`
- `build_owner_transfer_envelope(...)`
- `build_member_role_changed_envelope(...)`

后续新增构造器必须满足同样约束，否则视为不符合平台审计标准。

## 7. 测试标准

每一类 actor 传播路径至少要有一个回归测试。

当前最小回归集：

- `system -> conversation.member_joined`
- `agent -> conversation.read_cursor_updated`
- `system -> conversation.owner_transferred`
- `system -> conversation.member_role_changed`

新增事件时，必须补充：

1. 失败前复现测试
2. 修复后通过测试
3. 至少一个非 `user` actor 的断言

## 8. 设计收益

采用该标准后，平台获得以下能力：

- 审计日志可以准确区分 `user / agent / system`
- 控制面与数据面在安全审计上语义一致
- 后续策略引擎、自动化、风控、合规系统可以直接依赖事件身份信息
- `system_channel`、`agent_handoff` 等专用会话类型可以复用同一套 actor 传播规则

## 9. 后续要求

1. `system_channel` 专用创建合同必须沿用本标准。
2. `agent_handoff` 专用创建合同必须沿用本标准。
3. projection、automation、notification 等下游消费侧如果依赖 actor 身份，也必须按本标准校验。
