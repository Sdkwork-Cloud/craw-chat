> Migrated from `docs/架构/200-商业化落地-阶段0-真值层与租户隔离重建实施计划-2026-06-15.md` on 2026-06-24.
> Owner: SDKWork maintainers

> 状态：待评审
> 日期：2026-06-15
> 上游审查依据：`docs/架构/02`、`152CJ-current-architecture-as-built-alignment-2026-04-09.md`、本次代码级审查
> 适用范围：`sdkwork-im` 全工作区
> 决策前提（已确认）：新应用、零用户、不保留兼容性、不留技术债、打磨至最规范终态

## 0. 定位与边界

本文档是商业化落地**阶段 0** 的唯一实施计划，只解决"消息真值、租户隔离、ID 规范"这三类阻断商业化的硬伤。阶段 1（实时层 + Redis）、阶段 2（事件总线 + 分区）、阶段 3（可观测 + 治理）另起独立文档。

阶段 0 的成功标准：**单进程即可对外提供不丢、不重、顺序一致、租户/组织双重隔离、ID 全局规范的消息服务，且 `conversation-runtime` 可被安全地水平复制（为阶段 1 的多 gateway 铺路）。**

不在阶段 0 范围（显式排除，避免范围蔓延）：

- Redis 接入、跨节点实时推送（阶段 1）
- Redpanda / Kafka 事件总线、表分区、RLS（阶段 2）
- Prometheus / OpenTelemetry、归档 job、KMS（阶段 3）

## 1. 问题基线（来自代码级审查，非文档表述）

下列结论均以代码路径与行号为证据，阶段 0 必须逐条消除。

| 编号 | 问题 | 证据 | 阶段 0 是否处理 |
| --- | --- | --- | --- |
| P0-1 | 消息时序由进程内存 `high_watermark += 1` 生成，无法多实例 | `crates/im-domain-core/src/message.rs:193 next_message_seq`；`services/conversation-runtime/src/runtime.rs:2017` | ✅ |
| P0-2 | `im_commit_journal.commit_offset` 直接复用业务 `aggregate_seq`，造成写入热点 | `adapters/postgres-journal/src/lib.rs:254` (`let commit_offset = aggregate_seq;`) | ✅ |
| P0-3 | 消息真值表 `im_conversation_messages` 已建表但无任何 Rust 读写代码，仅出现在 schema 契约测试 | `findstr im_conversation_messages` 仅命中 `database_schema_contract_test.rs` | ✅ |
| P0-4 | Outbox 为本地 JSONL/内存（`FileMessageSideEffectOutboxStore`），非分布式；`im_outbox_events` 表闲置 | `sdkwork-im-server/src/node/build.rs:1084`；`side_effect_outbox.rs` | ✅ |
| P0-5 | organization_id 已贯穿 `AppContext` 与双 token 头，但全部 `im_*` 表与持久化层无该列 | `im-app-context/src/lib.rs:45`；`001_im_core_schema.sql` 全表无 `organization_id` | ✅ |
| P1-1 | 多租户仅靠应用层 `WHERE tenant_id`，无数据库级隔离 | `001_im_core_schema.sql` 无 RLS（阶段 2 处理，阶段 0 先补 org 列与一致过滤） | ⚠️ 部分 |
| 技术债 | 业务实体 ID 用 `format!("msg_{conversation_id}_{seq}")`、`evt_`、`cm_` 等字符串拼接，非全局规范 ID | `runtime.rs:1705 generated_message_id`、`:2044` | ✅ |

> P1-1 的 RLS（Row Level Security）落在阶段 2 与分区一并上线；阶段 0 只保证"列存在、过滤一致、索引覆盖"，避免在阶段 0 引入跨阶段的迁移耦合。

## 2. 目标终态设计（阶段 0 交付后的形态）

### 2.1 三层真值分离（Durable Truth / Query Truth / Coordination）

