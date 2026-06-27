> Migrated from `docs/架构/12-RTC-自定义信令设计.md` on 2026-06-24.
> Owner: SDKWork maintainers

# RTC 自定义信令设计

## 1. 目标

在现有 RTC 会话能力之上，补齐一条标准化的“自定义信令”入口，用于承载 WebRTC SDP/ICE 交换、语音房控制事件、机器人协同事件，以及其他需要通过 IM 会话分发的实时信令数据。

这条能力的设计目标如下：

- 不要求调用方在请求体重复传 `tenantId` 或 `senderId`，统一从授权上下文解析租户和发送者身份。
- 复用统一 `sender` 结构，保证后续可扩展为成员、设备、代理、机器人或系统身份。
- 当 RTC 会话绑定了 `conversationId` 时，自定义信令自动投影为 IM 时间线中的 `signal` 消息，打通多端同步、会话摘要、通知和审计链路。
- 当 RTC 会话未绑定会话时，仍允许作为纯 RTC 信令能力独立使用，不强制落入 IM 消息流。
- 保持协议可插拔，业务方只需定义 `signalType`、`schemaRef` 和 `payload`，不被具体 WebRTC 或游戏协议锁死。

## 2. 标准定位

RTC 相关能力拆成两层：

- 会话状态层：`create / invite / accept / reject / end`
- 自定义信令层：`POST /im/v3/api/calls/sessions/{rtc_session_id}/signals`

会话状态层负责管理 RTC 会话生命周期，自定义信令层负责承载生命周期之外的实时数据交换。两层共享统一的授权、租户隔离、会话标识和可观测性约束，但互相解耦。

这意味着：

- `rtc.accept`、`rtc.reject`、`rtc.end` 这类平台定义事件，仍由平台内置投影生成。
- `rtc.offer`、`rtc.answer`、`rtc.ice-candidate`、`room.mute`、`agent.handoff` 等业务自定义事件，统一走自定义信令入口。

## 3. 协议契约

### 3.1 请求

`POST /im/v3/api/calls/sessions/{rtc_session_id}/signals`

请求体：

```json
{
  "signalType": "rtc.offer",
  "schemaRef": "webrtc.offer.v1",
  "payload": "{\"sdp\":\"demo\"}",
  "signalingStreamId": "stream_rtc_offer_001"
}
```

字段约束：

- `signalType`：必填，表示业务信令类型，是路由和渲染的一级分类键。
- `schemaRef`：可选，表示信令载荷的结构版本，例如 `webrtc.offer.v1`。
- `payload`：必填，当前标准使用字符串承载，允许直接放 JSON 字符串，也允许放文本或编码后的二进制描述。
- `signalingStreamId`：可选，用于把 RTC 会话与更长生命周期的数据流或流式交换过程绑定。

### 3.2 响应

服务端返回标准化的 `RtcSignalEvent`：

```json
{
  "tenantId": "100001",
  "rtcSessionId": "rtc_demo_001",
  "conversationId": "c_demo",
  "rtcMode": "voice",
  "signalType": "rtc.offer",
  "schemaRef": "webrtc.offer.v1",
  "payload": "{\"sdp\":\"demo\"}",
  "sender": {
    "id": "u_alice",
    "kind": "user",
    "memberId": null,
    "clientRouteId": "d_mac_001",
    "sessionId": "s_demo_001",
    "metadata": {}
  },
  "signalingStreamId": "stream_rtc_offer_001",
  "occurredAt": "2026-04-05T10:03:00Z"
}
```

返回值的关键要求：

- `tenantId` 必须由鉴权上下文注入，不接受客户端伪造。
- `sender` 必须使用统一结构，而不是裸 `senderId`。
- `conversationId` 允许为空，表示这条信令只在 RTC 域内生效，不自动进入 IM 时间线。

## 4. 领域模型

领域层新增了 `RtcSignalEvent`，字段如下：

- `tenant_id`
- `rtc_session_id`
- `conversation_id`
- `rtc_mode`
- `signal_type`
- `schema_ref`
- `payload`
- `sender`
- `signaling_stream_id`
- `occurred_at`

该模型的职责边界很明确：

- `RtcSession` 表示 RTC 会话聚合状态。
- `RtcSignalEvent` 表示一次业务信令事件。

这样设计的原因是，后续若要把自定义信令落到 Kafka / NATS / Redpanda / 自研事件总线，都可以直接以 `RtcSignalEvent` 作为标准事件，不需要再从会话快照反推一次业务含义。

## 5. 运行时处理流程

当前实现的处理顺序如下：

