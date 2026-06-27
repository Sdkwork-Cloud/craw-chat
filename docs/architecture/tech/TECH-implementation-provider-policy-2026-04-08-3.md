> Migrated from `docs/架构/09I-实施计划-provider-policy预览补充-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 09I 实施计划补充: provider-policy 预览
## 对应闭环

- `Step 07`
- `07-C6`
- `150I`

## 本轮范围

- 只补“写前预览”。
- 不扩展 rollback preview。
- 不引入草稿态或批量事务。

## 代码动作

- `crates/im-platform-contracts/src/provider.rs`
  - 新增 `ProviderPolicyPreview`
  - 新增 `preview_upsert(...)`
- `services/control-plane-api/src/lib.rs`
  - 新增 `POST /backend/v3/api/control/provider-policies/preview`
  - 直接复用现有写请求体

## 契约冻结

- 权限: `control.write`
- 响应字段:
  - `baseVersion`
  - `previewVersion`
  - `tenantId`
  - `previewBinding`
  - `diff`

## 质量要求

- preview 必须无副作用。
- preview 不得写 audit。
- preview 不得刷新 ops provider bindings。
- preview 后再次读取 `provider-policies` 时，真实 `currentVersion` 不变。

## 验证冻结

- `cargo test -p im-platform-contracts --offline --test provider_registry_contract_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test provider_registry_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test public_auth_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test governance_loop_test -- --nocapture`

## 后续建议

- 下一轮进入 `expectedBaseVersion` / preview confirm，形成 preview 到真实写入之间的并发保护链路。

