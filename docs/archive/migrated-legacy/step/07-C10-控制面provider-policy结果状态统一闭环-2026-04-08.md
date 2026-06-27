# Step 07-C10: 控制面 provider-policy 结果状态统一闭环
## 当前闭环编号

- 所属 step: `Step 07`
- 当前波次: `07-C10 / CP07-10A`
- 目标: 为成功路径补统一 `status`，用稳定枚举表达 `preview / applied / noop`，减少调用方按多个字段组合推断状态。

## 本轮实现

- `crates/im-platform-contracts/src/provider.rs`
  - 新增 `ProviderPolicyResultStatus`
  - `ProviderPolicyPreview` 新增 `status=preview`
  - `ProviderPolicyCommit` 新增 `status=applied|noop`
- `services/control-plane-api/src/lib.rs`
  - `POST /backend/v3/api/control/provider_bindings` 成功回包新增 `status`
  - 继续保留 `applied` 布尔字段，兼容现有调用方
- 冲突路径保持不变:
  - `POST /backend/v3/api/control/provider_bindings`
  - `409`
  - `provider_policy_conflict`

## 接口冻结

- `POST /backend/v3/api/control/provider-policies/preview`
  - `status=preview`
  - `baseVersion`
  - `previewVersion`
  - `previewBinding`
- `POST /backend/v3/api/control/provider_bindings`
  - `status=applied|noop`
  - `applied`
  - `currentVersion`
  - `committedBinding`
  - `diff`

## 验证

- `cargo test -p im-platform-contracts --offline --test provider_registry_contract_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test provider_registry_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test governance_loop_test -- --nocapture`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test provider_plugin_docs_test -- --nocapture`

## 文档同步

- `docs/step/07-C10-控制面provider-policy结果状态统一闭环-2026-04-08.md`
- `docs/架构/09M-实施计划-provider-policy结果状态补充-2026-04-08.md`
- `docs/架构/150M-control-plane-provider-policy结果状态设计-2026-04-08.md`
- `docs/review/continuous-optimization-control-plane-provider-policy-status-2026-04-08.md`

## 下一缺口

- 下一轮可进入 `07-C11`，评估是否需要继续统一 error / rollback / history 的结果表达；如果不做扩展，也应明确成功路径统一到 `status` 即为冻结边界。
