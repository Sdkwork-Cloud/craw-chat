# IM 三能力对齐设计（弱网 / 万人群 / 桌面离线）

**Status:** Phase 1 MVP **shipped** (2026-07-06)  
**Updated:** 2026-07-06  
**Owner:** SDKWork IM Team

## 目标

对齐行业产品三项能力。Phase 1 MVP 已交付；Phase 2 为产品级增量（FEC 自适应、Telegram 200K cascade）。

| 能力 | Phase 1（已交付） | Phase 2 |
|------|------------------|---------|
| 弱网优化 | CCP `events.nack` + 服务端 ARQ 重放（IM SDK / session-gateway / runtime-link） | FEC + `NetworkQualityEstimator` 自适应冗余 |
| 万人群 | 微信级 10K（`DEFAULT_CHAT_GROUP_MAX_MEMBERS` + `member_directory_by_scope` 索引分页） | Telegram 200K cascade / outbox 分片 |
| 桌面离线 | Tauri SQLite 缓存 + ChatService 读回退 + 文本/媒体（Drive 已上传）待发队列 + 重连 flush | 离线草稿编辑冲突合并 |

## 1. 弱网优化

### Phase 1（已交付）

- IM SDK：`createRealtimeSeqTracker` + `sendEventsNack`
- `sdkwork-im-runtime-link`：`plan_nack_replay` / `rewind_for_nack`
- `session-gateway`：`events.nack` 业务帧处理，复用 pull 窗口重放

### Phase 2

- CCP `RealtimeNackFrame` 控制帧（可选）
- `im-domain-core::network_optimization` FEC 接入 call signaling / 关键 realtime 帧
- `NetworkQualityEstimator` 驱动 batch size / parity 冗余

## 2. 万人群（10K）

### Phase 1（已交付）

- `DEFAULT_CHAT_GROUP_MAX_MEMBERS = 10_000`
- `ProjectionMemberRuntimeStore::member_directory_by_scope` + `collect_member_directory_window`
- 成员目录 RPC/HTTP 列表走索引窗口，无全量 roster collect

### Phase 2（Telegram 200K）

- Outbox recipient 分片 + cascade 节点
- 大群只推摘要，消息按需 SQL keyset

## 3. 桌面离线持久化

### Phase 1（已交付）

- `offline_store.rs`：`offline_conversations`、`offline_messages`、`offline_sync_cursors`、`offline_pending_sends`
- Tauri invoke：缓存 CRUD + sync cursor + pending send enqueue/list/delete
- `sdkwork-im-pc-core`：`desktopOfflineChatCache` + `desktopOfflineSendQueue`
- `ChatService`：在线写入缓存；离线读回退；可重试发送失败入队；WebSocket 重连 hydrate + flush
- PC UI：`Message.sendState`（`pending` / `failed`）+ `MessageList` 指示器 + `retryFailedMessage` 文本重试

### 边界

- IM 服务端仍 PostgreSQL-only；桌面 SQLite 为 **客户端缓存/待发队列**，非 journal 权威

## 4. 投影/RPC 对齐（2026-07-06）

- `rpc_projection_dispatch.rs`：8 个 projection-owned app RPC 操作已接通
- `message_favorites_index`：收藏列表（含 `favorite_type` / `q` 过滤）走索引扫描，无全 principal collect

## 验收

| 能力 | 验收 |
|------|------|
| 弱网 | `cargo test -p sdkwork-im-runtime-link nack`；`node scripts/dev/sdkwork-im-three-capabilities-standard.test.mjs` |
| 万人群 | `cargo test -p projection-service member_directory` |
| 桌面离线 | `cargo test offline_store --lib`（`sdkwork-im-pc` src-tauri）；治理脚本 pending send 断言 |
| 治理 | `pnpm run test:three-capabilities-standard` |
