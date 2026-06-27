> Migrated from `docs/review/step-04-质量审计与复盘-2026-04-07.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 04 质量审计与复盘- 2026-04-07

## 1. 基本信息

- Step 编号：`04`
- 波次：`Wave B`
- 执行日期：`2026-04-07`
- 关联 step 文档
  - `docs/step/04-link-plane与route-plane运行时重md`
  - `docs/step/91-Step质量审计清单与复盘模md`
  - `docs/step/95-架构能力闭环验收标准.md`
  - `docs/step/97-Step完成后的架构回写与能力兑现清单md`
- 关联架构文档
  - `docs/架构/130-连接优先的AI时代即时通讯架构蓝图-2026-04-06.md`
  - `docs/架构/131-连接管理与分层弹性扩容架构设计-2026-04-06.md`
  - `docs/架构/136-关键业务链路与跨Plane时序设计-2026-04-06.md`
  - `docs/架构/149-多Cell多Region协议升级与灾备兼容设计-2026-04-06.md`

## 2. 本轮完成

- 已用 TDD 方式先编写 `runtime_plane_split_test`
- 已新增真实 `runtime-link / runtime-route` crate
- 已让 `session-gateway` 测试能够真实消费这两个新 runtime crate
- 已把 route epoch / drain / migration 模型拉成最小可验证 owner

## 3. 检查点结果

- `CP04-1`：通过
  - `runtime-link` `runtime-route` 骨架已存在且有真实测
- `CP04-2`：未通过
  - 连接热路径尚未从 `session-gateway` 真实抽离
- `CP04-3`：部分通过
  - route epoch / drain / migration 模型已有最runtime owner
  - 但现cluster bridge 还未正式切到owner
- `CP04-4`：未通过
  - `sdkwork-im-server` 还未明显朝纯装配层收口

## 4. 测试结果

### 4.1 Red / Green 证据

- Red
  - `$env:CARGO_TARGET_DIR='target/local-minimal-step04-red'; cargo test -p session-gateway --test runtime_plane_split_test --offline`
- Green
  - `cargo fmt --check --package sdkwork-im-runtime-link --package sdkwork-im-runtime-route --package session-gateway`
  - `$env:CARGO_TARGET_DIR='target/local-minimal-step04-green'; cargo test -p session-gateway --test runtime_plane_split_test --offline`
  - `$env:CARGO_TARGET_DIR='target/local-minimal-step04-green'; cargo test -p session-gateway --test cluster_routing_test --offline`

### 4.2 结论

- 本轮 slice red/green 已完
- 当前验证只能证明 `Step 04` 已开始形成真实 runtime owner，不能证明 `Step 04` 已闭环

## 5. 本轮发现问题

### 5.1 运行数owner 仍在旧服务内

- `session-gateway/src/cluster.rs` 仍然route lifecycle 的主要实owner
- `session-gateway/src/websocket.rs` 仍然是连接状态与握手运行时的主要实现 owner

### 5.2 crate 还处skeleton 阶段

- `runtime-link` 当前只有状态模型和队列/恢复窗口配置
- `runtime-route` 当前只有 route directory node lifecycle 的最小实
- 距离 Step 04 的真实“热路径抽离”还差一整段集成工作

## 6. 风险评估

- 当前残余风险
  - 如果下一轮直接把大量逻辑搬迁到新 crate，而不沿现cluster / websocket 测试回归推进，容易破坏实时路
  - 如果 `runtime-route` 与现cluster bridge 分别定义 route 语义，会造成owner 冲突
- 是否允许进入下一 step：`否`
- 当前阻塞项：
  - `CP04-2`
  - `CP04-4`
  - `CP04-3` 仍未全量兑现

## 7. Step 04 架构能力闭环判定

对应 `docs/step/95-架构能力闭环验收标准.md` Step 04 条目

- Link Plane：未闭环
- Route Plane：未闭环
- `resume / reconnect / drain / fencing`：只有部owner 开始收口

综合判定

- `95` 当前不通过

## 8. 架构回写判定

对应 `docs/step/97-Step完成后的架构回写与能力兑现清单md`

- 本轮已做部分 as-built 回写
- Step 04 尚未整体闭环，因此 `97` 当前只能判为部分完成

当前判定

- `97`：`部分完成，整体未通过`

## 9. 下一轮输出

- 继续留在 `Step 04`
- 优先在 `session-gateway` 的现有 route lifecycle 开始依赖`runtime-route`
- 再让连接状态与 resume window owner 开始从 `websocket.rs` `runtime-link` 移动

## 10. 质量评分

| 维度 | 分数 | 说明 |
| --- | --- | --- |
| 架构对齐 | 9 | slice `131 / 136` 对齐，边界清单|
| 边界清晰 | 10 | 明确只做 runtime skeleton，不Step 05/07 |
| 路径真实 | 10 | 代码、测试和文档都是真实仓库路径 |
| 实施可执行| 9 | red/green 路径清晰，但仍是起步 slice |
| 测试完整 | 9 | 已有 fail-first 与回归补齐|
| 验证可重| 10 | 命令可重|
| 检查点可判| 8 | 能清楚指出哪CP 还未|
| 风险与回写| 8 | 通过最slice 控制风险，但集成工作未开|
| 依赖明确 | 10 | 下一轮入口清单|
| 复盘沉淀 | 9 | review 与架构回写已启动 |
| 总分 | 92 | Step 04 当前 slice 质量合格，但仍远未闭环|

## 11. 2026-04-07 增量审计更新（覆盖前文旧判断

### 11.1 关键状态更深

- `CP04-3` 由“部分通过”提升为“通过
- 原因不是只改了文档或类型别名，而是
  - 先写fail-first 测试 `test_cluster_bridge_public_route_models_use_runtime_route_owner_types`
  - `runtime-route` 已补齐`session_id / connection_kind / bound_at / route_epoch`
  - `session-gateway/src/cluster.rs` 已改为通过 `RouteDirectory` 持有 route owner node lifecycle
  - `session-gateway` 的公开 route 模型已收口为 `runtime-route` 类型

### 11.2 对旧问题判断的修改

- 旧判断“`session-gateway/src/cluster.rs` 仍然route lifecycle 的主要实owner”不再准
- 当前更准确的判断应为
  - `cluster.rs` 仍然route handoff、runtime 状态转移、错误映射的集成
  - route ownership / epoch / drain / migration 的公开模型与目owner 已收口到 `sdkwork-im-runtime-route`
- 旧风险“`runtime-route` 与现cluster bridge owner 冲突”已从现实风险降为已消除风险
  - 当前 `cluster bridge` 已消费同一套owner 模型

### 11.3 本轮补充验证

- `$env:CARGO_TARGET_DIR='target/local-minimal-step04-route-red'; cargo test -p session-gateway --test cluster_routing_test test_cluster_bridge_public_route_models_use_runtime_route_owner_types --offline`
- `$env:CARGO_TARGET_DIR='target/local-minimal-step04-route-green'; cargo test -p session-gateway --test cluster_routing_test test_cluster_bridge_public_route_models_use_runtime_route_owner_types --offline`
- `cargo fmt --check --package sdkwork-im-runtime-route --package session-gateway`
- `$env:CARGO_TARGET_DIR='target/local-minimal-step04-route-green'; cargo test -p session-gateway --offline`

### 11.4 当前剩余阻塞

- `CP04-2` 仍未通过
  - `websocket.rs` 仍持有连接热路径 owner
- `CP04-4` 仍未通过
  - `sdkwork-im-server` 还未明显收敛为装配层
- `91`：仍未整体通过
- `95`：仍未整体通过
- `97`：仍为部分完

### 11.5 最新审计结

- 本轮代码、测试、review、架构回写均已追
- `Step 04` 已从“runtime skeleton 被测试消费”推进到“route owner 已接入真实cluster bridge
- 但由Link Plane 与本地节点装配层尚未闭环，`Step 04` 依旧不能判定完成

## 12. 2026-04-07 Link Plane 增量审计更新

## 13. 2026-04-07 sdkwork-im-server 装配收口与迁移护栏审计更深

### 13.1 审计背景

- 当前波次仍为 `Wave B`
- 当前 Step 仍为 `Step 04`
- 本轮目标不是重新开启 `Step 03`，而是确认 `CP04-4` 是否已经真正闭环，并验证 drain 迁移期间 route owner / runtime owner 不会被旧节点回流污染

### 13.2 本轮新增代码与行为证

- `services/session-gateway/src/assembly.rs`
  - 新增统一装配 bundle `RealtimePlaneAssembly`
- `services/session-gateway/src/lib.rs`
  - 已公开 re-export `RealtimePlaneAssembly`
- `services/sdkwork-im-cloud-gateway/src/node/build.rs`
  - 已由拆散的 `cluster / realtime runtime / presence runtime` 构造，改为通过 `RealtimePlaneAssembly` 统一装配 realtime plane
- `services/session-gateway/src/cluster.rs`
  - 新增 `RealtimeClusterBridge::ensure_client_route_local(...)`
- `services/sdkwork-im-cloud-gateway/src/node/access.rs`
  - takeover 的设备绑定路径已在状态变更前执行 owner locality 校验
  - 当旧节点已不是权route owner 时，请求会被直接拒绝，不再把 runtime ownership 抢回本地

### 13.3 fail-first 与回归验证

- fail-first
  - `$env:CARGO_TARGET_DIR='target/local-minimal-step04-node-red'; cargo test -p sdkwork-im-cloud-gateway --test lib_structure_test test_local_minimal_node_build_surface_assembles_realtime_plane_via_session_gateway_bundle --offline`
- green
  - `$env:CARGO_TARGET_DIR='target/local-minimal-step04-node-green'; cargo test -p sdkwork-im-cloud-gateway --test lib_structure_test test_local_minimal_node_build_surface_assembles_realtime_plane_via_session_gateway_bundle --offline`
  - `cargo fmt --check --package session-gateway --package sdkwork-im-server`
  - `$env:CARGO_TARGET_DIR='target/local-minimal-step04-node-final'; cargo test -p sdkwork-im-cloud-gateway --test cluster_drain_rebalance_e2e_test --offline`
  - `$env:CARGO_TARGET_DIR='target/local-minimal-step04-node-final'; cargo test -p sdkwork-im-cloud-gateway --test cluster_realtime_routing_e2e_test --offline`
  - `$env:CARGO_TARGET_DIR='target/local-minimal-step04-session-full'; cargo test -p session-gateway --offline`
  - `$env:CARGO_TARGET_DIR='target/local-minimal-step04-node-full'; cargo test -p sdkwork-im-cloud-gateway --offline`

### 13.4 审计结论更新

- `CP04-4`：更新为通过
  - `sdkwork-im-server` 已经通过统一 `RealtimePlaneAssembly` 显式装配 Link / Route runtime
  - drain 后的旧节点不会在失败前先改写本地 route/runtime ownership
- `CP04-3`：保持通过
- `CP04-2`：仍未通过
  - accept / upgrade / queue / backpressure / resume / reconnect 热路径仍未完整迁移到 `runtime-link`
- `91`：仍未整体通过
  - 剩余阻塞已收敛到 `CP04-2`
- `95`：仍未整体通过
  - Step 04 仍不能宣告整体验收通过
- `97`：继续为部分完成
  - 允许补写 as-built，但不允许宣Step 04 已全部兑
- `93`：不触发
  - `Wave B` 尚未完成

### 13.5 风险与后续动

- 当前未发现与 `131 / 136` 冲突的实现偏
- 剩余主风险已收敛`session-gateway/src/websocket.rs` Link Plane hot path owner
- 下一轮必须继续只围绕 `CP04-2` 推进。
  - accept / upgrade
  - queue / backpressure
  - resume / reconnect

## 14. 2026-04-07 runtime-link 默认队列 owner 审计更新

### 14.1 审计事实

- `queue / backpressure` 默认阈值已不再`session-gateway/src/websocket.rs` 私有常量定义
- `crates/sdkwork-im-runtime-link/src/lib.rs` 已新增：
  - 默认实时出站队列阈值常
  - `OutboundQueuePolicy::realtime_default()`
- `services/session-gateway/src/websocket.rs` `build_link_session(...)` 已改为直接消费 `runtime-link` owner 默认策略

### 14.2 验证证据

- Red
  - `$env:CARGO_TARGET_DIR='target/local-minimal-step04-queue-red'; cargo test -p session-gateway test_build_link_session_uses_runtime_link_default_queue_owner_policy --offline`
- Green
  - `$env:CARGO_TARGET_DIR='target/local-minimal-step04-queue-green'; cargo test -p sdkwork-im-runtime-link --offline`
  - `$env:CARGO_TARGET_DIR='target/local-minimal-step04-queue-green'; cargo test -p session-gateway test_build_link_session_uses_runtime_link_default_queue_owner_policy --offline`
  - `cargo fmt --check --package sdkwork-im-runtime-link --package session-gateway`
  - `$env:CARGO_TARGET_DIR='target/local-minimal-step04-queue-full'; cargo test -p session-gateway --offline`

### 14.3 审计结论

- 该增量说`CP04-2` 已继续向前推进，但仍未通过
- 本轮只能确认
  - Link Plane 默认队列 owner 已收口到 `runtime-link`
  - `session-gateway` 不再持有这组默认阈值的权威定义
- 本轮仍不能确认：
  - 真实 queue/backpressure 执行策略已完全迁`runtime-link`
  - accept / upgrade / resume / reconnect 已整体迁
- 因此
  - `91` 仍未整体通过
  - `95` 仍未整体通过
  - `97` 仍为部分完成

### 12.1 新推

- `runtime-link` 不再只存在于 `runtime_plane_split_test`
- `services/session-gateway/src/websocket.rs` 已开始真实消费：
  - `LinkSession`
  - `ResumeWindow`
- CCP 握手现已把连接状态推进到
  - `HelloNegotiated`
  - `Authenticated`
- checkpoint 映射已进入 `ResumeWindow`

### 12.2 新证

- fail-first
  - `websocket::tests::test_build_active_link_session_maps_checkpoint_into_runtime_link_owner`
- 回归
  - `$env:CARGO_TARGET_DIR='target/local-minimal-step04-link-green'; cargo test -p session-gateway --test websocket_smoke_test --offline`
  - `$env:CARGO_TARGET_DIR='target/local-minimal-step04-link-green'; cargo test -p session-gateway --offline`

### 12.3 当前审计判断

- `CP04-2` 仍未通过，但已从“完全未接线”更新为“真实热路径已开始消费 runtime-link owner
- `95` 仍不通过
  - 原因不是 Link Plane 零进展，而是尚未完成完整热路径抽
- `97` 仍为部分完成
  - 本轮已对 Link Plane as-built 做增量回写

## 15. 2026-04-07 runtime-link auth owner 审计更新

### 15.1 审计背景

- 当前仍位于 `Wave B / Step 04`
- `CP04-3 / CP04-4` 已通过后，`CP04-2` 继续成为唯一主阻塞
- 本轮只审计一个新hot path slice
  - `auth_bind` 身份匹配 owner 是否继续由 `session-gateway` 收口到 `runtime-link`

### 15.2 本轮新增代码与行为证

- `crates/sdkwork-im-runtime-link/src/lib.rs`
  - `LinkSession` 新增 `actor_kind`
  - 新增 `LinkSession::matches_auth_bind(...)`
  - 新增 owner 单测
    - `test_link_session_matches_auth_bind_identity`
    - `test_link_session_rejects_mismatched_auth_bind_identity`
- `services/session-gateway/src/websocket.rs`
  - `build_link_session(...)` 改为actor kind 一并收口到 `runtime-link`
  - `complete_ccp_handshake(...)` 不再保留服务侧私有 `auth_bind_matches_context(...)`
  - `AuthOkFrame` 改为`LinkSession` 回填身份字段
- `services/session-gateway/tests/runtime_plane_split_test.rs`
  - 跟进 `LinkSession::new(...)` owner 新签

### 15.3 fail-first 与回归验证

- Red
  - `$env:CARGO_TARGET_DIR='target/local-minimal-step04-auth-owner-red'; cargo test -p session-gateway test_build_link_session_preserves_actor_identity_for_runtime_link_auth_owner --offline`
  - 失败原因：`LinkSession` 尚未提供 `matches_auth_bind(...)`
- Green
  - `$env:CARGO_TARGET_DIR='target/local-minimal-step04-auth-owner-green'; cargo test -p sdkwork-im-runtime-link --offline`
  - `$env:CARGO_TARGET_DIR='target/local-minimal-step04-auth-owner-green'; cargo test -p session-gateway test_build_link_session_preserves_actor_identity_for_runtime_link_auth_owner --offline`
  - `cargo fmt --check --package sdkwork-im-runtime-link --package session-gateway`
  - `$env:CARGO_TARGET_DIR='target/local-minimal-step04-auth-owner-full'; cargo test -p session-gateway --offline`

### 15.4 审计结论

- `CP04-2`：仍未通过，但继续前推
  - `auth_bind` 身份匹配 owner 已从 websocket 服务侧条件判断继续收口到 `runtime-link`
  - `runtime-link` 已不再只是状态容器，也开始持有连接身份语
- 本轮仍不能宣告：
  - accept / upgrade 已迁移到 `runtime-link`
  - 真实 queue/backpressure 执行已迁移到 `runtime-link`
  - resume / reconnect 已迁移到 `runtime-link`
- 因此维持总判断：
  - `CP04-3` 通过
  - `CP04-4` 通过
  - `CP04-2` 未通过
  - `91 / 95 / 97` 仍未整体通过
  - `Wave B / 93` 仍不触发

## 16. 2026-04-07 runtime-link hello owner 审计更新

### 16.1 审计背景

- 当前仍位于 `Wave B / Step 04`
- 上一轮已把默认队owner `auth_bind` 身份 owner 推入 `runtime-link`
- 本轮继续只围绕 `CP04-2`，检查 `hello` 协商 owner 是否继续由 `session-gateway` 收口

### 16.2 本轮新增代码与行为证

- `crates/sdkwork-im-runtime-link/Cargo.toml`
  - 新增 `sdkwork-im-ccp-core / sdkwork-im-ccp-control`
- `crates/sdkwork-im-runtime-link/src/lib.rs`
  - 新增 `LinkHelloError`
  - 新增 `LinkSession::negotiate_hello(...)`
  - `runtime-link` 现在开始持有：
    - 协议校验
    - 绑定校验
    - capability 协商
    - `HelloNegotiated` 状态推
  - 新增 owner 单测
    - `test_link_session_negotiates_supported_hello`
    - `test_link_session_rejects_unsupported_hello_protocol`
    - `test_link_session_rejects_unsupported_hello_binding`
- `services/session-gateway/src/websocket.rs`
  - `complete_ccp_handshake(...)` 改为直接消费 `link_session.negotiate_hello(...)`
  - 删除服务`supported_capabilities / negotiated_capabilities`

### 16.3 fail-first 与回归验证

- Red
  - `$env:CARGO_TARGET_DIR='target/local-minimal-step04-hello-owner-red'; cargo test -p session-gateway test_build_link_session_negotiates_hello_via_runtime_link_owner --offline`
  - 失败原因：`LinkSession` 尚未提供 `negotiate_hello(...)`
- Green
  - `$env:CARGO_TARGET_DIR='target/local-minimal-step04-hello-owner-green'; cargo test -p sdkwork-im-runtime-link --offline`
  - `$env:CARGO_TARGET_DIR='target/local-minimal-step04-hello-owner-green'; cargo test -p session-gateway test_build_link_session_negotiates_hello_via_runtime_link_owner --offline`
  - `cargo fmt --check --package sdkwork-im-runtime-link --package session-gateway`
  - `$env:CARGO_TARGET_DIR='target/local-minimal-step04-hello-owner-full'; cargo test -p session-gateway --offline`

### 16.4 审计结论

- `CP04-2`：仍未通过，但继续前推
  - `hello` 协商 owner 已从 websocket 服务侧开始迁移到 `runtime-link`
  - `runtime-link` 已不再只承载状队列/身份匹配，也开始承载连接协商语
- 本轮仍不能宣告：
  - HTTP/WebSocket accept / upgrade 已完整迁移到 `runtime-link`
  - 真实 queue/backpressure 执行已迁移到 `runtime-link`
  - resume / reconnect 已迁移到 `runtime-link`
- 因此维持总判断：
  - `CP04-3` 通过
  - `CP04-4` 通过
  - `CP04-2` 未通过
  - `91 / 95 / 97` 仍未整体通过
  - `Wave B / 93` 仍不触发

## 17. 2026-04-07 runtime-link resume owner 审计更新

### 17.1 审计背景

- 当前仍位于 `Wave B / Step 04`
- `CP04-3 / CP04-4` 已通过，Step 04 剩余阻塞已收敛到 `CP04-2`
- 根据 `docs/架构/131` `docs/架构/136`，`resume / reconnect` 语义应继续从服务层回收到 `runtime-link`

### 17.2 本轮新增代码与行为证

- `crates/sdkwork-im-runtime-link/src/lib.rs`
  - 新增 `ResumeDecision`
  - 新增 `decide_resume(...)`
  - `runtime-link` 开始持有：
    - `resume_required`
    - `resume_from_sync_seq`
    - `latest_sync_seq`
    这组三元语义的权威计
- `services/session-gateway/src/presence.rs`
  - `PresenceRuntime::resume(...)` 改为直接消费 `runtime-link` resume 计算结果
  - 服务层不再本地保留该段纯判断逻辑
- `services/session-gateway/tests/presence_runtime_persistence_test.rs`
  - 新增集成回归
    - `test_presence_runtime_resume_returns_incremental_sync_window_from_runtime_link_owner`

### 17.3 fail-first 与回归验证

- Red
  - `$env:CARGO_TARGET_DIR='target/local-minimal-step04-resume-owner-red'; cargo test -p sdkwork-im-runtime-link test_runtime_link_decides_incremental_resume_window --offline`
  - 失败原因：`runtime-link` 尚未提供 `ResumeDecision / decide_resume(...)`
- Green
  - `$env:CARGO_TARGET_DIR='target/local-minimal-step04-resume-owner-green'; cargo test -p sdkwork-im-runtime-link test_runtime_link_decides_incremental_resume_window --offline`
  - `$env:CARGO_TARGET_DIR='target/local-minimal-step04-resume-owner-green'; cargo test -p session-gateway test_presence_runtime_resume_returns_incremental_sync_window_from_runtime_link_owner --offline`
  - `cargo fmt --package sdkwork-im-runtime-link --package session-gateway`
  - `cargo fmt --check --package sdkwork-im-runtime-link --package session-gateway`
  - `$env:CARGO_TARGET_DIR='target/local-minimal-step04-resume-owner-full'; cargo test -p sdkwork-im-runtime-link --offline`
  - `$env:CARGO_TARGET_DIR='target/local-minimal-step04-resume-owner-full'; cargo test -p session-gateway --offline`

### 17.4 审计结论

- `CP04-2`：仍未通过，但继续前推
  - `resume` 纯语义 owner 已开始从 `session-gateway` 迁入 `runtime-link`
  - 这说明 Link Plane hot path 已继续从“握手 owner 回收”推进到“恢复窗口判定 owner 回收
- 本轮仍不能宣告：
  - `session_resume / session_resumed` 控制帧时序已完整迁入 `runtime-link`
  - HTTP/WebSocket accept / upgrade 已迁移到 `runtime-link`
  - 真实 `queue/backpressure` 执行已迁移到 `runtime-link`
- 因此维持总判断：
  - `CP04-3` 通过
  - `CP04-4` 通过
  - `CP04-2` 未通过
  - `91 / 95 / 97` 仍未整体通过
  - `Wave B / 93` 仍不触发

## 18. 2026-04-07 runtime-link goaway owner 审计更新

### 18.1 审计背景

- 当前仍位于 `Wave B / Step 04`
- `resume` 纯语义 owner 已开始迁移到 `runtime-link` 后，`goaway` 仍是 Step 04 文档明确列出的 Link Plane 运行时语
- 当前 websocket CCP 断连路径只发送 close，不发送`goaway` 控制帧，与架构定义不完全一致

### 18.2 本轮新增代码与行为证

- `crates/sdkwork-im-runtime-link/src/lib.rs`
  - 新增 `LinkGoAwayDirective`
  - 新增 `session_disconnect_goaway()`
  - 新增断连 close/goaway 权威常量
    - `SESSION_DISCONNECT_CLOSE_CODE`
    - `SESSION_DISCONNECT_CLOSE_REASON`
    - `SESSION_DISCONNECT_GOAWAY_CODE`
    - `SESSION_DISCONNECT_GOAWAY_MESSAGE`
- `services/session-gateway/src/websocket.rs`
  - 设备断连信号到来时：
    - CCP 模式先发送`ControlFrame::GoAway`
    - 再发送 transport close
  - close code/reason 已改为消费 `runtime-link` owner 常量
- `services/session-gateway/tests/websocket_smoke_test.rs`
  - 新增 `test_realtime_websocket_sends_ccp_goaway_before_disconnect_close`

### 18.3 fail-first 与回归验证

- Red
  - `$env:CARGO_TARGET_DIR='target/local-minimal-step04-goaway-owner-red'; cargo test -p sdkwork-im-runtime-link test_runtime_link_builds_session_disconnect_goaway_owner_contract --offline`
  - `$env:CARGO_TARGET_DIR='target/local-minimal-step04-goaway-owner-red'; cargo test -p session-gateway test_realtime_websocket_sends_ccp_goaway_before_disconnect_close --offline`
- Green
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-goaway-owner'; cargo test -p sdkwork-im-runtime-link test_runtime_link_builds_session_disconnect_goaway_owner_contract --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-goaway-owner'; cargo test -p session-gateway test_realtime_websocket_sends_ccp_goaway_before_disconnect_close --offline`
  - `cargo fmt --check --package sdkwork-im-runtime-link --package session-gateway`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-goaway-owner'; cargo test -p sdkwork-im-runtime-link --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-goaway-owner'; cargo test -p session-gateway --offline`

### 18.4 审计结论

- `CP04-2`：仍未通过，但继续前推
  - `goaway/close` 断连语义已开始从 `session-gateway` 迁入 `runtime-link`
  - 这说明 Step 04 Link Plane hot path 已继续从“恢复窗口 owner 回收”推进到“断连语义 owner 回收
- 本轮仍不能宣告：
  - `session_resume / session_resumed` 控制帧时序已完整迁入 `runtime-link`
  - HTTP/WebSocket accept / upgrade 已迁移到 `runtime-link`
  - 真实 `queue/backpressure` 执行已迁移到 `runtime-link`
- 因此维持总判断：
  - `CP04-3` 通过
  - `CP04-4` 通过
  - `CP04-2` 未通过
  - `91 / 95 / 97` 仍未整体通过
  - `Wave B / 93` 仍不触发

## 19. 2026-04-07 runtime-link session_resume 控制帧owner 审计更新

### 19.1 审计事实

- `crates/sdkwork-im-runtime-link/src/lib.rs`
  - 新增长
    - `LinkSessionResumeDirective`
    - `LinkSessionResumeError`
    - `LinkSession::negotiate_session_resume(...)`
  - 新增测试
    - `test_runtime_link_builds_session_resumed_owner_contract`
- `services/session-gateway/src/websocket.rs`
  - CCP 握手已从
    - `hello -> auth_bind -> auth_ok -> connected`
  - 更新为：
    - `hello -> auth_bind -> auth_ok -> session_resume -> session_resumed -> connected`
  - `catchup_after_seq` 已开始消费 runtime owner 决策结果
- `services/session-gateway/tests/websocket_smoke_test.rs`
  - CCP smoke 已更新为校验证
    - `session_resumed` 控制帧先于 `connected` 业务

### 19.2 fail-first 与回归验证

- Red
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-session-resume-red'; cargo test -p session-gateway test_realtime_websocket_negotiates_ccp_subprotocol_and_wraps_business_frames --offline`
  - 失败点：
    - 期望收到 `control / cc.control.session_resumed.v1`
    - 实际收到 `evt / cc.realtime.connected.v1`
