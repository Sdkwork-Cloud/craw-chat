# SDKWork IM 2026-08 全面审计与修复记录

**审计日期**: 2026-08-12
**审计范围**: 全仓库（Rust 服务、前端 PC/H5/Flutter、数据库、API/SDK、安全、性能）
**方法**: 8 个并行深度审计 + 官方验证工具 + 关键缺陷逐条复核 + 全量商业化 gate 验证

---

## 验证结果

- `check-pagination.mjs` / `check-api-response-envelope.mjs`：通过
- 租户隔离静态扫描（676 个 Rust 文件）：0 缺口
- OpenAPI 路由覆盖：0 缺失
- `cargo check --workspace` + 核心服务测试：通过
- **商业化就绪检查：52/52 gate 通过（0 失败）**；剩余发布证据门禁（Pre-Release/Capacity/AppRelease）需真实 staging 与容量运行（非代码缺陷）
- `workflow-commercial-gates`：83/83 通过

## 已修复问题清单

### 构建与契约（P0）
1. **H5 构建断链**：补 `apps/sdkwork-im-h5/specs/topology.spec.json`（H5 build 通过）
2. **PC tsconfig paths**：补齐 11 个 SDK paths 条目（auth/company/catalog/shop/order/membership/mail/drive/notary/kb/rtc/im-sdk/im-app-sdk/im-backend-sdk/utils）
3. **three-capabilities 断言**：同步为实际表名 `im_local_message_cache`/`im_local_pending_send`
4. **IAM 启动顺序**：网关启动先 `bootstrap_iam_database_from_env()` 再 tenant application bootstrap（原顺序会导致 IAM schema 未就绪时 tenant 静默跳过）
5. **契约测试对齐**：13 个 gate（drive/notary/kb/commerce/mail/auth/im-api/sdk-integration/dev-command/sidebar/portal/topology-baggage/three-capabilities）全部修复

### 前端假数据清零（P0）
6. H5 账单页（`BillingRecordsPage`）硬编码假账单 → fail-closed
7. H5 豆豆充值假成功（`ProfileAssets`）→ 移除，`ProfileService.updateUserProfile` 未接入字段抛 typed error
8. H5 假地址簿/假游戏中心（`ProfileAssets`/`GamesPage`）→ fail-closed
9. PC 日历假成功 toast（`CalendarSidebar`）→ 移除；`CalendarService` 读接口静默空 → fail-closed
10. H5 enterprise 假企业目录、recruitment 假数据 → 统一 fail-closed（本地包 + company 包共 25 个页面）
11. PC/company recruitment 页面假统计（`RecruitmentHeader ongoingCount={12}` 等）→ fail-closed
12. H5 `ChatActionPanel` 硬编码示例媒体 URL → 真实文件选择 + fail-closed 提示
13. `SettingsService` localStorage 假实现 → fail-closed（PRD：settings 无 owner SDK 不持久化）
14. `KnowledgeBaseService` localStorage 空间注册表 → 进程内存（服务器仍为权威）
15. `ProfileService`/`WorkService`/`ProductService`/`LifeService` 统一 `UserCapabilityUnavailableError`

### 消息链路可靠性（P0）
16. **message.posted 虚假兜底修复**：post 事务内始终写 outbox 记录；直发成功 `mark_published_direct`（relay 不重复），失败由 relay 兜底（事件不再静默丢失）；新增 `OutboxStore::mark_published_direct`

### 内存与 OOM（P0）
17. **C1 会话缓存无淘汰**：`ConversationStateService` 增加 LRU 访问跟踪 + `evict_idle_conversations`（按写入后检查触发），清除 17 个派生映射
18. **C2 delivery offers 累积**：FIFO 上限 100k（`MESSAGE_DELIVERY_OFFER_INDEX_CAP`）

### 安全 fail-closed（P0）
19. **S1 免签 token**：dev/test 环境外还需显式 `SDKWORK_IM_ALLOW_UNSIGNED_TOKENS=true` 才接受 raw JSON/key=value token
20. **S2 内部 RPC 签名**：`SDKWORK_IM_APP_CONTEXT_REQUIRE_SIGNATURE` 缺省改为 true（fail-closed）
21. **M4 环境缺省**：网关 `resolve_environment()` 缺省改为 `production`（原缺省 development 会静默降级安全策略）

### 数据库（P0/P1）
22. **baseline 漂移**：`im_notification_tasks` 的 `attempt_count`/`available_at` 列与 claim 索引折入 baseline（原只在 migration，baseline-only 环境启动即故障）
23. **审计 WORM 角色**：补 `deployments/database/postgres/roles.sql`（`im_audit_writer` INSERT/SELECT-only）
24. **31 个重复索引**：baseline 中去重（保留 tenant 前缀权威版本）

### API/认证契约（P1）
25. **App/Backend 权威源**：升级为 `x-sdkwork-auth-mode: api-key-or-dual-token` + ApiKey 双 security 方案；`prepare-openapi-source.mjs` 传 `authProfile`；SDK 家族重新生成并验证通过
26. **workflow 依赖清单**：注册 `sdkwork-company`（Cargo/tsconfig/workflow.yml/package.yml）

### 跨仓库修复
27. **sdkwork-company**：`index.tsindex.ts` 重命名事故（recruitment 导出合并）；`company-pc-company` exports 指向不存在的 index.ts；`CompanyPcHostProvider` 导出；recruitment/enterprise 页面 fail-closed；`EnterprisePostJob` headcount 字段
28. **sdkwork-iam**：`wrapCredentialEntryClient` 公共 API（credential-entry bootstrap token 包装）
29. **sdkwork-knowledgebase / sdkwork-order**：CapabilityUnavailablePage 兜底组件与页面

### 文档残留
30. `OPERATIONS_MANUAL.md`：退役 split-services 部署路径、18080 端口、错误 docker-compose 路径
31. `.env.postgres.example`：补 `SDKWORK_IM_RUNTIME_DIR` 文档
32. 部署参考更新为 `deployments/kubernetes/cloud/` 与 `deployments/docker/docker-compose.yml`

## 待办（非代码缺陷）

- **发布证据门禁**：Pre-Release/Capacity/AppRelease 需要真实 staging/容量运行与发布包决策（PRD 禁止伪造证据）
- 僵尸表清理（`im_rtc_*` 等 8 张）为破坏性数据库变更，需 owner 评审后实施
- 保留期清理补全（审计表按 retention_class 分批 purge）
- HTTP 限流接线（Redis rate limit store）
- 推送适配器接线（APNs/FCM）
