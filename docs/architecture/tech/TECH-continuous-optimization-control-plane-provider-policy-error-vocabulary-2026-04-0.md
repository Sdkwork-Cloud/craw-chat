> Migrated from `docs/review/continuous-optimization-control-plane-provider-policy-error-vocabulary-2026-04-08.md` on 2026-06-24.
> Owner: SDKWork maintainers

# Continuous Optimization: control-plane provider-policy error vocabulary

## 结论

- 新增 `07-C14 / 09Q / 150Q`
- `unknown provider policy version` 已冻结为 `provider_policy_conflict`
- 未知版本路由返回 `status=conflict`
- provider-policy routes do not emit `status=not_found`

## 覆盖路径

- `GET /backend/v3/api/control/provider-policies/diff`
- `POST /backend/v3/api/control/provider-policies/rollback`

## 验证

- `cargo test -p control-plane-api --offline --test provider_registry_test test_control_plane_returns_conflict_status_for_unknown_provider_policy_versions -- --nocapture`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test provider_plugin_docs_test -- --nocapture`

