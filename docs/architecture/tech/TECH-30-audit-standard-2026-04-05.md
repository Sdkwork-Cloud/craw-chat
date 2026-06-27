> Migrated from `docs/架构/30-审计与运维接口最小权限标准-2026-04-05.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 审计与运维接口最小权限标准（2026-04-05）

## 1. 背景

在 `29-剩余独立服务公网认证收口与Public-Builder补齐-2026-04-05`
完成后，`sdkwork-im` 已经把所有独立服务的公网入口收敛到 `Bearer-only`。

但继续 review 后发现：

- `audit-service`
- `ops-service`
- `sdkwork-im-server` 中复用的 `/backend/v3/api/audit/*` 与 `/backend/v3/api/ops/*`

虽然都已经要求 Bearer 或可信认证上下文，
却仍然只做“身份认证”，没有做“权限授权”。

这意味着任意已认证用户都可以：

- 导出租户级审计数据
- 读取节点级 route ownership、cluster 状态与 diagnostics 明细
- 伪造新的 audit anchor 记录

这与文档中已经冻结的 `最小权限`、`tenant-aware authz`、
`控制面 / 数据面 / 运维面三层分离` 标准冲突。

## 2. 本轮发现

### 2.1 高风险：`ops-service` 缺少最小权限校验

受影响接口：

- `GET /backend/v3/api/ops/health`
- `GET /backend/v3/api/ops/cluster`
- `GET /backend/v3/api/ops/lag`
- `GET /backend/v3/api/ops/diagnostics`

根因：

- handler 仅调用 `resolve_auth_context(...)`
- 之后直接返回运维视图
- 没有校验调用者是否具备运维读权限

风险：

- 普通终端用户可以读取节点拓扑、route ownership、drain 状态和 diagnostics
- 在 SaaS 场景中，这属于不必要的租户内高敏运维暴露

### 2.2 高风险：`audit-service` 缺少最小权限校验

受影响接口：

- `POST /backend/v3/api/audit/records`
- `GET /backend/v3/api/audit/records`
- `GET /backend/v3/api/audit/export`

根因：

- handler 仅校验身份，不校验权限
- 任意 Bearer 都可以读审计导出或写审计锚点

风险：

- 普通用户可导出租户级审计记录
- 普通用户可通过公开接口伪造审计锚点，污染审计链路

### 2.3 `sdkwork-im-server` 继承同样问题

`sdkwork-im-server` 没有直接复用 `ops-service` / `audit-service` 的 Router，
而是在自身聚合路由中实现了同样 handler。

因此即使独立服务补了权限校验，
`sdkwork-im-server` 仍可能继续暴露同样的授权缺口。

## 3. 冻结标准

本轮之后，冻结以下授权规则：

1. `GET /backend/v3/api/ops/*` 必须要求 `ops.read`
2. `GET /backend/v3/api/audit/records` 与 `GET /backend/v3/api/audit/export` 必须要求 `audit.read`
3. `POST /backend/v3/api/audit/records` 必须要求 `audit.write`
4. `tenant.admin` 作为通用高权限通配权限，可满足上述检查
5. `permissions` 解析标准：
   - Bearer claims 支持 `permissions` / `perms` 数组
   - Bearer claims 支持 `scope` / `scp` 空格分隔字符串
   - internal/test trusted headers 支持 `x-permissions` / `x-scope`
6. 认证与授权分层：
   - `resolve_bearer_auth_context(...)` 负责身份来源合法性
   - handler 层负责资源级或能力级授权

## 4. 本轮落地

### 4.1 `im-auth-context`

新增通用权限能力：

- `AuthContext.permissions`
- `AuthContext::has_permission(...)`
- Bearer claims 中的 `permissions / perms / scope / scp` 解析
- trusted headers 中的 `x-permissions / x-scope / x-scopes` 解析

### 4.2 服务侧授权

已补齐：

- `services/ops-service`：`ops.read`
- `services/audit-service`：`audit.read`、`audit.write`
- `services/sdkwork-im-cloud-gateway`：同步补齐以上权限检查

### 4.3 测试与示例

新增或更新：

- `crates/im-auth-context/tests/auth_context_test.rs`
- `services/ops-service/tests/public_auth_test.rs`
- `services/audit-service/tests/public_auth_test.rs`
- `services/sdkwork-im-cloud-gateway/tests/public_auth_e2e_test.rs`
- `services/ops-service/tests/http_smoke_test.rs`
- `services/audit-service/tests/http_smoke_test.rs`
- `services/sdkwork-im-cloud-gateway/tests/task10_capabilities_e2e_test.rs`
- `services/sdkwork-im-cloud-gateway/tests/cluster_drain_rebalance_e2e_test.rs`
- `services/sdkwork-im-cloud-gateway/tests/cluster_realtime_routing_e2e_test.rs`

测试契约：

- 普通 Bearer 访问 audit/ops 高权限路由必须返回 `403 permission_denied`
- 带显式权限 claims 的 Bearer 必须通过
- internal/test trusted headers 必须显式携带 `x-permissions` 才能通过高权限接口

## 5. 验证结果

本轮执行并通过：

- `cargo test -p im-auth-context --test auth_context_test --offline`
- `cargo test -p ops-service --test public_auth_test --offline`
- `cargo test -p audit-service --test public_auth_test --offline`
- `cargo test -p sdkwork-im-cloud-gateway --test public_auth_e2e_test --offline`
- `cargo test -p im-auth-context --offline`
- `cargo test -p ops-service --offline`
- `cargo test -p audit-service --offline`
- `cargo test -p sdkwork-im-cloud-gateway --offline`
- `cargo fmt --all`
- `cargo test --workspace --offline`

## 6. 当前结论

截至本轮：

1. `sdkwork-im` 对高敏审计与运维读写接口，已经从“只有认证”升级到“认证 + 最小权限授权”。
2. Bearer-only 公网入口与 handler 内授权规则已经对齐。
3. `sdkwork-im-server` 不再是独立服务修复后的旁路回退。

## 7. 下一步

本轮仍然是“permission-based 最小闭环”，后续建议继续推进：

1. 将 `tenant.admin`、`ops.read`、`audit.read`、`audit.write` 进一步映射到更正式的角色模型，例如 operator / admin / auditor
2. 为 `control-plane-api` 补同样 capability 权限，例如 `control.write`
3. 在部署层补静态配置检查，确保 demo token 与 operator token 不被混用到生产环境

