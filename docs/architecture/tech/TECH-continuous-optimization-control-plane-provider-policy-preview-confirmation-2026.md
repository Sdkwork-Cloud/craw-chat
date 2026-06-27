> Migrated from `docs/review/continuous-optimization-control-plane-provider-policy-preview-confirmation-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization: control-plane provider-policy preview confirmation

## 本轮交付

- 新增 `07-C7 / 09J / 150J` 文档闭环。
- `RuntimeProviderRegistry` 支持基于 `expectedBaseVersion` 的确认写入。
- `POST /backend/v3/api/control/provider_bindings` 新增可选 `expectedBaseVersion`。
- 当真实版本漂移时，接口返回 `provider_policy_conflict`。
- governance 回归已证明冲突失败不会刷新 ops，也不会写 audit。

## 验证

- `cargo test -p im-platform-contracts --offline --test provider_registry_contract_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test provider_registry_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test governance_loop_test -- --nocapture`

## 评价

- 这一轮把 `07-C6` 的 preview 从“能看”推进到“能按预览版本安全提交”。
- 冲突语义直接绑定真实 `currentVersion`，能清楚解释 preview 过期的原因。
- 同时保持克制，没有为此引入新的 token、租约或草稿态。

## 下一步

- 下一轮优先补提交成功后的版本与结果回显，减少控制台在 confirm 成功后再次读取 history/binding 的额外往返。

