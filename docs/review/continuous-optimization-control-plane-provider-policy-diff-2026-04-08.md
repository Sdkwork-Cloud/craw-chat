# Continuous Optimization: control-plane provider-policy diff

## 本轮交付

- 新增 `07-C5 / 09H / 150H` 文档闭环。
- `RuntimeProviderRegistry` 支持 `diff_versions(fromVersion, toVersion)`。
- control-plane 新增 `GET /backend/v3/api/control/provider-policies/diff`。
- 新响应冻结 `deploymentProfileChanges`、`tenantOverrideChanges`、`changeKind`。
- public auth 已覆盖新读接口的 `control.read` 权限约束。

## 验证

- `cargo test -p im-platform-contracts --offline --test provider_registry_contract_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test provider_registry_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test public_auth_test -- --nocapture`

## 评价

- 这一轮把 `07-C4` 的“能看历史、能回滚”推进到“能比较两个已提交版本”。
- 设计保持克制，没有重复扩展写接口，也没有把 preview 强行塞进本轮。

## 下一步

- 下一轮优先补 `provider policy preview`，直接复用 `ProviderPolicyDiff` 模型，避免未来控制面、审计、预览各自实现一套比对逻辑。
