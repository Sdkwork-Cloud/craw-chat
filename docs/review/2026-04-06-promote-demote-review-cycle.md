# Promote Demote Review Cycle（2026-04-06）

## 1. 当前阶段

- 成员治理矩阵、`leave`、`owner transfer`、`left -> rejoin` 已经冻结。
- 当前剩余的核心会话治理缺口，是缺少通用 `promote / demote` 角色治理标准。

## 2. 问题列表

### 2.1 高风险：缺少通用 role mutation 链路，owner 无法完成日常角色治理

问题表现：

- 当前只有 `owner transfer`，没有通用的成员角色变更命令。
- owner 无法安全地执行：
  - `member -> admin`
  - `admin -> member`
  - `member <-> guest`

风险：

- 群治理能力不完整，商业化场景下无法表达正式的管理员授予和降级流程。
- 上层接入若自行补“角色变更”逻辑，会再次把治理真相从 runtime 中打散。

### 2.2 高风险：新的 role mutation 若不防御 stale `memberId`，会打穿 membership episode 边界

问题表现：

- `left -> rejoin` 已经改为新 episode 生成新 `memberId`。
- 如果通用角色治理仍接受历史 `memberId`，旧成员身份就可能影响当前 active member。

风险：

- 历史数据会重新参与当前治理动作。
- membership episode 标准被新命令绕开。

### 2.3 中风险：projection 读模型如果不跟进 role change，会留下新的数据漂移

问题表现：

- runtime 若更新成员 role，而 projection 不应用对应事件，
- 则后续任何依赖投影成员快照的场景都会读到旧角色。

风险：

- runtime 与 projection 再次出现语义漂移。
- 后续成员视图、审计视图、策略视图都会埋下不一致隐患。

## 3. 根因分析

- 当前代码只有特例化的 `conversation.owner_transferred`，没有通用 `conversation.member_role_changed`。
- `owner transfer` 只解决 owner 交接，不适合承载普通角色治理。
- local node 侧没有角色治理路由和审计动作。
- projection 侧没有角色变更事件的应用逻辑。

## 4. 本轮冻结标准

### 4.1 会话类型

- 仅 `group` 支持通用角色治理。
- `direct / agent_dialog / agent_handoff / system_channel` 当前阶段统一拒绝。

### 4.2 授权

- 仅当前 active `owner` 可发起。
- `admin / member / guest` 不允许发起通用 role mutation。

### 4.3 目标成员与目标角色

- 目标必须是当前 active non-owner member。
- 目标角色只允许变更为 `admin / member / guest`。
- 涉及 `owner` 的角色变化必须使用 `transfer-owner`，不能复用通用 role mutation。
- stale `memberId` 必须被拒绝。

### 4.4 事件与审计

- 新增事件：`conversation.member_role_changed`
- 新增审计动作：`conversation.member_role_changed`
- 事件与响应统一返回：
  - `previousMember`
  - `updatedMember`
  - `changedAt`

## 5. 本轮实现

### 5.1 领域层

- `services/conversation-runtime/src/lib.rs`
  - 新增 `ChangeConversationMemberRoleCommand`
  - 新增 `ChangeConversationMemberRolePayload`
  - 新增 `ChangeConversationMemberRoleResult`
  - 新增 `change_conversation_member_role(...)`
  - 新增 `ensure_member_role_change_allowed(...)`
  - 新增 `conversation.member_role_changed` 事件构建

### 5.2 接入与审计

- `services/conversation-runtime/src/lib.rs`
  - 新增 runtime HTTP 路由 `/members/change-role`
- `services/sdkwork-im-cloud-gateway/src/lib.rs`
  - 新增 app 路由 `/members/change-role`
  - 新增 `conversation.member_role_changed` 审计锚点

### 5.3 投影层

- `services/projection-service/src/lib.rs`
  - 新增 `conversation.member_role_changed` 事件应用
  - 更新成员快照读模型

## 6. 验证结果

已执行并通过：

- `cargo fmt --all`
- `cargo test -p conversation-runtime --test conversation_flow_test --offline`
- `cargo test -p conversation-runtime --test http_smoke_test --offline`
- `cargo test -p projection-service --test timeline_projection_test --offline`
- `cargo test -p sdkwork-im-cloud-gateway --test access_control_e2e_test --offline`
- `cargo test -p conversation-runtime --offline`
- `cargo test -p projection-service --offline`
- `cargo test -p sdkwork-im-cloud-gateway --offline`
- `cargo test --workspace --offline`

## 7. 当前剩余风险

1. 特殊会话类型的成员生命周期与治理矩阵仍未冻结。
2. membership episode 当前缺少独立的历史查询视图。
3. 通用角色治理事件尚未进一步扩展到更细分的 policy / automation 语义。

## 8. 下一轮计划

1. 继续 review `agent_dialog / agent_handoff / system_channel` 的成员生命周期标准。
2. 评估是否增加 membership episode 的审计/查询视图。
3. 评估是否需要更细粒度的成员治理策略事件，但保持当前接口稳定。
