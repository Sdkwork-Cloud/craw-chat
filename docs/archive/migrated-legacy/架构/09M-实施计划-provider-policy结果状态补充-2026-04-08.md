# 09M 实施计划补充: provider-policy 结果状态
## 对应闭环

- `Step 07`
- `07-C10`
- `09M`
- `150M`

## 本轮范围

- 只统一成功路径状态表达。
- 覆盖 `POST /backend/v3/api/control/provider-policies/preview` 与 `POST /backend/v3/api/control/provider_bindings`。
- 保持 `409 + provider_policy_conflict` 冲突语义不变。

## 代码动作

- `crates/im-platform-contracts/src/provider.rs`
  - 新增 `ProviderPolicyResultStatus`
  - `ProviderPolicyPreview.status=preview`
  - `ProviderPolicyCommit.status=applied|noop`
- `services/control-plane-api/src/lib.rs`
  - `ProviderBindingCommitResponse` 新增 `status`
  - `status` 直接透传 commit 结果，避免 HTTP 层重复推断

## 契约冻结

- `POST /backend/v3/api/control/provider-policies/preview`
  - `status`
  - `preview`
- `POST /backend/v3/api/control/provider_bindings`
  - `status`
  - `applied`
  - `noop`
  - `applied` 布尔字段继续保留

## 质量要求

- `status` 必须稳定输出，不能让调用方再依赖路由名或字段组合猜测结果。
- preview 不得触发 ops / audit 副作用。
- noop 继续保持不推进版本、不追加 history、不写 audit。
- applied 继续保持真实写入与现有回包字段不变。

## 验证冻结

- `cargo test -p im-platform-contracts --offline --test provider_registry_contract_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test provider_registry_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test governance_loop_test -- --nocapture`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test provider_plugin_docs_test -- --nocapture`

## 后续建议

- 后续若继续扩展，优先评估是否为错误路径补统一结果分类；若不扩展，则以 `ProviderPolicyResultStatus` 作为成功路径标准。
