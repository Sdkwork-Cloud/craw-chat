> Migrated from `docs/架构/55-成员治理与Direct会话成员边界标准-2026-04-06.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 55-成员治理与 Direct 会话成员边界标准-2026-04-06

## 1. 背景

在前两轮已经冻结“会话私有读写必须以前置 active member 校验作为入口门槛”之后，继续 review 发现 `conversation-runtime` 的成员治理仍然存在高危缺口：

- `add_member(...)` 与 `remove_member(...)` 之前只校验“调用者是不是 active member”。
- 这意味着普通群成员此前可以直接拉人、踢人。
- `add_member(...)` 还允许请求体直接指定 `role`，因此普通成员或普通管理员此前还能直接创建 `admin / owner`，把群治理模型整体打穿。
- `direct` 会话此前也可以继续通过通用 `members/add` 入口扩容到第三人，退化成“伪群聊”。

这类问题不属于接入层校验漏网，而是会话内核缺少“成员治理就是治理动作，不是普通成员写操作”的标准。

## 2. 风险

若不冻结此标准，会带来以下商业级风险：

- 任意普通群成员可扩张或裁剪群成员集合。
- 管理权限可被低权限成员通过 `role` 参数直接提升。
- `direct` 会话的二人语义被破坏，后续消息权限、已读、通知 fanout、RTC 范围都会随之失真。
- 不同接入层即使各自补了前置校验，只要 runtime 不收口，就仍然无法保证统一安全边界。

## 3. 标准

### 3.1 成员治理必须收敛到 runtime

- `add_member` / `remove_member` 属于会话治理动作。
- 其授权判定必须在 `conversation-runtime` 内执行，不能只依赖 gateway 或聚合节点前置判断。
- 外部入口可以做更早的拒绝，但真正的授权真相必须以 runtime 为准。

### 3.2 Group 会话的治理矩阵

对于 `conversation_type = group`：

- `owner`
  - 可以添加 `admin / member / guest`
  - 不允许通过 `add_member` 再创建第二个 `owner`
  - 可以移除 `admin / member / guest`
  - 不允许通过 `remove_member` 移除 `owner`
- `admin`
  - 只能添加 `member / guest`
  - 不允许创建 `admin / owner`
  - 只能移除 `member / guest`
  - 不允许移除 `admin / owner`
- `member / guest`
  - 不允许添加任何成员
  - 不允许移除任何成员

当前阶段不在 `add_member/remove_member` 上承载以下能力：

- owner 转移
- 自我离开群并进入 `left` 状态
- 邀请态与审批流

这些能力需要在后续单独冻结标准，不能隐式复用当前“加入/移除”语义。

### 3.3 Direct 会话的成员边界

对于 `conversation_type = direct`：

- 允许 `owner` 添加一个且仅一个对端参与者，用于补齐 direct 的第二参与方。
- direct 的新增对端不允许被赋予 `owner / admin` 角色。
- 当 active participants 已达到 2 个后，任何继续 `add_member` 的请求都必须被拒绝。
- `direct` 会话不允许使用通用 `remove_member` 做成员裁剪。

这保证了 direct 会话始终保持“双人边界”，不会被错误扩展为多成员会话。

### 3.4 其他会话类型

对于当前尚未冻结成员治理语义的会话类型，例如：

- `agent_dialog`
- `agent_handoff`
- `system_channel`

当前阶段统一拒绝通用 `add_member/remove_member` 治理动作，直到对应会话类型的成员模型和治理矩阵被单独定义。

## 4. 实现约束

### 4.1 授权顺序

`add_member(...)`：

1. 先解析目标会话。
2. 解析当前调用者的 active member 身份。
3. 先校验“调用者是否具备该会话类型下的治理资格”。
4. 对已存在 active member 保留 `MemberAlreadyExists` 语义。
5. 再校验角色提升、direct 二人上限等治理约束。
6. 只有全部通过后才允许写入成员状态和提交 `conversation.member_joined` 事件。

`remove_member(...)`：

1. 先解析目标会话。
2. 解析当前调用者的 active member 身份。
3. 读取目标成员。
4. 在真正修改成员状态之前执行角色治理校验。
5. 只有治理校验通过后，才允许写入 `removed` 状态并提交 `conversation.member_removed` 事件。

### 4.2 错误语义

- 治理动作被拒绝时，统一返回 `403 conversation_permission_denied`。
- 已存在 active member 的重复加入，继续返回既有 `conversation_member_exists` / `MemberAlreadyExists` 语义。
- 当前阶段不额外引入“owner transfer required”之类新错误码，先保持最小闭环。

## 5. 本轮落地

### 5.1 代码实现

- `services/conversation-runtime/src/lib.rs`
  - 新增成员治理辅助校验：
    - `ensure_member_add_actor_allowed(...)`
    - `ensure_member_add_request_allowed(...)`
    - `ensure_member_remove_allowed(...)`
  - `add_member(...)` 改为先在 runtime 内完成角色治理校验，再写成员状态。
  - `remove_member(...)` 改为先在 runtime 内完成角色治理校验，再写成员状态。

### 5.2 回归测试

- `services/conversation-runtime/tests/conversation_flow_test.rs`
  - `test_group_member_cannot_manage_other_members`
  - `test_group_admin_can_manage_regular_members_but_cannot_escalate_roles`
  - `test_group_owner_cannot_create_second_owner`
  - `test_direct_conversation_owner_can_add_only_single_non_elevated_peer`
  - `test_direct_conversation_rejects_member_removal`
- `services/sdkwork-im-cloud-gateway/tests/access_control_e2e_test.rs`
  - `test_group_member_governance_requires_owner_or_admin`
  - `test_direct_conversation_member_management_is_restricted`

## 6. 验证

本轮已执行并通过：

- `cargo fmt --all`
- `cargo test -p conversation-runtime --test conversation_flow_test --offline`
- `cargo test -p sdkwork-im-cloud-gateway --test access_control_e2e_test --offline`

随后还需继续纳入工作区全量离线验证，确保与其它服务组合时没有行为回归。

## 7. 后续计划

下一轮建议继续围绕“成员生命周期”而不是“成员治理”推进，重点补齐：

1. `leave` 语义与 `left` 状态，不再让成员退出依赖 `remove_member`
2. owner transfer / admin promote / demote 的显式治理命令
3. `agent_dialog / agent_handoff / system_channel` 的成员模型与治理矩阵
4. 成员治理事件与 audit / moderation / policy 模型的进一步对齐

## 8. 后续状态更新（2026-04-06）

- `leave` 标准已在 `docs/架构/56-成员主动离开会话与left状态标准-2026-04-06.md` 中冻结。
- `owner transfer` 标准已在 `docs/架构/57-group-owner-transfer标准-2026-04-06.md` 中冻结。
- 通用 `promote / demote` 标准已在 `docs/架构/59-group-member-role-change标准-2026-04-06.md` 中冻结。
- 当前剩余未关闭项收敛为：
  - 特殊会话类型成员生命周期标准
  - membership episode 的审计/查询视图扩展

