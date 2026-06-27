# 09N 实施计划补充: provider-policy 历史 diff 回滚状态
## 对应闭环

- `Step 07`
- `07-C11`
- `09N`
- `150N`

## 本轮范围

- 只处理 `history / diff / rollback` 成功响应状态。
- 不改写底层 contract 模型。
- 不扩展 `provider_policy_conflict` 等错误响应。

## 代码动作

- `services/control-plane-api/src/lib.rs`
  - 新增 provider-policy 读取/回滚响应包装层
  - `GET /backend/v3/api/control/provider-policies` 输出 `status=history`
  - `GET /backend/v3/api/control/provider-policies/diff` 输出 `status=diff`
  - `POST /backend/v3/api/control/provider-policies/rollback` 输出 `status=rolled_back`

## 契约冻结

- provider-policy 成功路径状态:
  - `preview`
  - `applied`
  - `noop`
  - `history`
  - `diff`
  - `rolled_back`

## 质量要求

- 新增 `status` 不能破坏既有 `currentVersion / items / fromVersion / toVersion` 字段读取。
- rollback 继续保持 ops refresh 和 audit 行为不变。
- history / diff 继续保持只读语义，不引入副作用。

## 验证冻结

- `cargo test -p control-plane-api --offline --test provider_registry_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test governance_loop_test -- --nocapture`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test provider_plugin_docs_test -- --nocapture`

## 后续建议

- 下一轮优先统一错误路径，避免调用方只在成功场景有显式状态、失败场景仍依赖 `code + http status` 推断。
