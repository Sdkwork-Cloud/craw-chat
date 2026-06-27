> Migrated from `docs/架构/57-group-owner-transfer标准-2026-04-06.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 57-group owner transfer 标准-2026-04-06

## 1. 背景

在成员治理矩阵和成员主动离开标准冻结后，`group owner` 仍存在一个关键缺口：

- owner 被禁止直接 leave 是正确的
- 但如果没有 transfer 机制，owner 生命周期会被永久锁死

这不符合商用 IM 的治理要求。

## 2. 目标

本标准只解决一个问题：

- 如何把 group conversation 的 owner，从当前 active owner 安全交接给另一个 active member

本标准不承载以下能力：

- 通用 promote / demote
- 成员重新加入
- 特殊会话类型成员生命周期

## 3. 标准

### 3.1 命令与接口

- 新增命令：`transfer_conversation_owner`
- 新增接口：`POST /im/v3/api/chat/conversations/{conversationId}/members/transfer-owner`
- 请求体：

```json
{
  "memberId": "cm_xxx"
}
```

### 3.2 请求约束

- 请求体只允许指定目标 `memberId`
- 不允许客户端提交：
  - `tenantId`
  - `principalId`
  - `actorId`
- 调用者身份必须完全来自认证上下文

### 3.3 会话类型约束

- `group`
  - 支持 owner transfer
- `direct`
  - 不支持 owner transfer
- `agent_dialog / agent_handoff / system_channel`
  - 当前阶段统一拒绝 owner transfer

### 3.4 授权约束

对于 `group`：

- 只有当前 active `owner` 可以 transfer
- 目标必须是“另一个 active member”
- transfer 不允许把 owner 转给自己

当前阶段不额外限制目标原始角色：

- `admin`
- `member`
- `guest`

只要目标是 active member，即可成为新 owner。

## 4. 状态变更标准

owner transfer 成功后必须满足：

- 原 owner：`role = admin`
- 新 owner：`role = owner`
- 继续保持“单 owner”约束
- 不修改成员生命周期字段：
  - `state`
  - `joinedAt`
  - `removedAt`

这样可以保证：

- owner transfer 是治理动作，不是成员重新加入或离开
- 后续 `leave`、审计和再加入标准都不会被混淆

## 5. 与 leave 标准的关系

- owner transfer 本身不导致成员离开
- owner transfer 完成后，旧 owner 已不再是 `owner`
- 因此旧 owner 可以按 `group` 普通 active member 规则执行 `leave`

换言之：

- `owner` 不能直接 leave
- `owner transfer -> previous owner leave` 是允许的标准链路

## 6. 事件标准

### 6.1 事件类型

- `conversation.owner_transferred`

### 6.2 事件 payload

payload 至少应包含：

- `tenantId`
- `conversationId`
- `previousOwner`
- `newOwner`
- `transferredAt`

其中 `previousOwner` 与 `newOwner` 都应是 transfer 后的权威成员快照。

## 7. 审计标准

- owner transfer 必须留下审计动作：
  - `conversation.owner_transferred`
- 审计 payload 至少应包含：
  - `previousOwnerMemberId`
  - `previousOwnerPrincipalId`
  - `previousOwnerRole`
  - `newOwnerMemberId`
  - `newOwnerPrincipalId`
  - `newOwnerRole`
  - `transferredAt`

## 8. 错误语义

- 权限不满足时：
  - `403 conversation_permission_denied`
- 目标成员不存在时：
  - `404 conversation_member_not_found`
- 当前阶段不额外新增 error code 族

## 9. 一期落地要求

一期必须具备：

- runtime 层 owner transfer 命令
- gateway / app 层 `/members/transfer-owner`
- `conversation.owner_transferred` 事件追加
- transfer 后旧 owner 可以 leave
- 覆盖以下回归场景：
  - group owner transfer 成功
  - admin 不能 transfer
  - direct 不能 transfer
  - transfer 后旧 owner leave 成功，新 owner 成为唯一 owner

## 10. 后续演进

后续按顺序推进：

1. `left -> rejoin` 标准
2. promote / demote 标准
3. 特殊会话类型成员生命周期标准

