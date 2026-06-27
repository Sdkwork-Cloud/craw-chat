# 59-group member role change 标准（2026-04-06）

## 1. 背景

在成员治理矩阵、`leave`、`owner transfer` 和 `left -> rejoin` 标准冻结后，群角色治理仍存在一个关键缺口：

- owner 可以交接 owner 身份
- 但还不能完成普通的管理员授予、管理员降级和成员/访客切换

因此必须单独冻结通用 `promote / demote` 标准，而不能把它继续塞进 `add_member`、`remove_member` 或 `owner transfer`。

## 2. 目标

本标准只解决一个问题：

- 如何在 `group` conversation 中，对当前 active non-owner member 做通用角色治理

本标准不覆盖：

- owner transfer
- membership episode 查询视图
- 特殊会话类型生命周期

## 3. 核心标准

### 3.1 命令与接口

- 新增命令：`change_conversation_member_role`
- 新增接口：`POST /im/v3/api/chat/conversations/{conversationId}/members/change-role`

请求体：

```json
{
  "memberId": "cm_xxx",
  "role": "admin"
}
```

### 3.2 会话类型

- `group`
  - 支持通用角色治理
- `direct`
  - 不支持
- `agent_dialog / agent_handoff / system_channel`
  - 当前阶段统一拒绝，直到对应成员模型冻结

### 3.3 授权

对于 `group`：

- 仅当前 active `owner` 可发起通用 role mutation
- `admin / member / guest` 不允许发起

### 3.4 目标成员与目标角色

- 目标必须是当前 active non-owner member
- 目标角色只允许变更为：
  - `admin`
  - `member`
  - `guest`
- 不允许通过通用 role mutation 直接把目标改为 `owner`
- 不允许对当前 `owner` 使用通用 role mutation
- 涉及 `owner` 的变化必须通过 `transfer-owner`

### 3.5 membership episode 边界

- 通用角色治理必须只作用于当前 active episode
- 若传入 stale `memberId`，必须拒绝
- 历史 `left / removed` episode 不能再被当作当前治理目标

## 4. 状态变更标准

- 角色变更只修改：
  - `role`
- 不修改：
  - `memberId`
  - `state`
  - `joinedAt`
  - `removedAt`

这意味着：

- 通用角色治理是治理动作，不是生命周期迁移
- 通用角色治理不会新建 membership episode

## 5. 事件与审计标准

### 5.1 事件

- 新增事件：`conversation.member_role_changed`

payload 至少应包含：

- `tenantId`
- `conversationId`
- `previousMember`
- `updatedMember`
- `changedAt`

其中 `previousMember` 与 `updatedMember` 都应是完整成员快照。

### 5.2 审计

- 新增审计动作：`conversation.member_role_changed`
- 审计 payload 至少应包含：
  - `previousMemberId`
  - `previousPrincipalId`
  - `previousRole`
  - `updatedMemberId`
  - `updatedPrincipalId`
  - `updatedRole`
  - `changedAt`

## 6. 投影标准

- projection 读模型必须应用 `conversation.member_role_changed`
- 应把对应 principal 的成员快照更新为 `updatedMember`
- 不得让 runtime 与 projection 在角色视图上再次产生漂移

## 7. 错误语义

- 权限不满足或会话类型不支持时：
  - `403 conversation_permission_denied`
- 目标成员不存在，或传入 stale `memberId` 时：
  - `404 conversation_member_not_found`

当前阶段不新增新的 error code 族，保持与既有治理接口一致。

## 8. 一期落地要求

一期必须具备：

- runtime 层通用角色治理命令
- runtime HTTP 与 app HTTP 的 `/members/change-role`
- `conversation.member_role_changed` 事件追加
- projection 成员快照更新
- 覆盖以下回归场景：
  - owner 可以对 non-owner member 做升降级
  - admin 不能做通用角色治理
  - `direct` 不能做通用角色治理
  - stale `memberId` 不能命中新 episode

## 9. 后续演进

后续按顺序推进：

1. 特殊会话类型成员生命周期标准
2. membership episode 的审计/查询视图
3. 成员治理事件与 policy / automation / moderation 语义进一步对齐
