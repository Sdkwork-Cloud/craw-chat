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

## 第二轮修复（同日期续作）

### 僵尸表清理（P1-1b，已完成）
- 删除 8 张僵尸表：`im_conversation_settings`、`im_idempotency_keys`、`im_rtc_outbox_events`、`im_rtc_participant_credentials`、`im_rtc_quality_reports`、`im_rtc_signals`、`im_threads`、`im_thread_subscriptions`（baseline DDL 63→55 表，含遗留 conversation-id 重写块清理）
- 同步：`table-registry.json`、`schema.yaml`（63→55）、`0001_organization_id_not_null.up.sql`、`database_schema_contract_test.rs`、`organization-isolation.spec.json`、retention cleanup/metrics/scheduler（`rtc_signals_deleted` 全链路移除）、`ensure-im-h5-demo-conversations.mjs`、rtc-signaling/retention-enforcement 测试、`docs/database-design.md` 重新生成（55 表）

### 保留期清理补全（P1-2，已完成）
- 审计记录写入时按 action 命名空间派生 `retention_class`（security=730d/access=180d/admin=365d/data_lifecycle=1095d）并写 `retention_until`（`audit-service/src/retention.rs`）
- retention cleanup 按 4 个审计类分批 purge（`PURGE_AUDIT_RECORDS_BY_CLASS_SQL`），报告/指标/调度日志/ops 响应（DTO+OpenAPI `auditRecordsDeleted`，移除僵尸 `rtcSignalsDeleted`）全链路补齐
- 语义说明：审计链不重写，保留窗口验证如实报告

### HTTP 限流接线（P1-5，已完成）
- 网关 `WebFrameworkBuilder` 接线 Redis rate-limit/idempotency/concurrent-admission store（`SDKWORK_IM_GATEWAY_RATE_LIMIT_REDIS_URL`/`SDKWORK_IM_REDIS_URL`+`SDKWORK_IM_REDIS_ENABLED`），生产 assembly gate 可满足
- 新增边缘 IP 限流中间件（`edge_ip_rate_limit.rs`，RPM/BURST/MAX_ENTRIES 环境变量，infra 探针豁免）
- `etc/topology/README.md` 移除虚假的 circuit-breaker/trusted-proxy/openapi-cache/query-token 声明，改写为真实保护描述

### ops /lag 真实计算（P1-8a，已完成）
- `upsert_lag_items` 合并语义；realtime 高水位窗口 + cluster 聚合项（1s 镜像）；outbox 待投递采样（pending scopes → counts → lag + side-effect outbox 诊断），`/lag` 不再恒为空

### media-service 收口（P1-8b，已完成）
- `provider_health_snapshot` 不再谎报 sdkwork-drive 健康：报告自身能力 + `driveAvailability: not-verified-by-media-service`

### channel 访问规则执行（P1-8c，已完成）
- `ChannelAccessRuleStore::effective_permission`（deny-wins，精确/kind 级/空间级匹配）+ `enforce_channel_permission`（owner/admin 豁免）
- get_channel 强制 view、update/delete_channel 强制 manage；`send` 权限无执行点 → 运行时拒绝并说明

### automation 出站 agent 分发桥接（P1-8d，已完成）
- `AutomationRuntime::requested_executions`/`fail_execution`；`automation_agent_bridge.rs`：轮询 Requested 执行 → 幂等 turn 查询 → 解析 Automation 会话 → complete_turn → 回写响应流（start/deltas/complete），失败转 Failed；standalone runtime 接线

### spaces 错误响应补齐（P1-9a，已完成）
- 37 个 spaces 操作补全 400/401/403/404（authority+mirror+sdkgen×2+fragment，fragment 为组合权威）

### manifest/IAM 对齐（P1-9b，已完成）
- app-api 3 个操作补 `x-sdkwork-idempotent: true` + Idempotency-Key minLength 1（规范配对）
- profile/settings PATCH 204→200（handler 返回更新资源 + 契约/镜像同步）
- `messages.search` → `messages.search.list`（契约 4 文件 + 路由 manifest）
- Cargo 依赖归一为 workspace 形式（im-time/sdkwork-im-contract-agent/im-adapters-redis-cache/im-domain-core）

