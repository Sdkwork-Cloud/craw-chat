> Migrated from `docs/review/2026-04-06-会话成员主动离开语义-review-cycle.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 会话成员主动离开语义 Review Cycle（2026-04-06）

## 1. 当前阶段

- 前置基础已经完成：
  - 群组成员治理矩阵已经冻结
  - `direct` 会话成员上限与角色边界已经冻结
  - `active member` 已经成为会话读写、RTC、stream 的统一前置门槛
- 本轮 review 聚焦的问题不是“谁可以治理别人”，而是“成员自己如何结束活跃成员关系”。

## 2. 问题列表

### 2.1 高风险：数据模型存在 `left`，但运行时没有显式 leave 语义

问题表现：

- `ConversationMember.state` 已经存在 `left`
- 运行时只有 `add_member(...)` 和 `remove_member(...)`
- 成员主动退出会话只能缺省为空能力，或者被错误地复用为 `remove_member`

风险：

- 主动离开与被治理移除的审计语义混淆
- 后续 owner transfer、重新加入、审批流都无法建立在清晰生命周期之上
- 接入层容易用错误动作模拟 leave，导致行为在不同入口不一致

### 2.2 中风险：事件模型缺少 `conversation.member_left`

问题表现：

- 事件层此前只有 `conversation.member_joined` 与 `conversation.member_removed`
- 投影与审计无法区分“治理移除”和“主动退出”

风险：

- inbox、成员投影、审计报表都只能看到终态，无法区分业务原因
- 后续对接工作流、策略、通知时会丢失关键因果

### 2.3 中风险：HTTP 接入层没有统一暴露 leave 能力

问题表现：

- `conversation-runtime` 测试已进入 red，声明了 `/members/leave` 路由但未完成 handler
- `sdkwork-im-server` 未挂接 `/members/leave`，导致 HTTP 直接返回 `404`

风险：

- 领域层、接入层、投影层语义不一致
- 文档标准无法落地为真实可调用接口

## 3. 根因分析

- 上一轮 `docs/架构/55-成员治理与Direct会话成员边界标准-2026-04-06.md` 冻结的是治理矩阵，不是成员生命周期矩阵。
- 代码实现只补齐了 “join / remove” 两个治理分支，没有补齐“self leave”这个生命周期分支。
- 因为没有独立事件类型，投影服务默认把成员生命周期收敛成 `joined -> removed`，使 `left` 字段停留在数据模型层面，没有形成系统行为。

## 4. 本轮冻结标准

### 4.1 接口标准

- 新增 `POST /im/v3/api/chat/conversations/{conversationId}/members/leave`
- 不接受 `tenantId`、`principalId`、`memberId`
- 当前操作者身份必须从认证上下文推导
- 请求体允许为空

### 4.2 权限标准

- `group`
  - `admin / member / guest` 的 active member 可以主动离开
  - `owner` 暂不允许主动离开，直到 owner transfer 标准冻结
- `direct`
  - 当前阶段不支持 self leave
- 其他会话类型
  - 当前阶段统一拒绝，直到对应成员生命周期标准单独冻结

### 4.3 事件与投影标准

- 主动离开必须提交 `conversation.member_left`
- `remove_member` 继续只表达治理移除，不得复用为自离开
- `member_left` 进入投影后，应立即从 active member 视图移除
- 成员离开后，应立即失去该会话的 active member 访问权

## 5. 本轮实现

### 5.1 领域层

- `services/conversation-runtime/src/lib.rs`
  - 完成 `leave_conversation(...)`
  - 新增 `ensure_member_leave_allowed(...)`
  - `build_member_envelope(...)` 支持 `conversation.member_left`
  - `group` leave 进入 `MembershipState::Left`
  - 写入 `removed_at`
  - 从 `principal_members` 活跃索引移除

### 5.2 投影层

- `services/projection-service/src/lib.rs`
  - `apply(...)` 新增 `conversation.member_left`
  - `apply_member_left(...)` 复用 active member 移除逻辑

### 5.3 接入层

- `services/sdkwork-im-cloud-gateway/src/lib.rs`
  - 新增 `/im/v3/api/chat/conversations/{conversation_id}/members/leave`
  - 新增 `leave_conversation(...)`
  - 新增审计动作 `conversation.member_left`

## 6. 新增与覆盖测试

- `services/conversation-runtime/tests/conversation_flow_test.rs`
  - `test_group_member_can_leave_and_loses_access`
  - `test_group_owner_cannot_leave_without_transfer`
  - `test_direct_conversation_rejects_leave_for_now`
- `services/conversation-runtime/tests/http_smoke_test.rs`
  - `test_group_member_can_leave_roster_over_http`
- `services/sdkwork-im-cloud-gateway/tests/access_control_e2e_test.rs`
  - `test_group_member_can_leave_and_then_loses_conversation_access`

## 7. 验证结果

已执行并通过：

- `cargo test -p conversation-runtime --test conversation_flow_test --offline`
- `cargo test -p conversation-runtime --test http_smoke_test --offline`
- `cargo test -p sdkwork-im-cloud-gateway --test access_control_e2e_test --offline`
- `cargo fmt --all`
- `cargo test -p conversation-runtime --offline`
- `cargo test -p projection-service --offline`
- `cargo test -p sdkwork-im-cloud-gateway --offline`
- `cargo test --workspace --offline`

## 8. 当前剩余风险

### 8.1 中风险：owner transfer 仍未冻结

- 当前标准只做“owner 不能 leave”
- 但 owner 如何转移、何时允许最后一个 owner 退出，仍未定义

### 8.2 中风险：`left` 后重新加入语义未冻结

- 当前标准只保证 active member 立即失效
- 但 `left` 后是否允许被重新邀请、是否保留历史 cursor、是否生成新 memberId，尚未冻结

### 8.3 中风险：其他会话类型成员生命周期仍未冻结

- `agent_dialog`
- `agent_handoff`
- `system_channel`

这些类型目前拒绝 generic leave 是正确的，但后续仍需单独标准化。

## 9. 下一轮计划

1. 冻结 owner transfer 标准，避免 group owner 永久不能演化。
2. 冻结 `left -> rejoin` 标准，明确是复活旧 member 还是生成新 member。
3. 补成员生命周期与通知、工作流、策略系统的事件联动标准。
4. 继续 review 其他会话类型的成员模型，避免通用治理接口被误用。

