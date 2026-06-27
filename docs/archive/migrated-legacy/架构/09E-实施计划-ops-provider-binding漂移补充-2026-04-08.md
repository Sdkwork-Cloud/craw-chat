# 09E 实施计划：ops provider binding漂移补充

## 目标

- 把 `ops-service` 从“看到 provider binding 快照”推进到“看到租户相对全局基线的 drift”。
- 继续坚持 `control-plane` 负责求值、`ops-service` 负责展示，不在 ops 侧维护第二套 provider 选择逻辑。

## 实施范围

1. `OpsRuntime` 基于已镜像的 provider binding 快照计算 drift 视图
2. `ops-service` 新增 `GET /backend/v3/api/ops/provider_bindings/drift`
3. `diagnostic bundle` 携带 `providerBindingDrift`
4. `public auth` 继续要求 `ops.read`

## 最小契约

drift 视图最小包含：

- `baselineTenantId`
- `tenantId`
- `domain`
- `baselineSelectedPluginId`
- `selectedPluginId`
- `baselineSelectionSource`
- `selectionSource`
- `driftKind`

当前规则冻结为：

- 基线只取 `tenantId = null` 的全局快照
- ops 侧不重新做 provider 求值
- 当前 drift 类型只允许：
  - `plugin_changed`
  - `selection_source_changed`
  - `plugin_and_selection_source_changed`

## 实施步骤

1. 先写 `ops-service` runtime 红测，冻结 global baseline 对 tenant snapshot 的 drift 语义
2. 再写 `ops-service` HTTP 红测与 `diagnostics` 红测
3. 再写 public auth 红测，保护 drift 接口
4. 最后补 `control-plane -> OpsRuntime` 联动验证，证明 mirror 后的快照可直接生成 drift

## 验证命令

- `cargo test -p ops-service --offline --test ops_runtime_test -- --nocapture`
- `cargo test -p ops-service --offline --test http_smoke_test -- --nocapture`
- `cargo test -p ops-service --offline --test public_auth_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test governance_loop_test -- --nocapture`

## 收口标准

- `ops-service` 能返回 provider binding drift 结果
- `diagnostics` 中能看到 `providerBindingDrift`
- `plugin_changed / selection_source_changed / plugin_and_selection_source_changed` 三类语义被测试冻结
- public app 访问 drift 接口仍要求 `ops.read`
