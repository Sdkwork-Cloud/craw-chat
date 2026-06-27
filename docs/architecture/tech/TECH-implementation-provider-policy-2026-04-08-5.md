> Migrated from `docs/架构/09K-实施计划-provider-policy提交结果补充-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 09K 实施计划补充: provider-policy 提交结果
## 对应闭环

- `Step 07`
- `07-C8`
- `09K`
- `150K`

## 本轮范围

- 只补成功写入后的提交结果回显。
- 不改变 GET `provider-bindings` 的只读快照语义。
- 不处理 no-op 抑制。

## 代码动作

- `crates/im-platform-contracts/src/provider.rs`
  - 新增 `ProviderPolicyCommit`
  - 新增 `commit_upsert(...)`
- `services/control-plane-api/src/lib.rs`
  - `POST /backend/v3/api/control/provider_bindings` 改为返回 committed 结果
  - 保留原有 `effectiveBindings / precedence`

## 契约冻结

- Success Response:
  - `currentVersion`
  - `committedBinding`
  - `diff`
  - `effectiveBindings`
  - `precedence`

## 质量要求

- `diff.toVersion` 必须等于 `currentVersion`。
- `committedBinding` 必须与本次真实提交后的目标 binding 一致。
- 成功回显不得通过额外非原子查询拼装。
- preview confirm 成功路径必须能直接闭合到 committed 回包。

## 验证冻结

- `cargo test -p im-platform-contracts --offline --test provider_registry_contract_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test provider_registry_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test governance_loop_test -- --nocapture`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test provider_plugin_docs_test -- --nocapture`

## 后续建议

- 下一轮进入 `07-C9`，补 no-op 提交语义，减少空变更导致的版本噪声。

