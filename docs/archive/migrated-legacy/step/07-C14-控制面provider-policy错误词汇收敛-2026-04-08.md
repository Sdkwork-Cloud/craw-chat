# Step 07-C14: 控制面 provider-policy 错误词汇收敛

## 目标

- 冻结 provider-policy 未知版本错误的真实语义
- 明确 `unknown provider policy version` -> `provider_policy_conflict`
- 明确 provider-policy routes do not emit `status=not_found`
- 同步 `07-C14 / 09Q / 150Q`

## 路径

- `GET /backend/v3/api/control/provider-policies/diff?fromVersion=1&toVersion=9`
- `POST /backend/v3/api/control/provider-policies/rollback`

## 契约

- `unknown provider policy version`
- `status=conflict`
- `code=provider_policy_conflict`
- provider-policy routes do not emit `status=not_found`

## 验证

- `cargo test -p control-plane-api --offline --test provider_registry_test test_control_plane_returns_conflict_status_for_unknown_provider_policy_versions -- --nocapture`
- `cargo test -p sdkwork-im-cloud-gateway --offline --test provider_plugin_docs_test -- --nocapture`
