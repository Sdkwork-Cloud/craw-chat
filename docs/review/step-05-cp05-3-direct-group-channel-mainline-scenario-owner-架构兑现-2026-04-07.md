# Step 05 CP05-3 direct/group/channel mainline scenario owner 架构兑现

## 1. 对应架构能力

- `docs/架构/09-实施计划.md`
  - `Wave B / Step 05 / CP05-3`
- `docs/架构/130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md`
  - direct / group / channel 主链路场景 owner 应由领域模型承载，而不是 runtime 粘连字符串分支
- `docs/架构/136-关键业务链路与跨Plane时序设计-2026-04-06.md`
  - 主链路时序中的场景判定应在 owner boundary 内完成，再由 runtime 复用
- `docs/架构/139-权限能力模型与协议演进设计-2026-04-06.md`
  - direct / group / system channel 权限能力模型需要共享统一的场景 owner
- `docs/架构/147-CCP到Crate与接口模块落地映射设计-2026-04-06.md`
  - `im-domain-core` 需要承载会话场景语义，`conversation-runtime` 消费而不是重建 owner

## 2. 已兑现

- `im-domain-core` 现在拥有 `ConversationScenario`
- `ConversationAggregateState` 暴露 `scenario()` 作为 aggregate-level owner 语义
- `conversation-runtime` policy 已统一消费 aggregate scenario 进行 direct / group / channel 主链路能力分流
- 基于当前证据，`CP05-3` 可以回写为通过

## 3. 未兑现

- `CP05-4` 尚未完成
- `Step 05` 仍未整体闭环
- `Wave B / 93` 仍不能启动

## 4. 偏离判断

- 无新的架构偏离
- 本轮没有创建第二套 direct / group / channel 主链路场景 owner
- 本轮严格属于 `Step 05 / CP05-3`，没有跨 step 乱序推进

## 5. 回写决议

- 需要回写：
  - `docs/架构/09-实施计划.md`
  - `docs/架构/130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md`
  - `docs/架构/136-关键业务链路与跨Plane时序设计-2026-04-06.md`
  - `docs/架构/139-权限能力模型与协议演进设计-2026-04-06.md`
  - `docs/架构/147-CCP到Crate与接口模块落地映射设计-2026-04-06.md`
- 不允许回写：
  - `Step 05` 已完成
  - `91 / 95 / 97` 已整体通过
  - `Wave B / 93` 已触发
