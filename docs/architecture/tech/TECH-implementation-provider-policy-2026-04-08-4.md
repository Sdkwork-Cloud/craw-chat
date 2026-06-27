> Migrated from `docs/架构/09J-实施计划-provider-policy预览确认补充-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 09J 实施计划补充: provider-policy 预览确认
## 对应闭环

- `Step 07`
- `07-C7`
- `09J`
- `150J`

## 本轮范围

- 只补 preview 后的确认写入保护。
- 不引入 token、租约或草稿工作区。
- 不改变现有 preview 路径。

## 代码动作

- `crates/im-platform-contracts/src/provider.rs`
  - 新增带 `expectedBaseVersion` 的写入方法
  - 在真实写入前校验当前版本
- `services/control-plane-api/src/lib.rs`
  - `POST /backend/v3/api/control/provider_bindings` 新增 `expectedBaseVersion`
  - 版本漂移时透传 `provider_policy_conflict`

## 契约冻结

- 请求字段:
  - `tenantId`
  - `domain`
  - `pluginId`
  - `expectedBaseVersion`
- 冲突语义:
  - HTTP `409`
  - `code = provider_policy_conflict`
  - `message` 暴露 expected/current 版本

## 质量要求

- 没带 `expectedBaseVersion` 的旧调用保持兼容。
- 带 `expectedBaseVersion` 的调用必须在版本漂移时失败。
- 失败时不得追加 history。
- 失败时不得刷新 ops provider bindings。
- 失败时不得写 audit。

## 验证冻结

- `cargo test -p im-platform-contracts --offline --test provider_registry_contract_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test provider_registry_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test governance_loop_test -- --nocapture`

## 后续建议

- 下一轮进入 `07-C8`，把提交成功后的版本与提交结果回显标准化，减少调用方二次读取成本。

