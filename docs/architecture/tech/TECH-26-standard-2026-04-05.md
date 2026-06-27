> Migrated from `docs/架构/26-消息变更授权与通知收件人标准-2026-04-05.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 消息变更授权与通知收件人标准（2026-04-05）

## 1. 目标

本标准用于冻结两类此前仍存在歧义或实现缺口的行为边界：

- `message.edited / message.recalled` 的授权矩阵
- `message.posted -> notification.requested` 的收件人选择规则

目标不是增加额外能力，而是先把商业化产品必须具备的“谁能改、谁能撤回、谁能收到通知”收敛为稳定标准，并让当前 `standalone.split-services.development` 最小实现与该标准一致。

## 2. 消息变更授权标准

### 2.1 编辑 `message.edited`

- 仅原发送者可以编辑自己的消息。
- 活跃成员但非原发送者，不得编辑他人消息。
- `owner / admin` 不具备“代编辑”权限。

原因：

- 编辑会改变消息内容本身，属于作者语义的一部分。
- 管理角色可以做治理，但不应伪造或篡改他人表达内容。
- 对 IM、Bot、Agent、Webhook 场景，统一保持“谁发的，谁能改”最稳定。

### 2.2 撤回 `message.recalled`

- 原发送者可以撤回自己的消息。
- 在 `group` 会话中，`owner / admin` 可以撤回他人消息，作为治理动作。
- 在 `direct` 会话中，不允许 `owner / admin` 覆盖式撤回他人消息。
- 非活跃成员始终不得撤回任何消息。

原因：

- 撤回是治理动作，不一定等同于内容编辑。
- 群聊场景需要 moderation 能力，否则无法支撑商业群组治理。
- 单聊场景不应因为建会话方是 `owner` 就获得对对端消息的管理权限。

### 2.3 Agent / Bot / System 身份

- `sender.id` 代表稳定发送主体。
- 只要 `sender.id == 当前认证主体`，即视为“原发送者”。
- 群聊中的 `owner / admin` 仍可撤回 Agent / Bot / System 发出的消息，但不能代编辑。

## 3. 通知收件人标准

### 3.1 收件人来源

- 消息通知收件人必须来自当前会话的 active members。
- 当前最小实现统一通过 `conversation_runtime.list_members(...)` 推导收件人集合。

### 3.2 默认规则

- `message.posted` 产生的 `message.new` 或 `rtc.event` 通知，默认发给“除发送者以外的其他活跃成员”。
- 发送者默认不回给自己站内通知。
- 若会话当前没有其他活跃成员，则不生成 `message.posted` 站内通知任务。

原因：

- 消息提交成功已经由写路径响应给发送者，重复发站内通知没有产品价值。
- 对群聊、单聊、Agent 互通场景，这一规则更符合主流 IM 产品行为。

### 3.3 任务粒度

- 每个收件人生成一条独立通知任务。
- `notification_id` 必须包含 `message_id + recipient_id`，避免多收件人时互相覆盖。
- 当前最小实现仍采用同步 `requested -> dispatched` 最小流水；后续可以替换为异步 worker，不改变本标准。

### 3.4 未来扩展边界

当前阶段先冻结“按成员 fanout”的最小标准，暂不引入以下复杂度：

- 按设备在线态分流通知
- Push / Email / SMS 多通道编排
- 免打扰、优先级、合并折叠策略
- mention 定向增强、关键词通知

这些能力后续可以在不破坏本标准的前提下向下扩展。

## 4. 本轮落地映射

- `conversation-runtime`
  - 新增会话类型感知，区分 `group` 与 `direct`
  - 编辑仅允许原发送者
  - 撤回允许“原发送者”或“群 owner/admin”
- `sdkwork-im-server`
  - `message.posted` 后按 active members fanout 站内通知
  - 默认排除发送者自身
  - 通知任务按收件人唯一化
- `task10_capabilities_e2e_test`
  - 同步更新为“消息不自通知、自动化结果仍通知调用者”的标准

## 5. 仍待下一轮继续收敛的边界

- 外部入口仍支持 trusted headers fallback，尚未收敛为严格 bearer-only 模式
- 通知仍未按设备在线态、免打扰、push channel 做策略分流
- 群治理动作尚未单独产出 moderation / policy 事件

