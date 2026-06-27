# 150H control-plane provider-policy 差异查询设计
## 1. 背景

`07-C4` 已经补齐 provider policy 的版本快照、历史查询与回滚，但控制面仍缺少“任意两个已提交版本之间发生了什么变化”的标准读接口。没有 diff，控制台、审计比对和后续 preview 都只能自己拼快照。

## 2. 目标

- 以 `RuntimeProviderRegistry` 中的历史快照为唯一真源。
- 提供 `GET /backend/v3/api/control/provider-policies/diff` 标准读接口。
- 同时覆盖 `deployment profile` 与 `tenant override` 两类变更。
- 输出稳定、可序列化、可直接给控制面或后续 preview 复用的差异模型。

## 3. 非目标

- 不在本轮引入未提交草稿态。
- 不在本轮引入 preview 写接口。
- 不在本轮追加 audit 事件或 ops 同步，因为本接口是只读查询。

## 4. 差异模型

- `ProviderPolicyDiff`
  - `fromVersion`
  - `toVersion`
  - `fromRecordedAt`
  - `toRecordedAt`
  - `deploymentProfileChanges`
  - `tenantOverrideChanges`
- `ProviderPolicyChange`
  - `domain`
  - `changeKind`
  - `fromPluginId`
  - `toPluginId`
- `TenantProviderPolicyChange`
  - `tenantId`
  - `domain`
  - `changeKind`
  - `fromPluginId`
  - `toPluginId`
- `changeKind`
  - `added`
  - `removed`
  - `changed`

## 5. 计算规则

- 先按 `fromVersion`、`toVersion` 定位两份 `ProviderPolicySnapshot`。
- `deploymentProfileChanges` 按 `domain` 逐项比较。
- `tenantOverrideChanges` 按 `tenantId + domain` 逐项比较。
- 仅输出发生变化的项；未变化项不进入响应。
- 版本不存在时返回 `provider_policy_conflict`。

## 6. HTTP 面

- 路径: `GET /backend/v3/api/control/provider-policies/diff`
- Query: `fromVersion`、`toVersion`
- 权限: `control.read` 或 `control.write`
- 不写审计，不刷新 `replace_provider_binding_snapshots`

## 7. 与后续闭环关系

- `07-C5` 解决“已提交版本之间怎么比”。
- 下一轮 `07-C6` 应基于同一模型做 `preview`，解决“提交前怎么预览”。
