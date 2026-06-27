> Migrated from `docs/架构/131-连接管理与分层弹性扩容架构设计-2026-04-06.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 连接管理与分层弹性扩容架构设计

## 1. 文档目标

本文档用于详细定义`sdkwork-im` 的连接管理层、在线路由层与分层扩容模型，解决以下关键问题：

- 如何支撑高连接密度
- 如何将连接治理与业务逻辑解耦
- 如何实现按层扩容，而不是全系统一起扩容
- 如何支持多客户端路由、AI 流式输出、AIoT 长连
- 如何在后续逐步演进到更高单机能力和更高集群规模

本设计是 `130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md` 的专项细化文档

## 2. 总体目标

连接层的核心目标分为两级：

### 2.1 第一落地目标

- 单机 `10 万级` 长连接能力
- 节点排空与重平衡可控
- 设备 resume / reconnect 正确
- 慢连接不拖垮整体节点
- Link Plane 可独立扩容

### 2.2 设计上限目标

- 架构上预留`50 万级以上` 单机长连接的演进空间
- 可替换更专用的连接运行时实现
- 可演进到更底层协议栈与更高性能网络模型

## 3. 分层模型

系统的扩容不是按“服务个数”定义，而是按压力面定义

### 3.1 Link Plane

职责：

- WebSocket / HTTP / SSE / MQTT / QUIC 接入
- 握手
- 鉴权
- 心跳
- 背压
- 本机连接分片
- 本机发送队列

扩容触发条件：

- 在线连接数升
- 握手 TPS 升高
- 出站带宽升高
- 心跳与保活压力升

### 3.2 Route Plane

职责：

- device -> session -> node 归属
- route lookup
- fencing
- drain
- rebalance
- failover

扩容触发条件：

- 在线设备数升
- 会话迁移频率升高
- 路由查询与写入压力升

### 3.3 Messaging Plane

职责：

- 会话
- 成员
- 消息提交
- 已读推进
- 广播fanout 编排

扩容触发条件：

- 消息写入 TPS 升高
- 群聊 fanout 压力升高
- 多租户热点会话升

### 3.4 Stream / AI Plane

职责：

- stream.open / append / checkpoint / finalize
- LLM token streaming
- 转写入
- telemetry 
- 长任务渐进输出

扩容触发条件：

- LLM 对话并发升高
- 流帧吞吐升高
- 长任务渐进输出增加

### 3.5 Projection Plane

职责：

- inbox
- timeline
- unread
- summary
- topic/thread
- 搜索与知识投影

扩容触发条件：

- 查询流量升高
- 读写比失败
- AI/搜索投影任务升高

### 3.6 Storage Plane

职责：

- MetadataStore
- RouteStore
- MessageLog
- StreamStore
- BlobStore

扩容触发条件：

- 元数据事务压力升
- 热路由状态膨胀
- 消息日志写入膨胀
- 大对象存储容量与吞吐升高

## 4. Link Runtime 设计

### 4.1 组件清单

建议将连接运行时抽象为独立模块：

- `acceptor`
- `protocol_upgrade`
- `auth_pipeline`
- `connection_registry`
- `shard_dispatcher`
- `outbound_queue`
- `resume_window_manager`
- `heartbeat_manager`
- `backpressure_controller`
- `connection_metrics`

### 4.2 连接分片

本机连接必须shard 管理，避免全局锁

分片建议：

- 仅`device_id`
- 或按 `principal_id + device_id`
- 或按连接内部生成熟`conn_hash`

约束：

- 单连接只归属于一shard
- 连接生命周期只在归属 shard 内修改
- shard 间仅传递轻量事件，不共享大状态对象

### 4.3 出站发送队列

每个连接必须具备独立的有界出站队列

优先级建议：

- P0: 控制帧、ACK、disconnect、resume、route update
- P1: RTC signal、presence、session lifecycle
- P2: 普通消息
- P3: 流式 delta、大对象引用提示

治理策略：

- 队列满时触发背压
- 慢消费者优先降级低优先级事件
- 极端情况下主动关闭异常连接

### 4.4 背压模型

必须同时支持：

- per-connection 背压
- per-shard 背压
- per-node 背压
- per-tenant 背压

背压动作：

- 降低低优先级事件发送频率
- 暂停部分 stream delta 下推
- 延后次级投影通知
- 对热点租户做局部限

### 4.5 Resume / Reconnect

每个设备连接必须具备

- `session_id`
- `device_id`
- `resume_token`
- `last_seen_event_seq`
- `last_seen_stream_checkpoint`

恢复原则：

- session 恢复与消息补偿分
- session 恢复stream 恢复分离
- 恢复只依赖权威状态，不依赖单点内存缓存

## 5. Route Plane 设计

### 5.1 路由模型

建议统一模型：

- `tenant_id`
- `principal_id`
- `device_id`
- `session_id`
- `owner_node_id`
- `route_epoch`
- `bound_at`
- `drain_state`

### 5.2 Fencing

为避免陈旧连接继续投递，必须引入 fencing

- 每次 route 绑定有`epoch`
- 新 route 绑定会提升epoch
- 老连接看到更高 epoch 必须主动退出
- 所有投递都要检查route epoch 是否匹配

### 5.3 Drain 与 Rebalance

节点不能粗暴下线，必须支持：

- `active`
- `draining`
- `drained`

`draining` 阶段规则：

- 不再接收新连接
- 允许已有连接平滑迁移
- 允许短时保留路由，直到目标节点接

### 5.4 RouteStore 建议

第一阶段建议：

- Redis 承担热点 route state
- 持久化元数据进入 PostgreSQL

长期演进：

- 保留 RouteStore 抽象
- 后续可切换自建 route directory 或轻量内嵌一致性层

## 6. 分层扩容策略

### 6.1 横向扩容粒度

扩容单位不应是整套系统，而应是：

- `link node`
- `route node`
- `messaging node`
- `stream node`
- `projection worker`
- `admin/control node`

### 6.2 典型扩容场景

#### 场景 A：在线人数暴

优先扩：

- Link Plane
- Route Plane

无需立即扩：

- Projection Plane
- Storage Plane

#### 场景 B：群聊与频道 fanout 暴涨

优先扩：

- Messaging Plane
- Projection Plane

可能联动：

- Link Plane

#### 场景 C：AI 流式对话暴涨

优先扩：

- Stream / AI Plane
- Link Plane

必要时联动：

- ObjectStore
- Projection Plane

#### 场景 D：AIoT 设备大规模在

优先扩：

- Link Plane
- Route Plane
- Stream / AI Plane

### 6.3 热点治理

必须考虑：

- 热租户
- 热群聊
- 热频道
- 热设备
- 热 agent 会话

治理手段：

- tenant lane
- scoped throttling
- hot shard rebalance
- high-fanout 分离处理
- stream 降级策略

## 7. Cell 与区域扩容

### 7.1 Cell 原则

一个 Cell 应作为：

- 故障域
- 扩展单元
- 部署单元
- 运维单元

### 7.2 SaaS 形态

建议：

- 小租户共享 cell
- 大租户可进入 dedicated cell
- 特高负载 tenant 可独立 link lane / route lane / storage lane

### 7.3 私有化形态

建议：

- 默认单 cell 起步
- 保留多 cell 模式接口
- 与云版本保持同一协议与模块边界

## 8. 可观测性与 SLO

连接层必须提供以下指标：

- 当前连接
- 活跃连接数
- 握手成功率
- 握手耗时
- 平均心跳延迟
- 每连接积压长度
- shard 负载分布
- route lookup 延迟
- reconnect 成功
- resume 成功
- 主动断开原因分布

建议核心 SLO：

- 连接建立成功率
- 在线消息投递成功率
- reconnect 恢复成功率
- 路由迁移成功率
- 节点排空完成时间

## 9. AI / IoT 的关

连接层必须直接支持：

- LLM token streaming
- AI tool result streaming
- 设备 telemetry streaming
- 设备命令回执
- 多端实时 presence

因此连接层协议绝不能只围绕“发送一条静态消息”设计

## 10. 实施建议

第一阶段优先事项：

- 从现有`session-gateway` `sdkwork-im-server` 中抽离连接热路径
- 建立 `runtime-link` `runtime-route` 边界
- 统一连接 frame 模型
- 统一 session / route / resume / reconnect 语义
- 建立连接层指标与诊断接口

后续阶段：

- 引入 MQTT
- 引入 QUIC 预留
- 更强 shard 调度
- 更高连接密度优化

## 11. 结论

`sdkwork-im` 的扩容能力必须首先建立在“连接与路由是独plane”的前提上。只有这样，系统才能同时具备：

- 高连接密度
- 多客户端路由一致性
- AI 流式能力
- AIoT 设备接入
- 分层独立扩容
- 商业化可控演进

## 2026-04-09 增补：连接架构标准与当前实现边界

### A. 当前实现真相

- 当前仓库已存储`session-gateway`、`runtime-link`、`runtime-route`、`sdkwork-im-server` 等真实连接与路由资产，文档`As-Built` 才是现状证据
- 当前并不等于本文中的分层弹性扩容、cell 扩展、全量排空迁移与所SLO 目标都已完整兑现有

### B. 本文哪些是目标：

- 第 2-11 节中的分层模型、弹性扩容、cell/区域扩展与连接 SLO，属于标准目标面
- 这些章节用于冻结连接层长期工程形态，不等于当天代码已全部达到终局架构

### C. 文档口径规则

- 写当前连接现状时，以本文 `As-Built`、`152CJ` 与当前`runtime-link / runtime-route / session-gateway` 代码为准
- 写未来连接标准时，必须显式标注 `目标态`、`标准目标面` 或 `Deferred`

## 2026-04-07 As-Built 1

- `Step 04` 已开始把 Link Plane 从架构概念兑现到真实 crate
  - `crates/sdkwork-im-runtime-link`
  - 当前已落地`LinkConnectionState / OutboundQueuePolicy / ResumeWindow / LinkSession`
- 当前这批类型尚未接管 `session-gateway` 的真实连接热路径，但已经冻结了一个最小 owner
  - 连接状态推进
  - 出站队列边界
  - resume window 边界
- 这意味着后续从`session-gateway/src/websocket.rs` 抽连接状态时，不需要再重新设计最底层状态模型，只需要把现有实现迁到已存在的 owner 中

## 2026-04-07 As-Built 2

- `Step 04` Route Plane 不再只停留在 skeleton
  - `services/session-gateway/src/cluster.rs` 已开始通过 `sdkwork-im-runtime-route::RouteDirectory` 承担真实 route owner 集成
  - gateway 公开 route 模型已收口为 `RouteBinding / RouteNodeLifecycle / RouteMigrationResult`
- 这次落地证明：
  - `route_epoch`
  - `session_id`
  - `connection_kind`
  - `bound_at`
  已经不是架构图里的占位字段，而是 cluster bridge 真实对外语义的一部分
- 当前仍未完成的部分：
  - `session-gateway/src/websocket.rs` 仍未切到 `runtime-link`
  - `sdkwork-im-server` 仍未收敛为纯装配层

## 2026-04-07 As-Built 3

## 2026-04-07 As-Built 4

- `sdkwork-im-server` 已开始按 Link / Route runtime 的统一装配边界工作，而不是继续在节点侧拆散持有realtime owner
  - `services/session-gateway/src/assembly.rs` 提供 `RealtimePlaneAssembly`
  - `services/sdkwork-im-cloud-gateway/src/node/build.rs` 通过bundle 装配 realtime plane
- drain / migrate 期间的owner 一致性护栏已补齐：
  - `RealtimeClusterBridge::ensure_client_route_local(...)` 先验证当前节点是否仍为客户端路由 owner
  - 旧节点若已进入draining ownership 已迁出，会在状态变更前拒绝 takeover 请求
- 这意味着连接管理架构中的关键约束继续成立：
  - 单设备单权威 route owner
  - route owner 中runtime owner 不允许在 drain 节点上被逆向抢回
  - 节点排空过程中不会因旧请求回流而制造 ghost owner
- 本轮只允许把 `CP04-4` 更新为通过，不允许把`CP04-2` 一并算过：
  - websocket accept / resume / backpressure 热路径owner 仍未完整迁入 `runtime-link`

## 2026-04-07 As-Built 5

- Link Plane 的默认出站队列策略 owner 已继续向 runtime owner 收口
  - `sdkwork-im-runtime-link::OutboundQueuePolicy` 现在提供 `realtime_default()`
  - `session-gateway` 不再维护私有默认队列阈值常量
- 这意味着连接管理与弹性扩容架构中：queue/backpressure 默认语义，开始从“服务实现细节”变成“Link runtime owner 规则
- 但当前仍不能宣告 `CP04-2` 通过
  - 仅默认策略 owner 完成迁移
  - 真实背压执行、resume/reconnect 热路径owner 仍未完整迁入 `runtime-link`

- Link Plane 已不再完全停留在设计稿：
  - `services/session-gateway/src/websocket.rs` 已开始通过 `LinkSession` 表达真实连接状态
  - `ResumeWindow` 已从 runtime checkpoint 真实映射
  - CCP 握手已推进`HelloNegotiated / Authenticated`
- 这说明 Link Plane 已进入“真实接线阶段”，但还未进入“完整抽离阶段

## 2026-04-07 As-Built 6

- Link Plane 的连接身份 owner 继续由 `runtime-link` 收口
  - `LinkSession` 新增 `actor_kind`
  - `LinkSession::matches_auth_bind(...)` 开始持有`principal / actor_kind / device / session` 的匹配语义
- `session-gateway` websocket 握手热路径已开始直接消费该 owner
  - `build_link_session(...)` 创建 `LinkSession` 时一并冻结actor kind
  - `complete_ccp_handshake(...)` 不再保留服务侧私有`auth_bind` 匹配判断
  - `AuthOkFrame` 的身份字段开始从 `LinkSession` 回填
- 这次回写说明：
  - 连接管理架构中的“连接身份语义”不再完全依赖服务层散落条件判断
  - `runtime-link` 已从“状态容器”继续推进到“身份 owner 起点
- 这次回写仍不说明：
  - accept / upgrade 已完整迁入`runtime-link`
  - resume / reconnect 已完整迁入`runtime-link`
  - 真实 queue/backpressure 执行已完整迁入`runtime-link`
- 因此 Step 04 判断保持不变：
  - `CP04-2` 仍未通过
  - `CP04-3` 通过
  - `CP04-4` 通过

## 2026-04-07 As-Built 7

- Link Plane `hello / accept` 协商 owner 继续由 `runtime-link` 收口
  - `LinkSession` 新增 `negotiate_hello(...)`
  - `runtime-link` 已开始持有：
    - 协议校验
    - 绑定校验
    - capability 协商
    - `HelloNegotiated` 状态推进
- `session-gateway` websocket 热路径已开始直接消费该 owner
  - `complete_ccp_handshake(...)` 不再自行判断 `hello.protocol / hello.binding`
  - 服务`supported_capabilities / negotiated_capabilities` 已去除
- 这次回写说明：
  - 连接管理架构中的 `hello` 协商不再完全依赖服务层散落逻辑
  - `runtime-link` 正从“状态 owner 起点”继续推进到“协商owner 起点
- 这次回写仍不说明：
  - HTTP/WebSocket accept / upgrade 已完整迁入`runtime-link`
  - 真实 queue/backpressure 执行已完整迁入`runtime-link`
  - `resume / reconnect` 已完整迁入`runtime-link`
- 因此 Step 04 判断保持不变：
  - `CP04-2` 仍未通过
  - `CP04-3` 通过
  - `CP04-4` 通过

## 2026-04-07 As-Built 8

- Link Plane `resume_window_manager` 继续由 `runtime-link` 收口
  - `runtime-link` 新增 `ResumeDecision`
  - `runtime-link` 新增 `decide_resume(last_seen_sync_seq, latest_sync_seq)`
  - `session-gateway` `PresenceRuntime::resume(...)` 已改为直接消费该 owner 结果
- 这意味着连接管理架构中的以下语义开始摆脱服务层私有判断：
  - `resume_required`
  - `resume_from_sync_seq`
  - `latest_sync_seq`
- 这次回写说明：
  - `resume / reconnect` 的恢复窗口边界已出现新的 runtime owner 接线
  - `runtime-link` 正从“状态 owner + 协商 owner”继续推进到“恢复语义 owner
- 这次回写仍不说明：
  - 完整 `session_resume / session_resumed` 控制帧时序已归属 `runtime-link`
  - HTTP/WebSocket accept / upgrade 已归`runtime-link`
  - 真实 queue/backpressure 执行已归`runtime-link`
- 因此 Step 04 判断继续保持有
  - `CP04-2` 未通过
  - `CP04-3` 通过
  - `CP04-4` 通过

## 2026-04-07 As-Built 9

- Link Plane `goaway` 断连语义继续由 `runtime-link` 收口
  - `runtime-link` 新增 `LinkGoAwayDirective`
  - `runtime-link` 新增 `session_disconnect_goaway()`
  - `session-gateway` CCP websocket 断连路径已改为直接消费该 owner
- 这意味着连接管理架构中的以下语义开始摆脱服务层私有实现有
  - 断连 close code
  - 断连 close reason
  - CCP `goaway` 控制帧负载
  - `goaway -> close` 顺序
- 这次回写说明：
  - Link Plane 的受控断连语义已出现新的 runtime owner 接线
  - `runtime-link` 正从“恢复语义 owner”继续推进到“受控迁出断连语义 owner
- 这次回写仍不说明：
  - 完整 `session_resume / session_resumed` 控制帧时序已归属 `runtime-link`
  - HTTP/WebSocket accept / upgrade 已归`runtime-link`
  - 真实 queue/backpressure 执行已归`runtime-link`
- 因此 Step 04 判断继续保持有
  - `CP04-2` 未通过
  - `CP04-3` 通过
  - `CP04-4` 通过

## 2026-04-07 As-Built 10

- Link Plane `session_resume / session_resumed` 控制帧时序继续向 `runtime-link` 收口
  - `runtime-link` 新增了
    - `LinkSessionResumeDirective`
    - `LinkSessionResumeError`
    - `LinkSession::negotiate_session_resume(...)`
  - `session-gateway` CCP websocket 热路径已改为直接消费owner
- 这意味着连接管理架构中的以下语义开始摆脱服务层私有实现有
  - `session_id` 恢复校验
  - resume catchup 起点计算
  - `session_resumed` 控制帧负载
  - `auth_ok -> session_resumed -> connected` 顺序
- 这次回写说明：
  - `resume / reconnect` 已继续从“恢复窗口纯语义 owner”推进到“控制帧时序 owner
  - `runtime-link` 正在从“协商与恢复判断 owner”继续推进到“完整连接热路径 owner
- 这次回写仍不说明：
  - HTTP/WebSocket accept / upgrade 已归属于 `runtime-link`
  - 真实 queue/backpressure 执行已归属于 `runtime-link`
- 因此 Step 04 判断继续保持有
  - `CP04-2` 未通过
  - `CP04-3` 通过
  - `CP04-4` 通过
## 2026-04-07 As-Built 11

- Link Plane 入站连接WebSocket upgrade contract 开始从 `session-gateway` 收回到`runtime-link`
  - `supported_websocket_subprotocols()` 负责声明接受的 WS 子协议集
  - `select_websocket_mode(...)` 负责根据协商结果决定 `LegacyJson / CcpJson`
  - `session-gateway` 仅保留Axum `WebSocketUpgrade` 适配`on_upgrade(...)` 执行
- 这样，`accept / upgrade` 从“网关硬编码常量 + 布尔判断”收敛为 “runtime owner contract + transport adapter
- 当前仍未闭环
  - Axum `on_upgrade(...)` 执行动作未回
  - live `queue/backpressure` 未回
  - `CP04-2` 仍未通过
## 2026-04-07 As-Built 12

- Link Plane live outbound batch/backpressure 裁决开始从 `session-gateway` 收回到`runtime-link`
  - `LinkOutboundBatchPlan` 成为新的连接batch owner contract
  - `plan_stream_batch(...)` 负责 catchup / push 的默认批量裁剪
  - `plan_pull_batch(...)` 负责客户端pull limit hard clamp 裁决
- `session-gateway` 现在只做：
  - backlog 观察
  - `runtime-link` owner 调用
  - `runtime.list_events(...)` transport/runtime 适配
- 这说明连接管理架构已从“gateway 既持有queue 参数又持有live batch 判断”继续推进到“runtime crate 持有 batch/backpressure authority，gateway 仅消费结果
- 当前仍未闭环
  - `on_upgrade(...)` 执行 owner 未回
  - 完整 async queue / buffer / drop / degrade controller 未回
  - 因此 `CP04-2` 仍未通过

## 2026-04-07 As-Built 13

- Link Plane websocket upgrade handoff execute 开始从 `session-gateway` 收回到`runtime-link`
  - `prepare_websocket_upgrade(...)` 负责selected subprotocol 与业务上下文固化handoff owner
  - `LinkWebsocketUpgradeHandoff<TContext>` 负责持有 mode/context 并提供`execute(...)`
  - `session-gateway` 改为只保留Axum `WebSocketUpgrade` adapter、上下文装配`upgrade.execute(...)` 调用途
- 这说明连接管理架构已从“gateway 私有持有 upgrade 热路径执行”继续推进到“runtime crate 持有 handoff execute authority，gateway 仅做 transport adapter
- 当前仍未闭环
  - Axum `WebSocketUpgrade` adapter `.on_upgrade(...)` 入口调用仍在 `session-gateway`
  - 完整 async queue / buffer / drop / degrade / overload controller 仍未回收
  - 因此 `CP04-2` 仍未通过

## 2026-04-07 As-Built 14

- Link Plane live push overload 降级裁决开始从 `session-gateway` 收回到`runtime-link`
  - `LinkPushMode::Immediate / PullOnly` 成为新的连接侧降级 owner contract
  - `LinkPushPlan` 负责携带 batch/backpressure 与降级模型
  - `session-gateway` 改为只保留watch 通知接入、runtime 调用途websocket 发送适配
- 这说明连接管理架构已从“gateway 私有判断积压是否继续自动 push”推进到“runtime crate 持有 degrade authority，gateway 仅消费裁决
- 当前仍未闭环
  - 完整 async queue / buffer 实体仍未回收
  - backlog drop / overload close 语义仍未回收
  - Axum `WebSocketUpgrade` adapter `.on_upgrade(...)` 入口调用仍在 `session-gateway`
  - 因此 `CP04-2` 仍未通过

## 2026-04-07 As-Built 15

- Link Plane buffered push recovery 继续从 `session-gateway` 收回到 `runtime-link`
  - `LinkPushCursor` 开始成为连接侧 backlog 恢复 owner
  - `LinkBufferedPushPlan` 负责描述`PullOnly -> Immediate` 过程中应补发送buffered window
  - `session-gateway` 改为只保留：
    - watch 通知接入
    - `events.pull` transport 适配
    - runtime owner 调用后的 websocket 发送
- 这说明连接管理架构已从“gateway 私有持有 backlog 恢复判断”继续推进到“runtime crate 持有 buffered push recovery authority，gateway 仅消费恢复计划
- 当前仍未闭环
  - 完整 async queue / buffer 实体仍未回收
  - backlog drop / overload close 语义仍未回收
  - Axum `WebSocketUpgrade` adapter `.on_upgrade(...)` 入口调用仍在 gateway
  - 因此 `CP04-2` 仍未通过

## 2026-04-07 As-Built 16

- Link Plane extreme overload close 继续从 `session-gateway` 收回到 `runtime-link`
  - `LinkPushMode::Disconnect` 开始成为连接侧 extreme overload owner contract
  - `LinkPushPlan::disconnect` 负责携带 close directive
  - `realtime_overload_goaway()` 负责产出 extreme overload 下的 goaway/close 语义
  - `session-gateway` 改为只保留：
    - websocket transport 适配
    - runtime owner 调用后的 close 执行
- 这说明连接管理架构已从“gateway 私有持有极端积压断链判断”继续推进到“runtime crate 持有 overload close authority，gateway 仅消费关闭裁决
- 当前仍未闭环
  - backlog drop 语义仍未回收
  - 完整 async queue / buffer 实体仍未回收
  - Axum `WebSocketUpgrade` adapter `.on_upgrade(...)` 入口调用仍在 gateway
  - 因此 `CP04-2` 仍未通过

## 2026-04-07 As-Built 17

- Link Plane websocket upgrade 服务侧保留边界继续收敛：
  - `runtime-link` 继续持有：
    - subprotocol contract
    - websocket mode selection
    - upgrade handoff execute owner
  - `session-gateway` 不再upgrade helper 散落保留`src/lib.rs`
  - 新增单一 transport seam 模块：
    - `services/session-gateway/src/websocket_upgrade.rs`
- 这说明连接管理架构已从“gateway 大文件散落持有upgrade seam helper”继续推进到“gateway 仅保留单一 Axum transport seam 模块：
- 当前仍未闭环
  - Axum `WebSocketUpgrade` adapter `.on_upgrade(...)` 入口调用仍在 `session-gateway`
  - backlog drop 语义仍未回收
  - 完整 async queue / buffer 实体仍未回收
  - 因此 `CP04-2` 仍未通过

## 2026-04-07 As-Built 19

- Link Plane queue/controller 输入 authority 继续收敛到
  - `runtime-link` 新增公开 owner 公式：`pending_outbound_events(...)`
  - `session-gateway` 不再私有持有 backlog 计算 helper
- 这说明连接管理架构已从“gateway 复制 backlog 输入数学”推进到“runtime crate 统一持有 backlog 输入数学
- 当前仍未闭环
  - Axum `WebSocketUpgrade` adapter `.on_upgrade(...)` 入口调用仍在 `session-gateway`
  - backlog drop 语义仍未回收
  - 完整 async queue / buffer 实体仍未回收
  - 因此 `CP04-2` 仍未通过

## 2026-04-07 As-Built 18

- Link Plane websocket 服务侧保留边界继续收窄：
  - `session-gateway` 不仅把upgrade helper 集中：`src/websocket_upgrade.rs`
  - `/im/v3/api/realtime/ws` route handler 也已集中到同一 transport seam 模块
  - `src/lib.rs` 只保留路由装配，不再直接导入 `WebSocketUpgrade`
- 这说明连接管理架构已从“gateway 单一 seam 模块承接 upgrade helper”继续推进到“gateway 单一 seam 模块承接完整 websocket route 入口
- 当前仍未闭环
  - Axum `WebSocketUpgrade` adapter `.on_upgrade(...)` 入口调用仍在 `session-gateway`
  - backlog drop 语义仍未回收
  - 完整 async queue / buffer 实体仍未回收
  - 因此 `CP04-2` 仍未通过
## 2026-04-07 As-Built 20

- connection-management as-built moved one more outbound queue boundary into `runtime-link`
  - new owner: `LinkOutboundQueueState`
  - entry point: `LinkSession::start_outbound_queue(...)`
  - owner responsibilities now include:
    - catchup plan
    - pull plan
    - delivered vs last-sent state
    - buffered push recovery
    - ack bookkeeping
- `session-gateway` no longer keeps those combined mutable queue-state fields in `src/websocket.rs`
- this narrows the service-side boundary to transport concerns plus still-open adapter semantics
- still not closed
  - Axum `WebSocketUpgrade` / `.on_upgrade(...)` remains in the gateway transport seam
  - backlog drop semantics remain outside `runtime-link`
  - therefore `CP04-2` and `Step 04` still remain open

## 2026-04-07 As-Built 21

- connection-management as-built moved one truthful overload-time stale replay rule into `runtime-link`
  - `LinkOutboundQueueState::plan_pull(...)` now clamps a stale replay request below `last_sent_after_seq` when backlog measured from that sent frontier still exceeds `hard_limit`
  - the gateway continues to consume the runtime owner instead of deciding whether replay rewind is still allowed under pressure
- this means the Link Plane owner now carries one real backlog-drop boundary without pretending to delete durable history
- still not closed
  - Axum `WebSocketUpgrade` / `.on_upgrade(...)` remains in the gateway transport seam
  - a complete runtime-owned async outbound queue / buffer entity still does not exist
  - therefore `CP04-2` and `Step 04` still remain open

## 2026-04-07 As-Built 22

- connection-management as-built moved the buffered push drain async loop into `runtime-link`
  - runtime owner
    - `LinkBufferedPushDrainDriver`
    - `LinkBufferedPushFetchedWindow<TWindow>`
    - `LinkBufferedPushDrainStatus`
    - `LinkOutboundQueueState::drain_buffered_push_windows(...)`
  - gateway role
    - `BufferedPushDrainDriver` only adapts:
      - `RealtimeDeliveryRuntime::list_events(...)`
      - websocket payload send
      - transport/runtime error mapping
- this means the connection-management boundary has moved from:
  - "gateway owns the buffered push drain loop and runtime only owns queue state fragments"
- to:
  - "runtime crate owns the buffered push drain loop and gateway only bridges transport concerns"
- still not closed
  - Axum `WebSocketUpgrade` / `.on_upgrade(...)` remains in the gateway transport seam and still needs explicit final-boundary acceptance
  - therefore `CP04-2` and `Step 04` still remain open

## 2026-04-07 As-Built 23

- connection-management architecture now accepts a pure retained Axum transport seam as the final Step 04 gateway boundary
  - `services/session-gateway/src/websocket_route.rs`
    - owns websocket route preflight only:
      - auth
      - device resolve
      - route/session fence
      - websocket registration
  - `services/session-gateway/src/websocket_upgrade.rs`
    - owns only the framework-local Axum seam:
      - `WebSocketUpgrade`
      - subprotocol offer
      - runtime-link handoff prepare
      - `.on_upgrade(...)`
      - transport-to-runtime bridge
- this changes the connection-management boundary from:
  - "gateway transport seam still mixes framework adapter and route preflight"
- to:
  - "gateway retains only a pure framework adapter seam, while route preflight and runtime semantics are separated"
- closed judgement
  - `CP04-2` is passed
  - `Step 04` is closed

## 2026-04-08 As-Built 24

- `Wave D / Step 11 / CP11-1` has frozen the connection validation baseline into a shared scenario catalog instead of leaving Step 11 to ad-hoc test picking.
- the catalog now declares:
  - `CI Smoke Tier`
  - `Pre-Release Tier`
  - `Capacity Tier`
- the `connection` family is explicitly anchored to:
  - `services/sdkwork-im-cloud-gateway/tests/websocket_e2e_test.rs`
  - `tools/chat-cli/tests/chat_cli_e2e_test.rs`
  - `tools/smoke/local_stack_smoke.ps1`
  - `tools/smoke/local_stack_smoke.sh`
- this means the connection-management architecture now has a truthful Step 11 baseline for:
  - handshake success
  - active connection count
  - resume success
  - connection latency measurement vocabulary
- still not closed for Step 11 capacity claims:
  - no quantitative density result has been written yet
  - no `1k-10k` or higher connection evidence exists yet
  - multi-cell or multi-region connection drills remain out of scope for this increment

## 2026-04-08 As-Built 25

- `Wave D / Step 11 / CP11-2` has now produced the first local quantitative connection result on top of the previously frozen catalog.
- executed baseline:
  - `profile = standalone.split-services.development`
  - `tier = CI Smoke Tier`
  - `connectionCount = 32`
- measured result:
  - `32 / 32` successful websocket connections
  - `totalDurationMs = 19.789`
  - `connectP95Ms = 17.216`
  - `connectionsPerSecond = 1617.068`
- this means the connection-management architecture now has:
  - one truthful quantitative baseline
  - one repeatable in-repo execution path
  - one fixed metric vocabulary for later larger-tier comparison
- still not closed for Step 11 capacity claims:
  - `1k-10k` or higher density has not been exercised
  - multi-cell or multi-region connection drills remain out of scope

## 2026-04-08 As-Built 26

- `Wave D / Step 11 / CP11-3` has now added two local route/connection resilience drills on top of the earlier quantitative baseline.
- measured drill results:
  - `drain-rebalance`
    - `migratedRouteCount = 1`
    - `deliveryPreserved = true`
    - `drillDurationMs = 0.661`
  - `failover`
    - `activeOwnerNodeId = node_b`
    - `staleDisconnectRejected = true`
    - `takeoverDurationMs = 0.438`
- this means the connection-management architecture now has:
  - one truthful local drain/rebalance drill
  - one truthful local failover/takeover drill
  - one repeatable in-repo execution path for both
- still not closed for higher-tier HA claims:
  - multi-route or multi-tenant failover has not been exercised
  - multi-cell or multi-region route failover remains out of scope

## 2026-04-08 As-Built 27

- `Wave D / Step 11 / CP11-4` has now closed the last connection-plane evidence gap by adding a rollback-safe hello negotiation drill on top of the earlier quant and failover baselines.
- latest truthful local quantitative result:
  - `32 / 32` successful websocket connections
  - `totalDurationMs = 17.754`
  - `connectP95Ms = 15.108`
  - `connectionsPerSecond = 1802.431`
- latest rollback-safe connection evidence:
  - `compatibleClientCount = 4 / 4`
  - `compatibilityMatrixPassRate = 1.0`
  - `rollbackActivationMs = 0.007`
  - `postRollbackProtocolErrorRate = 0.0`
- this means the connection-management architecture now has:
  - one truthful local connection throughput baseline
  - one truthful local failover/takeover drill
  - one truthful local rollback-safe binding negotiation drill
- still not closed for higher-tier claims:
  - `1k-10k` or higher density has not been exercised
  - multi-cell or multi-region connection orchestration remains out of scope