- Green
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-session-resume-green'; cargo test -p sdkwork-im-runtime-link test_runtime_link_builds_session_resumed_owner_contract --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-session-resume-green'; cargo test -p session-gateway test_realtime_websocket_negotiates_ccp_subprotocol_and_wraps_business_frames --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-session-resume-green'; cargo test -p session-gateway test_realtime_websocket_sends_ccp_goaway_before_disconnect_close --offline`
  - `cargo fmt --check --package sdkwork-im-runtime-link --package session-gateway`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-session-resume-full'; cargo test -p sdkwork-im-runtime-link --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-session-resume-full'; cargo test -p session-gateway --offline`

### 19.3 审计结论

- `CP04-2`：仍未通过，但继续前推
  - 这次新增已证明确认
    - `session_resume / session_resumed` 控制帧时序已不再由 `session-gateway` 私有拼装
    - `runtime-link` 已开始作为该时序authority owner
  - 这次仍未证明确认认
    - accept / upgrade 已迁移到 `runtime-link`
    - 真正的 `queue/backpressure` 执行 owner 已迁移到 `runtime-link`
- `CP04-3`：保持通过
- `CP04-4`：保持通过
- `91`：仍未整体通过
- `95`：仍未整体通过
- `97`：继续为部分完成
- `Wave B / 93`：仍不触及
## 20. 2026-04-07 runtime-link accept/upgrade owner 审计更新

### 20.1 审计对象

- `crates/sdkwork-im-runtime-link/Cargo.toml`
- `crates/sdkwork-im-runtime-link/src/lib.rs`
- `services/session-gateway/src/lib.rs`
- `services/session-gateway/src/websocket.rs`

### 20.2 审计结论

- 通过
  - `runtime-link` 已开始直接拥WebSocket upgrade contract
    - `LINK_WEBSOCKET_SUBPROTOCOL`
    - `LinkWebsocketMode`
    - `supported_websocket_subprotocols()`
    - `select_websocket_mode(...)`
  - `session-gateway` `realtime_websocket(...)` 已不再硬编码 `CCP_WS_SUBPROTOCOL` `selected_protocol().is_some()`
  - WebSocket CCP smoke 仍通过，说owner contract 下沉没有破坏现有对外协议行为
- 未通过
  - Axum `on_upgrade(...)` 执行动作仍在 `session-gateway`
  - live `queue/backpressure` 执行 owner 仍在 `session-gateway`
  - 因此 `CP04-2` 仍未整体通过

### 20.3 fail-first fresh 验证

- Red
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-upgrade-owner-red'; cargo test -p session-gateway test_realtime_websocket_upgrade_uses_runtime_link_owner_contract --offline`
  - 失败原因是缺失`realtime_websocket_subprotocols / select_realtime_websocket_mode`，与本轮新增 owner API 精确对应
