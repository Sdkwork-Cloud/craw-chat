# 20-WebSocket实时传输绑定标准

## 1. 目标

本标准定义 realtime kernel 的 WebSocket 绑定方式。目标不是再做一套新的实时系统，而是把既有的：

- `realtime/subscriptions/sync`
- `realtime/events`
- `realtime/events/ack`

映射成低延迟长连接传输层。

核心要求：

- HTTP 轮询与 WebSocket 必须共享同一套订阅、窗口、`realtimeSeq`、ack、trim 语义
- 业务事件仍然只写一次 realtime window，WebSocket 只是额外下行通道
- WebSocket 不能绕过 auth、device scope、tenant scope
- 历史补偿仍由 durable 通路负责，不能把 WebSocket 当历史仓库

## 2. 设计原则

### 2.1 WebSocket 是传输绑定，不是第二内核

消息、流、RTC、通知等业务写路径继续调用同一套 `RealtimeDeliveryRuntime`。  
WebSocket 只负责：

- 接收客户端控制帧
- 监听设备窗口的新事件
- 把窗口内容编码成文本帧下行

### 2.2 协议语义与 HTTP 一一对应

- `subscriptions.sync` 对应 `POST /im/v3/api/realtime/subscriptions/sync`
- `events.pull` 对应 `GET /im/v3/api/realtime/events`
- `events.ack` 对应 `POST /im/v3/api/realtime/events/ack`

这意味着：

- WebSocket ack 也会推进同一设备的 `ackedThroughSeq`
- WebSocket ack 也会触发同一窗口的 trim
- WebSocket 与 HTTP 可以交叉使用，状态必须一致

### 2.3 按设备建窗口，不按连接建窗口

realtime window 的所有权属于 `tenantId + principalId + clientRouteId`，不属于某条 WebSocket 连接。

因此：

- 同一设备的 HTTP 轮询与 WebSocket 连接共享 ack/checkpoint
- 同一设备的多条 WebSocket 连接会共享同一窗口状态
- 商业化实现中应避免同一设备的多个活跃连接无协调 ack

## 3. 鉴权与设备绑定

### 3.1 设备标识来源

本阶段标准要求 WebSocket 在握手时就绑定设备。  
推荐优先级：

1. 来自 JWT / session 中的设备声明
2. 来自可信反向代理注入的设备上下文
3. 开发或最小实现阶段，允许使用现有 header 绑定

当前 `sdkwork-im-server` 与 `session-gateway` 的落地实现采用现有 auth context header 方式：

- `x-tenant-id`
- `x-user-id`
- `x-session-id`
- `x-device-id`

### 3.2 浏览器端商用要求

浏览器原生 WebSocket 不能自由设置任意自定义 header。商用版本必须预留以下任一能力：

- Cookie 会话绑定
- `Authorization` 握手透传
- `Sec-WebSocket-Protocol` 中携带 token 引用
- 握手 query 中只放一次性短期票据，不直接暴露长期 access token

本地最小实现暂不承担浏览器票据交换，只验证 transport binding 本身。

## 4. 路由标准

`GET /im/v3/api/realtime/ws`

连接建立后，服务端必须先发送一帧连接状态：

```json
{
  "type": "realtime.connected",
  "tenantId": "100001",
  "principalId": "1",
  "clientRouteId": "d_pad",
  "ackedThroughSeq": 0,
  "trimmedThroughSeq": 0,
  "latestRealtimeSeq": 0
}
```

如果当前设备窗口中存在尚未 ack 的在线事件，服务端可以在 `realtime.connected` 之后立即发送 `reason=catchup` 的 `event.window`。

## 5. 客户端控制帧

### 5.1 `subscriptions.sync`

请求：

```json
{
  "type": "subscriptions.sync",
  "requestId": "req_sync_1",
  "items": [
    {
      "scopeType": "conversation",
      "scopeId": "c_demo",
      "eventTypes": ["message.posted"]
    }
  ]
}
```

响应：

```json
{
  "type": "subscriptions.synced",
  "requestId": "req_sync_1",
  "snapshot": {
    "tenantId": "100001",
    "principalId": "1",
    "clientRouteId": "d_pad",
    "items": [
      {
        "scopeType": "conversation",
        "scopeId": "c_demo",
        "eventTypes": ["message.posted"],
        "subscribedAt": "2026-04-05T10:10:00Z"
      }
    ],
    "syncedAt": "2026-04-05T10:10:00Z"
  }
}
```

### 5.2 `events.pull`

请求：

```json
{
  "type": "events.pull",
  "requestId": "req_pull_1",
  "afterSeq": 0,
  "limit": 100
}
```

响应：

```json
{
  "type": "event.window",
  "requestId": "req_pull_1",
  "reason": "pull",
  "window": {
    "clientRouteId": "d_pad",
    "items": [],
    "nextAfterSeq": null,
    "hasMore": false,
    "ackedThroughSeq": 0,
    "trimmedThroughSeq": 0
  }
}
```

规则：

- `limit` 语义与 HTTP 完全一致，必须大于 0
- 若 `afterSeq` 为空，服务端可按当前连接的已发送游标继续拉取
- `events.pull` 允许与 `push` 并存，属于显式拉取而非隐式 ack

