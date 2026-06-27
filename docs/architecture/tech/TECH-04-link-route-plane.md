> Migrated from `docs/step/04-link-plane与route-plane运行时重构.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 04 - Link Plane 与 Route Plane 运行时重构

## 1. 目标与范围

本 step 用于把当前分散在 `session-gateway`、`sdkwork-im-server` 等服务中的连接热路径和在线路由能力，真正抽离为独立的运行时层。

本 step 的目标是建立：

- `runtime-link`
- `runtime-route`
- 统一的连接分片、出站队列、恢复窗口、背压治理
- 统一的 route ownership、epoch、fencing、drain、rebalance

### 1.1 执行输入

- step 03 已冻结的 `CCP` 协议骨架
- 当前 `session-gateway`、`sdkwork-im-server` 的连接与 cluster 测试资产
- 当前 reconnect / resume / route 相关运行态实现
- 连接密度和分层扩容的目标架构文档

### 1.2 本步非目标

- 不在本 step 内重构消息领域和投影领域的全部逻辑
- 不在本 step 内接入完整 AI / IoT 扩展能力
- 不在本 step 内完成控制面全量治理功能

### 1.3 最小输出

- `runtime-link` 骨架与连接热路径抽离
- `runtime-route` 骨架与 route epoch / drain 模型
- 连接与路由的关键 smoke / E2E 测试
- `session-gateway` 与 `sdkwork-im-server` 的边界收敛起点

## 2. 架构对齐

本 step 重点对齐：

- `docs/架构/130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md`
- `docs/架构/131-连接管理与分层弹性扩容架构设计-2026-04-06.md`
- `docs/架构/136-关键业务链路与跨Plane时序设计-2026-04-06.md`
- `docs/架构/149-多Cell多Region协议升级与灾备兼容设计-2026-04-06.md`

## 3. 当前现状与问题

当前仓库已经具备 WebSocket 和实时事件路径，但连接层和路由层的职责尚未完全独立：

- 连接状态机、实时投递、业务判断可能混在同一服务文件中
- 路由归属与节点治理还未形成独立 plane
- `session-gateway` 仍承担过多运行态职责
- `sdkwork-im-server` 既做装配，又承载大量细节逻辑

如果不先抽出 Link / Route Plane，后续扩容、排空、恢复、兼容升级都会很难做稳。

## 4. 设计

### 4.1 Link Runtime

`runtime-link` 负责：

- accept / upgrade
- `hello / auth_bind / session_resume`
- connection registry
- shard dispatcher
- outbound queue
- heartbeat manager
- resume window manager
- backpressure controller
- link metrics

### 4.2 Route Runtime

`runtime-route` 负责：

- route ownership
- route epoch
- session fencing
- node drain / rebalance / takeover
- route lookup
- reconnect 与 route 恢复衔接

### 4.3 分层职责

- `interface-ws`：建立连接并把连接接入 `runtime-link`
- `runtime-link`：管理连接生命周期
- `runtime-route`：管理在线归属与迁移
- `app-*`：处理业务命令，不直接感知底层 transport
- `services/*`：只装配以上模块

### 4.4 关键设计决策

- 单连接只归属于一个 shard
- 每连接独立有界出站队列
- 慢消费者优先做低优先级降级或断开
- 所有 route 写入带 `epoch`
- 所有迁移都必须走 drain 状态机

## 5. 实施落地规划

### 5.1 任务拆解

1. 从 `session-gateway` 中抽出连接运行态模块
2. 从现有 cluster / route 逻辑中抽出 route ownership 模块
3. 建立 shard、queue、resume、metrics 子模块
4. 建立 route epoch、drain、rebalance 模块
5. 让 `interface-ws` 改为调用 `runtime-link`
6. 让 `sdkwork-im-server` 只负责装配和 profile 组合

### 5.2 重点路径

重点涉及：

- `services/session-gateway/src/lib.rs`
- `services/session-gateway/src/websocket.rs`
- `services/session-gateway/src/realtime.rs`
- `services/session-gateway/src/cluster.rs`
- `services/sdkwork-im-cloud-gateway/src/lib.rs`
- `services/control-plane-api/`

### 5.3 运行时拆分建议

建议形成如下模块：

- `runtime-link/acceptor/`
- `runtime-link/session/`
- `runtime-link/shard/`
- `runtime-link/queue/`
- `runtime-link/recovery/`
- `runtime-link/metrics/`
- `runtime-route/ownership/`
- `runtime-route/fencing/`
- `runtime-route/drain/`
- `runtime-route/rebalance/`

### 5.4 与控制面的衔接

本 step 不要求一次性做完控制面，但必须为 step 07 预留接口：

- 节点状态查询
- drain 指令入口
- route 迁移状态观测
- capability 与 rollout 的运行态读取

## 6. 测试计划

建议重点测试：

- WebSocket 握手 smoke 测试
- reconnect / resume 测试
- route epoch fencing 测试
- cluster routing 测试
- drain / rebalance E2E 测试
- checkpoint 和 disconnect fence 测试

建议重点执行当前已有测试并继续扩展：

- `services/session-gateway/tests/websocket_smoke_test.rs`
- `services/session-gateway/tests/cluster_routing_test.rs`
- `services/sdkwork-im-cloud-gateway/tests/cluster_realtime_routing_e2e_test.rs`
- `services/sdkwork-im-cloud-gateway/tests/cluster_drain_rebalance_e2e_test.rs`
- `services/sdkwork-im-cloud-gateway/tests/disconnect_fence_persistence_test.rs`

## 7. 结果验证

本 step 完成后，需要验证：

- 连接生命周期与业务命令已完成解耦
- route 迁移与节点排空具备稳定语义
- `session-gateway` 和 `sdkwork-im-server` 不再是运行态逻辑黑洞
- Link Plane 与 Route Plane 可以独立扩容和独立观测

## 8. 检查点

- `CP04-1`：`runtime-link` 与 `runtime-route` 的骨架和职责已明确
- `CP04-2`：连接热路径已从 `session-gateway` 中抽离
- `CP04-3`：route ownership / epoch / drain 模型已落地并有测试覆盖
- `CP04-4`：`sdkwork-im-server` 已明显朝装配层收敛

### 8.1 推荐 review 产物

- `docs/review/step-04-执行卡-YYYY-MM-DD.md`
- `docs/review/step-04-link-route-runtime重构-YYYY-MM-DD.md`
- `docs/review/step-04-会话路由状态模型-YYYY-MM-DD.md`
- `docs/review/step-04-drain-rebalance演练记录-YYYY-MM-DD.md`

### 8.2 推荐并行车道

- `04-A`：Link Plane 握手、resume、backpressure、连接热路径
- `04-B`：Route Plane ownership、epoch、drain、rebalance
- `04-C`：gateway 集成、观测埋点、与主链路的 hook 收敛
- 收口要求：共享 session / route 状态模型由 `04-Owner` 统一冻结，连接层与路由层可以并行实现，但不得分别定义冲突语义。
- 车道编排参考：[`94-Step并行执行编排与车道拆分建议`](./94-Step并行执行编排与车道拆分建议.md)

### 8.3 架构能力闭环判定

- 必须验证连接热路径已经与业务逻辑分离，且 route epoch / drain / rebalance 可重复验证。
- 如果只是把代码从单文件拆到多文件，但 reconnect/resume 仍不稳定，或 route 语义仍漂移，本 step 不算闭环。
- 闭环验收以 [`95-架构能力闭环验收标准`](./95-架构能力闭环验收标准.md) 中 Step 04 条目为准。

### 8.4 快速并行执行建议

- 先冻结 session / route 状态模型，再并行拆 Link Plane、Route Plane、gateway 适配。
- 推荐每天至少做一次 reconnect、resume、drain 的烟雾回归，避免多车道把状态机改散。
- 本步只解决连接与路由运行时，不混入消息领域重构，保证车道边界干净。

### 8.5 完成后必须回写的架构文档

- 强制范围：本文件 `## 2. 架构对齐` 中列出的全部架构文档。
- 回写重点：Link Plane、Route Plane、route epoch、drain/rebalance、跨 cell/region 兼容假设是否已被当前运行时实现验证。
- 必备证据：`docs/review/step-04-架构兑现-YYYY-MM-DD.md` 与 `docs/review/step-04-架构回写决议-YYYY-MM-DD.md`。

## 9. 风险与回滚

### 9.1 风险

- 连接层和业务层纠缠较深，拆分时容易破坏现有路径
- route epoch 与 reconnect 处理不当，会引入陈旧连接投递问题
- 如果只拆代码、不补指标，会让排障更难

### 9.2 回滚

- 可以保留原 service 内 facade，逐步切换到新 runtime
- 先让 runtime 对现有逻辑做包裹，再逐步内聚实现
- 关键状态机变更必须在切换前保留旧路径 smoke 测试

## 10. 完成定义

满足以下条件视为完成：

- Link Plane 与 Route Plane 已有清晰的代码边界
- 核心连接与路由测试继续通过
- 高风险服务文件显著变薄
- 后续消息主链路可直接依赖新的运行时层

## 11. 下一步准入条件

进入 step 05 前必须确认：

- 连接与路由运行态已经足够稳定，不会再反向把业务逻辑拖回 runtime