1. Gateway/HTTP 层从请求头解析授权上下文，得到 `tenantId`、`actorId`、`actorKind`、`sessionId`、`clientRouteId`。
2. `im-call-runtime` 校验 `rtc_session_id` 是否存在。
3. 若会话状态为 `Rejected` 或 `Ended`，拒绝写入自定义信令，返回 `rtc_session_closed`。
4. 若请求携带 `signalingStreamId`，将其同步挂到 RTC 会话上，形成后续流式信令关联键。
5. 运行时构造 `RtcSignalEvent`，写入本地信令存储。
6. 若会话绑定了 `conversationId`，`sdkwork-im-server` 立即把该信令投影成一条 IM `MessageType::Signal` 消息。
7. IM 消息复用统一消息后处理链，自动更新：
   - 会话摘要
   - inbox 计数
   - 多端同步 feed
   - 通知任务
   - 审计锚点

## 6. IM 投影标准

当存在 `conversationId` 时，系统会把 RTC 自定义信令包装为 IM `signal` 消息，约束如下：

- `messageType` 固定为 `signal`
- `summary` 使用 `signalType`
- `SignalPart.signalType` 使用 `signalType`
- `SignalPart.schemaRef` 优先使用请求传入的 `schemaRef`，否则回退到 `rtc.signal.v1`
- `renderHints.channel = "rtc"`

投影后的 `payload` 不是直接透传原始字符串，而是包装为统一 JSON：

```json
{
  "rtcSessionId": "rtc_demo_001",
  "conversationId": "c_demo",
  "rtcMode": "voice",
  "signalingStreamId": "stream_rtc_offer_001",
  "signalType": "rtc.offer",
  "signalPayload": {
    "sdp": "demo"
  }
}
```

这样处理有三个好处：

- 消费侧不需要额外查询 RTC 会话，就能拿到投影上下文。
- 原始 `payload` 若是 JSON，可以直接结构化消费；若不是 JSON，则退化成字符串也能保持兼容。
- 为后续把 `signalPayload` 进一步映射到 WebSocket、MQ、Webhook、机器人工作流提供稳定包络。

## 7. 安全与多租户约束

该能力沿用平台统一安全标准：

- 租户身份只来自鉴权上下文，不允许请求体传租户。
- 发送者身份只来自鉴权上下文，不允许请求体伪造发送者。
- `rtc_session_id` 的查找按 `tenantId + rtcSessionId` 组合隔离。
- 关闭态 RTC 会话禁止继续接收自定义信令，避免回放或脏写。
- 若会话未绑定 `conversationId`，不会产生额外的 IM 扩散面。

后续进入生产版时，还需要补齐以下控制点：

- 每租户 / 每会话的信令速率限制
- 每 `signalType` 的 payload 大小限制
- 可选的 schema 白名单校验
- 高风险信令的审计级别提升

## 8. 存储与可靠性标准

当前 `standalone.split-services.development` 实现使用内存存储，定位是本地可运行基线，不代表生产最终形态。

生产版标准建议如下：

- RTC 会话状态写入强一致主存储
- `RtcSignalEvent` 作为事件日志写入顺序事件流
- IM `signal` 消息作为投影结果进入消息存储

这样拆分后可以同时满足：

- RTC 实时处理低延迟
- IM 时间线长期可检索
- 失败重放时可以用事件流重新构建投影

## 9. 可插拔扩展标准

为了满足后续“先实现能力，再逐步替换底层组件”的路线，这条能力的扩展点已经固定为：

- 信令入口协议可替换：HTTP、WebSocket、gRPC、MQ 均可复用相同领域模型
- 信令存储可替换：内存、Redis Stream、Kafka、NATS JetStream、自研日志
- 投影器可替换：同步投影、异步投影、批投影
- 消费器可替换：IM 会话、多端同步、Webhook、机器人、工作流引擎

统一不变的是：

- 授权上下文驱动的租户与发送者解析
- `RtcSignalEvent` 领域契约
- `signal` 消息投影标准

## 10. 当前落地状态

本轮已落地：

- 领域模型 `RtcSignalEvent`
- RTC 服务入口 `POST /im/v3/api/calls/sessions/{rtc_session_id}/signals`
- 关闭态 RTC 会话拒绝自定义信令
- `sender` 统一结构接入 RTC 自定义信令
- 会话绑定 IM 时自动投影为 `signal` 消息
- `sdkwork-im-server` 端到端测试覆盖自定义信令路径

本轮验证基线：

- `cargo test -p im-domain-core --test model_contract_test --offline`
- `cargo test -p im-call-runtime --test rtc_signal_flow_test --offline`
- `cargo test -p sdkwork-im-cloud-gateway --test http_e2e_test --offline`

## 11. 下一步

这一能力完成后，下一阶段优先推进以下几个方向：

- 为自定义信令增加幂等键和去重策略
- 为 `signalingStreamId` 对接统一流式发送/中止标准
- 把 RTC 自定义信令纳入统一事件总线抽象
- 明确生产版的持久化和重放机制
- 补齐 WebSocket 推送与私有化部署场景下的节点间广播标准

