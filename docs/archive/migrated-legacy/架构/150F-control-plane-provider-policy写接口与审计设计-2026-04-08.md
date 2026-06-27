# 150F control-plane provider-policy写接口与审计设计

## 设计目标

- 给 control-plane 增加最小可写 provider policy 能力。
- 保持 provider 求值真相仍在同一个 registry owner，不在 control-plane、ops、runtime 各自复制选择逻辑。
- 让 provider policy 写操作天然具备 ops 镜像和 audit 证据。

## 核心模型

当前最小 owner 为 `RuntimeProviderRegistry`，职责固定为：

- 保存插件矩阵与全局默认
- 保存 `deployment_profile`
- 保存 `tenant_override`
- 基于同一份状态计算 `effectiveBinding`

这意味着写接口不直接操作 HTTP 层临时对象，而是写入 registry owner。

## 控制面写接口

当前最小写接口固定为：

- `POST /backend/v3/api/control/provider_bindings`

请求最小字段：

- `tenantId`
- `domain`
- `pluginId`

当前语义：

- `tenantId = null`：更新 `deployment_profile`
- `tenantId != null`：更新 `tenant_override`

响应仍返回同一份：

- `interfaceVersion`
- `tenantId`
- `effectiveBindings`
- `precedence`

## 校验规则

- `pluginId` 必须存在
- `pluginId` 必须属于请求的 `domain`
- 若插件不允许 tenant override，则不能写租户级策略

当前最小坏例子是：

- `domain = rtc`
- `pluginId = object-storage-aws`

该请求必须被拒绝。

## Side Effect

写成功后必须同时触发：

1. control-plane 返回更新后的 `effectiveBindings`
2. 同一份结果镜像到 `OpsRuntime`
3. 记录 audit anchor

当前最小 audit 动作冻结为：

- `control.provider_deployment_profile_updated`
- `control.provider_tenant_override_updated`

## 当前非目标

- 不在本轮实现 provider policy 持久化存储
- 不在本轮实现配置版本与 rollback snapshot
- 不在本轮实现 override 删除接口
- 不在本轮把 drift 直接接进告警规则引擎

## 后续演进

- 引入配置版本和回滚快照
- 引入 tenant override 清理/回退接口
- 把 drift + audit + rollback 串成完整 provider 治理编排
