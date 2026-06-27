> Migrated from `docs/架构/38-RTC会话状态机与幂等更新标准-2026-04-05.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 38-RTC会话状态机与幂等更新标准

## 1. 目标

本标准用于收敛 `im-call-runtime` 与 `sdkwork-im-server` 中 RTC 会话写路径的状态机、幂等语义、冲突语义和副作用执行规则，避免以下商业化风险：

- 终态会话被后续请求覆盖，导致通话记录失真
- 相同请求重试重复写入 IM `signal` 消息，造成多端重复渲染
- 不同请求复用同一 `rtcSessionId` 时静默改写会话状态

## 2. 状态定义

`RtcSessionState` 当前只允许以下状态：

- `started`
- `accepted`
- `rejected`
- `ended`

其中：

- `rejected` 是拒绝型终态
- `ended` 是结束型终态

## 3. 状态流转规则

### 3.1 允许的流转

- `started -> accepted`
- `started -> rejected`
- `started -> ended`
- `accepted -> ended`

### 3.2 不允许的流转

以下流转必须显式拒绝，返回冲突错误，不允许静默覆盖：

- `accepted -> rejected`
- `accepted -> accepted` 且请求内容不同
- `rejected -> accepted`
- `rejected -> ended`
- `rejected -> rejected` 且请求内容不同
- `ended -> accepted`
- `ended -> rejected`
- `ended -> ended` 且请求内容不同

## 4. 幂等规则

### 4.1 创建会话

同一 `tenantId + rtcSessionId` 下：

- 创建请求内容完全一致：返回现有会话，`200`
- 创建请求内容不同：返回 `409 rtc_session_conflict`

### 4.2 更新会话

当前更新接口包括：

- `invite`
- `accept`
- `reject`
- `end`

幂等定义如下：

- 同一状态更新接口，对同一会话重复提交完全相同请求：返回现有会话，`200`
- 相同接口但请求内容不同，且当前状态已不允许再次应用：返回 `409 rtc_session_state_conflict`
- 不同接口尝试覆盖当前已落定状态：返回 `409 rtc_session_state_conflict`

当前请求等价性定义：

- `invite`：`signalingStreamId` 相同
- `accept` / `reject` / `end`：`artifactMessageId` 相同

## 5. 错误码标准

### 5.1 创建冲突

- HTTP: `409`
- code: `rtc_session_conflict`

适用场景：

- 相同 `rtcSessionId` 的创建请求内容不同

### 5.2 状态机冲突

- HTTP: `409`
- code: `rtc_session_state_conflict`

适用场景：

- 非法状态流转
- 已落定更新的重复提交但请求内容不同
- 关闭态会话继续执行 `invite`

### 5.3 关闭态信令写入

- HTTP: `400`
- code: `rtc_session_closed`

适用场景：

- `rejected` 或 `ended` 会话继续写入自定义 RTC signal

## 6. 副作用执行规则

`sdkwork-im-server` 在以下接口成功后会向 IM 时间线投影 `signal` 消息：

- `invite -> rtc.invite`
- `accept -> rtc.accept`
- `reject -> rtc.reject`
- `end -> rtc.end`

为了避免重复投影，必须遵循：

- 仅当 RTC runtime 本次请求真实改变会话状态或字段时，才允许发出副作用
- 对幂等重试返回的现有会话，禁止再次写入 IM `signal` 消息
- 对冲突请求，禁止写入任何 IM `signal` 消息

## 7. 当前实现约束

本标准已要求运行时返回“变更结果”语义：

- `applied = true`：本次请求首次生效，可触发副作用
- `applied = false`：本次请求属于幂等重试，不可触发副作用

## 8. 回归测试要求

至少覆盖以下场景：

- `accept` 重试同请求返回 `200`
- `accept` 后 `reject` 返回 `409 rtc_session_state_conflict`
- `accept` 后相同 `accept` 不重复产生 IM `rtc.accept`
- `end` 重试同请求返回 `200`
- `end` 后再次 `accept` 返回 `409 rtc_session_state_conflict`

## 9. 后续收敛方向

下一轮 review 优先继续检查：

- `invite` 在 `accepted` 态下的精细幂等语义
- RTC 状态消息投影后的通知与审计副作用是否也需要 `applied` 抑制
- 分布式部署后同一 `rtcSessionId` 的跨节点并发写一致性