```
写路径（单一权威）:
  conversation-runtime (无状态命令处理)
    -> ConversationCommandHandler
       1. 校验 + 聚合状态加载（从 MessageStore + MemberStore 按需读）
       2. SnowflakeIdGenerator 分配 message_id / event_id
       3. MessageStore.insert_message()           // 写 im_conversation_messages（真值）
       4. CommitJournal.append()                   // 写 im_commit_journal（事件流，offset 独立）
       5. OutboxStore.enqueue()                    // 写 im_outbox_events（副作用待投递）
       全部在同一 PostgreSQL 事务内

读路径（查询真值，可重建）:
  projection-service / API -> MessageStore.read_window()      // 直读 im_conversation_messages
                          -> ConversationSummaryStore.read()  // 直读 im_projection_*

投递路径（异步，与写解耦）:
  outbox-relay-worker
    -> SELECT ... FROM im_outbox_events WHERE publish_status='pending'
       ORDER BY available_at, outbox_id FOR UPDATE SKIP LOCKED
    -> 投递实时 / 通知 / 审计
    -> UPDATE publish_status='published'
```

### 2.2 租户 + 组织双重隔离

所有 `im_*` 表的主键与索引前置 `(tenant_id, organization_id, ...)`，`organization_id` 为 `TEXT NOT NULL DEFAULT 'default'`（兜底单组织租户）。应用层查询、适配器 SQL、投影重建全部强制携带 `organization_id` 过滤。

### 2.3 ID 生成规范（统一 Snowflake）

所有持久化实体的主键 ID 统一由 `sdkwork_id::SnowflakeIdGenerator` 生成（`i64`，存为 `BIGINT`），废弃所有 `format!("msg_...")` / `evt_...` / `cm_...` 字符串拼接。业务可读性通过单独的 `*_display_code` 文本列承载，不污染主键。

> Snowflake 实现：`sdkwork-appbase/crates/sdkwork-platform-id-service`，profile = 41 timestamp + 10 node + 12 sequence，单节点单毫秒 4096 ID，节点 0–1023，30 年寿命。阶段 0 单进程默认 `node_id=0`，阶段 1 多 gateway 时由配置注入唯一 `node_id`。

### 2.4 无状态化 `conversation-runtime`

移除 `RwLock<RuntimeState>` 中"会话常驻"的强依赖。会话聚合状态改为按需从存储加载 + 内存 LRU 软缓存（缓存丢失不丢数据，仅触发一次冷加载）。`message_seq` 不再来自内存 `high_watermark`，改为数据库原子分配（见 §3.3）。

## 3. 详细变更清单

### 3.1 数据库迁移（新增 3 个迁移文件，废弃 `001` 中的临时实现）

**原则**：新应用零用户，不写 `ALTER ... ADD COLUMN` 式补丁迁移；直接重建为终态 schema。`001_im_core_schema.sql` 保留为历史只读档案，新迁移以 `010_` 起编号作为新的"干净起点"，部署时只执行 `010+`。

#### 3.1.1 `deployments/database/postgres/migrations/010_im_tenant_organization_isolation.sql`

为所有 `im_*` 业务表引入 `organization_id`，并将主键/索引前置 `tenant_id, organization_id`。覆盖表清单（来自 `specs/database-table-registry.json`）：

- `im_conversation_messages`、`im_message_media_refs`
- `im_realtime_device_events`、`im_realtime_checkpoints`、`im_realtime_subscriptions`、`im_realtime_subscription_scopes`
- `im_presence_states`、`im_route_bindings`、`im_realtime_disconnect_fences`
- `im_rtc_sessions`、`im_rtc_signals`
- `im_audit_records`、`im_notification_tasks`、`im_automation_executions`
- `im_projection_*`（全部 8 张投影表）
- `im_stream_sessions`、`im_stream_frames`

约定：

```sql
-- 每张表统一执行：
organization_id TEXT NOT NULL DEFAULT 'default',
-- 主键与唯一约束前置 (tenant_id, organization_id, ...)
-- 所有二级索引前置 (tenant_id, organization_id, ...)
```

