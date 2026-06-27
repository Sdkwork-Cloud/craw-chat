> Migrated from `docs/review/continuous-optimization-control-plane-provider-policy-error-status-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization: control-plane provider-policy error status

## 本轮交付

- 新增 `07-C12 / 09O / 150O` 文档闭环
- `ControlPlaneError` 响应新增 `status`
- `POST /backend/v3/api/control/provider_bindings` 与 `POST /backend/v3/api/control/provider-policies/preview` 的失败响应现在都稳定输出 `status + code + message`
- provider-policy 失败路径补齐显式错误分类:
  - `invalid`
  - `conflict`
  - `unavailable`
  - `forbidden`
  - `unauthorized`
- 现有 `code / message / HTTP status` 全部保留

## 验证

- `cargo test -p control-plane-api --offline --test provider_registry_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test public_auth_test -- --nocapture`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test provider_plugin_docs_test -- --nocapture`

## 评价

- 这一轮让 provider-policy 的失败响应也具备显式 `status`
- 调用方现在可以先读 `status` 做稳定分类，再按需读取 `code`
- 典型映射已经冻结:
  - cross-domain plugin id -> `invalid`
  - stale confirm write -> `conflict`
  - runtime 未启用 -> `unavailable`
  - 无权限 -> `forbidden`
  - 缺失鉴权上下文 -> `unauthorized`
- success/error 两侧的消费方式已经明显收敛

## 下一步

- 下一轮评估是否需要进一步统一成单一结果 envelope；若不继续扩展，可把当前 success/error 双侧 `status` 方案作为冻结边界
## 补充校正

- `unknown provider policy version` 已确认为 `provider_policy_conflict`
- provider-policy routes do not emit `status=not_found`
- `GET /backend/v3/api/control/provider-policies/diff` / `POST /backend/v3/api/control/provider-policies/rollback` 的未知版本分支应稳定返回 `status=conflict`