- Green
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-upgrade-owner-green'; cargo test -p session-gateway test_realtime_websocket_upgrade_uses_runtime_link_owner_contract --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-upgrade-owner-green'; cargo test -p sdkwork-im-runtime-link test_runtime_link_exposes_websocket_upgrade_owner_contract --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-upgrade-owner-green'; cargo test -p session-gateway test_realtime_websocket_negotiates_ccp_subprotocol_and_wraps_business_frames --offline`
  - `cargo fmt --package sdkwork-im-runtime-link --package session-gateway`
  - `cargo fmt --check --package sdkwork-im-runtime-link --package session-gateway`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-upgrade-owner-runtime-full'; cargo test -p sdkwork-im-runtime-link --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-upgrade-owner-session-full'; cargo test -p session-gateway --offline`

### 20.4 风险与残留问题

- 当前只回收了 `accept / upgrade` 的最contract，没有回写transport adapter `on_upgrade(...)`
- 当前也没有触及真实队列缓冲、丢弃、降级、背压控制，因此 `queue/backpressure` 风险仍在
- `CP04-2` 已显著收敛，但距`91 / 95` 通过仍差最后一段热路径 owner

### 20.5 审计判定

- `CP04-2`：不通过
- `CP04-3`：通过
- `CP04-4`：通过
- `91`：本review 证据已补齐，`Step 04` 整体不通过
- `95`：Link Plane owner 未闭环，不通过
- `97`：本轮增量已执行架构回写，但 `Step 04` 整体不通过
- `Wave B / 93`：未触发
## 21. 2026-04-07 runtime-link queue/backpressure batch owner 审计更新

### 21.1 审计对象

- `crates/sdkwork-im-runtime-link/src/lib.rs`
- `services/session-gateway/src/websocket.rs`
- `services/session-gateway/tests/websocket_smoke_test.rs`

### 21.2 审计结论

- 通过
  - `runtime-link` 已开始持live outbound batch/backpressure 的最早 owner contract
    - `LinkOutboundBatchPlan`
    - `plan_stream_batch(...)`
    - `plan_pull_batch(...)`
  - `session-gateway` websocket 热路径已不再硬编catchup / push `100`
  - `events.pull` 已开始通过 `runtime-link` hard limit 执行限流裁决
  - e2e smoke 已证明真实websocket 执行路径开始消费上owner
- 未通过
  - Axum `on_upgrade(...)` 执行 owner 仍在 `session-gateway`
  - 目前回收的是 live batch/limit 裁决，不是完async queue / buffer / drop / degrade controller
  - 因此 `CP04-2` 仍未整体通过

### 21.3 fail-first fresh 验证

- Red
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-queue-owner-red-runtime'; cargo test -p sdkwork-im-runtime-link test_runtime_link_plans_live_outbound_queue_batches_from_owner_limits --offline`
    - 失败原因：缺失`plan_stream_batch / plan_pull_batch`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-queue-owner-red-session'; cargo test -p session-gateway test_realtime_websocket_uses_runtime_link_queue_owner_limits_for_catchup_and_pull --offline`
    - 失败原因：catchup 实际仍为硬编码 `100`
