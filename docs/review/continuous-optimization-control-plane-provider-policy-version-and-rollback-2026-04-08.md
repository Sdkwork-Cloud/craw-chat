# Continuous Optimization: control-plane provider-policy version and rollback

## 本轮结论

- `07-C4 / 09G / 150G` 已闭环。
- provider policy 现在具备“写入、历史查询、版本回滚、ops 同步、审计留痕”完整链路。

## 已完成

- `RuntimeProviderRegistry` 增加历史与回滚快照能力。
- control-plane 暴露 `GET /backend/v3/api/control/provider-policies`。
- control-plane 暴露 `POST /backend/v3/api/control/provider-policies/rollback`。
- 回滚快照输出 `rollbackFromVersion`。
- ops 在回滚后通过 `replace_provider_binding_snapshots` 清理旧租户快照。
- audit 新增 `control.provider_policy_rolled_back`。

## 验证

- `cargo test -p im-platform-contracts --offline --test provider_registry_contract_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test provider_registry_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test governance_loop_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test public_auth_test -- --nocapture`

## 风险与下一步

- 当前回滚是“按版本恢复全量快照”，后续可继续补差异视图与回滚预览。
- 若进入下一轮，优先考虑 provider policy diff / preview，而不是重复扩展写接口。
