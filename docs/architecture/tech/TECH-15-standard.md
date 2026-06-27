> Migrated from `docs/架构/15-会话成员实时广播标准.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 会话成员实时广播标准

## 1. 目标

当前实时订阅内核已经支持：

- 设备级订阅同步
- 设备级实时事件窗口
- 同一主体多客户端路由的实时下行

但这还不等于 IM 的真实实时能力。商业 IM 的最小可用标准是：一个成员发送消息后，其他成员已订阅设备也能收到实时事件。

本标准的目标是补齐“会话成员实时广播”这一层。

## 2. 范围

本轮只做 `conversation` scope 的最小广播闭环：

- 支持 `message.posted`
- 广播对象是会话内当前活跃成员
- 广播载体是已存在的 `RealtimeDeliveryRuntime`

暂不扩展：

- `message.edited / message.recalled`
- 跨节点广播
- 在线连接路由
- 大规模房间分片

## 3. 标准语义

### 3.1 广播对象

当一条 `message.posted` 写入会话后，系统应读取当前会话的活跃成员列表，对每个成员的已注册设备进行实时广播评估。

### 3.2 广播过滤

是否真正投递，仍由该设备的订阅决定：

- 只有订阅了 `scopeType = conversation`
- 且 `scopeId = 当前 conversationId`
- 且 `eventTypes` 包含 `message.posted`

的设备，才进入实时窗口。

### 3.3 与 durable 补偿的边界

广播只负责在线期低延迟下行，不替代：

- 时间线查询
- `client-route event window`
- `session.resume`

如果某设备离线或未订阅，不会因为错过实时广播而丢 durable 数据。

## 4. 事件载荷标准

广播 `message.posted` 时，建议 payload 至少包含：

```json
{
  "conversationId": "c_demo",
  "messageId": "msg_c_demo_1",
  "messageSeq": 1,
  "messageType": "standard",
  "summary": "hello"
}
```

这样订阅端无需立即反查时间线，也能完成最小提示渲染。

## 5. 本地最小落地策略

在 `sdkwork-im-server` 中，广播实现基于已有能力：

- 会话成员来源：`conversation_runtime.list_members`
- 设备来源：`projection_service.registered_devices`
- 实时窗口写入：`realtime_runtime.publish_scope_event`

这条路径的价值在于：

- 不新增外部依赖
- 不破坏现有 `client-route event window`
- 可直接演进到跨节点 fanout

## 6. 后续扩展

下一步在此基础上继续推进：

- `message.edited / recalled` 实时广播
- `stream.frame.appended` 的会话关联 fanout
- WebSocket 在线连接态绑定
- 节点间广播与分片路由

