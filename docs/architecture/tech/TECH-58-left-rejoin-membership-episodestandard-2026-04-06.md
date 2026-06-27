> Migrated from `docs/架构/58-left-rejoin-membership-episode标准-2026-04-06.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 58-left-rejoin-membership-episode 标准（2026-04-06）

## 1. 背景

在 `leave` 与 `owner transfer` 冻结后，会话成员生命周期中剩余的关键缺口，是成员主动离开后如何重新加入。

如果 `rejoin` 复用历史 `memberId`，会立即引出三个问题：

- read cursor 无法准确区分历史 episode 与当前 episode
- runtime 与 projection 可能对同一个 `memberId` 产生不同处理
- 审计上无法区分“同一个人重新加入”与“同一个成员记录被复活”

因此 `left -> rejoin` 必须独立冻结为 membership episode 标准，而不能作为旧成员记录的状态回滚。

## 2. 目标

本标准只解决一个问题：

- 成员在 `left / removed` 之后再次通过 `add_member` 加入时，如何定义身份、cursor 与历史数据边界

本标准不覆盖：

- promote / demote
- 特殊会话类型生命周期
- 基于 episode 的历史查询 API

## 3. 核心标准

### 3.1 membership episode

- 每次进入 active member 集合，都是一个新的 membership episode。
- `leave` 结束当前 episode。
- `remove_member` 也结束当前 episode。
- 再次 `add_member` 时，必须创建新的 episode，而不是把旧 episode 改回 `joined`。

### 3.2 memberId 规则

- 第一轮 membership episode 保持现有基础格式：

```text
cm_{conversationId}_{principalId}
```

- 第二轮及之后的 episode 使用带序号的扩展格式：

```text
cm_{conversationId}_{principalId}_e2
cm_{conversationId}_{principalId}_e3
```

- `memberId` 的职责是标识一个具体的 membership episode，而不是永久标识 principal。

### 3.3 active member 规则

- 对同一 `principalId`，任一时刻只能有一个 active episode。
- 当前 active member 关系始终指向最新 episode。
- 历史 episode 继续保留在成员历史记录中，但不再出现在 active member 集合中。

## 4. Read Cursor 标准

- read cursor 绑定到 `memberId`，因此天然绑定到 membership episode。
- 新 episode 创建时，必须同时创建新的默认 read cursor：

```json
{
  "readSeq": 0,
  "lastReadMessageId": null
}
```

- 旧 episode 的 cursor 保留，不迁移到新 episode。
- `GET /im/v3/api/chat/conversations/{id}/read-cursor` 只能读取当前 active episode 的 cursor。
- `POST /im/v3/api/chat/conversations/{id}/read-cursor` 只能推进当前 active episode 的 cursor。

## 5. 治理动作标准

### 5.1 add_member

- 若 principal 当前仍 active，则拒绝重复加入。
- 若 principal 历史上存在 `left / removed` episode，则允许重新加入。
- 重新加入时必须生成新的 `memberId`。

### 5.2 remove_member

- `remove_member` 只能作用于当前 active episode。
- 传入历史 stale `memberId` 时，必须拒绝。
- 历史 episode 不允许被当作当前成员再治理一次。

### 5.3 leave

- `leave` 只作用于当前 active episode。
- `leave` 之后当前 principal 立即失去 active member 权限。
- 后续是否回到 active 集合，只能通过新的 `add_member` episode 完成。

## 6. 投影与事件标准

- `conversation.member_joined` 继续作为新 episode 进入 active 集合的统一事件。
- 不新增 `member_rejoined` 事件类型。
- 事件区分依赖于新的 `memberId`，而不是新增一套平行事件族。

这样做的原因：

- 保持事件模型简单
- 让投影层基于新的 `memberId` 自然创建新 cursor
- 避免把“重入”做成状态回滚语义

## 7. 对外 API 影响

- `POST /im/v3/api/chat/conversations/{id}/members/add` 的请求体不变。
- 但当目标 principal 是历史成员重新加入时，响应中的 `memberId` 会变化为新的 episode 身份。
- `GET /read-cursor` 返回的 `memberId` 也会切换为当前 active episode 的新值。

## 8. 一期落地要求

一期必须具备：

- 领域层按 episode 生成新 `memberId`
- 历史成员记录保留
- 新 episode 的默认 read cursor 从 0 开始
- stale `memberId` 被拒绝
- 至少覆盖以下回归场景：
  - `left` 后重新加入会生成新 `memberId`
  - 重新加入后的 read cursor 为新 episode 初始值
  - 使用旧 `memberId` 不能误删当前 active member

## 9. 后续演进

后续可以在不破坏本标准的前提下继续扩展：

1. membership episode 历史查询视图
2. 基于 episode 的审计回放
3. promote / demote 与 membership episode 的组合治理规则

