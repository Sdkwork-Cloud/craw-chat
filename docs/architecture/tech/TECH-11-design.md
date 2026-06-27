> Migrated from `docs/架构/11-流式中止能力设计.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 流式中止能力设计

## 目标

为标准流式消息传输补齐 `abort` 生命周期能力，支持客户端、网关或上层业务在流未完成时主动中止，并让本地最小部署版本具备一致行为。

## API

### `POST /im/v3/api/streams/{stream_id}/abort`

Request:

```json
{
  "frameSeq": 12,
  "reason": "client_cancelled"
}
```

Response:

```json
{
  "tenantId": "t_xxx",
  "streamId": "st_xxx",
  "streamType": "custom.delta.text",
  "scopeKind": "conversation",
  "scopeId": "c_xxx",
  "durabilityClass": "durableSession",
  "orderingScope": "stream",
  "schemaRef": "custom.delta.text.v1",
  "state": "aborted",
  "lastFrameSeq": 12,
  "lastCheckpointSeq": 12,
  "resultMessageId": null,
  "openedAt": "2026-04-05T10:00:00Z",
  "closedAt": "2026-04-05T10:04:00Z",
  "expiresAt": null
}
```

## 语义约束

- `abort` 只允许作用于未关闭流。
- 流一旦进入 `completed` 或 `aborted`，后续 `checkpoint` 和 `complete` 必须拒绝。
- `frameSeq` 为可选。
- 当请求携带 `frameSeq` 时，运行时会推进 `lastFrameSeq`，并同步更新 `lastCheckpointSeq`。
- `reason` 当前作为兼容字段保留，用于后续审计、指标和策略扩展；本期不投影到 `StreamSession` 视图。

## 适用场景

- 客户端主动取消大模型流式输出
- 用户中断上传中的业务数据流
- 网关检测上游取消或超时后执行有界关闭
- RTC 相关信令流在会话未完成时提前终止

## 本期实现范围

- `services/streaming-service`
- `services/sdkwork-im-cloud-gateway`

## 后续增强

- 将 `reason` 写入审计锚点和指标标签
- 为 `eventLog` 持久化策略补齐 `stream.aborted` 事件
- 在 WebSocket 下行链路接入 `stream.abort` 事件广播

