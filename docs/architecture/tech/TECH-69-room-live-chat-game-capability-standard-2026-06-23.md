> Migrated from `docs/架构/69-room-live-chat-game-capability-standard-2026-06-23.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 69. 直播间 / 聊天室 / 游戏房间能力标准（2026-06-23）

## 1. 目标

在 sdkwork-im 会话内核之上，冻结 **live / chat / game** 三类房间的业务绑定与生命周期入口，避免各业务重复发明会话拓扑。

## 2. 设计原则

### 2.1 房间是会话绑定，不是第二消息内核

- 房间 `roomId` 通过 `ConversationBusinessBinding` 绑定到 `group` 会话
- 消息、实时 fanout、ack、trim 仍走既有 conversation + realtime 内核
- 游戏走棋、直播弹幕、聊天室文本统一复用 `message.posted`

### 2.2 三类房间语义

| roomKind | businessType | 默认 historyVisibility | 默认 maxMembers | 典型场景 |
|----------|--------------|------------------------|-----------------|----------|
| `live` | `live_room` | `shared` | 10000 | 直播间评论（IM 路径适合中等规模；超大规模热路径需业务层 Redis 广播） |
| `chat` | `chat_room` | `shared` | 1000 | 语聊房 / 兴趣群 |
| `game` | `game_room` | `joined` | 8 | 棋牌 / 桌游 |

### 2.3 游戏走棋载荷

- 使用 `DataPart` + `urn:sdkwork:sdkwork-im:message:custom:game.{gameKey}`
- 权威规则与状态机在 Game Service；IM 负责有序持久化与 fanout

## 3. HTTP 入口

| 方法 | 路径 | 说明 |
|------|------|------|
| POST | `/im/v3/api/chat/rooms` | 创建房间并绑定 group 会话 |
| GET | `/im/v3/api/chat/rooms/{roomId}` | 查询房间视图 |
| POST | `/im/v3/api/chat/rooms/{roomId}/enter` | 自助进房（无需 admin 邀请） |
| POST | `/im/v3/api/chat/rooms/{roomId}/leave` | 离房 |

OpenAPI operationId：`rooms.create` / `rooms.get` / `rooms.enter` / `rooms.leave`

创建请求体：

```json
{
  "conversationId": "c_live_001",
  "roomId": "room_live_001",
  "roomKind": "live"
}
```

## 4. 实时订阅

客户端绑定会话后，订阅：

```json
{
  "scopeType": "conversation",
  "scopeId": "c_live_001",
  "eventTypes": ["message.posted", "conversation.member_joined", "conversation.member_left"]
}
```

## 5. 安全与性能

- `live_room` 发消息默认 **5 条/秒/用户**（`SDKWORK_IM_LIVE_ROOM_MESSAGE_RATE_LIMIT`，上限 60）
- 房间 enter 受 `maxMembers` 约束
- 所有 room 命令必须携带 `organization_id`（来自 AppContext，不得硬编码 `default`）

## 6. 边界

- 超大规模直播弹幕热路径：业务层 Redis/SSE 广播 + IM 存精选/公告
- 游戏防作弊：Game Service 校验后再代发 IM 消息
- RTC 音视频： sibling `sdkwork-rtc`