- Green
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-queue-owner-green-runtime'; cargo test -p sdkwork-im-runtime-link test_runtime_link_plans_live_outbound_queue_batches_from_owner_limits --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-queue-owner-green-session'; cargo test -p session-gateway test_realtime_websocket_uses_runtime_link_queue_owner_limits_for_catchup_and_pull --offline`
  - `cargo fmt --package sdkwork-im-runtime-link --package session-gateway`
  - `cargo fmt --check --package sdkwork-im-runtime-link --package session-gateway`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-queue-owner-runtime-full'; cargo test -p sdkwork-im-runtime-link --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-queue-owner-session-full'; cargo test -p session-gateway --offline`

### 21.4 风险与残留问题

- 当前 batch owner 已进入真实发送路径，但还没有完整异步队列实体
- 当前没有回收口
  - backlog 丢弃策略
  - 降级策略
  - 过载断链策略
  - `on_upgrade(...)` transport 执行 owner
- 因此本轮只能认定 `queue/backpressure` 已从“纯配置 owner”推进到“live batch owner”，还不能认定为完整闭环

### 21.5 审计判定

- `CP04-2`：不通过，但继续前推
- `CP04-3`：通过
- `CP04-4`：通过
- `91`：本review 证据完整，但 `Step 04` 整体不通过
- `95`：Link Plane hot path owner 仍未完整闭环，不通过
- `97`：本轮增量已执行架构回写，但 `Step 04` 整体不通过
- `Wave B / 93`：未触发

