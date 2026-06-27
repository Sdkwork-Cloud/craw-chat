> Migrated from `docs/架构/06-Gateway-API-与协议设计.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Gateway API 与协议设计

## 0. 协议基线引用

本文件的网关与协议设计，从本轮开始统一受以下协议基线约束：

- `143-统一协议总纲与分层设计-2026-04-06`
- `144-CCP传输绑定与握手协商设计-2026-04-06`
- `145-CCP数据协议与版本兼容安全设计-2026-04-06`

其中：

- `HTTP / WebSocket / SSE / MQTT` 是标准传输
- `CCP = Sdkwork IM Protocol` 是统一应用协议
- `JSON / CBOR` 是物理编码，不是协议本身

## 1. API 总体原则

- 外部采用 `HTTP + WebSocket + SSE + MQTT`，统一承载 `CCP`
- 内部 RPC 可使用 `gRPC` 或其他服务间协议，但不属于客户端协议族的一部分
- 所有外部请求的 `tenant / actor / sender` 都从认证上下文推导
- 统一 `CCP envelope` 与错误模型
- 协议版本与schema 版本双轨治理
- 支持离散消息与流式消息

## 2. Gateway 分层

- `Edge Gateway`
- `Session Gateway`
- `App Gateway API`
- `Admin / Ops API`

## 3. 核心外部 API（标准目标面）

- 当前代码已落地的最小闭环包括：会话创建与摘要查询、消息发送与时间线查询、媒体上传创建/完成/查询/绑定、流开启/检查点/完成、RTC 创建/邀请/接受/拒绝/结束

### 3.1 认证与会话

- `sdkwork-appbase` login verification
- `sdkwork-appbase` dual-token refresh
- `POST /im/v3/api/presence/heartbeat`
- `GET /im/v3/api/presence/me`
- `POST /im/v3/api/realtime/subscriptions/sync`
- `GET /im/v3/api/realtime/events`
- `POST /im/v3/api/realtime/events/ack`

### 3.2 会话与成员

- `POST /im/v3/api/chat/conversations`
- `GET /im/v3/api/chat/conversations/{id}`
- `GET /im/v3/api/chat/conversations/{id}/members`
- `POST /im/v3/api/chat/conversations/{id}/members/add`
- `POST /im/v3/api/chat/conversations/{id}/members/remove`
- `POST /im/v3/api/chat/conversations/{id}/members/transfer-owner`
- `POST /im/v3/api/chat/conversations/{id}/members/change-role`
- `POST /im/v3/api/chat/conversations/{id}/members/leave`
- `GET /im/v3/api/chat/conversations/{id}/read-cursor`
- `POST /im/v3/api/chat/conversations/{id}/read-cursor`
- `GET /im/v3/api/chat/inbox`
- `POST /im/v3/api/chat/conversations` 当前只接受以下 `conversationType`：
  - `group`
  - `direct`
- special conversation dedicated create 状态：
  - `agent_dialog` 只能通过 `POST /im/v3/api/chat/conversations/agent_dialogs` 创建
  - `agent_handoff / system_channel` 当前仍是保留类型，不能通过 generic create 创建
- 未注册类型和保留 special type 都必须在 create-time 直接返回 `400 conversation_type_invalid`
- `POST /im/v3/api/chat/conversations` 的创建者身份必须来自认证上下文，而不是请求体
  - `tenantId` 来自认证上下文
  - `creatorId` 来自认证上下文
  - `creatorKind` 来自认证上下文中：`actor_kind`
- `agent_dialog` 当前阶段已经改为 dedicated create 暴露；generic create 仍然禁止直接创建
- `agent_handoff / system_channel` 当前阶段仍不允许通过 generic create 暴露；否则会生成没有专用参与者拓扑的残缺会话
- `POST /im/v3/api/chat/conversations/{id}/members/transfer-owner` 用于 group owner 向另一名 active member 交接 owner 角色，请求体只接受 `memberId`
- 当前最小实现中：
  - 仅 `group` 支持 transfer-owner
  - 仅当前 active `owner` 可以发起
  - 目标必须是另一名 active member
  - 交接后旧 owner 降为 `admin`，新 owner 成为唯一 `owner`
- owner transfer 统一写入 `conversation.owner_transferred`，并作为原 owner 后续 `leave` 的前置治理动作
- `POST /im/v3/api/chat/conversations/{id}/members/change-role` 用于 group owner 对另一名 active non-owner member 执行通用角色治理，请求体只接受 `memberId` 与目标 `role`
- 当前最小实现中：
  - 仅 `group` 支持 `change-role`
  - 仅当前 active `owner` 可以发起
  - 目标必须是当前 active non-owner member
  - 目标 `role` 只允许变更为 `admin / member / guest`
  - 涉及 `owner` 的角色变化必须使用 `transfer-owner`
  - stale `memberId` 会被拒绝，不能命中历史 membership episode
- 通用角色治理统一写入 `conversation.member_role_changed`，返回 `previousMember / updatedMember / changedAt`，并同步更新读模型中的成员快照
- `POST /im/v3/api/chat/conversations/{id}/members/leave` 用于当前认证主体主动离开当前会话，请求体可为空，不接受 `tenantId`、`principalId`、`memberId`
- 当前最小实现中：
  - `group` `admin / member / guest` active member 可以 leave
  - `group owner` 必须先完成owner transfer，之后按新 owner 规则 leave
  - `direct` 与尚未冻结成员生命周期的特殊会话类型统一拒绝 leave
- 主动离开统一写入 `conversation.member_left`，并立即失去 active member 访问权；不得复用 `members/remove` 模拟 self leave
- `left` 后若原 owner/admin 再次通过 `members/add` 重新加入，服务端必须创建新的 membership episode
  - 历史 `left / removed` 成员记录保留，不覆盖
  - 新 episode 生成新的 `memberId`
  - 当前 active member 关系总是指向最episode

### 3.3 消息

- `POST /im/v3/api/chat/conversations/{id}/messages`
- `GET /im/v3/api/chat/conversations/{id}/messages`
- `POST /im/v3/api/chat/messages/{id}/edit`
- `POST /im/v3/api/chat/messages/{id}/recall`
- 当前最小实现中，`POST /im/v3/api/chat/conversations/{id}/messages` 支持 `text` 与 `parts` 混合提交；`parts` 可包含 `text/data/media/signal/stream_ref`
- 已读推进统一使用 `POST /im/v3/api/chat/conversations/{id}/read-cursor`，避免把 read/ack 状态混入消息发送命令
- `POST /im/v3/api/chat/messages/{id}/edit` `POST /im/v3/api/chat/messages/{id}/recall` 作用于既有消息，服务端必须自行解析消息所属会话，客户端无需重复提交 `conversationId`
- 编辑和撤回都采用事件追加，不允许原地覆盖 durable message log

`POST /im/v3/api/chat/messages/{id}/edit` 请求示例：

```json
{
  "summary": "edited summary",
  "text": "edited body"
}
```

`POST /im/v3/api/chat/messages/{id}/recall` 请求示例：

```json
{}
```

### 3.4 流

- `POST /im/v3/api/streams`
- `POST /im/v3/api/streams/{id}/checkpoint`
- `POST /im/v3/api/streams/{id}/complete`
- `POST /im/v3/api/streams/{id}/abort`
- `WebSocket stream.command / stream.event` 用于连续数据帧传输，HTTP 只负责流生命周期命令

### 3.5 RTC 信令

- `POST /im/v3/api/calls/sessions`
- `POST /im/v3/api/calls/sessions/{id}/invite`
- `POST /im/v3/api/calls/sessions/{id}/accept`
- `POST /im/v3/api/calls/sessions/{id}/reject`
- `POST /im/v3/api/calls/sessions/{id}/end`
- `POST /im/v3/api/calls/sessions/{id}/signals`
- 当前最小实现先通过 `invite/accept/reject/end` 写入信令状态，独立 `signals` 接口保留为后续扩展
- 当前最小实现中，当 RTC 会话已绑定 `conversationId` 时，`invite/accept/reject/end` 会额外提交一条 `messageType=signal` 的消息，消息体包含 `SignalPart`，并进入时间线与摘要投影

### 3.6 文件资源

- 文件上传、完成、版本、权限和访问 URL 接口属于 `sdkwork-drive`
- IM 只通过 `POST /im/v3/api/chat/conversations/{id}/messages` 接收媒体消息内容
- 媒体消息 part 必须携带 `ContentPart.drive` (`DriveReference`) `source=drive` `MediaResource` 使用快照
- Gateway 必须拒绝 sdkwork-im 自有媒体生命周期路由和存储内部字段：

### 3.7 通知

- `POST /im/v3/api/notifications/requests`
- `GET /im/v3/api/notifications`
- `GET /im/v3/api/notifications/{id}`
- 当前最小实现中，消息提交后会在 `sdkwork-im-server` 中以 side-effect 方式触发 `notification.requested -> notification.dispatched`

### 3.8 自动化

- `POST /im/v3/api/automation/executions`
- `GET /im/v3/api/automation/executions/{id}`
- 当前最小实现中，自动化请求会同步走通 `automation.execution_requested -> automation.execution_completed` 最小链路，并为调用者生成一条 `automation.result` 站内通知

### 3.9 审计与运维

- `POST /backend/v3/api/audit/records`
- `GET /backend/v3/api/audit/records`
- `GET /backend/v3/api/audit/export`
- `GET /backend/v3/api/ops/health`
- `GET /backend/v3/api/ops/cluster`
- `GET /backend/v3/api/ops/lag`
- `GET /backend/v3/api/ops/diagnostics`
- 当前最小实现中，消息提交、通知请求、自动化执行都会留下审计锚点；运维面暴露单节点 `standalone.split-services.development` 的 cluster / lag / diagnostic 视图：

### 3.10 认证上下文约定

- 外部 API 请求体不得显式提供 `tenantId`、`creatorId`、`senderId`、`initiatorId`
- `tenant`、`actor`、`session` 必须从已校验的 JWT / session / trusted identity context 中提取
- `clientRouteId` 优先从 JWT claim 或可信头提取；仅在设备首次注册时允许显式输入，并在进入命令与投影层前绑定到认证上下文档
- gateway 负责把认证上下文转换成内部命令上下文与 envelope 字段：
- 对外 app-facing 入口默认必须要求 `Authorization: Bearer ...`；`trusted identity headers` 仅允许用于内部可信链路、测试装配或显式声明internal profile
- 因此 `tenant_id` 仍然会出现在命令 envelope、事件envelope、持久化对象与审计日志中，但这些字段属于服务端权威写入字段，不属于客户端可覆盖字段：
- 消息载荷中的发送者统一使用 `sender` 对象，而不是 `senderId` + `senderKind` 平铺字段：

### 3.11 已读游标 API 约定

- `GET /im/v3/api/chat/conversations/{id}/read-cursor`
- `POST /im/v3/api/chat/conversations/{id}/read-cursor`
- 租户、操作者、成员身份都必须由认证上下文和服务端成员关系推导，请求体不接受 `tenantId`、`principalId`、`memberId`
- `POST` 请求体建议：

```json
{
  "readSeq": 12,
  "lastReadMessageId": "msg_c_xxx_12"
}
```

- `GET/POST` 返回体建议：

```json
{
  "tenantId": "t_xxx",
  "conversationId": "c_xxx",
  "memberId": "cm_xxx",
  "principalId": "u_xxx",
  "readSeq": 12,
  "lastReadMessageId": "msg_c_xxx_12",
  "updatedAt": "2026-04-05T10:00:10Z",
  "unreadCount": 3
}
```

- `readSeq` 只能前进不能回退；小于当前游标的重复提交按幂等处理，返回当前高水位
- 当成员经由 `left -> rejoin` 形成新的 membership episode 时：
  - `GET /read-cursor` 只返回当前 active episode 对应的 `memberId`
  - 新 episode 的初始游标必须从 `readSeq = 0` 开启
  - 历史 episode 的游标仅作为历史数据保留，不能复用到新 episode

### 3.12 Inbox API 约定

- `GET /im/v3/api/chat/inbox`
- inbox 只返回当前认证主体具备活跃成员关系的会话
- 返回体建议：

```json
{
  "items": [
    {
      "tenantId": "t_xxx",
      "principalId": "u_xxx",
      "memberId": "cm_xxx",
      "conversationId": "c_xxx",
      "conversationType": "group",
      "messageCount": 12,
      "lastMessageId": "msg_c_xxx_12",
      "lastMessageSeq": 12,
      "lastSenderId": "u_other",
      "lastSenderKind": "user",
      "lastSummary": "hello",
      "unreadCount": 3,
      "lastActivityAt": "2026-04-05T10:00:10Z"
    }
  ]
}
```

- 最小实现阶段 inbox 以查询视图方式构建，不承担精确分页、标签过滤与置顶规则；这些能力后续在不破坏基础结构的前提下追加

### 3.13 Client Route Presence API 约定

- `POST /im/v3/api/presence/heartbeat`
- `GET /im/v3/api/presence/me`
- `POST /im/v3/api/realtime/subscriptions/sync`
- `GET /im/v3/api/realtime/events`
- `POST /im/v3/api/realtime/events/ack`
- `GET /im/v3/api/realtime/ws`

IM 只保留客户端路由、presence 与 realtime 传输语义；设备目录、设备孪生、命令、遥测和 IoT 协议能力归属 sibling `sdkwork-aiot`，通过 `/app/v3/api/iot/*`、`/backend/v3/api/iot/*` `sdkwork-aiot-*` SDK 消费，不在 Sdkwork IM IM API 中暴露旧 IM 设备目录、同步流或孪生路由

`clientRouteId` 作为兼容的客户端路由身份字段仍可来自认证上下文或受信请求头；服务端内部应使用 client-route 语义处理路由、presence、realtime ack WebSocket 绑定，不再把它建模为 Sdkwork IM 自有设备能力

