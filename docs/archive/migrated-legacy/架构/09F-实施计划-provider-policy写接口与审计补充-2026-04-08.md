# 09F 实施计划：provider policy写接口与审计补充

## 目标

- 把 provider 治理从“只读快照/漂移”推进到“最小可写 policy”。
- 在不重建第二套求值逻辑的前提下，让 control-plane 写 provider policy 后，读面、ops 镜像和 audit 同步闭环。

## 实施范围

1. `im-platform-contracts` 新增可写的 `RuntimeProviderRegistry`
2. `control-plane-api` 新增 `POST /backend/v3/api/control/provider_bindings`
3. 写入后继续复用 `GET /backend/v3/api/control/provider_bindings` 的返回契约
4. 写入后同步：
   - 镜像到 `OpsRuntime`
   - 记录 audit anchor

## 最小契约

写请求最小字段为：

- `tenantId`
- `domain`
- `pluginId`

当前语义冻结为：

- `tenantId = null`：写 `deployment_profile`
- `tenantId != null`：写 `tenant_override`

当前最小 audit 动作为：

- `control.provider_deployment_profile_updated`
- `control.provider_tenant_override_updated`

## 实施步骤

1. 先写 `RuntimeProviderRegistry` 红测，冻结 policy write 与跨域 plugin 校验
2. 再写 `control-plane-api` 红测，冻结 read-after-write 契约
3. 再写 governance loop 红测，冻结 ops 镜像与 audit side effect
4. 再写 public auth 红测，确认写接口仍受 `control.write` 保护
5. 最后补最小实现，不提前引入版本/回滚

## 验证命令

- `cargo test -p im-platform-contracts --offline --test provider_registry_contract_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test provider_registry_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test governance_loop_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test public_auth_test -- --nocapture`

## 收口标准

- control-plane 能写入 provider policy
- 读接口能直接返回写后的 `effectiveBindings`
- ops-runtime 能消费写后的镜像结果
- audit 中能看到 provider policy 变更锚点
- cross-domain plugin id 会被拒绝