## 26. 2026-04-07 session-gateway websocket upgrade transport seam 审计更新

### 26.1 审计对象

- `services/session-gateway/src/websocket_upgrade.rs`
- `services/session-gateway/src/lib.rs`
- `services/session-gateway/tests/lib_structure_test.rs`

### 26.2 审计结论

- 通过
  - `session-gateway` 已把 websocket upgrade 相关 helper 收敛到单一 transport seam 模块
  - `services/session-gateway/src/lib.rs` 不再继续私有持有 upgrade helper，实现边界比此前清晰
  - 结构测试已证websocket upgrade seam 已从 `lib.rs` 移出，并建立了防回退约束
- 未通过
  - Axum `WebSocketUpgrade` adapter `.on_upgrade(...)` 入口调用仍保留在 `session-gateway`
  - backlog drop 语义仍未形成 `runtime-link` owner
  - 完整 async queue / buffer 实体仍未形成 `runtime-link` owner
  - 因此 `CP04-2` 仍未整体通过

### 26.3 fail-first fresh 验证

- Red
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-upgrade-seam-red'; cargo test -p session-gateway test_session_gateway_websocket_upgrade_transport_seam_moves_out_of_lib_impl --offline`
    - 失败原因：websocket upgrade helper 仍滞留在 `services/session-gateway/src/lib.rs`
- Green
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-upgrade-seam-green-2'; cargo test -p session-gateway test_session_gateway_websocket_upgrade_transport_seam_moves_out_of_lib_impl --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-upgrade-seam-green-2'; cargo test -p session-gateway test_realtime_websocket_upgrade_uses_runtime_link_owner_contract --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-upgrade-seam-green-2'; cargo test -p session-gateway test_realtime_websocket_upgrade_prepares_runtime_link_handoff_owner --offline`
  - `cargo fmt --package session-gateway`
  - `cargo fmt --check --package session-gateway`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-upgrade-seam-session-full'; cargo test -p session-gateway --offline`

### 26.4 风险与残留问题

- transport seam 虽然被集中到了单一模块，但仍没有证明“服务层保留 Axum adapter 是否就是最终可接受边界：
- 当前 `runtime-link` 仍缺少：
  - backlog drop 权威语义
  - 完整 async queue / buffer 实体
- 因此本轮只能认定“upgrade transport seam 已被收敛并受结构测试保护”，不能认定 `Link Plane` hot path owner 已完整闭环

### 26.5 审计判定

- `CP04-2`：不通过，但边界进一步收口
- `CP04-3`：通过
- `CP04-4`：通过
- `91`：本review 证据完整，但 `Step 04` 整体不通过
- `95`：Link Plane hot path owner 仍未完整闭环，不通过
- `97`：本轮增量已执行架构回写
- `Wave B / 93`：未触发

## 28. 2026-04-07 runtime-link pending backlog 数学 owner 审计更新

### 28.1 审计对象

- `crates/sdkwork-im-runtime-link/src/lib.rs`
- `services/session-gateway/src/websocket.rs`
- `services/session-gateway/tests/lib_structure_test.rs`

### 28.2 审计结论

- 通过
  - `runtime-link` 已开始拥pending backlog 数学 owner
    - `pending_outbound_events(...)`
  - `LinkPushCursor` 内部`session-gateway` catchup / pull 路径现在都消费同一套runtime 公式
  - 结构测试已证明 `services/session-gateway`/src/websocket.rs` 不再保留本地 backlog helper
- 未通过
  - 这只是输入数owner 回收，不backlog drop / queue 实体闭环
  - Axum `WebSocketUpgrade` adapter `.on_upgrade(...)` 入口调用仍保留在 `session-gateway`
  - backlog drop 语义仍未形成 `runtime-link` owner
  - 完整 async queue / buffer 实体仍未形成 `runtime-link` owner
  - 因此 `CP04-2` 仍未整体通过

### 28.3 fail-first fresh 验证

- Red
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-pending-math-red'; cargo test -p session-gateway test_session_gateway_websocket_pending_backlog_math_moves_out_of_service_impl --offline`
    - 失败原因：`services/session-gateway/src/websocket.rs` 仍保留本backlog helper
- Green
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-pending-math-green-runtime'; cargo test -p sdkwork-im-runtime-link test_runtime_link_counts_pending_outbound_events_with_saturating_math --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-pending-math-green-session'; cargo test -p session-gateway test_session_gateway_websocket_pending_backlog_math_moves_out_of_service_impl --offline`
  - `cargo fmt --check --package sdkwork-im-runtime-link --package session-gateway`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-pending-math-runtime-full'; cargo test -p sdkwork-im-runtime-link --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-pending-math-session-full'; cargo test -p session-gateway --offline`

### 28.4 风险与残留问题

- pending backlog 公式虽已统一，但这还没有引入真正async queue / buffer 实体
- 当前仍缺失
  - backlog drop 权威语义
  - 对“单一 Axum transport seam 是否最终可接受”的最终闭环判
- 因此本轮只能认定 queue/controller 输入 authority 更一致，不能认定 `CP04-2` 已闭环

### 28.5 审计判定

- `CP04-2`：不通过，但 authority 继续收敛
- `CP04-3`：通过
- `CP04-4`：通过
- `91`：本review 证据完整，但 `Step 04` 整体不通过
- `95`：Link Plane hot path owner 仍未完整闭环，不通过
- `97`：本轮增量已执行架构回写
- `Wave B / 93`：未触发

## 27. 2026-04-07 session-gateway websocket route handler seam 审计更新

### 27.1 审计对象

- `services/session-gateway/src/lib.rs`
- `services/session-gateway/src/websocket_upgrade.rs`
- `services/session-gateway/tests/lib_structure_test.rs`

### 27.2 审计结论

- 通过
  - `/im/v3/api/realtime/ws` route handler 已从 `lib.rs` 移到 `websocket_upgrade.rs`
  - `services/session-gateway/src/lib.rs` 已不再直接导`WebSocketUpgrade`
  - 结构测试已证`lib.rs` 不再直接保留 websocket route handler upgrade 调用入口
  - `session-gateway` 全包测试通过，说HTTP / websocket 现有路径未因 handler 收敛而回写
- 未通过
  - Axum `WebSocketUpgrade` adapter `.on_upgrade(...)` 入口调用仍保留在 `session-gateway` transport seam 模块
  - backlog drop 语义仍未形成 `runtime-link` owner
  - 完整 async queue / buffer 实体仍未形成 `runtime-link` owner
  - 因此 `CP04-2` 仍未整体通过

### 27.3 fail-first fresh 验证

- Red
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-route-seam-red'; cargo test -p session-gateway test_session_gateway_websocket_route_handler_moves_out_of_lib_impl --offline`
    - 失败原因：`lib.rs` 仍保`WebSocketUpgrade` 导入websocket route handler
- Green
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-route-seam-green'; cargo test -p session-gateway test_session_gateway_websocket_route_handler_moves_out_of_lib_impl --offline`
  - `cargo fmt --package session-gateway`
  - `cargo fmt --check --package session-gateway`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-route-seam-session-full'; cargo test -p session-gateway --offline`