#### 3.1.2 `deployments/database/postgres/migrations/011_im_message_truth_layer.sql`

落地消息真值层与 outbox：

```sql
-- 消息真值表（替换 001 中仅作占位的版本），主键改 Snowflake BIGINT
CREATE TABLE im_conversation_messages (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT 'default',
    conversation_id     TEXT NOT NULL,
    message_id          BIGINT NOT NULL,           -- Snowflake，全局唯一
    message_seq         BIGINT NOT NULL,           -- 会话内严格递增，来自 im_conversation_seq_counters
    sender_principal_kind TEXT NOT NULL,
    sender_principal_id TEXT NOT NULL,
    sender_device_id    TEXT,
    client_msg_id       TEXT,
    message_type        TEXT NOT NULL,
    payload_json        JSONB NOT NULL,
    payload_hash        TEXT NOT NULL,
    created_at          TIMESTAMPTZ NOT NULL,
    updated_at          TIMESTAMPTZ NOT NULL,
    deleted_at          TIMESTAMPTZ,
    retention_until     TIMESTAMPTZ,
    CONSTRAINT pk_im_conversation_messages PRIMARY KEY (tenant_id, organization_id, conversation_id, message_seq),
    CONSTRAINT uk_im_conversation_messages_id UNIQUE (tenant_id, message_id)
);

-- 会话内消息序号分配器（行级原子，替代内存 high_watermark）
CREATE TABLE im_conversation_seq_counters (
    tenant_id       TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT 'default',
    conversation_id TEXT NOT NULL,
    next_seq        BIGINT NOT NULL DEFAULT 1,
    updated_at      TIMESTAMPTZ NOT NULL,
    CONSTRAINT pk_im_conversation_seq_counters PRIMARY KEY (tenant_id, organization_id, conversation_id)
);

-- 客户端幂等键（会话 + 发送者 + client_msg_id 唯一）
CREATE UNIQUE INDEX uk_im_conversation_messages_client
    ON im_conversation_messages (tenant_id, organization_id, conversation_id, sender_principal_kind, sender_principal_id, client_msg_id)
    WHERE client_msg_id IS NOT NULL;

-- timeline 读取索引
CREATE INDEX idx_im_messages_tenant_conv_seq
    ON im_conversation_messages (tenant_id, organization_id, conversation_id, message_seq DESC);

-- outbox 待投递队列索引（relay worker 用）
CREATE INDEX idx_im_outbox_events_status_available
    ON im_outbox_events (tenant_id, organization_id, publish_status, available_at, outbox_id);
```

#### 3.1.3 `deployments/database/postgres/migrations/012_im_journal_offset_independence.sql`

解除 journal 写入热点：`commit_offset` 改为独立 Snowflake 全局序，不再复用 `aggregate_seq`：

```sql
-- im_commit_journal.commit_offset 改由 Snowflake 生成（应用层写入）
-- 保留主键 (partition_key, commit_offset)，但 commit_offset 现在是全局递增、无业务耦合
-- partition_key 改为 (tenant_id:organization_id:aggregate_type:aggregate_id) 显式分片
```

> 说明：journal 仍承担"事件流回放"职责，但不再是消息查询的来源。`aggregate_seq` 字段保留，表示业务侧的聚合版本号，与 `commit_offset`（存储侧全局偏移）解耦。

### 3.2 存储契约层（`crates/im-storage-contracts`）

新增三个核心 trait，放在 storage 契约层，由 `adapters/postgres-journal` 实现：

