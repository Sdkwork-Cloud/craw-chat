# Step 07-C8: 控制面 provider-policy 提交结果回显闭环
## 当前闭环编号

- 所属 step: `Step 07`
- 当前波次: `07-C8 / CP07-8A`
- 目标: 让 `POST /backend/v3/api/control/provider_bindings` 在成功提交后直接回显 `currentVersion / committedBinding / diff`，补齐 preview -> confirm -> committed 链路。

## 本轮实现

- `RuntimeProviderRegistry` 新增 `commit_upsert(...)`。
- 新增 `ProviderPolicyCommit`，固定返回：
  - `currentVersion`
  - `tenantId`
  - `committedBinding`
  - `diff`
- `POST /backend/v3/api/control/provider_bindings` 成功响应新增提交结果字段，同时保留：
  - `interfaceVersion`
  - `tenantId`
  - `effectiveBindings`
  - `precedence`
- `diff` 直接回显本次真实提交产生的版本差异。

## 接口冻结

- 路径: `POST /backend/v3/api/control/provider_bindings`
- 权限: `control.write`
- Success Response:
  - `currentVersion`
  - `committedBinding`
  - `diff`
  - `effectiveBindings`
  - `precedence`

## 验证

- `cargo test -p im-platform-contracts --offline --test provider_registry_contract_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test provider_registry_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test governance_loop_test -- --nocapture`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test provider_plugin_docs_test -- --nocapture`

## 文档同步

- `docs/step/07-C8-控制面provider-policy提交结果回显闭环-2026-04-08.md`
- `docs/架构/09K-实施计划-provider-policy提交结果补充-2026-04-08.md`
- `docs/架构/150K-control-plane-provider-policy提交结果设计-2026-04-08.md`
- `docs/review/continuous-optimization-control-plane-provider-policy-commit-response-2026-04-08.md`

## 下一缺口

- 下一轮优先进入 `07-C9`，对 provider-policy 相同值重复提交做 no-op 抑制或显式 no-op 语义，避免无实际变化却继续增加版本。
