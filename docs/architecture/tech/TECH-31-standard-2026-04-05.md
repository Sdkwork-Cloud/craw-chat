> Migrated from `docs/架构/31-控制面写接口最小权限标准-2026-04-05.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 控制面写接口最小权限标准 2026-04-05

## 1. 背景

在完成 `audit-service`、`ops-service` 和 `sdkwork-im-server` 的最小权限收敛后，继续对 `control-plane-api` 进行公网入口审查，发现控制面写接口虽然已经具备 Bearer 鉴权，但仍然缺少能力级授权判断。

该问题会导致以下高风险操作只要持有任意有效租户 Bearer，即可越权调用：

- 节点进入 drain
- 节点恢复 active
- 路由迁移与重平衡

这类接口属于平台运维级写操作，必须从“已认证”升级为“已认证 + 已授权”。

## 2. 审查结论

本次收敛覆盖以下接口：

- `POST /backend/v3/api/control/nodes/{node_id}/drain`
- `POST /backend/v3/api/control/nodes/{node_id}/activate`
- `POST /backend/v3/api/control/nodes/{node_id}/routes/migrate`

统一要求：

1. 公网入口只接受 Bearer 身份，不接受 trusted identity headers。
2. 通过 Bearer 解析出的 `AuthContext` 必须显式具备 `control.write`。
3. 缺少权限时返回 `403 permission_denied`。
4. `tenant.admin`、`*`、`control.*` 等上位权限可视为满足 `control.write`。

## 3. 实现标准

### 3.1 鉴权边界

- `build_public_app()` 路径下的控制面接口必须走 `resolve_auth_context(...)`。
- 对公网请求不得回退到 `x-tenant-id` / `x-user-id` / `x-permissions` 之类 trusted headers 作为身份来源。
- trusted headers 仅用于内部测试或显式 trusted builder。

### 3.2 授权边界

- 控制面写接口在进入业务逻辑前统一执行 `ensure_control_write_access(...)`。
- 业务逻辑、状态机、节点重平衡、路由迁移不得假定“通过认证即允许写”。
- 所有新增控制面写接口默认也必须纳入 `control.write` 或更细粒度权限映射。

### 3.3 测试标准

至少保留以下回归测试：

1. trusted headers 调用公网控制面接口返回 `401 auth_context_missing`
2. 无 `control.write` 的 Bearer 调用公网控制面接口返回 `403 permission_denied`
3. 具备 `control.write` 的请求可以完成 drain / migrate / activate 正常流程
4. 下游聚合节点或 e2e 用例同步携带控制面权限，避免测试语义落后于实现

## 4. 本次落地文件

- `services/control-plane-api/src/lib.rs`
- `services/control-plane-api/tests/public_auth_test.rs`
- `services/control-plane-api/tests/drain_routes_test.rs`
- `services/sdkwork-im-cloud-gateway/tests/cluster_drain_rebalance_e2e_test.rs`

## 5. 设计原则

控制面接口不应依赖“调用方知道内部节点拓扑”这一事实来放松授权要求。对 SaaS 和私有化集群都应遵守同一条规则：

- 认证解决“你是谁”
- 授权解决“你能改什么”

只有将控制面写操作显式绑定到 `control.write`，才能保证后续水平扩容、动态加节点、跨节点迁移、无人值守运维等场景仍具备可审计、可收敛、可最小授权的安全边界。

