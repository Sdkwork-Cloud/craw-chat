# IM 通信功能审查报告

**最后更新**: 2026-07-06  
**范围**: 好友 / 群组 / 会话 / 消息 / RTC / 连接 / Projection / Space / PC/H5 客户端  
**状态**: 预上线基线就绪（分页/并发/内存债务已清理）

---

## 当前基线能力

| 域 | 能力 |
|---|---|
| 消息 | `message_seq` + 独立 `commit_seq`；Outbox at-least-once；写入侧无 deliverable recipients 不入队；relay fail-closed |
| 连接 | Pre-auth WS 预算 + 认证后正式槽；Upgrade / Frame RPM（Redis + local fallback） |
| Social | PG materialize → journal append → compensate；**多 commit 单 PG 事务**；终端幂等 replay；keyset 分页 |
| Space/Group | PG materialize-before-append；**多 commit 单 PG 事务**（space 创建批次）；journal compensate |
| RTC | Ring timeout reaper；终端 evict + DB retention；IM 信令 + `@sdkwork/rtc-sdk` 媒体；PC 远端视频 Volcengine 绑定 |
| Projection | Postgres tiered timeline + hot cache；HS256 keyset cursor；生产拒绝 numeric offset |
| PC 客户端 | 游标分页单源 + `pageSize=20` 默认；catch-up 有页数上限；mail/shop/devices 服务端分页 + UI load-more |
| 构建 | `pnpm install --frozen-lockfile`；sibling composed SDK workspace 对齐 |
| 推送 | APNs HTTP/2 JWT |

---

## 架构要点

### 消息与 Outbox

- **message_seq**: 业务序号（Postgres/Redis 原子）
- **commit_seq**: Journal `ordering_seq`（与会话内 message_seq 解耦）
- **Aggregate 恢复**: `PostgresAggregateStore.load_aggregate_state` 从 `im_conversation_messages` 读取真实 `high_watermark`
- **写入侧**: social / conversation / rtc 在无法解析 deliverable recipients 时不创建 outbox 行
- **Relay**（conversation / social / rtc，共享 `outbox_relay_common`）:
  - 空收件人 → `mark_failed`
  - 非本域 aggregate → `mark_failed`
  - 发布失败 → `mark_failed`（可重试）

### Social

- **写路径**: PG materialize → journal append → compensate on failure
- **多 commit 批次**: 2+ commit 走单 PG 事务
- **读路径 / 限流 / cursor**: 生产 fail-closed
- **补充 Postgres 路由**: block / direct_chat 变更类接口 fail-closed，走事件溯源 `/im/v3/api/social`

### Space / Group

- **写路径**: PG materialize → journal append → compensate on failure
- **多 commit 批次**: space 创建等 2+ commit 走 `materialize_space_commits_in_transaction`
- **单 commit**: `group.created` 等仍走 store 内建事务（`insert_with_owner_member`）

### Projection

- **Timeline tier**: Postgres `load_timeline_window` + hot cache（`SDKWORK_IM_PROJECTION_TIMELINE_MEMORY_CAP`，默认 1000）
- **无 durable store**: dev/test 不限 cap；production/staging 强制默认 cap
- **列表 cursor**: contacts / inbox / member_directory / pins / favorites 使用 HS256

### RTC（IM 信令 + sdkwork-rtc 媒体）

- **信令**: `im-calls-service` + IM SDK `calls` facade
- **媒体**: PC `RtcMediaService` 经 Volcengine provider 绑定本地/远端视频 DOM
- **边界**: `pnpm test:rtc-signaling-boundary` 禁止 IM 仓库内嵌信令栈

### 未集成（明确非阻塞上线）

| 项 | 说明 |
|---|---|
| FEC + ARQ | `im-domain-core::network_optimization` 库代码，**未接入** WS 运行时 |
| E2EE | 仅 TLS；无 Signal/Double Ratchet |
| 超大群外置 timeline | Postgres tier + cap；Scylla 分片为 post-launch |

---

## 验证命令

