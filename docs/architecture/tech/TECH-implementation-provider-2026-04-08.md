> Migrated from `docs/架构/09P-实施计划-provider快照状态补充-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# 09P 实施计划补充: provider 快照状态
## 对应闭环

- `Step 07`
- `07-C13`
- `09P`
- `150P`

## 本轮范围

- 只处理 provider control-plane 的快照读面
- 覆盖:
  - `GET /backend/v3/api/control/provider-registry`
  - `GET /backend/v3/api/control/provider_bindings`
- 不改动 provider-policy 提交、预览、历史、diff、回滚与错误状态

## 代码动作

- `services/control-plane-api/src/lib.rs`
  - provider registry 读取响应新增 `status=registry`
  - provider bindings GET 响应新增 `status=bindings`

## 契约冻结

- provider snapshot 读面统一具备:
  - `status`
  - 业务数据平铺字段
- 冻结状态值:
  - `registry`
  - `bindings`

## 质量要求

- registry/bindings 的原有字段不得丢失
- `provider-bindings` 的 GET 与 POST 响应状态语义必须清晰区分
- ops provider bindings 同步逻辑不得受影响

## 验证冻结

- `cargo test -p control-plane-api --offline --test provider_registry_test -- --nocapture`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test provider_plugin_docs_test -- --nocapture`

## 后续建议

- 下一轮优先决定是否继续统一 envelope；如果不继续扩展，应把当前 status 方案作为 provider control-plane 边界冻结。