```rust
// crates/im-storage-contracts/src/message_store.rs（新建）
pub trait MessageStore: Send + Sync {
    fn allocate_message_seq(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<u64, ContractError>;
    // 原子分配：UPDATE im_conversation_seq_counters SET next_seq = next_seq + 1 ... RETURNING

    fn insert_message(&self, message: StoredMessage) -> Result<(), ContractError>;
    // INSERT，唯一冲突（message_id 或 client_msg_id）映射为 ContractError::Conflict

    fn read_window(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        after_seq: u64,
        limit: usize,
    ) -> Result<MessageWindow, ContractError>;

    fn read_message_by_id(
        &self,
        tenant_id: &str,
        message_id: i64,
    ) -> Result<Option<StoredMessage>, ContractError>;
}

pub trait OutboxStore: Send + Sync {
    fn enqueue(&self, event: OutboxEvent) -> Result<(), ContractError>;
    fn drain_pending(
        &self,
        tenant_id: &str,
        organization_id: &str,
        batch_size: usize,
    ) -> Result<Vec<OutboxEvent>, ContractError>;
    // SELECT ... FOR UPDATE SKIP LOCKED，多 worker 安全
    fn mark_published(&self, tenant_id: &str, outbox_id: &str) -> Result<(), ContractError>;
    fn mark_failed(
        &self,
        tenant_id: &str,
        outbox_id: &str,
        reason: &str,
    ) -> Result<(), ContractError>;
}

pub trait ConversationAggregateStore: Send + Sync {
    // 会话元数据 + 成员 + 已读游标的持久化读写
    // 替代 conversation-runtime 内存 ConversationRoster 的强依赖
    fn load_members(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
    ) -> Result<Vec<ConversationMember>, ContractError>;
    fn upsert_member(&self, member: ConversationMember) -> Result<(), ContractError>;
    fn load_read_cursor(
        &self,
        tenant_id: &str,
        organization_id: &str,
        conversation_id: &str,
        member_id: i64,
    ) -> Result<Option<ReadCursor>, ContractError>;
    fn upsert_read_cursor(&self, cursor: ReadCursor) -> Result<(), ContractError>;
}
```

**约束**：所有 trait 方法签名首参强制 `(tenant_id, organization_id)`，编译期杜绝漏过滤。`StoredMessage.message_id` / `ConversationMember.member_id` / `ReadCursor` 主键统一为 `i64`（Snowflake）。

### 3.3 领域内核改造（`crates/im-domain-core` + `services/conversation-runtime`）

#### 3.3.1 移除内存时序

`crates/im-domain-core/src/message.rs:193`：删除 `next_message_seq`（`high_watermark += 1`）。`ConversationMessageLog` 退化为只读视图，`message_seq` 由 `MessageStore.allocate_message_seq` 在命令处理时分配，写入后回填。

#### 3.3.2 `ConversationRuntime` 重构

`services/conversation-runtime/src/runtime.rs`：

- 移除 `RwLock<RuntimeState>` 中 `conversations: HashMap<String, ConversationState>` 的"唯一真相"地位，改为 LRU 软缓存。
- `ensure_conversation_loaded` 改为从 `ConversationAggregateStore` 加载成员/已读游标，而非扫全量 journal（消除当前 O(N) 全表扫描）。
- `post_message` 命令处理顺序变更：
  1. 校验 body / sender
  2. `load_members` 校验成员资格与权限
  3. `allocate_message_seq`（DB 原子）
  4. `SnowflakeIdGenerator.generate()` → `message_id`
  5. 组装 `Message`，`insert_message` + `journal.append` + `outbox.enqueue` 在**同一 DB 事务**
  6. 缓存更新（best effort，失败不影响真值）

#### 3.3.3 ID 规范化

废弃全部字符串拼接 ID，替换为 Snowflake：

| 旧（字符串拼接） | 新（Snowflake i64） | 位置 |
| --- | --- | --- |
| `format!("msg_{conv}_{seq}")` | `message_id: i64` | `runtime.rs:1705 generated_message_id` |
| `format!("evt_{msg_id}_posted")` | `event_id: i64`（journal 独立生成） | `runtime.rs:2044` |
| `format!("cm_{conv}_{kind}_{id}")` | `member_id: i64` | `im-domain-core/src/conversation.rs:867 member_episode_id` |

新增 `crates/im-platform-contracts/src/id.rs`：