```bash
cargo test -p im-domain-core rtc_outbox --lib
cargo test -p im-adapters-postgres-journal --lib
cargo test -p social-service -p calls-service -p session-gateway --lib
node scripts/dev/sdkwork-im-retention-enforcement-standard.test.mjs
node scripts/dev/sdkwork-im-projection-tier-standard.test.mjs
node scripts/dev/sdkwork-im-social-materializer-standard.test.mjs
node scripts/dev/sdkwork-im-space-materializer-standard.test.mjs
node scripts/dev/sdkwork-im-monorepo-frozen-install-standard.test.mjs
node scripts/dev/sdkwork-im-pc-client-pagination-standard.test.mjs
node scripts/dev/sdkwork-im-apis-authority-standard.test.mjs
node scripts/dev/sdkwork-im-database-naming-standard.test.mjs
node scripts/dev/sdkwork-im-runtime-id-standard.test.mjs
node scripts/dev/sdkwork-im-rtc-signaling-boundary.test.mjs
node ../sdkwork-specs/tools/check-api-response-envelope.mjs --workspace .
node ../sdkwork-specs/tools/check-pagination.mjs --workspace .
node apps/sdkwork-im-pc/scripts/community-app-sdk-integration-contract.test.mjs
node scripts/dev/sdkwork-im-pc-sidebar-module-sdk-boundary.test.mjs
pnpm run check:commercial-readiness
```

---

## 行业对齐

| 能力 | 状态 |
|------|------|
| Per-channel 单调 seq + 独立 journal seq | ✅ |
| Post-auth WS 配额 + pre-auth 预算 | ✅ |
| Outbox at-least-once（三域 + 写入侧过滤） | ✅ |
| Social PG materialize + compensate + 多 commit 单事务 | ✅ |
| Space PG materialize + 多 commit 单事务 + compensate | ✅ |
| RTC 信令 + Volcengine 远端视频渲染 | ✅ |
| Presence stale 自动过期 | ✅ |
| Projection 核心列表 keyset 分页 | ✅ |
| Timeline tiered storage + 生产内存 cap | ✅ |
| PC 客户端游标分页 + 内存硬上限 | ✅ |
| Community 产品逻辑归属 `sdkwork-community`（feeds / comments / reactions；IM 仅网关代理 + `@sdkwork/im-pc-community` 宿主适配） | ✅ |
| OpenAPI 列表 wire 使用 `pageSize`（`limit` 标记 deprecated） | ✅ |
| IM 核心 PostgreSQL-only；sqlite baseline 无 PG 搜索伪实现 | ✅ |
| Monorepo frozen install + composed SDK workspace | ✅ |
| Social 多 commit 单 PG 事务 materialize | ✅ |
| FEC+ARQ / E2EE / 超大群外置存储 | 📋 见 OPTIMIZATION_ROADMAP.md |

---

## 跨仓库依赖（商业化门禁）

| 依赖 | 用途 |
|------|------|
| `../sdkwork-rtc` | RTC 媒体 SDK + `.sdkwork-assembly.json` |
| `../sdkwork-utils` | `@sdkwork/utils`（`ResultValue` 类型守卫） |
| `../sdkwork-iam` | PC 认证壳（`@sdkwork/auth-pc-react`） |
| `../sdkwork-agents` | PC Agent 模块（`@sdkwork/agents-app-sdk`） |
| `../sdkwork-catalog` / `shop` / `order` | PC 商城/订单 T1 app SDK |
| `../sdkwork-course` | PC 课程模块 |
| `../sdkwork-community` | Community 产品（`@sdkwork/community-pc-community` + app SDK）；IM 经网关 `/app/v3/api/community/*` 集成 |
| `../sdkwork-mail` | Mail 列表服务端分页（OpenAPI `SdkWorkListQuery`） |

`pnpm run check:commercial-readiness` 串联 frozen install、PC/H5 lint+build、治理标准测试（含 `rtc-signaling-boundary`、`monorepo-frozen-install`、`pc-client-pagination`、`social-materializer`）及 SDK 集成契约。跨仓库 TypeScript 错误会阻塞 `pc-lint`，须在对应 sibling 仓库修复（非 IM 仓库内 stub）。
