# IM 通信功能审查报告

**最后更新**: 2026-07-07  
**范围**: IM 核心服务端、PC/H5/Flutter 客户端、Console/Admin 运营面  
**状态**: 实现对齐进行中 — P0 DDL/分页/安全/OOM 债务已收口；HTTP/WS 为主路径；gRPC Phase 1 部分托管；商业化 Pre-Release 仍需 `check:commercial-readiness` 以真实签名、SBOM、provenance、checksum 和 catalog media 证据通过后才能声明就绪

---

## 2026-07-07 修复清单

| ID | 问题 | 修复 |
|---|---|---|
| DDL-001 | PostgreSQL DDL 重复表定义（Migration 001 覆盖 010） | 移除旧 Migration 001，保留 010 organization_id 版本 |
| DDL-002 | SQLite DDL 使用 PostgreSQL 专有语法（JSONB/TIMESTAMPTZ/DO $$/pg_constraint） | 生成 SQLite 兼容 DDL（TEXT + json_valid CHECK） |
| PAG-039 | social-service block 列表使用 OFFSET 分页 | 改为 keyset `(created_at DESC, block_id DESC)` |
| PAG-040 | social-service direct_chat 列表使用 OFFSET 分页 | 改为 keyset `(updated_at DESC, direct_chat_id DESC)` |
| SEC-001 | 游标签名密钥硬编码回退 | 添加 `_FILE` 变体支持，移除硬编码密钥，fail-closed |
| OOM-001 | timeline 全量恢复无上限 | 添加 10,000 条安全上限 |
| PERF-001 | 会话驱逐 O(n log n) 全量排序 | 改为 `select_nth_unstable_by_key` O(n) |

---

## 当前基线能力

| 域 | 能力 |
|---|---|
| 消息 | `message_seq` + `commit_seq`；Outbox at-least-once |
| 连接 | Pre-auth WS 预算；认证后正式槽；帧/升级 RPM 限流 |
| Social / Space | PG materialize-before-append；多 commit 单 PG 事务 |
| Projection | HS256 keyset 分页；timeline 热缓存 cap；embedded apply 生产 fail-closed |
| Portal API | `portal-service` + `im-portal-snapshots` 从 ops 健康面与 audit 记录聚合 |
| PC 客户端 | 游标分页；SQLite 离线缓存 + claim/lease 待发队列 |
| H5 客户端 | 游标分页；IndexedDB 离线待发队列 + claim/lease |
| Flutter | Inbox 游标多页同步 + `shared_preferences` 离线待发（claim/lease） |
| 数据库 | IM 核心 **PostgreSQL-only**；SQLite 为契约 parity + 桌面/网关缓存 |
| Realtime | 生产 fail-closed（PG pool + membership gate）；canonical mutex 锁序 |

---

## 数据库引擎边界

| 表面 | PostgreSQL | SQLite |
|---|---|---|
| Journal / Projection / Social | ✅ 唯一权威 | ❌ 不持久化 |
| Notification / Automation 任务表 | ✅ Postgres store | DDL parity only |
| 桌面离线缓存 | — | ✅ Tauri `offline_store`（WAL + 事务 + claim） |
| Gateway webstore | — | ✅ `chat.sqlite` |

---

## 后端服务边界

| 服务 | 职责 | 持久化 |
|---|---|---|
| `portal-service` | Console/Workspace portal 快照 HTTP | 无状态；读 ops/audit |
| `sdkwork-comms-conversation-service` | 会话写路径 + RPC unary | PostgreSQL journal；`spawn_blocking` 写路径 |
| `sdkwork-comms-conversation-service` | Inbox/timeline 规范化读写模型 | PostgreSQL + 有界热缓存 |
| `session-gateway` | WebSocket + RPC realtime | Redis/PG realtime stores；生产 fail-closed |
| `im-calls-service` | RTC 信令 HTTP | Postgres/Redis durable state；`spawn_blocking` handlers |
| `audit-service` | 审计记录 | PostgreSQL；生产 fail-closed panic |

---

## RPC 与客户端路径

详见 [`RPC-AVAILABILITY.md`](architecture/tech/RPC-AVAILABILITY.md)。

- **生产客户端**：HTTP app-sdk + WebSocket（session-gateway）
- **gRPC Phase 1**：`PresenceService`、`RealtimeService`、conversation unary（3 个 rpc-bin）
- **未托管 RPC**：Contact/Social/Call/Notification/Automation/admin — 使用 HTTP，勿调用 gRPC stub

---

## 明确未集成（非 IM 核心上线阻塞）

| 项 | 说明 |
|---|---|
| E2EE | 仅 TLS；见 `OPTIMIZATION_ROADMAP.md` Phase 2 |
| FEC 自适应弱网 | Phase 2；ARQ `events.nack` 已交付 |
| Conversation RPC server-stream | 实时流在 session-gateway WS |
| Telegram 级 200K 群 | 当前 10K cap |
| Flutter 离线 SQLite | Inbox 游标分页 + SharedPreferences 待发 claim/lease 已交付；完整 SQLite 消息缓存 Phase 2 |

---

## 验证命令

```bash
node ../sdkwork-specs/tools/check-pagination.mjs --workspace .
node ../sdkwork-specs/tools/check-api-response-envelope.mjs --workspace .
node scripts/dev/sdkwork-im-production-security-standard.test.mjs
cargo check -p session-gateway -p im-calls-service -p sdkwork-comms-conversation-service
pnpm run check:commercial-readiness
```

---

## 行业对齐摘要

| 能力 | 状态 |
|------|------|
| Per-channel seq + journal seq | ✅ |
| Outbox at-least-once | ✅ |
| PG materialize + 单事务多 commit | ✅ |
| 核心列表 keyset 分页 + SdkWorkPageData | ✅ |
| 生产 fail-closed（audit/conversation/session-gateway） | ✅ |
| 桌面 + H5 + Flutter 离线待发 claim/lease | ✅ |
| gRPC 全 manifest 托管 | ❌ Phase 2 |
| E2EE / FEC Phase 2 / 超大群 | 📋 路线图 |