### PC 聊天增强（P1-11，已完成）
- 服务端搜索：`ChatService.searchHistoryMessages`（cursor 分页）+ ChatHistoryModal 防抖服务端搜索（命中本地解析 + 命中行 + load-more + i18n en/zh）
- 重连回填：`handleConnectionOpen` 对活跃订阅会话拉取最新页合并（断线期间消息回填，受页界约束）
- 消息数组裁剪：既有 `LOCAL_MESSAGES_PER_CONVERSATION_CAP` 已生效

### 商业化 gate 对齐（终验）
- H5 architecture standard：sdkwork-company 企业中心 13 个页面已迁移为真实 port 实现 → 测试从 deferred 列表移除并新增正面断言（真实实现 + 注入 port + 无伪造数据）
- H5 SDK 访问边界：`imH5CallSignaling` 改为经 `@sdkwork/im-h5-core/sdk` 消费 SDK 类型（core 补 `ImCallSession` 再导出）
- session-gateway 测试套件修复：
  - `checkpoint_store_error_test.rs` 重复 `dev_test_environment()` 导致 env 互斥锁死锁（套件整体挂起）→ 移除重复调用
  - `http_smoke_test.rs` 20 个测试补 dev 环境姿态（S2 签名门 + S1 未签名 token 显式 opt-in）
  - `test_env.rs` 补 `SDKWORK_IM_ALLOW_UNSIGNED_TOKENS=true`；`websocket_auth_init_test.rs` 同样补姿态
- **商业化就绪：52/52 gate 通过（0 失败）**；剩余发布证据门禁（Pre-Release/Capacity/AppRelease/cloud-image-evidence）需真实 staging 与容量运行（非代码缺陷，PRD 禁止伪造证据）

## 第三轮修复（P2a：邀请流契约↔实现对齐 + spaces 状态码对齐）

### 邀请契约↔实现漂移（P2a-1，已完成）
- **发现**：`spaces.invites.*` 契约与实现完全脱节——契约 `SpaceInviteCreateRequest` 只有 `maxUses`，而 handler 要求 `inviteeUserId/inviteeEmail/inviteePhone/targetType/targetId/role/message/expiresAt`（走生成 SDK 的客户端必然 400）；契约 `SpaceInviteView` 只有 `inviteCode/spaceId`，实现返回 `invitationId/inviterUserId/status/...`；`accept` 契约 200+SdkWorkCommandData 而实现返回 204
- **修复**：`SpaceInviteCreateRequest`/`SpaceInviteView` 按实现真实字段重写（6 个契约文件：authority + SDK mirror + sdkgen ×2 + fragment ×2，统一 schema，已验证 12 项一致）；`InviteCodePath` 补描述（值为 create 返回的 invitationId）；`InvitationResponse` 补 `role` 字段
- **修复碎片损坏**：`im-spaces-paths.fragment.yaml` 存在 74 个孤儿响应块（P1-9a 遗留的 `description: HTTP 401/404 problem` 无状态码键，YAML 重复键无法解析）→ 全部删除（每操作下方已有完整 400/401/403/404 块，185 = 37×5 验证吻合）
- **修复碎片↔镜像漂移**：fragment 37 个操作停留在 P1-9a 手工修复前状态（缺 `ApiKey` security 块与 `x-sdkwork-request-context/api-surface/route-auth/auth-mode` 4 个元数据键）——若重跑 merge 脚本会剥掉 ApiKey 授权声明。已从权威镜像同步回 fragment（37 操作全匹配），merge 管线恢复幂等
- **验证**：6 个 YAML 文件 js-yaml 解析通过；`check-api-operation-patterns`/`check-api-response-envelope` 通过