### 27.4 风险与残留问题

- websocket route handler upgrade helper 虽然都已集中`websocket_upgrade.rs`，但这仍只是“服务侧单一 seam”收敛，不等owner 完整回收
- 当前仍缺失
  - backlog drop 权威语义
  - 完整 async queue / buffer 实体
  - 对“Axum adapter 是否就是最终可接受服务边界”的明确闭环判断
- 因此本轮只能认定 `session-gateway` websocket 入口边界更清晰，不能认定 `CP04-2` 已闭环

### 27.5 审计判定

- `CP04-2`：不通过，但边界进一步收口
- `CP04-3`：通过
- `CP04-4`：通过
- `91`：本review 证据完整，但 `Step 04` 整体不通过
- `95`：Link Plane hot path owner 仍未完整闭环，不通过
- `97`：本轮增量已执行架构回写
- `Wave B / 93`：未触发

## 22. 2026-04-07 runtime-link websocket upgrade handoff execute owner 审计更新

### 22.1 审计对象

- `crates/sdkwork-im-runtime-link/src/lib.rs`
- `services/session-gateway/src/lib.rs`
- `services/session-gateway/src/websocket.rs`

### 22.2 审计结论

- 通过
  - `runtime-link` 已开始拥websocket upgrade handoff execute owner
    - `prepare_websocket_upgrade(...)`
    - `LinkWebsocketUpgradeHandoff<TContext>`
    - `LinkWebsocketUpgradeHandoff::execute(...)`
  - `session-gateway` `realtime_websocket(...)` 已不再私有持socket -> runtime serve handoff 执行逻辑
  - handoff execute 仍保`LinkWebsocketMode` 与业务上下文，说owner contract 已从“协议协商”推进到“升级执行移交
- 未通过
  - Axum `WebSocketUpgrade` adapter `.on_upgrade(...)` 调用入口仍在 `session-gateway`
  - 完整 async queue / buffer / drop / degrade / overload controller 仍未回收口`runtime-link`
  - 因此 `CP04-2` 仍未整体通过

### 22.3 fail-first fresh 验证

- Red
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-upgrade-handoff-red-runtime'; cargo test -p sdkwork-im-runtime-link test_runtime_link_prepares_websocket_upgrade_handoff_owner_contract --offline`
    - 失败原因：`prepare_websocket_upgrade` 尚不存在
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-upgrade-handoff-red-session'; cargo test -p session-gateway test_realtime_websocket_upgrade_prepares_runtime_link_handoff_owner --offline`
    - 失败原因：`prepare_realtime_websocket_upgrade` 尚不存在
- Green
  - `cargo test -p sdkwork-im-runtime-link test_runtime_link_prepares_websocket_upgrade_handoff_owner_contract --offline`
  - `cargo test -p session-gateway test_realtime_websocket_upgrade_prepares_runtime_link_handoff_owner --offline`
  - `cargo test -p session-gateway test_realtime_websocket_upgrade_uses_runtime_link_owner_contract --offline`
  - `cargo test -p session-gateway test_realtime_websocket_negotiates_ccp_subprotocol_and_wraps_business_frames --offline`
  - `cargo fmt --package sdkwork-im-runtime-link --package session-gateway`
  - `cargo fmt --check --package sdkwork-im-runtime-link --package session-gateway`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-upgrade-handoff-runtime-full'; cargo test -p sdkwork-im-runtime-link --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-upgrade-handoff-session-full'; cargo test -p session-gateway --offline`

### 22.4 风险与残留问题

- 当前回收的是 websocket upgrade handoff execute owner，不Axum transport adapter 本体
- 当前也没有触及完async queue / buffer / drop / degrade / overload controller，因此高压场景下的真实背压与降级语义仍未闭环
- `CP04-2` 已继续收敛，但距`91 / 95` 放行仍差最后一段热路径 owner

### 22.5 审计判定

- `CP04-2`：不通过
- `CP04-3`：通过
- `CP04-4`：通过
- `91`：本review 证据已补齐，`Step 04` 整体不通过
- `95`：Link Plane hot path owner 仍未完整闭环，不通过
- `97`：本轮增量已完成架构回写，但 `Step 04` 整体不通过
- `Wave B / 93`：未触发

## 23. 2026-04-07 runtime-link live push degrade-to-pull owner 审计更新

### 23.1 审计对象

- `crates/sdkwork-im-runtime-link/src/lib.rs`
- `services/session-gateway/src/websocket.rs`
- `services/session-gateway/tests/websocket_smoke_test.rs`

### 23.2 审计结论

- 通过
  - `runtime-link` 已开始拥live push overload 时的降级裁决 owner
    - `LinkPushMode`
    - `LinkPushPlan`
    - `plan_push_batch(...)`
  - `session-gateway` realtime watch push 路径已不再私有持有“积压过大时是否仍继续自push”的判断
  - smoke test 已证backlog 超过 hard limit 时，连接进入 pull-only 降级，而不是继续自push
- 未通过
  - 当前只回收了 degrade-to-pull owner，没有回收完async queue / buffer 实体
  - backlog drop / overload close 语义仍未形成 runtime owner
  - Axum `WebSocketUpgrade` adapter `.on_upgrade(...)` 入口调用仍在 `session-gateway`
  - 因此 `CP04-2` 仍未整体通过

### 23.3 fail-first fresh 验证

- Red
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-push-degrade-red-runtime'; cargo test -p sdkwork-im-runtime-link test_runtime_link_degrades_live_push_to_pull_only_when_backlog_exceeds_hard_limit --offline`
    - 失败原因为缺失`plan_push_batch / LinkPushMode`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-push-degrade-red-session'; cargo test -p session-gateway test_realtime_websocket_degrades_live_push_to_pull_only_when_runtime_link_detects_overload --offline`
    - 失败原因：gateway overload backlog 下仍继续自动 push
- Green
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-push-degrade-green-runtime'; cargo test -p sdkwork-im-runtime-link test_runtime_link_degrades_live_push_to_pull_only_when_backlog_exceeds_hard_limit --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-push-degrade-green-session'; cargo test -p session-gateway test_realtime_websocket_degrades_live_push_to_pull_only_when_runtime_link_detects_overload --offline`
  - `cargo fmt --package sdkwork-im-runtime-link --package session-gateway`
  - `cargo fmt --check --package sdkwork-im-runtime-link --package session-gateway`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-push-degrade-runtime-full-2'; cargo test -p sdkwork-im-runtime-link --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-push-degrade-session-full-2'; cargo test -p session-gateway --offline`

### 23.4 风险与残留问题

- pull-only 降级已经进入 runtime owner，但 backlog drop / overload close 语义仍未落地
- 当前仍没有真实async queue / buffer 实体，因此高压场景下的完整排队、丢弃、过载断链语义还未闭环
- `CP04-2` 已从“live batch owner”推进到“live push degrade owner”，但距`91 / 95` 通过仍差最后一queue controller owner

### 23.5 审计判定

- `CP04-2`：不通过
- `CP04-3`：通过
- `CP04-4`：通过
- `91`：本review 证据已补齐，`Step 04` 整体不通过
- `95`：Link Plane hot path owner 仍未完整闭环，不通过
- `97`：本轮增量已完成架构回写，但 `Step 04` 整体不通过
- `Wave B / 93`：未触发

## 24. 2026-04-07 runtime-link buffered push recovery after pull owner 审计更新

### 24.1 审计对象

- `crates/sdkwork-im-runtime-link/src/lib.rs`
- `services/session-gateway/src/websocket.rs`
- `services/session-gateway/tests/websocket_smoke_test.rs`

### 24.2 审计结论

- 通过
  - `runtime-link` 已开始拥有“pull 降压后恢buffered push”的 owner contract
    - `LinkBufferedPushPlan`
    - `LinkPushCursor`
    - `LinkSession::start_push_cursor(...)`
  - `session-gateway` realtime watch push 路径已不再要求“等待下一条新发布事件”才能从 `PullOnly` 恢复到继续自push
  - smoke test 已证backlog 超过 hard limit 后先退化为 `pull-only`，随后客户端 pull backlog 降到阈值以下时，已buffered 事件会立即恢复自push
- 未通过
  - 当前只回收了 buffered push recovery owner，没有回收完async queue / buffer 实体
  - backlog drop / overload close 语义仍未形成 runtime owner
  - Axum `WebSocketUpgrade` adapter `.on_upgrade(...)` 入口调用仍在 `session-gateway`
  - 因此 `CP04-2` 仍未整体通过

