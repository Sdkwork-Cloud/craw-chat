# 09O 实施计划补充: provider-policy 错误状态
## 对应闭环

- `Step 07`
- `07-C12`
- `09O`
- `150O`

## 本轮范围

- 只统一 provider-policy 相关 error `status`
- 保持现有 `code / message / HTTP status` 兼容
- 不引入新的统一 error envelope 结构

## 代码动作

- `services/control-plane-api/src/lib.rs`
  - 为 `ControlPlaneError` 响应新增 `status`
  - 使用 HTTP 语义分类错误:
    - `unauthorized`
    - `forbidden`
    - `invalid`
    - `conflict`
    - `unavailable`
    - `not_found`

## 契约冻结

- provider-policy 失败响应统一具备:
  - `status`
  - `code`
  - `message`
- 关键分类:
  - `invalid`
  - `conflict`
  - `unavailable`
  - `forbidden`
  - `unauthorized`

## 质量要求

- cross-domain provider plugin id 必须稳定返回 `status=invalid`
- stale confirm write 必须稳定返回 `status=conflict`
- provider-policy runtime 未启用必须稳定返回 `status=unavailable`
- 无权限访问 provider-policy 路由必须稳定返回 `status=forbidden`
- 缺失合法鉴权必须稳定返回 `status=unauthorized`

## 验证冻结

- `cargo test -p control-plane-api --offline --test provider_registry_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test public_auth_test -- --nocapture`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test provider_plugin_docs_test -- --nocapture`

## 后续建议

- 下一轮评估是否需要把 success/error 继续收口到同一 envelope；如果不继续扩展，应将当前 `status + code + message` 作为错误路径标准冻结。
## 补充校正

- provider-policy routes do not emit `status=not_found`
- `unknown provider policy version` 必须继续归类到 `provider_policy_conflict`
- 对 `GET /backend/v3/api/control/provider-policies/diff` 与 `POST /backend/v3/api/control/provider-policies/rollback`，未知版本统一返回 `status=conflict`
