# Step 05 / CP05-4 message-posted notification request owner 架构兑现 - 2026-04-07

## 1. 对应架构能力

- `docs/架构/09-实施计划.md`
  - `Wave B / Step 05 / CP05-4`
- `docs/架构/130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md`
  - message-posted notification 默认字段不应长期停留在 service edge 手工装配
- `docs/架构/136-关键业务链路与跨Plane时序设计-2026-04-06.md`
  - message posted side-effect 应先进入 notification owner，再进入通用 fanout 流程
- `docs/架构/139-权限能力模型与协议演进设计-2026-04-06.md`
  - `notification_id_seed / source_event_type / category / payload` 等默认字段边界应由单一 owner 维护
- `docs/架构/147-CCP到Crate与接口模块落地映射设计-2026-04-06.md`
  - 该 seam 应映射到 `notification-service` runtime，而不是 `sdkwork-im-server` effects

## 2. 本轮架构兑现

- `notification-service` 新增 `RequestMessagePostedNotifications` 与 `request_message_posted_notifications(...)`。
- `sdkwork-im-server` effects 不再直接构造 `RequestNotificationFanout` 的 message-posted 默认字段。
- owner / consumer 边界变为：
  - owner：`services/notification-service/src/lib.rs`
  - consumer：`services/sdkwork-im-cloud-gateway/src/node/effects.rs`

## 3. 当前决议

- 认定本轮为 `CP05-4` 的有效架构兑现增量。
- 不认定 `CP05-4`、`Step 05`、`91 / 95 / 97`、`Wave B / 93` 已完成。

## 4. 回写对象

- `docs/架构/09-实施计划.md`
- `docs/架构/130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md`
- `docs/架构/136-关键业务链路与跨Plane时序设计-2026-04-06.md`
- `docs/架构/139-权限能力模型与协议演进设计-2026-04-06.md`
- `docs/架构/147-CCP到Crate与接口模块落地映射设计-2026-04-06.md`