```rust
pub trait IdGenerator: Send + Sync {
    fn next_id(&self) -> Result<i64, IdError>;
}

pub struct SnowflakeIdGeneratorAdapter {
    inner: sdkwork_id::SnowflakeIdGenerator,
}

impl IdGenerator for SnowflakeIdGeneratorAdapter { /* delegate */ }
```

> 由 `sdkwork_id`（已为 workspace 依赖，`Cargo.toml:94`）提供。`node_id` 从 `SDKWORK_IM_NODE_ID` 环境变量读取，默认 0。

### 3.4 组织上下文贯通

`AppContext.organization_id` 当前是 `Option<String>`（`im-app-context/src/lib.rs:45`）。阶段 0 收敛为：

- 解析侧：`resolve_app_context` 从 `x-sdkwork-organization-id` 头读取，缺失时填 `'default'`（而非 `None`），保证下游永远拿到非空组织。
- 传递侧：所有命令结构体（`PostMessageCommand`、`CreateConversationCommand` 等）新增 `organization_id: String` 字段，`from_auth_context` 构造时从 `AppContext` 填充。
- 持久化侧：所有 `MessageStore` / `OutboxStore` / `ConversationAggregateStore` 调用传入 `organization_id`。

### 3.5 Outbox Relay Worker

新增 `services/sdkwork-im-cloud-gateway/src/node/outbox_relay.rs`（阶段 0 与主进程同生命周期，阶段 1 拆为独立 deployment）：

- 启动一个 tokio 后台 task，周期（默认 50ms）调用 `OutboxStore.drain_pending`。
- 投递到现有实时投递 runtime（`RealtimeDeliveryRuntime`）与通知 runtime（`NotificationRuntime`）。
- 投递成功 `mark_published`，失败累计 `attempt_count`，超阈值 `mark_failed` 并告警（阶段 3 接 metrics）。
- **废弃** `FileMessageSideEffectOutboxStore` / `MemoryMessageSideEffectOutboxStore` 的生产用法，仅保留测试实现。

### 3.6 表注册表与 schema 契约测试同步

- `specs/database-table-registry.json`：为每张表补 `organizationId` 字段语义说明；新增 `im_conversation_seq_counters` 条目。
- `specs/database-prefix-registry.json`：无变化（`im_` 前缀不变）。
- `database_schema_contract_test.rs`：重写为校验 `010+` 迁移产物，强制断言每张业务表含 `organization_id` 列且相关索引前置。

## 4. 实施顺序（可独立验证的 6 个提交）

每个提交须独立通过 `cargo fmt --all --check` + `cargo clippy -p <touched> --tests -- -D warnings` + 相关单测。

1. **迁移与契约**：`010/011/012` 迁移 + `MessageStore`/`OutboxStore`/`ConversationAggregateStore` trait 定义 + `database_schema_contract_test` 重写。
2. **适配器实现**：`adapters/postgres-journal` 实现三个 trait（含 `allocate_message_seq` 的 `UPDATE ... RETURNING` 原子分配）。
3. **ID 规范化**：`SnowflakeIdGeneratorAdapter` + 领域模型 `message_id`/`member_id`/`event_id` 改 `i64`，移除字符串拼接。
4. **organization 贯通**：`AppContext` 默认 `'default'` + 所有命令结构体加字段 + 存储调用传参。
5. **conversation-runtime 无状态化**：移除内存 `high_watermark`，改 `allocate_message_seq`；`ensure_conversation_loaded` 改走 store；写路径三表同事务。
6. **outbox relay**：`outbox_relay.rs` 后台 task + 废弃文件 outbox 的生产用法。

## 5. 验证计划（证据先于完成）

每个提交对应的最小验证命令，汇总提交须全绿：

