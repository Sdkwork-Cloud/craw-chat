> Migrated from `docs/review/im-server-db-security-performance-report-2026-06-01.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Sdkwork IM 即时通信服务端与数据库审计报告（2026-06-01）

## 1. 审计范围
- 代码范围：`services/*`（重点：`control-plane-api`、`sdkwork-im-server`、`session-gateway`、`conversation-runtime`、`projection-service`）。
- 数据库范围：`deployments/database/postgres/migrations/001_im_core_schema.sql`。
- 规范基线：`../../specs/API_SPEC.md`、`../../specs/DATABASE_SPEC.md`。

## 2. 本轮已完成整改（含验证）

### 2.1 API 错误模型统一（ProblemDetail）
- `session-gateway`、`sdkwork-im-server`、`projection-service`、`control-plane-api` 已统一返回 `application/problem+json`，并保留兼容字段（如 `code/message`，控制面保留 `errorStatus`）。
- 已通过：`cargo test -p projection-service --test http_smoke_test`、`cargo test -p control-plane-api --test http_smoke_test`、`cargo test -p control-plane-api --test openapi_export_test`。

### 2.2 数据库约束与保留策略补齐
- 去除 `im_idempotency_keys` 重复唯一约束（保留主键边界）。
- 为所有 `retention_until` 表补齐清理索引（部分索引）。
- 补齐缺失 `CHECK`：outbox/inbox/presence/route_bindings 状态约束。
- 已通过：`cargo test -p session-gateway --test database_schema_contract_test`、`cargo test -p session-gateway --test postgres_realtime_sql_contract_test`。

### 2.3 OpenAPI operationId 生成器对齐规范（本轮新增）
- `crates/sdkwork-im-openapi/src/lib.rs` 已从 `method_path` 改为 `dotted lowerCamel` 资源动作风格，并移除路径参数、处理标准动作词。
- 新增规范映射单测（覆盖 `sessions.create`、`users.retrieve`、`organizationMemberships.list`、`roles.permissions.delete` 等）。
- 已通过：`cargo test -p sdkwork-im-openapi`、`cargo check -p sdkwork-im-server`、`cargo test -p sdkwork-im-cloud-gateway --test openapi_im_v3_contract_test`。

## 3. 当前仍存在的问题（按优先级）

### P1 安全：AppContext 签名校验仍是“可选开启”
- 证据：`crates/im-app-context/src/lib.rs` 中 `AppContextSignatureConfig::from_env()` 依赖 `SDKWORK_IM_APP_CONTEXT_REQUIRE_SIGNATURE`，默认不强制。
- 风险：未强制签名的部署中，上下文仍主要依赖透传头，存在伪造/越权窗口。
- 建议：受保护入口默认强制签名；将“签名 + token introspection/JWKS”设为统一生产基线。

### P2 API 合规：控制面/网关仍有手写 operationId 与参数命名不规范
- 证据：
  - 控制面仍使用 `getHealthz`、`getProtocolRegistry` 等非 dotted 风格：`services/control-plane-api/src/lib.rs`。
  - 网关发现接口仍使用 `getGatewayOpenapiIndex` 等：`services/web-gateway/src/lib.rs`。
  - 控制面路径参数仍有 `request_id`、`friendship_id`、`node_id`（规范要求 path parameter 为 lowerCamelCase）。
  - 控制面 tag 含 `social-runtime`（规范要求 tag 为 lowerCamelCase）。
- 风险：SDK 生成方法面不稳定，跨语言一致性与演进治理成本高。
- 建议：按 breaking change 流程，分阶段将手写 operationId、tag、path 参数统一到 SDKWork v3 规则。

### P2 性能：控制面文件态存储为“全量读写 + 全局互斥”
- 证据：`SocialStateStore::load/save` 对单文件执行 `read_to_string` / `to_string_pretty + 原子替换`，并通过 `io_lock` 串行化（`services/control-plane-api/src/lib.rs` 约 6475-6550 行）。
- 风险：高并发写入下延迟抖动明显，状态文件体积增大后会拉高尾延迟。
- 建议：生产默认切到数据库持久化；文件态仅用于单节点开发环境。

## 4. 结论
- **数据库设计**：本轮约束与清理索引补齐后，主干风险已显著下降。
- **模型/技术设计**：核心链路可用，但控制面状态存储与身份信任链仍有中高优先级改进项。
- **API 设计**：错误模型已统一；`operationId/tag/path-parameter` 在控制面/网关仍需按规范收口。