### 邀请流安全加固（P2a-2，已完成）
- `create_invitation`：`target_type` 仅接受 `space`（group/channel 邀请可创建但永远无法 accept，假成功路径 → 400 拒绝）；`expires_at` 校验 RFC3339 + 未来时间（此前坏值经 `rfc3339_cmp` 字符串回退永不失效）；`invitee_email/phone` 轻量格式校验
- `get_invitation`：移除 `actor_can_read_space` 门禁——非成员受邀者此前无法预览邀请（403），现仅校验受邀者/邀请人身份
- `accept_invitation`：204 → 200 + `SdkWorkCommandData{accepted:true, resourceId, status}`（API_SPEC §15.4 命令成功必须返回 `data.accepted: true`）；已接受邀请重复 accept 幂等重放成功（此前重试 400）
- 契约 `im_invitations` DDL CHECK 从 `('space','group','channel')` 收紧为 `('space')`

### spaces 块状态码全量对齐（P2a-2b，已完成）
- 8×create 从 200 → 201（`created_json`，calls-service 惯例）
- 5×update 从 204 → 200 + 更新后资源（API_SPEC §15.4 update 必须 200+item；space/member/group/group-member/channel）
- `spaces.invites.accept` 从 204 → 200 + CommandData（同 P2a-2）
- 对照契约 37 操作状态表逐一核对，delete=204/list=200 保持正确

### 邀请 PII 保留期（P2a-3/4，已完成）
- `im_invitations` 新增 `retention_until TIMESTAMPTZ` 列 + 部分索引（PRIVACY_SPEC 要求含个人数据表必须有保留策略；此前 invitee_email/phone 永驻）
- 创建时按 `standard` 类（365 天）计算 retention_until（`im-domain-core::retention`）
- retention purge 新增 `PURGE_INVITATIONS_SQL`（终态记录按 retention_until 分批清理，pending 永不清理）+ report `invitations_deleted` + Prometheus `store="invitations"` + ops DTO/backend 契约同步
- 顺手修复 P1-2 遗留：backend 契约 `RetentionPurgeResponse` 仍含僵尸字段 `rtcSignalsDeleted` 且缺 `auditRecordsDeleted` → 同步为 `invitationsDeleted`/`auditRecordsDeleted`

### 测试与验证（P2a-6）
- `http_smoke_test.rs` 新增 7 个邀请 API 用例（Scripted stores + WebRequestContext/AppContext 注入）：非 space target 400、过去 expiresAt 400、合法创建 201+invitationId+retention、非成员受邀者 200 预览、无关用户 403、accept 200+CommandData、重复 accept 幂等重放 → **8/8 通过**
- 相关 crate 全量测试通过（space-service/social-postgres/postgres-journal/ops-service，60+ 用例 0 失败）
- TS 生成 SDK：`space-invite-create-request`/`space-invite-view` 类型同步（dist 为构建产物，本地重生成验证）

### 文档更新（P2a-7）
- `TECH-150cj-im-social-space-conversation-ddd-design`：invitation 域对象由"适用 space/group/channel"更新为"适用 space；终态邀请按保留策略清理"
- `database-design.md` 为生成清单（无列级信息），无需改动；`docs/api-reference.md` 路径表无变化

## 待办（非代码缺陷，续）

- **发布证据门禁**：Pre-Release/Capacity/AppRelease 需真实 staging/容量运行（PRD 禁止伪造证据）
- 推送适配器接线（APNs/FCM，需外部 provider 凭据）
- P2 候选：审计链外部锚定（P2b，依赖 WORM notary 外部基础设施）
- **P2c 评估结论（本轮完成，无需代码修复）**：未读数由 `ReceivedMessageIndex`（收到消息 seq 索引）减 read_cursor 实时计算，是真实实现，无 stub/fake。read_cursor 有 DB 持久化（重启恢复），但 received 索引与 timeline/inbox 同为内存投影，服务重启后从空开始累积——这是 comms-conversation-service 的投影架构设计（DB 为真值、历史读取走 DB），非技术债务；"重启后未读数归零直至新消息"为已知产品取舍，如产品要求重启即恢复未读数，需按 feature 立项（启动/惰性 DB 回填）
- 环境说明：商业化 gate 的 DB 依赖步骤（session-gateway 等 cargo 套件）在本机无 PostgreSQL 时通过 env 跳过；`cargo run` 类步骤需独立服务