| 提交 | 主验证命令 | 附加 |
| --- | --- | --- |
| 1 | `cargo test -p im-storage-contracts --tests` | schema 契约测试断言 org 列 |
| 2 | `cargo test -p im-adapters-postgres-journal --tests` | 真实 PG（`SDKWORK_IM_POSTGRES_TEST_DATABASE_URL`）集成测试 |
| 3 | `cargo test -p im-domain-core --tests` | ID 单调唯一性测试 |
| 4 | `cargo test -p im-app-context --tests` | organization 解析回归 |
| 5 | `cargo test -p sdkwork-im-cloud-gateway --tests` | 双实例并发发消息，seq 单调且无空洞 |
| 6 | `cargo test -p sdkwork-im-cloud-gateway --tests` | outbox 投递幂等、重试、失败标记 |

汇总门禁（全绿方可标记阶段 0 完成）：

```bash
cargo fmt --all --check
cargo clippy --workspace --tests -- -D warnings
cargo test --workspace
pnpm run check:commercial-readiness
pnpm run test:workflow-commercial-gates
```

外加架构级验证（写入 `tests/` 集成测试）：

- **时序正确性**：两进程并发向同一会话发 N 条消息，断言 `message_seq` 连续无空洞、无重复、`message_id` 全局唯一。
- **租户/组织隔离**：跨 `(tenant_id, organization_id)` 查询互不可见（`read_window` 强制过滤）。
- **真值一致性**：`conversation-runtime` 重启后，`read_window` 结果与重启前一致（证明真值已脱离内存）。
- **outbox 不丢**：在 `mark_published` 前杀进程，重启后 `drain_pending` 重新投递成功。

## 6. 风险与回滚

| 风险 | 缓解 |
| --- | --- |
| Snowflake 时钟回拨 | `sdkwork_id::SnowflakeIdError::ClockMovedBackwards` 已处理；node_id 配置唯一；NTP 同步为部署前置（阶段 3 加监控） |
| `allocate_message_seq` 行锁成新热点 | 单会话写入本就需串行化（保证会话内顺序），锁粒度=单行，可接受；超大群聊在阶段 2 用分区进一步分散 |
| 写路径三表同事务变长 | 三表均在同一 PG 实例，事务跨表开销可忽略；outbox 写入是轻量 INSERT |
| 废弃 `001` 迁移影响既有部署 | 决策前提确认零用户；新部署只执行 `010+`；`001` 保留为档案不删除 |

回滚策略：每个提交独立可 revert；迁移 `010+` 为全新 schema，回滚即重置数据库（零用户前提成立）。

## 7. 阶段 0 完成定义（DoD）

全部为真时，阶段 0 视为完成：

- [ ] `cargo fmt --all --check` 通过
- [ ] `cargo clippy --workspace --tests -- -D warnings` 通过
- [ ] `cargo test --workspace` 通过
- [ ] `pnpm run check:commercial-readiness` 通过
- [ ] `pnpm run test:workflow-commercial-gates` 通过
- [ ] 所有 `im_*` 业务表含 `organization_id`，索引前置覆盖
- [ ] `message_seq` 由数据库原子分配，进程内无 `high_watermark` 自增
- [ ] 所有持久化主键为 Snowflake `i64`，无字符串拼接 ID
- [ ] `im_conversation_messages` / `im_outbox_events` / `im_conversation_seq_counters` 有 Rust 读写代码并经集成测试覆盖
- [ ] `conversation-runtime` 重启后数据无损
- [ ] outbox relay 投递幂等可重试
- [ ] 双实例并发发消息时序正确（集成测试证据）

## 8. 后续衔接（非本阶段）

阶段 0 完成后，立即开启：

- **阶段 1**（实时层）：Redis 接入、多 gateway、跨节点推送、租户级限流。依赖阶段 0 的"runtime 可水平复制"前提。
- **阶段 2**（隔离与总线）：PostgreSQL RLS、表分区、Redpanda 事件总线、`im_inbox_events` 消费幂等。依赖阶段 0 的 organization_id 列与 outbox。
- **阶段 3**（治理）：Prometheus metrics、OpenTelemetry trace、归档 job、KMS 密钥管理。

