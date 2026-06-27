# Continuous Optimization: control-plane provider-policy status

## 本轮交付

- 新增 `07-C10 / 09M / 150M` 文档闭环。
- `ProviderPolicyResultStatus` 冻结成功路径结果状态。
- `POST /backend/v3/api/control/provider-policies/preview` 返回 `status=preview`。
- `POST /backend/v3/api/control/provider_bindings` 返回 `status=applied|noop`。
- 旧的 `applied` 布尔字段继续保留，兼容已有调用方。

## 验证

- `cargo test -p im-platform-contracts --offline --test provider_registry_contract_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test provider_registry_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test governance_loop_test -- --nocapture`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test provider_plugin_docs_test -- --nocapture`

## 评价

- 这一轮把成功路径结果表达从隐式推断改成显式 `status`。
- preview / applied / noop 现在能被统一消费，不必再由调用方自己组合判断。
- 冲突语义保持克制，没有扩展出新的 error envelope。

## 下一步

- 下一轮可评估是否把错误路径也归一成稳定结果分类；如果不继续扩展，应把当前 `status` 方案作为成功路径标准冻结下来。
