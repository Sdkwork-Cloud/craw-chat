# 09H 实施计划补充: provider-policy 差异查询
## 对应闭环

- `Step 07`
- `07-C5`
- `150H`

## 本轮落点

- 在 `07-C4` 的 `GET /backend/v3/api/control/provider-policies` 与 `POST /backend/v3/api/control/provider-policies/rollback` 基础上，新增版本间差异读取面。
- 范围严格限定为 committed version diff，不扩展 preview、草稿态、批量回滚。

## 代码动作

- `crates/im-platform-contracts/src/provider.rs`
  - 新增 `ProviderPolicyDiff`
  - 新增 `ProviderPolicyChangeKind`
  - 新增 `diff_versions(fromVersion, toVersion)`
- `services/control-plane-api/src/lib.rs`
  - 新增 `GET /backend/v3/api/control/provider-policies/diff`
  - 新增 `ProviderPolicyDiffQuery`
  - 接口继续复用 `control.read`

## 契约冻结

- Query 参数: `fromVersion`、`toVersion`
- 响应字段:
  - `fromRecordedAt`
  - `toRecordedAt`
  - `deploymentProfileChanges`
  - `tenantOverrideChanges`
  - `changeKind`

## 验证冻结

- `cargo test -p im-platform-contracts --offline --test provider_registry_contract_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test provider_registry_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test public_auth_test -- --nocapture`

## 后续建议

- 下一轮进入 `provider policy preview`，让写接口在提交前先生成 diff 预览，而不是重复扩写新的写接口。
