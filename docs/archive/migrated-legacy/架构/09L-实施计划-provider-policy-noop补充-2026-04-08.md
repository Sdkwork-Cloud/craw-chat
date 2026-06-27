# 09L 实施计划补充: provider-policy no-op
## 对应闭环

- `Step 07`
- `07-C9`
- `09L`
- `150L`

## 本轮范围

- 只处理相同 policy state 的重复提交。
- 不改变冲突语义。
- 不处理 preview token 或批量事务。

## 代码动作

- `crates/im-platform-contracts/src/provider.rs`
  - `ProviderPolicyCommit` 新增 `applied`
  - `commit_upsert(...)` 对相同 state 做 no-op 抑制
- `services/control-plane-api/src/lib.rs`
  - POST 成功回包新增 `applied`
  - no-op 时跳过 ops / audit

## 契约冻结

- Success Response:
  - `applied`
  - `currentVersion`
  - `committedBinding`
  - `diff`
- no-op 语义:
  - `applied=false`
  - `currentVersion` 不前进
  - `diff` 为空

## 质量要求

- 相同值重复提交不得新增版本。
- no-op 不得追加 history。
- no-op 不得写 audit。
- no-op 不得刷新 ops provider bindings。
- 旧的冲突校验保持不变。

## 验证冻结

- `cargo test -p im-platform-contracts --offline --test provider_registry_contract_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test provider_registry_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test public_auth_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test governance_loop_test -- --nocapture`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test provider_plugin_docs_test -- --nocapture`

## 后续建议

- 下一轮进入 `07-C10`，统一 preview / conflict / noop / applied 的返回状态表达。