### 24.3 fail-first fresh 验证

- Red
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-push-buffer-red-runtime'; cargo test -p sdkwork-im-runtime-link test_runtime_link_recovers_buffered_push_after_pull_reduces_backlog_under_hard_limit --offline`
    - 失败原因：`LinkSession::start_push_cursor` 尚不存在
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-push-buffer-red-session'; cargo test -p session-gateway test_realtime_websocket_recovers_buffered_push_after_pull_reduces_backlog_under_hard_limit --offline`
    - 失败原因：pull 降压后没有恢buffered push
- Green
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-push-buffer-green-runtime'; cargo test -p sdkwork-im-runtime-link test_runtime_link_recovers_buffered_push_after_pull_reduces_backlog_under_hard_limit --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-push-buffer-green-session-2'; cargo test -p session-gateway test_realtime_websocket_recovers_buffered_push_after_pull_reduces_backlog_under_hard_limit --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-push-buffer-green-session-2'; cargo test -p session-gateway test_realtime_websocket_degrades_live_push_to_pull_only_when_runtime_link_detects_overload --offline`
  - `cargo fmt --package sdkwork-im-runtime-link --package session-gateway`
  - `cargo fmt --check --package sdkwork-im-runtime-link --package session-gateway`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-push-buffer-runtime-full'; cargo test -p sdkwork-im-runtime-link --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-push-buffer-session-full'; cargo test -p session-gateway --offline`

### 24.4 风险与残留问题

- buffered push recovery 已进runtime owner，但这仍不是完整 async queue / buffer 实体
- 当前仍没有回收：
  - backlog drop 语义
  - overload close 语义
  - Axum `WebSocketUpgrade` adapter `.on_upgrade(...)` transport 入口
- 因此本轮只能认定 `queue/backpressure` 已从“degrade-to-pull owner”继续推进到“buffered push recovery owner”，还不能认定为完整闭环

### 24.5 审计判定

- `CP04-2`：不通过，但继续前推
- `CP04-3`：通过
- `CP04-4`：通过
- `91`：本review 证据完整，但 `Step 04` 整体不通过
- `95`：Link Plane hot path owner 仍未完整闭环，不通过
- `97`：本轮增量已执行架构回写，但 `Step 04` 整体不通过
- `Wave B / 93`：未触发

## 25. 2026-04-07 runtime-link extreme overload close owner 审计更新

### 25.1 审计对象

- `crates/sdkwork-im-runtime-link/src/lib.rs`
- `services/session-gateway/src/websocket.rs`
- `services/session-gateway/src/lib.rs`
- `services/session-gateway/tests/websocket_smoke_test.rs`

### 25.2 审计结论

- 通过
  - `runtime-link` 已开始拥extreme overload close owner contract
    - `LinkPushMode::Disconnect`
    - `LinkPushPlan::disconnect`
    - `realtime_overload_goaway()`
  - websocket 热路径在 runtime owner 裁决`Disconnect` 时，已不再停留在 `PullOnly`，而是主动关闭异常连接
  - smoke test 已证backlog 达到 extreme overload 阈值后，连接会收到 owner 指定close code / reason
- 未通过
  - 当前只回收了 overload close owner，没有回写backlog drop 语义
  - 完整 async queue / buffer 实体仍未形成 runtime owner
  - Axum `WebSocketUpgrade` adapter `.on_upgrade(...)` 入口调用仍在 `session-gateway`
  - 因此 `CP04-2` 仍未整体通过

### 25.3 fail-first fresh 验证

- Red
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-overload-close-red-runtime'; cargo test -p sdkwork-im-runtime-link test_runtime_link_closes_connection_when_backlog_exceeds_overload_disconnect_limit --offline`
    - 失败原因：缺失`Disconnect` 裁决、过载关闭常量和 `disconnect` 指令字段
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-overload-close-red-session'; cargo test -p session-gateway test_realtime_websocket_closes_when_runtime_link_detects_extreme_overload_backlog --offline`
    - 失败原因：缺失`REALTIME_OVERLOAD_CLOSE_CODE / REALTIME_OVERLOAD_CLOSE_REASON` 导出
- Green
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-overload-close-green-runtime-2'; cargo test -p sdkwork-im-runtime-link test_runtime_link_closes_connection_when_backlog_exceeds_overload_disconnect_limit --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-overload-close-green-session-2'; cargo test -p session-gateway test_realtime_websocket_closes_when_runtime_link_detects_extreme_overload_backlog --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-overload-close-green-runtime-recovery'; cargo test -p sdkwork-im-runtime-link test_runtime_link_recovers_buffered_push_after_pull_reduces_backlog_under_hard_limit --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-overload-close-green-session-recovery'; cargo test -p session-gateway test_realtime_websocket_recovers_buffered_push_after_pull_reduces_backlog_under_hard_limit --offline`
  - `cargo fmt --package sdkwork-im-runtime-link --package session-gateway`
  - `cargo fmt --check --package sdkwork-im-runtime-link --package session-gateway`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-overload-close-runtime-full'; cargo test -p sdkwork-im-runtime-link --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-overload-close-session-full'; cargo test -p session-gateway --offline`

### 25.4 风险与残留问题

- overload close 已进runtime owner，但 backlog drop 仍无明确权威语义
- 当前也还没有真实 async queue / buffer 实体，因此“排队、降级、断开”三者只owner 裁决下沉，不是完整队列实现闭环
- `CP04-2` 已从“buffered push recovery owner”继续推进到“extreme overload close owner”，但距`91 / 95` 通过仍差最后一queue/controller 边界

### 25.5 审计判定

- `CP04-2`：不通过，但继续前推
- `CP04-3`：通过
- `CP04-4`：通过
- `91`：本review 证据完整，但 `Step 04` 整体不通过
- `95`：Link Plane hot path owner 仍未完整闭环，不通过
- `97`：本轮增量已执行架构回写，但 `Step 04` 整体不通过
- `Wave B / 93`：未触发
## 26. 2026-04-07 runtime-link outbound queue state owner audit update

### 26.1 audit scope

- `crates/sdkwork-im-runtime-link/src/lib.rs`
- `services/session-gateway/src/websocket.rs`
- `services/session-gateway/tests/lib_structure_test.rs`

### 26.2 audit result

- pass items
  - `runtime-link` now owns the combined outbound queue state instead of only disconnected queue math fragments
  - `session-gateway` websocket hot path no longer keeps local mutable `last_sent_seq` and `push_cursor`
  - targeted queue-state tests and full package tests for `sdkwork-im-runtime-link` and `session-gateway` are green
- not-yet-pass items
  - `CP04-2` is still not passed overall because transport adapter ownership and backlog drop semantics remain outside `runtime-link`

### 26.3 fresh evidence

- Red
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-queue-state-red-session'; cargo test -p session-gateway test_session_gateway_websocket_outbound_queue_state_moves_out_of_service_impl --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-queue-state-red-runtime'; cargo test -p sdkwork-im-runtime-link test_runtime_link_outbound_queue_state_owns_last_sent_seq_and_buffered_push_recovery --offline`
- Green
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-queue-state-green-runtime'; cargo test -p sdkwork-im-runtime-link test_runtime_link_outbound_queue_state_owns_last_sent_seq_and_buffered_push_recovery --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-queue-state-green-session'; cargo test -p session-gateway test_session_gateway_websocket_outbound_queue_state_moves_out_of_service_impl --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-runtime-full'; cargo test -p sdkwork-im-runtime-link --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-session-full'; cargo test -p session-gateway --offline`
  - `rustfmt --edition 2024 crates/sdkwork-im-runtime-link/src/lib.rs services/session-gateway/src/websocket.rs services/session-gateway/tests/lib_structure_test.rs`
  - `rustfmt --edition 2024 --check crates/sdkwork-im-runtime-link/src/lib.rs services/session-gateway/src/websocket.rs services/session-gateway/tests/lib_structure_test.rs`

### 26.4 residual risk

- this increment does not yet settle whether Axum `WebSocketUpgrade` / `.on_upgrade(...)` is the final allowed transport seam boundary
- this increment still does not introduce a real runtime-owned backlog drop contract
- because of those two remaining gaps, `Step 04` cannot be closed even though the combined outbound queue state owner is now in `runtime-link`

### 26.5 audit decision

- `CP04-2`: not passed
- `CP04-3`: passed
- `CP04-4`: passed
- `91`: evidence for this increment is complete, but `Step 04` overall is still not passed
- `95`: not passed overall
- `97`: increment writeback is complete, but `Step 04` overall is still not passed
- `Wave B / 93`: not triggered

