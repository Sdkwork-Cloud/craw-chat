# Continuous Optimization: control-plane provider-policy preview

## 本轮交付

- 新增 `07-C6 / 09I / 150I` 文档闭环。
- `RuntimeProviderRegistry` 支持 `preview_upsert(...)`。
- control-plane 新增 `POST /backend/v3/api/control/provider-policies/preview`。
- 新响应固定为 `baseVersion`、`previewVersion`、`previewBinding`、`diff`。
- preview 权限固定为 `control.write`。
- governance 回归已证明 preview 无副作用，不触碰 ops 与 audit。

## 验证

- `cargo test -p im-platform-contracts --offline --test provider_registry_contract_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test provider_registry_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test public_auth_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test governance_loop_test -- --nocapture`

## 评价

- 这一轮把 `07-C5` 的“能比较已提交版本”推进到“能预览即将提交的变更”。
- 预览与真实写路径共用同一套校验，避免控制台自己拼规则。
- 同时保持克制，没有在 preview 阶段引入新的持久化状态。

## 下一步

- 下一轮优先补 `expectedBaseVersion` 或 preview confirmation，让 preview 到真实写入之间具备并发保护。
