# Continuous Optimization: control-plane provider-policy commit response

## 本轮交付

- 新增 `07-C8 / 09K / 150K` 文档闭环。
- `RuntimeProviderRegistry` 支持 `commit_upsert(...)` 原子返回 committed 结果。
- `POST /backend/v3/api/control/provider_bindings` 成功响应新增：
  - `currentVersion`
  - `committedBinding`
  - `diff`
- POST 响应继续保留 `effectiveBindings / precedence`，兼顾提交结果回显与现有控制台兼容。

## 验证

- `cargo test -p im-platform-contracts --offline --test provider_registry_contract_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test provider_registry_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test governance_loop_test -- --nocapture`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test provider_plugin_docs_test -- --nocapture`

## 评价

- 这一轮把 preview -> confirm -> committed 链路在一次成功 POST 中闭合。
- committed 结果不再依赖额外的 history/diff 查询拼装，降低控制面二次往返和并发歧义。
- 设计保持克制，没有顺手扩展到 no-op 语义。

## 下一步

- 下一轮优先补 provider-policy no-op 提交语义，避免重复提交相同 binding 时继续增加版本。