## 27. 2026-04-07 runtime-link stale replay clamp audit update

### 27.1 audit scope

- `crates/sdkwork-im-runtime-link/src/lib.rs`
- `services/session-gateway/src/websocket.rs`
- `services/session-gateway/tests/websocket_smoke_test.rs`

### 27.2 audit result

- pass items
  - `runtime-link` now owns one truthful overload-time backlog-drop rule for stale pull replay
  - the websocket smoke test proves that after catchup advances the sent frontier to `128`, an overloaded stale `afterSeq: 0` replay request no longer rewinds to seq `1`
  - full package tests confirm that non-overloaded rewind behavior and earlier Step 04 slices remain green
- not-yet-pass items
  - `CP04-2` is still not passed overall because the transport adapter boundary and a complete runtime-owned async queue / buffer entity remain outside `runtime-link`

### 27.3 fresh evidence

- Red
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-stale-pull-red-runtime'; cargo test -p sdkwork-im-runtime-link test_runtime_link_drops_stale_pull_replay_when_backlog_is_still_over_hard_limit --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-stale-pull-red-session'; cargo test -p session-gateway test_realtime_websocket_clamps_stale_pull_replay_when_backlog_is_still_over_hard_limit --offline`
- Green
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-stale-pull-green-runtime'; cargo test -p sdkwork-im-runtime-link test_runtime_link_drops_stale_pull_replay_when_backlog_is_still_over_hard_limit --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-stale-pull-green-session'; cargo test -p session-gateway test_realtime_websocket_clamps_stale_pull_replay_when_backlog_is_still_over_hard_limit --offline`
  - `rustfmt --edition 2024 crates/sdkwork-im-runtime-link/src/lib.rs services/session-gateway/tests/websocket_smoke_test.rs`
  - `rustfmt --edition 2024 --check crates/sdkwork-im-runtime-link/src/lib.rs services/session-gateway/tests/websocket_smoke_test.rs`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-stale-pull-runtime-full'; cargo test -p sdkwork-im-runtime-link --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-stale-pull-session-full'; cargo test -p session-gateway --offline`

### 27.4 residual risk

- this increment intentionally does not invent trim or deletion semantics for durable event history
- the new owner rule only clamps stale replay requests under overload; it does not create a full async queue implementation
- the final status of the Axum `WebSocketUpgrade` / `.on_upgrade(...)` transport seam is still unresolved
- because of those remaining gaps, `Step 04` still cannot be closed

### 27.5 audit decision

- `CP04-2`: not passed
- `CP04-3`: passed
- `CP04-4`: passed
- `91`: evidence for this increment is complete, but `Step 04` overall is still not passed
- `95`: not passed overall
- `97`: increment writeback is complete, but `Step 04` overall is still not passed
- `Wave B / 93`: not triggered

## 28. 2026-04-07 runtime-link buffered push drain loop audit update

### 28.1 audit scope

- `crates/sdkwork-im-runtime-link/src/lib.rs`
- `services/session-gateway/src/websocket.rs`
- `services/session-gateway/tests/lib_structure_test.rs`

### 28.2 audit result

- pass items
  - `runtime-link` now owns the buffered push async drain loop instead of only owning queue math and queue state fragments
  - `session-gateway` no longer keeps `async fn flush_buffered_push_windows(...)` in `src/websocket.rs`
  - the gateway now only adapts runtime fetch/send concerns through `BufferedPushDrainDriver`
  - targeted runtime and structure tests plus fresh full-package tests are green
- not-yet-pass items
  - `CP04-2` is still not passed overall because the final status of the Axum `WebSocketUpgrade` / `.on_upgrade(...)` transport seam is still unresolved

### 28.3 fresh evidence

- Red
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-buffer-drain-red-session'; cargo test -p session-gateway test_session_gateway_websocket_buffered_push_drain_loop_moves_out_of_service_impl --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-buffer-drain-red-runtime'; cargo test -p sdkwork-im-runtime-link test_runtime_link_drains_buffered_push_windows_via_owner_async_loop --offline`
- Green
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-buffer-drain-green-runtime'; cargo test -p sdkwork-im-runtime-link test_runtime_link_drains_buffered_push_windows_via_owner_async_loop --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-buffer-drain-green-session'; cargo test -p session-gateway test_session_gateway_websocket_buffered_push_drain_loop_moves_out_of_service_impl --offline`
  - `rustfmt --edition 2024 crates/sdkwork-im-runtime-link/src/lib.rs services/session-gateway/src/websocket.rs services/session-gateway/tests/lib_structure_test.rs`
  - `rustfmt --edition 2024 --check crates/sdkwork-im-runtime-link/src/lib.rs services/session-gateway/src/websocket.rs services/session-gateway/tests/lib_structure_test.rs`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-buffer-drain-runtime-full'; cargo test -p sdkwork-im-runtime-link --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-buffer-drain-session-full'; cargo test -p session-gateway --offline`

### 28.4 residual risk

- this increment removes the service-local buffered push drain loop, but it intentionally does not settle whether the remaining Axum transport adapter is the final acceptable boundary
- because that boundary is still not explicitly accepted by `91 / 95 / 97`, `Step 04` still cannot be closed

### 28.5 audit decision

- `CP04-2`: not passed
- `CP04-3`: passed
- `CP04-4`: passed
- `91`: evidence for this increment is complete, but `Step 04` overall is still not passed
- `95`: not passed overall
- `97`: increment writeback is complete, but `Step 04` overall is still not passed
- `Wave B / 93`: not triggered

## 29. 2026-04-07 Axum transport seam boundary audit update

### 29.1 audit scope

- `services/session-gateway/src/websocket_route.rs`
- `services/session-gateway/src/websocket_upgrade.rs`
- `services/session-gateway/src/lib.rs`
- `services/session-gateway/tests/lib_structure_test.rs`

### 29.2 audit result

- pass items
  - websocket route preflight is now separated from the Axum adapter into `services/session-gateway/src/websocket_route.rs`
  - `services/session-gateway/src/websocket_upgrade.rs` now keeps only the retained framework seam:
    - `WebSocketUpgrade`
    - subprotocol offer
    - runtime-link handoff prepare
    - `.on_upgrade(...)`
    - transport-to-runtime websocket bridge
  - the new structure test locks that boundary so the gateway cannot silently grow the mixed hot path back into the adapter module
  - fresh full-package verification for `session-gateway`, `sdkwork-im-runtime-link`, `sdkwork-im-runtime-route`, and `sdkwork-im-server` is green
- pass judgement
  - this is sufficient Step 04 evidence that Axum `WebSocketUpgrade` / `.on_upgrade(...)` is the final acceptable retained transport seam
  - the remaining Step 04 boundary is now framework-local transport only, not business hot-path ownership

### 29.3 fresh evidence

- Red
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-axum-adapter-red-2'; cargo test -p session-gateway test_session_gateway_websocket_upgrade_module_stays_pure_axum_adapter --offline`
- Green
  - `rustfmt --edition 2024 services/session-gateway/src/websocket_route.rs services/session-gateway/src/websocket_upgrade.rs services/session-gateway/src/lib.rs services/session-gateway/tests/lib_structure_test.rs`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-axum-adapter-green-2'; cargo test -p session-gateway test_session_gateway_websocket_upgrade_module_stays_pure_axum_adapter --offline`
  - `rustfmt --edition 2024 --check services/session-gateway/src/websocket_route.rs services/session-gateway/src/websocket_upgrade.rs services/session-gateway/src/lib.rs services/session-gateway/tests/lib_structure_test.rs`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-session-full-axum-boundary'; cargo test -p session-gateway --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-runtime-link-full-axum-boundary'; cargo test -p sdkwork-im-runtime-link --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-runtime-route-full-axum-boundary'; cargo test -p sdkwork-im-runtime-route --offline`
  - `$env:CARGO_TARGET_DIR='C:\\Users\\admin\\.codex\\memories\\target-step04-local-node-full-axum-boundary'; cargo test -p sdkwork-im-cloud-gateway --offline`

### 29.4 residual risk

- no remaining Step 04-specific residual gap is left open after this boundary acceptance
- follow-on risk has moved to `Step 05`, not to an unresolved `Step 04` hot path

### 29.5 audit decision

- `CP04-2`: passed
- `CP04-3`: passed
- `CP04-4`: passed
- `91`: passed overall for `Step 04`
- `95`: passed overall for `Step 04`
- `97`: passed overall for `Step 04`
- `Step 04`: closed
- `Wave B / 93`: not triggered because `Wave B` still has open `Step 05` and `Step 06`

