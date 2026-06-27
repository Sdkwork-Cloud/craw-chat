> Migrated from `docs/架构/37-流会话打开幂等与冲突标准-2026-04-05.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 流会话打开幂等与冲突标准 - 2026-04-05

## 1. 背景

本轮 review 在 `streaming-service` 中发现重复打开流会话时的高风险一致性缺陷：

- `open_stream` 对同一 `tenant_id + stream_id` 直接覆盖已有 `StreamSession`。
- 同时会重置对应的 frame 存储，导致已写入的流帧历史被清空。
- `sdkwork-im-server` 对 `StreamingError` 做了统一降级映射，导致底层 `409 stream_conflict` 被错误包装成通用 `400`。

该问题会导致：

- 流式消息历史丢失。
- 上层 SDK 无法依赖 `stream_id` 做安全重试。
- 网关层返回错误码失真，破坏调用方的冲突处理语义。

## 2. 标准

### 2.1 幂等键

流会话打开请求的幂等键定义为：

- `tenant_id + stream_id`

### 2.2 同键同定义

当以下字段完全一致时，重复打开必须视为幂等重放：

- `streamId`
- `streamType`
- `scopeKind`
- `scopeId`
- `durabilityClass`
- `schemaRef`

处理要求：

- HTTP 返回 `200`
- 返回当前已存在的 `StreamSession`
- 不重置 `lastFrameSeq` / `lastCheckpointSeq` / `state`
- 不清空已存在的 frames

### 2.3 同键不同定义

当 `tenant_id + stream_id` 相同，但流定义与已存在会话不一致时，必须返回冲突。

处理要求：

- HTTP 返回 `409`
- 返回错误码 `stream_conflict`
- 保留首次创建成功的会话和帧历史

### 2.4 错误透传

聚合节点或网关层不得把 streaming 领域错误压平成通用错误。

必须透传：

- 原始 HTTP status
- 原始 `code`
- 原始 `message`

尤其是以下冲突类错误：

- `stream_conflict`
- `stream_frame_conflict`
- `stream_state_invalid`

## 3. 实现约束

### 3.1 运行时行为

`StreamingRuntime::open_stream(...)` 必须遵循以下顺序：

1. 解析并校验 `durabilityClass`
2. 基于 `tenant_id + stream_id` 查找已有会话
3. 若存在且定义一致，则直接返回已有会话
4. 若存在且定义不一致，则返回 `409 stream_conflict`
5. 若不存在，则创建会话并初始化 frame 存储

禁止在命中已有 `stream_id` 后重新初始化 frame 存储。

### 3.2 数据保护

幂等重放时，以下数据必须保持不变：

- 已写入的 `StreamFrame` 列表
- 会话当前状态 `state`
- `last_frame_seq`
- `last_checkpoint_seq`
- `result_message_id`

## 4. 验证清单

必须覆盖以下自动化测试：

1. streaming-service：重复打开同一流并已追加 frame 后，返回当前会话状态且 frames 不丢失。
2. streaming-service：同 `streamId` 不同定义重复打开时返回 `409 stream_conflict`。
3. sdkwork-im-server：通过聚合节点重复打开流时保持幂等，且冲突错误码不被压平。

## 5. 参考实现

- `services/streaming-service/src/lib.rs`
- `services/streaming-service/tests/stream_lifecycle_test.rs`
- `services/sdkwork-im-cloud-gateway/src/lib.rs`
- `services/sdkwork-im-cloud-gateway/tests/http_e2e_test.rs`

## 6. 后续 review 建议

流会话打开幂等补齐后，下一轮优先检查：

- `im-call-runtime` 是否存在 `rtc_session_id` 重复创建覆盖问题。
- `im-call-runtime` 的 invite / accept / reject / end 是否具备明确的状态机冲突语义。
- 控制层是否还有其他把领域错误压平成通用 `400` 的路径。

