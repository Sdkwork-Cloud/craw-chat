> Migrated from `docs/step/07-C5-控制面provider-policy差异查询闭环-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Step 07-C5: 控制面 provider-policy 差异查询闭环
## 当前闭环编号

- 所属 step: `Step 07`
- 当前波次: `07-C5 / CP07-5A`
- 目标: 在 `07-C4` 已有版本快照与回滚能力之上，补齐 committed version 之间的只读差异查询。

## 本轮实现

- `RuntimeProviderRegistry` 新增 `ProviderPolicyDiff`、`ProviderPolicyChange`、`TenantProviderPolicyChange`、`ProviderPolicyChangeKind`。
- 新增 `diff_versions(fromVersion, toVersion)`，以已提交快照为唯一真源计算差异。
- 差异分为 `deploymentProfileChanges` 与 `tenantOverrideChanges` 两组。
- `changeKind` 固定为 `added / removed / changed`，不回传未变化项。
- control-plane 新增 `GET /backend/v3/api/control/provider-policies/diff`。
- 新接口仅要求 `control.read`，保持只读，不追加 audit 写入，不触发 ops 刷新，也不引入 preview 写路径。

## 接口冻结

- 路径: `GET /backend/v3/api/control/provider-policies/diff?fromVersion=2&toVersion=4`
- Query: `fromVersion`、`toVersion`
- Response:
  - `fromVersion` / `toVersion`
  - `fromRecordedAt` / `toRecordedAt`
  - `deploymentProfileChanges`
  - `tenantOverrideChanges`
  - `changeKind`

## 验证

- `cargo test -p im-platform-contracts --offline --test provider_registry_contract_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test provider_registry_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test public_auth_test -- --nocapture`

## 文档同步

- `docs/step/07-C5-控制面provider-policy差异查询闭环-2026-04-08.md`
- `docs/架构/09H-实施计划-provider-policy差异查询补充-2026-04-08.md`
- `docs/架构/150H-control-plane-provider-policy差异查询设计-2026-04-08.md`
- `docs/review/continuous-optimization-control-plane-provider-policy-diff-2026-04-08.md`

## 下一缺口

- 下一轮优先进入 `07-C6`，基于同一快照体系补 `provider policy preview`，让控制面在提交前预览变更影响。

