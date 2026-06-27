# Step 05 / CP05-4 message-posted projection recipient owner 架构兑现 - 2026-04-07

## 1. 对应架构能力

- `09 / Step 05 / CP05-4` projection / notification 剩余连接点
- `130` message notification recipient authority 不应长期停留在 edge 侧透传
- `136` message-posted side-effect 应先进入 notification owner，再由 owner 命中 projection recipient seam
- `139` recipient authority 不应由 consumer 侧线程化 `recipient_ids`
- `147` 该 seam 应映射到 `notification-service::NotificationRuntime` 与其共享的 `projection-service` 接口 owner

## 2. 本轮架构兑现

- `notification-service::NotificationRuntime` 现在开始直接消费 `projection-service::access.active_conversation_principal_ids_from_auth_context(...)`。
- `sdkwork-im-server/effects.rs` 的 message side-effect 不再决定 recipient authority，只负责把已认证上下文和 message metadata 交给 notification owner。
- `sdkwork-im-server/build.rs` 改为把共享 `projection_service` 注入 `NotificationRuntime`，避免 owner seam 因运行时装配分叉而失真。

## 3. 当前决议

- 认定本轮为 `CP05-4` 的有效架构兑现增量。
- 暂不判定 `CP05-4`、`Step 05`、`91 / 95 / 97`、`Wave B / 93` 完成。

## 4. 回写对象

- `docs/架构/09-实施计划.md`
- `docs/架构/130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md`
- `docs/架构/136-关键业务链路与跨Plane时序设计-2026-04-06.md`
- `docs/架构/139-权限能力模型与协议演进设计-2026-04-06.md`
- `docs/架构/147-CCP到Crate与接口模块落地映射设计-2026-04-06.md`
