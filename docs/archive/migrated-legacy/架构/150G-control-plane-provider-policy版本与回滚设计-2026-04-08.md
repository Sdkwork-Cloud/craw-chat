# 150G control-plane provider-policy 版本与回滚设计

## 设计目标

- provider policy 不仅要能写，还要可追溯、可回滚、可审计。
- 历史必须保留完整快照链，回滚必须生成新的当前版本，不能原地篡改旧记录。
- ops 视图必须反映真实当前状态，尤其是租户 override 被回滚删除时不能残留旧快照。

## 核心模型

- `RuntimeProviderRegistry`
  - 保存 `deployment_profiles`
  - 保存 `tenant_overrides`
  - 保存历史快照 `ProviderPolicyHistory`
- `ProviderPolicySnapshot`
  - `version`
  - `recordedAt`
  - `rollbackFromVersion`
  - `deploymentProfiles`
  - `tenantOverrides`

## HTTP 面

- `GET /backend/v3/api/control/provider-policies`
  - 权限：`control.read`
  - 返回：`ProviderPolicyHistory`
- `POST /backend/v3/api/control/provider-policies/rollback`
  - 权限：`control.write`
  - 请求：`targetVersion`
  - 返回：最新 `ProviderPolicyHistory`

## 回滚语义

- 回滚到历史版本后，当前状态恢复为目标快照中的 provider 选择。
- 回滚动作会写入一个新快照，版本继续递增。
- 新快照通过 `rollbackFromVersion` 标识来源版本。
- 若 `targetVersion` 不存在，返回冲突错误。

## ops 与审计联动

- 写入或回滚后，control-plane 需要把当前 global snapshot 与所有 tenant override snapshot 一次性下发到 ops。
- 删除租户 override 的场景必须使用 `replace_provider_binding_snapshots`，不能只做增量更新。
- 审计动作新增 `control.provider_policy_rolled_back`，并记录：
  - `targetVersion`
  - `currentVersion`
  - `rollbackFromVersion`

## 冻结点

- `07-C4`
- `09G`
- `150G`
- `GET /backend/v3/api/control/provider-policies`
- `POST /backend/v3/api/control/provider-policies/rollback`
- `rollbackFromVersion`
- `control.provider_policy_rolled_back`
