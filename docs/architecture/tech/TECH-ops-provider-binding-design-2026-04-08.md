> Migrated from `docs/架构/150D-ops-provider-binding运行态消费与漂移视图设计-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 150D ops-provider-binding运行态消费与漂移视图设计

## 设计目标

- 让 `ops-service` 直接消费控制面已求值的 provider binding 结果。
- 避免 ops 侧为 RTC、对象存储、用户模块、设备接入、IoT 协议再维护一套平行求值逻辑。
- 为后续 `drift / rollback / rollout` 提供统一只读视图基线。

## 消费链路

当前最小链路固定为：

1. `control-plane-api` 计算 `effective provider bindings`
2. 结果镜像写入 `OpsRuntime`
3. `ops-service` 通过只读接口和 diagnostics 暴露同一份快照

当前接口：

- `GET /backend/v3/api/control/provider_bindings`
- `GET /backend/v3/api/ops/provider_bindings`

## ops 侧对象

`ops-service` 维护三层对象：

- `ProviderBindingItemView`
- `ProviderBindingSnapshotView`
- `ProviderBindingsView`

用途：

- item：单个 domain 的 binding 结果
- snapshot：单次求值结果，可带 `tenantId`
- collection：ops 当前持有的全部快照集合

## 当前语义

- global snapshot 与 tenant snapshot 并存
- `tenantId = null` 表示全局视角
- 其他 `tenantId` 表示租户视角
- 当前只做快照镜像，不做差异计算

这意味着 ops 现在能回答：

- 当前默认 RTC provider 是谁
- 当前对象存储 deployment profile 指向谁
- 指定租户是否命中了 provider override

## 诊断面

`GET /backend/v3/api/ops/diagnostics` 必须同时带出 `providerBindings`。

原因：

- provider 绑定已经属于治理结果的一部分
- 诊断包不能只展示 cluster / lag / projection，而看不到 provider 治理结果

## 安全边界

- `GET /backend/v3/api/ops/provider_bindings` 必须要求 `ops.read`
- public bearer 没有 `ops.read` 时必须拒绝访问
- ops 侧不可直接改写 provider 绑定结果

## 后续演进

- 增加 provider binding drift 计算
- 把 drift 结果接入告警和运维决策
- 将 provider policy 版本、审计 actor 与回滚快照接入该视图

