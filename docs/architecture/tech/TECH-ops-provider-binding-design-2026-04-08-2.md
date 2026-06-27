> Migrated from `docs/架构/150E-ops-provider-binding漂移检测与运维视图设计-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 150E ops-provider-binding漂移检测与运维视图设计

## 设计目标

- 在 `ops-service` 上直接展示租户 provider binding 相对全局 baseline 的漂移。
- 让运维面能基于同一份 control-plane 求值结果识别 override、profile 切换和插件偏移。
- 为后续告警、回滚和 provider policy 写治理提供统一只读基线。

## 基线模型

当前 drift 基线固定为：

- `tenantId = null` 的全局 snapshot

比较对象固定为：

- 每个租户 snapshot 中，且全局 snapshot 同样存在的 `domain`

当前阶段不做：

- ops 侧再次求值
- 多基线对比
- profile 级独立漂移矩阵

## 漂移判定

当前只比较两类字段：

- `selectedPluginId`
- `selectionSource`

对应 drift 类型冻结为：

- `plugin_changed`
- `selection_source_changed`
- `plugin_and_selection_source_changed`

当前输出项最小字段为：

- `tenantId`
- `domain`
- `baselineSelectedPluginId`
- `selectedPluginId`
- `baselineSelectionSource`
- `selectionSource`
- `driftKind`

## 运维输出面

当前只读输出面固定为：

- `GET /backend/v3/api/ops/provider_bindings/drift`
- `GET /backend/v3/api/ops/diagnostics` 中的 `providerBindingDrift`

这意味着运维面现在可以直接回答：

- 哪个租户的 RTC provider 已偏离全局默认
- 哪个租户的对象存储 provider 已从 `deployment_profile` 进入 `tenant_override`
- 当前偏移到底是插件变化、来源变化，还是二者同时变化

## 安全边界

- `GET /backend/v3/api/ops/provider_bindings/drift` 必须要求 `ops.read`
- drift 视图只读，不允许在 ops 侧改写 provider binding
- drift 仍以 control-plane 已求值并镜像的 snapshot 为唯一真源

## 当前非目标

- 不在本轮落地 provider policy 写接口
- 不在本轮引入 audit actor / 配置版本 / rollback snapshot
- 不在本轮接入告警规则引擎

## 后续演进

- 把 drift 结果接入告警和运维编排
- 把 provider policy 的写审计、版本和回滚能力接入同一条视图链
- 视需要扩展 profile / region / release channel 维度的 provider drift 检测

