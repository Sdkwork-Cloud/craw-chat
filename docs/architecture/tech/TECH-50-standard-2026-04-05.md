> Migrated from `docs/架构/50-接入层与会话内核真实时间戳标准-2026-04-05.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 50-接入层与会话内核真实时间戳标准-2026-04-05

## 1. 问题

在第 49 号标准之后继续 review 发现：

- `session-gateway` 的 realtime 运行时曾使用固定时间戳常量填充：
  - `synced_at`
  - `subscribed_at`
  - `acked_at`
  - `occurred_at`
  - checkpoint `updated_at`
- `conversation-runtime` 的会话与消息内核曾使用固定时间戳常量填充：
  - `conversation.created` 的 `occurred_at / committed_at`
  - `message.posted` 的 `occurred_at / committed_at`
  - `ConversationMember.joined_at / removed_at`
  - `ConversationReadCursor.updated_at`
  - `message.edited` 的 `edited_at`
  - `message.recalled` 的 `recalled_at`

这类实现会导致：

- 不同时间发生的业务动作在对外模型里表现为同一时间。
- 审计、排障、排序、补偿、投影回放语义失真。
- 同一设备或同一会话上的连续操作失去可观测的先后关系。

## 2. 标准

### 2.1 接入层时间字段必须是真实 UTC 时间

`session-gateway` 中所有对外暴露或持久化的时间字段必须使用真实 UTC 当前时间生成，不允许依赖演示常量。

本轮纳入约束的字段包括：

- realtime subscription `synced_at`
- realtime subscription item `subscribed_at`
- realtime ack `acked_at`
- realtime event `occurred_at`
- realtime checkpoint `updated_at`

### 2.2 会话内核时间字段必须随真实业务动作推进

`conversation-runtime` 中消息、成员、游标、编辑、撤回等动作必须在实际执行时生成时间戳，不能复用固定模板值。

本轮纳入约束的字段包括：

- `conversation.created.occurred_at`
- `conversation.created.committed_at`
- `message.posted.occurred_at`
- `message.posted.committed_at`
- `ConversationMember.joined_at`
- `ConversationMember.removed_at`
- `ConversationReadCursor.updated_at`
- `message.edited.edited_at`
- `message.recalled.recalled_at`

### 2.3 envelope 时间应与对应领域对象保持一致

如果某个事件 envelope 的时间来源于领域对象本身，则 envelope 必须优先复用该领域对象上的时间字段，而不是再次生成独立时间。

例如：

- `conversation.member_joined` 复用 `member.joined_at`
- `conversation.member_removed` 复用 `member.removed_at`
- `conversation.read_cursor_updated` 复用 `cursor.updated_at`
- `message.posted` 复用 `message.occurred_at / committed_at`
- `message.edited` 复用 `edited_at`
- `message.recalled` 复用 `recalled_at`

## 3. 落地

统一复用共享时间工具：

- `crates/im-time::utc_now_rfc3339_millis()`

禁止各模块继续私有实现一套固定常量或不一致的时间生成方式。

## 4. 验证要求

至少覆盖以下回归：

- 两次 realtime subscription sync 的 `synced_at / subscribed_at` 必须不同。
- 两次 realtime publish 的 `occurred_at` 必须不同。
- 两次 checkpoint 持久化的 `updated_at` 必须不同。
- 两条不同消息的 `occurred_at / committed_at` 必须不同。
- 两次 read cursor 更新的 `updated_at` 必须不同。
- 两次成员加入和两次成员移除的时间戳必须不同。
- 两次编辑与两次撤回的时间戳必须不同。

## 5. 后续

仍需继续 review 尚未收敛的生产模块：

- `streaming-service`
- `im-call-runtime`
- `media-service`
- `ops-service`
- `session-gateway` 其他非 realtime 子模块
- `projection-service` 中仍存在的固定时间辅助实现

