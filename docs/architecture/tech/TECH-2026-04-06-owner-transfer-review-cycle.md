> Migrated from `docs/review/2026-04-06-owner-transfer-review-cycle.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Owner Transfer Review Cycle（2026-04-06）

## 1. 当前阶段

- 会话成员治理矩阵已经冻结
- `leave` 与 `remove_member` 已经拆分
- 当前剩余的核心治理缺口，是 `group owner` 无法安全转移主责任

这会直接阻塞商用场景中的群主交接、人员离职、组织切换和私有化运维托管。

## 2. 问题列表

### 2.1 高风险：group owner 没有正式 transfer 机制

问题表现：

- `group owner` 当前只能被保护性禁止 leave
- 系统没有 `owner transfer` 命令、接口、事件和审计动作

风险：

- 群主无法交接，群治理链路不能完成闭环
- owner 生命周期被锁死，商业部署中会形成实际运维阻塞
- 后续 `left -> rejoin`、组织同步、审批流都缺少稳定支点

### 2.2 中风险：owner 交接缺少统一协议

问题表现：

- 当前没有标准 API
- 接入层与运行时没有统一返回形态

风险：

- 不同入口未来容易各自定义“转群主”逻辑
- 新 owner / 旧 owner 的角色落点可能漂移

### 2.3 中风险：owner 交接缺少事件与审计锚点

问题表现：

- 即使后续某一层私自修改成员 role，也无法通过事件流还原治理过程

风险：

- 无法追溯谁把 owner 转给了谁
- 后续工作流、策略和合规审计无法可靠消费

## 3. 根因分析

- 前两轮标准分别冻结了成员治理矩阵和成员主动离开语义
- 但“owner 如何从 A 安全转交给 B”没有单独标准
- 代码中也没有 role mutation 的正式命令链路，因此 owner leave 只能被动阻止，不能正向解决

## 4. 本轮冻结标准

### 4.1 API 标准

- 新增 `POST /im/v3/api/chat/conversations/{conversationId}/members/transfer-owner`
- 请求体：

```json
{
  "memberId": "cm_xxx"
}
```

- 不接受 `tenantId`、`principalId`、`actorId`
- 当前操作者完全来自认证上下文

### 4.2 授权标准

- 只支持 `conversation_type = group`
- 只有当前 active `owner` 可以发起 transfer
- 目标必须是“另一个 active member”
- `direct` 和其他未冻结生命周期的会话类型统一拒绝

### 4.3 状态标准

- 目标成员变为 `owner`
- 原 owner 降为 `admin`
- 不修改：
  - `state`
  - `joinedAt`
  - `removedAt`
- transfer 完成后，原 owner 可以按现有 `leave` 规则主动离开

### 4.4 事件与审计标准

- 新增事件：`conversation.owner_transferred`
- 新增审计动作：`conversation.owner_transferred`
- 事件 payload 必须同时包含：
  - `previousOwner`
  - `newOwner`
  - `transferredAt`

## 5. 本轮实现

### 5.1 领域层

- `services/conversation-runtime/src/lib.rs`
  - 新增 `TransferConversationOwnerCommand`
  - 新增 `TransferConversationOwnerPayload`
  - 新增 `TransferConversationOwnerResult`
  - 新增 `transfer_conversation_owner(...)`
  - 新增 `ensure_owner_transfer_allowed(...)`
  - 新增 `conversation.owner_transferred` envelope 构建

### 5.2 接入层

- `services/conversation-runtime/src/lib.rs`
  - 新增 runtime HTTP 路由 `/members/transfer-owner`
- `services/sdkwork-im-cloud-gateway/src/lib.rs`
  - 新增 app 路由 `/members/transfer-owner`
  - 新增 owner transfer handler
  - 新增 `conversation.owner_transferred` 审计锚点

## 6. 新增与覆盖测试

- `services/conversation-runtime/tests/conversation_flow_test.rs`
  - `test_group_owner_can_transfer_ownership_and_then_leave`
  - `test_group_admin_cannot_transfer_ownership`
  - `test_direct_conversation_rejects_owner_transfer`
- `services/conversation-runtime/tests/http_smoke_test.rs`
  - `test_group_owner_transfer_over_http`
- `services/sdkwork-im-cloud-gateway/tests/access_control_e2e_test.rs`
  - `test_group_owner_transfer_allows_safe_handoff_and_leave`

## 7. 验证结果

已执行并通过：

- `cargo test -p conversation-runtime --test conversation_flow_test --offline`
- `cargo test -p conversation-runtime --test http_smoke_test --offline`
- `cargo test -p sdkwork-im-cloud-gateway --test access_control_e2e_test --offline`

本轮完成后还需继续执行格式化、模块级和全工作区离线回归。

## 8. 当前剩余风险

### 8.1 中风险：`left -> rejoin` 仍未冻结

- owner transfer 已补齐
- 但成员主动离开后如何重新进入 active member 集合仍未定义

### 8.2 中风险：promote / demote 仍未标准化

- 本轮只做 owner handoff
- 没有扩张成通用角色治理命令

### 8.3 中风险：特殊会话类型生命周期仍未冻结

- `agent_dialog`
- `agent_handoff`
- `system_channel`

## 9. 下一轮计划

1. 冻结 `left -> rejoin` 标准，明确 member identity 与 cursor 保留策略。
2. 评估是否需要独立的 `promote / demote` 治理命令，而不是继续复用 owner transfer。
3. 继续 review 特殊会话类型的成员生命周期标准。

## 10. 后续状态更新（2026-04-06）

- `left -> rejoin` 已在 `docs/review/2026-04-06-left-rejoin-review-cycle.md` 中完成 review 闭环。
- 现行冻结标准为：
  - 重入不复活旧 member record
  - 重入创建新的 membership episode 与新的 `memberId`
  - 新 episode 的 read cursor 从 `0` 重新开始
  - stale `memberId` 不得再影响当前 active member
- 当前下一优先级 backlog 已切换为：
  - 通用 `promote / demote` 治理标准
  - 特殊会话类型生命周期治理