### 5.3 `events.ack`

请求：

```json
{
  "type": "events.ack",
  "requestId": "req_ack_1",
  "ackedSeq": 12
}
```

响应：

```json
{
  "type": "events.acked",
  "requestId": "req_ack_1",
  "ack": {
    "tenantId": "100001",
    "principalId": "1",
    "clientRouteId": "d_pad",
    "ackedThroughSeq": 12,
    "trimmedThroughSeq": 12,
    "retainedEventCount": 0,
    "ackedAt": "2026-04-05T10:10:00Z"
  }
}
```

规则：

- ack 单调递增
- ack 仍然会被钳制到该设备已分配的最大 `realtimeSeq`
- ack 仍然只是在线窗口消费确认，不表示 durable 业务确认

## 6. 服务端主动下行帧

### 6.1 `event.window`

服务端在以下时机主动发送：

1. 连接建立后的 `catchup`
2. 客户端主动 `pull`
3. 设备窗口收到新的实时事件时的 `push`

示例：

```json
{
  "type": "event.window",
  "requestId": null,
  "reason": "push",
  "window": {
    "clientRouteId": "d_other",
    "items": [
      {
        "tenantId": "100001",
        "principalId": "u_other_demo",
        "clientRouteId": "d_other",
        "realtimeSeq": 1,
        "scopeType": "conversation",
        "scopeId": "c_demo",
        "eventType": "message.posted",
        "deliveryClass": "ephemeral",
        "payload": "{\"conversationId\":\"c_demo\",\"messageId\":\"msg_c_demo_1\"}",
        "occurredAt": "2026-04-05T10:10:00Z"
      }
    ],
    "nextAfterSeq": 1,
    "hasMore": false,
    "ackedThroughSeq": 0,
    "trimmedThroughSeq": 0
  }
}
```

### 6.2 `error`

当客户端发送非法帧、未知类型、`limit=0` 等错误时，服务端返回：

```json
{
  "type": "error",
  "requestId": "req_x",
  "code": "limit_invalid",
  "message": "limit must be greater than 0"
}
```

错误帧默认不立即断开连接，除非底层连接已不可恢复。

## 7. 连接游标与窗口关系

服务端连接侧维护一个 `lastSentSeq`，它只是这条连接已下发到哪，不是业务确认点。

边界如下：

- `lastSentSeq` 仅用于避免同一连接重复下发
- 真正的消费确认仍然是 `ackedThroughSeq`
- 如果其他通道推进了相同设备的 ack，这条连接的后续下发必须以新的 ack 为下界

## 8. 与 durable 补偿的关系

WebSocket 仍然只负责在线期低延迟下行。  
以下场景必须走 durable 恢复链路：

- 客户端离线期间错过事件
- 窗口事件已经被 trim
- 节点重启后内存窗口不再可用
- 需要完整历史而不是在线增量

恢复顺序保持不变：

1. `session.resume`
2. `client-route event window`
3. 必要时补拉 timeline / projection / stream frame

## 9. 本地最小实现映射

当前代码落地边界：

- `session-gateway` 暴露 `/im/v3/api/realtime/ws`
- `sdkwork-im-server` 暴露同一路径，并复用同一 `RealtimeDeliveryRuntime`
- WebSocket `ack` 与 HTTP `GET /im/v3/api/realtime/events` 可交叉验证同一窗口状态
- 业务写路径继续走 `publish_scope_event`，WebSocket 通过设备 notifier 监听新增窗口事件

已验证能力：

- 连接建立后返回 `realtime.connected`
- WebSocket `subscriptions.sync`
- WebSocket `events.pull`
- WebSocket `events.ack`
- `message.posted` 可通过业务写路径实时推送到 WebSocket 客户端
- WebSocket ack 后，HTTP 查询同一设备窗口可见 `ackedThroughSeq` / `trimmedThroughSeq` 已同步推进

## 10. WebSocket 子协议与弃用窗口

### 10.1 标准子协议

- 客户端必须协商 `sdkwork-im.ccp.ws.v1`（实现常量 `LINK_WEBSOCKET_SUBPROTOCOL`）
- 业务与控制帧使用 CCP JSON envelope（`RealtimeWebsocketMode::CcpJson`）

### 10.2 `legacy.json` 弃用

- 未协商标准子协议或显式选择 `legacy.json` 时，服务端仍接受 plain JSON 帧（`LegacyJson`）以兼容旧客户端
- 服务端记录结构化弃用警告（`im.realtime.websocket.legacy_json_deprecated`）
- **移除目标**：2026-Q3 默认构建将拒绝 `legacy.json`；在此之前 PC SDK 与 CLI 必须完成 CCP 迁移

## 11. 后续演进

下一阶段建议沿这条线继续推进：

- 把 WebSocket 鉴权从开发 header 绑定升级到浏览器可用票据方案
- 持久化设备 ack checkpoint，支撑跨节点恢复
- 增加跨节点 fanout bridge，让任意节点都能把事件推到设备连接所在节点
- 引入窗口容量上限、背压和慢消费者策略
- 在控制面暴露实时连接观测、设备连接数、推送延迟、丢弃原因等指标
